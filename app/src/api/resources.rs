use gloo::net::http::Request;
use leptos::*;
use models::{Entity, Id, Settings};
use serde::de::DeserializeOwned;

use crate::auth::{use_token, Token};

use super::{refetch::use_refetch_signal, url::get_url};

#[derive(Debug, Clone)]
pub enum FetchError {
    Custom(&'static str),
    JsError(String),
    SerdeError(String),
    GlooError(String),
}
impl From<&'static str> for FetchError {
    fn from(value: &'static str) -> Self {
        Self::Custom(value)
    }
}
impl From<gloo::net::Error> for FetchError {
    fn from(value: gloo::net::Error) -> Self {
        use gloo::net::Error::*;
        match value {
            JsError(s) => Self::JsError(s.to_string()),
            SerdeError(s) => Self::SerdeError(s.to_string()),
            GlooError(s) => Self::GlooError(s),
        }
    }
}
type FetchResult<T> = Result<T, FetchError>;

async fn fetch<T: DeserializeOwned>(token: Token, url: &str) -> FetchResult<T> {
    let Some(token) = token.into_inner() else {
        return Err("missing token".into());
    };
    let res = Request::get(url)
        .header("Authorization", format!("Bearer {token}").as_str())
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(res)
}

trait WithRefetchEffect {
    fn with_refetch_effect<R: 'static>(self) -> Self;
}
impl<S: Clone, T> WithRefetchEffect for Resource<S, T> {
    fn with_refetch_effect<R: 'static>(self) -> Self {
        let refetch_listener = use_refetch_signal::<R>().listen();
        create_effect(move |prev| {
            refetch_listener.track();
            if prev.is_none() {
                return;
            }
            self.refetch();
        });
        self
    }
}

pub fn create_settings_resource() -> Resource<Token, FetchResult<Settings>> {
    create_local_resource(use_token(), move |token| async move {
        fetch::<Settings>(token, "api/settings").await
    })
    .with_refetch_effect::<Settings>()
}

async fn fetch_entities<T: Entity>(token: Token, url: &str) -> Vec<T> {
    fetch::<Vec<T>>(token, url).await.unwrap_or_default()
}

pub fn use_entities<T: Entity>() -> Resource<Token, Vec<T>> {
    create_local_resource(use_token(), move |token| async move {
        fetch_entities::<T>(token, get_url::<T>(None, None).as_str()).await
    })
    .with_refetch_effect::<T>()
}

pub fn use_filtered_entities<T: Entity>(parent_id: Id<T::Parent>) -> Resource<Token, Vec<T>> {
    create_local_resource( use_token(), move |token| {
        let parent_id = parent_id.clone();
        async move { fetch_entities::<T>(token, get_url::<T>(None, Some(parent_id)).as_str()).await }
    })
    .with_refetch_effect::<T>()
}

fn use_entity_resource<T: Entity>(id: Id<T>) -> Resource<Token, Option<T>> {
    create_local_resource(use_token(), move |token| {
        let id = id.clone();
        async move {
            fetch::<T>(token, get_url::<T>(Some(id), None).as_str())
                .await
                .ok()
        }
    })
    .with_refetch_effect::<T>()
}

pub fn use_entity<T: Entity>(entity: T) -> Memo<T> {
    let orig_entity = store_value(entity);
    let id = Signal::derive(move || orig_entity.with_value(|w| w.get_id().clone()));

    let entity_resource = use_entity_resource::<T>(id.get_untracked());
    create_memo(move |_| match entity_resource.read().flatten() {
        Some(entity) => entity,
        None => orig_entity(),
    })
}
