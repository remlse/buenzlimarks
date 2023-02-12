use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    db::DB,
    models::{bookmark::Bookmark, id::Id, user::User},
};

async fn get_bookmarks(user: User, State(db): State<DB>) -> (StatusCode, Json<Vec<Bookmark>>) {
    match db.get_bookmarks(&user.id) {
        Ok(bookmarks) => (StatusCode::OK, Json(bookmarks)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

async fn create_bookmark(
    user: User,
    State(db): State<DB>,
    Json(mut bookmark): Json<Bookmark>,
) -> Result<Json<Bookmark>, StatusCode> {
    bookmark.id = Id::random();
    db.insert_bookmark(&user.id, bookmark)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn routes() -> Router<DB> {
    Router::<DB>::new()
        .route("/", get(get_bookmarks))
        .route("/", post(create_bookmark))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;

    use crate::{
        db::MockDbTrait,
        models::{bookmark::Bookmark, user::dev_user_id},
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
            .with(predicate::eq(dev_user_id()))
            .returning(move |_| Ok(expected.clone()));

        let actual = get_bookmarks(User::dev(), State(Arc::new(db))).await;

        assert_eq!(actual.0, StatusCode::OK);
        assert_eq!(actual.1 .0, vec![bookmark]);
    }
}
