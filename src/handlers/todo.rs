use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::repositories::todo::{CreateTodo, TodoRepository, UpdateTodo};

use super::ValidatedJson;

pub async fn create_todo<T: TodoRepository>(
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = repository.all().await.or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todos)))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    // let todo = repository
    //     .update(id, payload)
    //     .map_err(|_| StatusCode::NOT_FOUND)?;

    let todo = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repositories): Extension<Arc<T>>,
) -> StatusCode {
    repositories
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
