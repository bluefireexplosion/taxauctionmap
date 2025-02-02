use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(feature="ssr")]
use sqlx::{FromRow, Pool, MySql, Error, MySqlPool, Query};

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                add_todo("So much to do!".to_string()).await;
            });
        }>
            "Add Todo"
        </button>
    }
}