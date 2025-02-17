use super::{
    session::{OptionalUser, User},
    text::Text,
    timestamp::Timestamp,
    AppState, Result, Router,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json,
};
use minty::{text, Modification, Post, PostParts, Uuid};

async fn add_objects(
    State(AppState { repo }): State<AppState>,
    Path((id, destination)): Path<(Uuid, Uuid)>,
    User(user): User,
    Json(objects): Json<Vec<Uuid>>,
) -> Result<Timestamp> {
    Ok(repo
        .with_user(user)
        .post(id)
        .await?
        .edit()?
        .add_objects(&objects, Some(destination))
        .await?
        .into())
}

async fn append_objects(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
    Json(objects): Json<Vec<Uuid>>,
) -> Result<Timestamp> {
    Ok(repo
        .with_user(user)
        .post(id)
        .await?
        .edit()?
        .add_objects(&objects, None)
        .await?
        .into())
}

async fn add_related_post(
    State(AppState { repo }): State<AppState>,
    Path((id, related)): Path<(Uuid, Uuid)>,
    User(user): User,
) -> Result<StatusCode> {
    repo.with_user(user)
        .post(id)
        .await?
        .edit()?
        .add_related_post(related)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn add_tag(
    State(AppState { repo }): State<AppState>,
    Path((id, tag)): Path<(Uuid, Uuid)>,
    User(user): User,
) -> Result<StatusCode> {
    repo.with_user(user)
        .post(id)
        .await?
        .edit()?
        .add_tag(tag)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_post(
    State(AppState { repo }): State<AppState>,
    User(user): User,
    Json(parts): Json<PostParts>,
) -> Result<String> {
    Ok(repo
        .with_user(user)
        .posts()
        .add(&parts)
        .await?
        .id()
        .to_string())
}

async fn delete_objects(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
    Json(objects): Json<Vec<Uuid>>,
) -> Result<Timestamp> {
    Ok(repo
        .with_user(user)
        .post(id)
        .await?
        .edit()?
        .delete_objects(&objects)
        .await?
        .into())
}

async fn delete_post(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
) -> Result<StatusCode> {
    repo.with_user(user)
        .post(id)
        .await?
        .edit()?
        .delete()
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_related_post(
    State(AppState { repo }): State<AppState>,
    Path((id, related)): Path<(Uuid, Uuid)>,
    User(user): User,
) -> Result<StatusCode> {
    repo.with_user(user)
        .post(id)
        .await?
        .edit()?
        .delete_related_post(related)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_tag(
    State(AppState { repo }): State<AppState>,
    Path((id, tag)): Path<(Uuid, Uuid)>,
    User(user): User,
) -> Result<StatusCode> {
    let status = if repo
        .with_user(user)
        .post(id)
        .await?
        .edit()?
        .delete_tag(tag)
        .await?
    {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    };

    Ok(status)
}

async fn get_post(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    OptionalUser(user): OptionalUser,
) -> Result<Json<Post>> {
    Ok(Json(repo.optional_user(user)?.post(id).await?.get().await?))
}

async fn publish_post(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
) -> Result<StatusCode> {
    repo.with_user(user)
        .post(id)
        .await?
        .edit()?
        .publish()
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn set_description(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
    Text(description): Text<text::Description>,
) -> Result<Json<Modification<String>>> {
    Ok(Json(
        repo.with_user(user)
            .post(id)
            .await?
            .edit()?
            .set_description(description)
            .await?,
    ))
}

async fn set_title(
    State(AppState { repo }): State<AppState>,
    Path(id): Path<Uuid>,
    User(user): User,
    Text(title): Text<text::PostTitle>,
) -> Result<Json<Modification<String>>> {
    Ok(Json(
        repo.with_user(user)
            .post(id)
            .await?
            .edit()?
            .set_title(title)
            .await?,
    ))
}

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post).put(publish_post).delete(delete_post))
        .route("/:id/description", put(set_description))
        .route("/:id/objects", post(append_objects).delete(delete_objects))
        .route("/:id/objects/:destination", post(add_objects))
        .route(
            "/:id/related/:related",
            put(add_related_post).delete(delete_related_post),
        )
        .route("/:id/tag/:tag", put(add_tag).delete(delete_tag))
        .route("/:id/title", put(set_title))
}
