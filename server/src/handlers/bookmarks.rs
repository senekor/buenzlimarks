use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::{error::DbError, DB},
    models::{bookmark::Bookmark, id::Id, user::User},
};

pub async fn get_bookmarks(user: User, State(db): State<DB>) -> (StatusCode, Json<Vec<Bookmark>>) {
    match db.get_bookmarks(&user.id) {
        Ok(bookmarks) => (StatusCode::OK, Json(bookmarks)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn create_bookmark(
    user: User,
    State(db): State<DB>,
    Json(mut bookmark): Json<Bookmark>,
) -> Result<Json<Bookmark>, StatusCode> {
    bookmark.id = Id::random();
    db.insert_bookmark(&user.id, bookmark)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn delete_bookmark(
    user: User,
    Path(bookmark_id): Path<Id<Bookmark>>,
    State(db): State<DB>,
) -> Result<(), StatusCode> {
    match db.delete_bookmark(&user.id, &bookmark_id) {
        Ok(_) => Ok(()),
        Err(DbError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(DbError::WhoopsieDoopsie) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        db::{error::DbError, MockDbTrait},
        models::bookmark::Bookmark,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_single_bookmark() {
        let mut db = MockDbTrait::new();

        let bookmark = Bookmark {
            id: "0".into(),
            name: "name".into(),
            url: "url".into(),
            widget_id: "0".into(),
        };
        let expected = vec![bookmark.clone()];

        db.expect_get_bookmarks()
            .returning(move |_| Ok(expected.clone()));

        let actual = get_bookmarks(User::dev(), State(Arc::new(db))).await;

        assert_eq!(actual.0, StatusCode::OK);
        assert_eq!(actual.1 .0, vec![bookmark]);
    }

    #[tokio::test]
    async fn get_bookmarks_should_handle_db_error() {
        let mut db = MockDbTrait::new();

        db.expect_get_bookmarks()
            .returning(move |_| Err(DbError::WhoopsieDoopsie));

        let actual = get_bookmarks(User::dev(), State(Arc::new(db))).await;

        assert_eq!(actual.0, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(actual.1 .0, Vec::new());
    }

    #[tokio::test]
    async fn should_create_bookmark() {
        let mut db = MockDbTrait::new();

        let bookmark = Bookmark {
            id: "0".into(),
            name: "name".into(),
            url: "url".into(),
            widget_id: "0".into(),
        };
        let user_provided_id = bookmark.id.clone();

        db.expect_insert_bookmark()
            .times(1)
            .withf(move |_, bm| bm.id != user_provided_id)
            .returning(move |_, _| Err(DbError::WhoopsieDoopsie));

        create_bookmark(User::dev(), State(Arc::new(db)), Json(bookmark))
            .await
            .ok();
    }
}
