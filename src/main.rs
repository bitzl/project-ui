mod config;
mod model;

use std::{net::SocketAddr, sync::Arc, str::FromStr};

use axum::{
    extract::{self, Path},
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::{context, Environment};
use model::{Item, Project};

use crate::model::AppState;

type AppEngine = Engine<Environment<'static>>;

async fn index(
    engine: AppEngine,
    Extension(state): Extension<Arc<AppState>>,
    Key(key): Key,
) -> impl IntoResponse {
    let projects = state.list();
    let context = context!(
       projects => projects,
       urls => state.urls
    );
    RenderHtml(key, engine, context)
}

async fn project(
    Extension(state): Extension<Arc<AppState>>,
    Key(key): Key,
    engine: AppEngine,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.get(&project_id) {
        Some(project) => Ok(RenderHtml(key, engine, context!(project => project))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn write_project(
    Extension(state): Extension<Arc<AppState>>,
    payload: extract::Json<Project>,
) -> StatusCode {
    match state.save(payload.0) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn add_items(
    Extension(state): Extension<Arc<AppState>>,
    payload: extract::Json<Vec<Item>>,
    Path(project_id): Path<String>,
) -> StatusCode {
    match state.add_items(&project_id, payload.0) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn viewer(
    Key(key): Key,
    engine: AppEngine,
    Path(project_id): Path<String>,
    Path(item): Path<String>,
) -> impl IntoResponse {
    let viewer_context = context!(
        project => project_id.to_string(),
        item => item.to_string(),
    );
    RenderHtml(key, engine, viewer_context)
}

async fn update(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.update() {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_css(Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    // return webfonts
    if path == "/files/ibm-plex-mono-latin-400-normal.woff2" {
        let data = include_bytes!("../node_modules/@fontsource/ibm-plex-mono/files/ibm-plex-mono-latin-ext-400-normal.woff2");
        return Ok(([(CONTENT_TYPE, "application/woff2")], data.to_vec()).into_response());
    }
    // return css
    let content = match path.as_str() {
        "/style.css" => include_str!("../target/style.css"),
        "/latin.css" => include_str!("../node_modules/@fontsource/ibm-plex-mono/latin.css"),
        "/tify.css" => include_str!("../node_modules/tify/dist/tify.css"),
        _ => return Err(StatusCode::NOT_FOUND),
    };
    Ok(([(CONTENT_TYPE, "text/css")], content.to_string()).into_response())
}

async fn get_js(Path(path): Path<String>) -> Result<String, StatusCode> {
    match path.as_str() {
        "/tify.js" => Ok(include_str!("../node_modules/tify/dist/tify.js").to_string()),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let config_path = std::path::PathBuf::from_str(&std::env::args().nth(1).unwrap_or("config.yml".to_string())).unwrap();
    let config = if config_path.exists() {
        config::Config::load("config.yaml").expect("Please provide a config.yaml file")
    } else {
        config::Config::default()
    };

    println!("Reading project infos from {}", &config.data);
    println!("Base URL: {}", &config.urls.base_url);
    println!("IIIF: {}/{}", &config.urls.base_url, &config.urls.iiif_base);

    let mut jinja = Environment::new();
    jinja
        .add_template("base.html", include_str!("templates/base.html"))
        .unwrap();
    jinja
        .add_template("/", include_str!("templates/index.html"))
        .unwrap();
    jinja
        .add_template("/:project_id", include_str!("templates/project.html"))
        .unwrap();

    let shared_state = Arc::new(AppState::new(config).unwrap());
    let app = Router::new()
        .route("/", get(index))
        .route("/update", post(update))
        .route("/:project_id", get(project))
        .route("/:project_id", post(write_project))
        .route("/:project_id/:item", get(viewer))
        .route("/:project_id/add", post(add_items))
        .route("/css/*path", get(get_css))
        .route("/js/*path", get(get_js))
        .layer(Engine::new(jinja))
        .layer(Extension(shared_state));

    println!("");
    println!("Listening on http://127.0.0.1:3000");
    println!("");
    println!("Press Ctrl+C to stop");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
