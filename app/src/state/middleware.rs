//! In the redux pattern, reducers must be pure functions. This means that they
//! cannot perform side effects. This has several benefits, but it means that we
//! need to do some extra work to perform side effects. This is where middleware
//! comes in. Instead of dispatching an action directly, we process it by a
//! middleware first. The middleware can then perform side effects and dispatch
//! a new action. This new action is then passed to the reducer. Pre- and
//! post-middleware actions have different types, so there is no risk of
//! forgetting to run a middleware outside of the store module.

use std::{collections::HashMap, fmt::Write, marker::PhantomData};

use gloo::net::http::{Method, Request, RequestBuilder};
use leptos::SignalGetUntracked;
use models::{Bookmark, Entity, Id, Page, Settings, Widget};

use crate::auth::{use_token, Token};

use super::{action::Action, State};

pub enum PreMiddlewareAction {
    Reload,
    SubmitPage(Page),
    SubmitWidget(Widget),
    SubmitBookmark(Bookmark),
    DeletePage(Page),
    DeleteWidget(Widget),
    DeleteBookmark(Bookmark),
}

pub async fn process(action: PreMiddlewareAction) -> Action {
    let token = use_token().get_untracked();
    let token = &token;

    match action {
        PreMiddlewareAction::Reload => {
            let settings = fetch_settings(token).await;
            let pages = fetch_all::<Page>(token).await;
            let widgets = {
                let widgets = fetch_all::<Widget>(token).await;
                let mut widgets_by_page = HashMap::<_, Vec<_>>::new();
                for widget in widgets {
                    if let Some(page_widgets) = widgets_by_page.get_mut(&widget.page_id) {
                        page_widgets.push(widget.clone());
                    } else {
                        widgets_by_page.insert(widget.page_id.clone(), vec![widget]);
                    }
                }
                widgets_by_page
            };
            let bookmarks = {
                let bookmarks = fetch_all::<Bookmark>(token).await;
                let mut bookmarks_by_widget = HashMap::<_, Vec<_>>::new();
                for bookmark in bookmarks {
                    if let Some(widgets) = bookmarks_by_widget.get_mut(&bookmark.widget_id) {
                        widgets.push(bookmark.clone());
                    } else {
                        bookmarks_by_widget.insert(bookmark.widget_id.clone(), vec![bookmark]);
                    }
                }
                bookmarks_by_widget
            };
            Action::Overwrite(State {
                settings,
                pages,
                widgets,
                bookmarks,
            })
        }
        PreMiddlewareAction::SubmitPage(page) => {
            let page = submit(page, token).await;
            Action::AddPage(page)
        }
        PreMiddlewareAction::SubmitWidget(widget) => {
            let widget = submit(widget, token).await;
            Action::AddWidget(widget)
        }
        PreMiddlewareAction::SubmitBookmark(bookmark) => {
            let bookmark = submit(bookmark, token).await;
            Action::AddBookmark(bookmark)
        }
        PreMiddlewareAction::DeletePage(page) => {
            delete(&page, token).await;
            Action::RemovePage(page)
        }
        PreMiddlewareAction::DeleteWidget(widget) => {
            delete(&widget, token).await;
            Action::RemoveWidget(widget)
        }
        PreMiddlewareAction::DeleteBookmark(bookmark) => {
            delete(&bookmark, token).await;
            Action::RemoveBookmark(bookmark)
        }
    }
}

async fn submit<T: Entity>(entity: T, token: &Token) -> T {
    RequestBuilder::new(&Url::new().id(entity.get_id()).build())
        .with_token(token)
        .post_or_put_by(entity.get_id())
        .json(&entity)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn delete<T: Entity>(entity: &T, token: &Token) {
    Request::delete(&Url::new().id(entity.get_id()).build())
        .with_token(token)
        .send()
        .await
        .unwrap();
}

async fn fetch_all<T: Entity>(token: &Token) -> Vec<T> {
    Request::get(&Url::<T>::new().build())
        .with_token(token)
        .send()
        .await
        .unwrap()
        .json::<Vec<T>>()
        .await
        .unwrap()
}

async fn fetch_settings(token: &Token) -> Settings {
    Request::get("api/settings")
        .with_token(token)
        .send()
        .await
        .unwrap()
        .json::<Settings>()
        .await
        .unwrap()
}

struct Url<'a, T: Entity> {
    id: Option<&'a Id<T>>,
    parent_id: Option<&'a Id<T::Parent>>,
    _entity: PhantomData<T>,
}

impl<'a, T: Entity> Url<'a, T> {
    fn new() -> Self {
        Self {
            id: None,
            parent_id: None,
            _entity: PhantomData,
        }
    }

    fn id(mut self, id: &'a Id<T>) -> Self {
        self.id = Some(id);
        self
    }

    fn _parent_id(mut self, parent_id: &'a Id<T::Parent>) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    fn build(self) -> String {
        let mut path = format!("api/{}", T::DATA.plural());
        if let Some(id) = self.id {
            write!(path, "/{id}", id = id).unwrap();
        }
        if let Some(parent_id) = self.parent_id {
            write!(
                path,
                "?{parent_id_key}={parent_id}",
                parent_id_key = T::DATA.parent_id(),
                parent_id = parent_id
            )
            .unwrap();
        }
        path
    }
}

trait RequestBuilderExtension {
    fn with_token(self, token: &Token) -> Self;
    fn post_or_put_by<T>(self, id: &Id<T>) -> Self;
}
impl RequestBuilderExtension for RequestBuilder {
    fn with_token(self, token: &Token) -> Self {
        if let Some(bearer) = token.as_ref() {
            self.header("Authorization", &format!("Bearer {bearer}"))
        } else {
            self
        }
    }
    fn post_or_put_by<T>(self, id: &Id<T>) -> Self {
        if id.is_empty() {
            self.method(Method::POST)
        } else {
            self.method(Method::PUT)
        }
    }
}
