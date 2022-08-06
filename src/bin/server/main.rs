use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension,
    Json,
    Router,
};
use battlesnake_game_types::wire_representation;
use battlesnakes::{
    aesthetic::{Aesthetic, HasAesthetic},
    snakes::spacewhale::Spacewhale,
};
use color_eyre::Result;

async fn root<S>(Extension(_snake): Extension<Arc<S>>) -> Json<Aesthetic>
where
    S: HasAesthetic,
{
    Json(S::aesthetic())
}

async fn start<S>(
    Json(_game): Json<wire_representation::Game>,
    Extension(_snake): Extension<Arc<S>>,
) where
    S: HasAesthetic,
{
    println!("{:#?}", _game);
}

async fn make_move<S>(
    Json(_game): Json<wire_representation::Game>,
    Extension(_snake): Extension<Arc<S>>,
) where
    S: HasAesthetic,
{
}

async fn end<S>(
    Json(_game): Json<wire_representation::Game>,
    Extension(_snake): Extension<Arc<S>>,
) where
    S: HasAesthetic,
{
}

async fn serve<S>(snake: S) -> Result<()>
where
    S: HasAesthetic + Sync + Send + 'static,
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
