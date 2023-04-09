use crate::db::Todo;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    routing::{get, patch},
    Json, Router,
};
use http::Request;
use serde::Deserialize;
use serde_json;
use spin_sdk::key_value::Store;
use std::{collections::HashMap, sync::Arc};
use tower_service::Service;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
pub async fn app(request: Request<String>) -> Response {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let store = Arc::new(Store::open_default().unwrap());
    // Compose the routes
    let mut app = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        .with_state(store);

    app.call(request).await.unwrap()
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(store): State<Arc<Store>>,
) -> impl IntoResponse {
    let keys = store.get_keys().unwrap();
    let mut todos = HashMap::new();
    for key in keys {
        let value = store.get(key).unwrap();
        let todo: Todo = serde_json::from_slice(&value).unwrap();
        todos.insert(todo.id, todo);
    }
    //let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_create(
    State(store): State<Arc<Store>>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };
    if let Ok(todo_str) = serde_json::to_string(&todo) {
        if let Ok(_) = store.set(todo.id.to_string(), todo_str) {
            return (StatusCode::CREATED, Json(todo));
        }
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

async fn todos_update(
    Path(id): Path<Uuid>,
    State(store): State<Arc<Store>>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo: Todo = serde_json::from_slice(&store.get(id.to_string()).unwrap()).unwrap();
    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }
    if let Ok(todo_str) = serde_json::to_string(&todo) {
        if let Ok(_) = store.set(todo.id.to_string(), todo_str) {
            return Ok(Json(todo));
        }
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn todos_delete(Path(id): Path<Uuid>, State(store): State<Arc<Store>>) -> impl IntoResponse {
    if let Ok(_) = store.delete(id.to_string()) {
        return StatusCode::OK;
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}
