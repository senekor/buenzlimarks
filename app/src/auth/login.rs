use leptos::*;

use crate::{auth::use_auth, components::FlexSpace};

#[component]
fn DevelLogin() -> impl IntoView {
    let auth = use_auth();

    let (user_id, set_user_id) = create_signal(String::new());
    let submit = move || {
        auth.login(format!("/api/auth/login/{}", user_id.get_untracked()));
    };

    view! {
        <input
            class="bg-slate-600 p-2 rounded text-white text-center text-3xl"
            placeholder="Enter a user name"
            prop:value=user_id
            on:input=move |ev| set_user_id(event_target_value(&ev))
            on:keydown=move |ev| {
                if &ev.key() == "Enter" {
                    submit()
                }
            }
        />
        <button
          class="text-white bg-slate-600 w-fit rounded px-4 py-2 disabled:text-gray-400 text-3xl"
          disabled=move || user_id.with(|uid| uid.is_empty())
          on:click=move |_| submit()
        >
            "Login"
        </button>
    }
}

#[component]
fn GithubLogin() -> impl IntoView {
    view! {
        <a
          class="bg-slate-600 w-fit rounded px-4 py-2 text-3xl"
          href="/api/auth/github/login"
          rel="external" // make sure leptos doesn't use client-side routing
        >
          "GitHub Login"
        </a>
    }
}

#[component]
fn ProfileDependentLogin() -> impl IntoView {
    #[cfg(debug_assertions)]
    {
        DevelLogin()
    }
    #[cfg(not(debug_assertions))]
    {
        GithubLogin()
    }
}

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 h-screen items-center">
          <FlexSpace />
          <img src="assets/logo.svg" height={256} width={256} />
          <ProfileDependentLogin />
          <FlexSpace />
          <FlexSpace />
        </div>
    }
}
