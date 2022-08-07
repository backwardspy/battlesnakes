use axum::{
    extract,
    response,
    routing::{get, post},
    Router,
};
use battlesnake_game_types::wire_representation::Game;
use battlesnakes::{
    snakes::{Snake, Spacewhale},
    wire_representation::{Aesthetic, Movement},
};
use color_eyre::Result;
use hyper::http::Method;
use tracing::{Level, info};

async fn root<S>() -> response::Json<Aesthetic>
where
    S: Snake,
{
    info!("aesthetic requested");
    response::Json(S::aesthetic())
}

async fn start<S>(extract::Json(game): extract::Json<Game>)
where
    S: Snake,
{
    info!("game {} started", game.game.id);
    S::start(game);
}

async fn make_move<S>(
    extract::Json(game): extract::Json<Game>,
) -> response::Json<Movement>
where
    S: Snake,
{
    info!("making move in game {}", game.game.id);
    response::Json(S::make_move(game))
}

async fn end<S>(extract::Json(game): extract::Json<Game>)
where
    S: Snake,
{
    info!("game {} ended", game.game.id);
    S::end(game);
}

async fn serve<S>() -> Result<()>
where
    S: Snake + Sync + Send + 'static,
{
    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let trace = tower_http::trace::TraceLayer::new_for_http();

    let app = Router::new()
        .route("/", get(root::<S>))
        .route("/start", post(start::<S>))
        .route("/move", post(make_move::<S>))
        .route("/end", post(end::<S>))
        .layer(cors)
        .layer(trace);

    let addr = "0.0.0.0:6502";
    info!("listening on {addr}");
    Ok(axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    #[cfg(feature = "spacewhale")]
    type S = Spacewhale;

    serve::<S>().await?;
    Ok(())
}
