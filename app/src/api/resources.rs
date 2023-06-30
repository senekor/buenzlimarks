use gloo::net::http::Request;
use leptos::*;
use models::{Entity, Id, Settings};
use serde::de::DeserializeOwned;

use crate::auth::{Token, use_token};

use super::{
    refetch::{use_refetch_entities, use_refetch_settings},
    url::get_url,
};

async fn fetch<T: DeserializeOwned>(token: Option<Token>, url: &str) -> Option<T> {
    let Some(token) = token else {
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

type Trigger = (Option<Token>, ());

pub fn create_settings_resource(cx: Scope) -> Resource<Trigger, Option<Settings>> {
    let token = use_token(cx);
    let refetch_listener = use_refetch_settings(cx).listen();
    create_resource(
        cx,
        move || (token(), refetch_listener()),
        move |(token, _)| async move { fetch::<Settings>(token, "api/settings").await },
    )
}

async fn fetch_entities<T: Entity>(token: Option<Token>, url: &str) -> Vec<T> {
    fetch::<Vec<T>>(token, url).await.unwrap_or_default()
}

pub fn use_entities<T: Entity>(cx: Scope) -> Resource<Trigger, Vec<T>> {
    let token = use_token(cx);
    let refetch_listener = use_refetch_entities::<T>(cx).listen();
    create_resource(
        cx,
        move || (token(), refetch_listener()),
        move |(token, _)| async move {
            fetch_entities::<T>(token, get_url::<T>(None, None).as_str()).await
        },
    )
}

pub fn use_filtered_entities<T: Entity>(
    cx: Scope,
    parent_id: Id<T::Parent>,
) -> Resource<Trigger, Vec<T>> {
    let token = use_token(cx);
    let refetch_listener = use_refetch_entities::<T>(cx).listen();
    create_resource(
        cx,
        move || (token(), refetch_listener()),
        move |(token, _)| {
            let parent_id = parent_id.clone();
            async move { fetch_entities::<T>(token, get_url::<T>(None, Some(parent_id)).as_str()).await }
        },
    )
}

pub fn use_entity<T: Entity>(cx: Scope, id: Id<T>) -> Resource<Trigger, Option<T>> {
    let token = use_token(cx);
    let refetch_listener = use_refetch_entities::<T>(cx).listen();
    create_resource(
        cx,
        move || (token(), refetch_listener()),
        move |(token, _)| {
            let id = id.clone();
            async move { fetch::<T>(token, get_url::<T>(Some(id), None).as_str()).await }
        },
    )
}
