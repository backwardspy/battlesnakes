use std::sync::Arc;

use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use backwardsnakes::{SnakeInfo, Spacewhale};
use color_eyre::Result;
use serde_json::{json, Value};

async fn root<S>(Extension(_snake): Extension<Arc<S>>) -> Json<Value>
where
    S: SnakeInfo,
{
    Json(json!(S::snake_info()))
}

async fn start<S>(Json(request): Json<Value>, Extension(_snake): Extension<Arc<S>>) -> Json<Value>
where
    S: SnakeInfo,
{
    Json(request)
}

async fn make_move<S>(
    Json(request): Json<Value>,
    Extension(_snake): Extension<Arc<S>>,
) -> Json<Value>
where
    S: SnakeInfo,
{
    Json(request)
}

async fn end<S>(Json(request): Json<Value>, Extension(_snake): Extension<Arc<S>>) -> Json<Value>
where
    S: SnakeInfo,
{
    Json(request)
}

async fn serve<S>(snake: S) -> Result<()>
where
    S: SnakeInfo + Sync + Send + 'static,
{
    let snake = Arc::new(snake);

    let app = Router::new()
        .route("/", get(root::<S>))
        .route("/start", post(start::<S>))
        .route("/move", post(make_move::<S>))
        .route("/end", post(end::<S>))
        .layer(Extension(snake));

    Ok(axum::Server::bind(&"0.0.0.0:6502".parse()?)
        .serve(app.into_make_service())
        .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let snake = match env!("SNAKE").to_lowercase().as_str() {
        "spacewhale" => Spacewhale,
        _ => panic!("unknown snake"),
    };
    serve(snake).await?;
    Ok(())
}
