use gloo::net::http::Request;
use leptos::*;
use models::{Entity, Id, Settings};
use serde::de::DeserializeOwned;

use crate::auth::{use_token, Token};

use super::{refetch::use_refetch_signal, url::get_url};

async fn fetch<T: DeserializeOwned>(token: Token, url: &str) -> Option<T> {
    let Some(token) = token.into_inner() else {
        return  None;
    };
    Request::get(url)
        .header("Authorization", format!("Bearer {token}").as_str())
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .ok()
}

trait WithRefetchEffect {
    fn with_refetch_effect<R: 'static>(self, cx: Scope) -> Self;
}
impl<S: Clone, T> WithRefetchEffect for Resource<S, T> {
    fn with_refetch_effect<R: 'static>(self, cx: Scope) -> Self {
        let refetch_listener = use_refetch_signal::<R>(cx).listen();
        create_effect(cx, move |prev| {
            refetch_listener.track();
            if prev.is_none() {
                return;
            }
            self.refetch();
        });
        self
    }
}

pub fn create_settings_resource(cx: Scope) -> Resource<Token, Option<Settings>> {
    create_resource(cx, use_token(cx), move |token| async move {
        fetch::<Settings>(token, "api/settings").await
    })
    .with_refetch_effect::<Settings>(cx)
}

async fn fetch_entities<T: Entity>(token: Token, url: &str) -> Vec<T> {
    fetch::<Vec<T>>(token, url).await.unwrap_or_default()
}

pub fn use_entities<T: Entity>(cx: Scope) -> Resource<Token, Vec<T>> {
    create_resource(cx, use_token(cx), move |token| async move {
        fetch_entities::<T>(token, get_url::<T>(None, None).as_str()).await
    })
    .with_refetch_effect::<T>(cx)
}

pub fn use_filtered_entities<T: Entity>(
    cx: Scope,
    parent_id: Id<T::Parent>,
) -> Resource<Token, Vec<T>> {
    create_resource(cx, use_token(cx), move |token| {
        let parent_id = parent_id.clone();
        async move { fetch_entities::<T>(token, get_url::<T>(None, Some(parent_id)).as_str()).await }
    })
    .with_refetch_effect::<T>(cx)
}

pub fn use_entity<T: Entity>(cx: Scope, id: Id<T>) -> Resource<Token, Option<T>> {
    create_resource(cx, use_token(cx), move |token| {
        let id = id.clone();
        async move { fetch::<T>(token, get_url::<T>(Some(id), None).as_str()).await }
    })
    .with_refetch_effect::<T>(cx)
}
