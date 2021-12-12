mod websocket;

use crate::lttp::{
    app_state::Update,
    AppState,
    DungeonUpdate,
    LocationUpdate,
};

use axum::{
    extract::{
        self,
        Extension,
        Path,
    },
    http::Method,
    response::{
        IntoResponse,
        Json,
    },
    routing::{
        get,
        post,
    },
    AddExtensionLayer,
    Router,
};
use axum_typed_websockets::{
    Message,
    WebSocket,
    WebSocketUpgrade,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{
    any,
    CorsLayer,
};

pub fn build(app_state: Arc<AppState>) -> Router {
    let cors_layer =
        CorsLayer::new().allow_methods(vec![Method::GET, Method::POST]).allow_origin(any());

    Router::new()
        .route("/config", get(get_config))
        .route("/dungeon_state", get(get_dungeon_state))
        .route("/dungeon_state/:dungeon", post(post_dungeon_state))
        .route("/game_state", get(get_game_state))
        .route("/location_state", get(get_location_state))
        .route("/location_state/:location", post(post_location_state))
        .route("/ws", get(websocket_upgrade_handler))
        .layer(ServiceBuilder::new().layer(cors_layer).layer(AddExtensionLayer::new(app_state)))
}

#[allow(clippy::unused_async)]
async fn get_config(Extension(app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let app_config = match app_state.app_config.read() {
        Ok(ac) => ac.clone(),
        Err(e) => return Err(format!("Unable to get app config: {:?}", e)),
    };

    Ok(Json(json!(app_config)))
}

#[allow(clippy::unused_async)]
async fn get_game_state(Extension(app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let game_state = match app_state.game_state.read() {
        Ok(gs) => gs.clone(),
        Err(e) => return Err(format!("Unable to get game state: {:?}", e)),
    };

    Ok(Json(json!(game_state)))
}

#[allow(clippy::unused_async)]
async fn get_location_state(Extension(app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let location_state = match app_state.location_state.read() {
        Ok(ls) => ls.clone(),
        Err(e) => return Err(format!("Unable to get location state: {:?}", e)),
    };

    Ok(Json(json!(location_state.locations)))
}

#[allow(clippy::unused_async)]
async fn get_dungeon_state(Extension(app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let dungeon_state = match app_state.dungeon_state.read() {
        Ok(ds) => ds.clone(),
        Err(e) => return Err(format!("Unable to get dungeon state: {:?}", e)),
    };

    Ok(Json(json!(dungeon_state.dungeons)))
}

#[allow(clippy::unused_async)]
async fn post_dungeon_state(
    Path(dungeon): Path<String>,
    extract::Json(dungeon_update): extract::Json<DungeonUpdate>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    if let Err(e) = app_state.set_dungeon_state(&dungeon, dungeon_update) {
        return Err(format!("Unable to set dungeon state: {:?}", e));
    };
    let new_state = match app_state.dungeon_state.read() {
        Ok(ds) => ds.get(&dungeon).clone(),
        Err(e) => return Err(format!("Unable to get dungeon state: {:?}", e)),
    };

    Ok(Json(json!(new_state)))
}

#[allow(clippy::unused_async)]
async fn post_location_state(
    Path(location): Path<String>,
    extract::Json(location_update): extract::Json<LocationUpdate>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    if let Err(e) = app_state.set_location_state(&location, location_update) {
        return Err(format!("Unable to set location state: {:?}", e));
    };

    let new_state = match app_state.location_state.read() {
        Ok(ls) => ls.get(&location).clone(),
        Err(e) => return Err(format!("Unable to get location state: {:?}", e)),
    };

    Ok(Json(json!(new_state)))
}

#[allow(clippy::unused_async)]
async fn websocket_upgrade_handler(
    ws: WebSocketUpgrade<websocket::ServerMessage, websocket::ClientMessage>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_handler(socket, Extension(app_state)))
}

#[allow(clippy::unused_async)]
async fn websocket_handler(
    mut socket: WebSocket<websocket::ServerMessage, websocket::ClientMessage>,
    Extension(app_state): Extension<Arc<AppState>>,
) {
    let mut updates = app_state.update_sender.subscribe();

    while let Ok(update_type) = updates.recv().await {
        let update_message = match update_type {
            Update::Items => {
                if let Ok(game_state) = app_state.game_state.read().map(|gs| gs.clone()) {
                    Some(websocket::ServerMessage::Item(game_state))
                } else {
                    None
                }
            }
            Update::Dungeons => {
                if let Ok(dungeon_state) = app_state.dungeon_state.read().map(|ds| ds.clone()) {
                    Some(websocket::ServerMessage::Dungeon(dungeon_state.dungeons))
                } else {
                    None
                }
            }
            Update::Locations => {
                if let Ok(location_state) = app_state.location_state.read().map(|ls| ls.clone()) {
                    Some(websocket::ServerMessage::Location(location_state.locations))
                } else {
                    None
                }
            }
        };

        if let Some(message) = update_message {
            socket.send(Message::Item(message)).await.ok();
        }
    }
}
