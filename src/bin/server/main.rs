use axum::{
    routing::{get, post},
    Router, response, extract,
};
use battlesnake_game_types::wire_representation::Game;
use battlesnakes::{
    snakes::{Snake, Spacewhale}, wire_representation::{Aesthetic, Movement},
};
use color_eyre::Result;

async fn root<S>() -> response::Json<Aesthetic>
where
    S: Snake,
{
    response::Json(S::aesthetic())
}

async fn start<S>(extract::Json(game): extract::Json<Game>)
where
    S: Snake,
{
    S::start(game);
}

async fn make_move<S>(extract::Json(game): extract::Json<Game>) -> response::Json<Movement>
where
    S: Snake,
{
    response::Json(S::make_move(game))
}

async fn end<S>(extract::Json(game): extract::Json<Game>)
where
    S: Snake,
{
    S::end(game);
}

async fn serve<S>() -> Result<()>
where
    S: Snake + Sync + Send + 'static,
{
    let app = Router::new()
        .route("/", get(root::<S>))
        .route("/start", post(start::<S>))
        .route("/move", post(make_move::<S>))
        .route("/end", post(end::<S>));

    Ok(axum::Server::bind(&"0.0.0.0:6502".parse()?)
        .serve(app.into_make_service())
        .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "spacewhale")]
    type S = Spacewhale;

    serve::<S>().await?;
    Ok(())
}
