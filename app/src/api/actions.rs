use gloo::net::http::Request;
use leptos::*;
use models::{Entity, Id};

use crate::auth::use_token;

use super::{refetch::use_refetch_signal, url::get_url};

pub fn create_submit_entity<T: Entity>(cx: Scope) -> Action<T, Result<T, gloo::net::Error>> {
    let token = use_token(cx);
    let refetch = use_refetch_signal::<T>(cx);
    create_action(cx, move |entity: &T| {
        let entity = entity.clone();
        let token = token.get_untracked().into_inner().unwrap();
        async move {
            let url = get_url::<T>(None, None);
            let req_builder = if entity.get_id().is_empty() {
                Request::post(url.as_str())
            } else {
                Request::put(url.as_str())
            };
            let res = req_builder
                .header("Authorization", format!("Bearer {token}").as_str())
                .json(&entity)
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await;
            refetch.broadcast();
            res
        }
    })
}

pub fn create_delete_entity<T: Entity>(cx: Scope) -> Action<Id<T>, bool> {
    let token = use_token(cx);
    let refetch = use_refetch_signal::<T>(cx);
    create_action(cx, move |id: &Id<T>| {
        let id = id.clone();
        let token = token.get_untracked().into_inner().unwrap();
        async move {
            let res = Request::delete(get_url::<T>(Some(id), None).as_str())
                .header("Authorization", format!("Bearer {token}").as_str())
                .send()
                .await
                .unwrap()
                .ok();
            refetch.broadcast();
            res
        }
    })
}
