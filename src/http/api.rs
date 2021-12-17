mod websocket;

use crate::lttp::{
    app_state::Update,
    AppState,
    DungeonState,
    DungeonUpdate,
    GameState,
    LocationState,
    LocationUpdate,
};

use axum::{
    extract::{
        self,
        ws::{
            Message,
            WebSocket,
            WebSocketUpgrade,
        },
        Extension,
        Path,
    },
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
use serde_json::json;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub fn build(app_state: Arc<AppState>) -> Router {
    let cors_layer = CorsLayer::permissive();

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
    let server_config = match app_state.server_config.read() {
        Ok(ac) => ac.clone(),
        Err(e) => return Err(format!("Unable to get app config: {:?}", e)),
    };

    Ok(Json(json!(server_config)))
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
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_handler(socket, Extension(app_state)))
}

#[allow(clippy::unused_async)]
async fn websocket_handler(mut socket: WebSocket, Extension(app_state): Extension<Arc<AppState>>) {
    if let Some(game_state) = clone_game_state(app_state.clone()) {
        if let Ok(message) = serde_json::to_string(&websocket::ServerMessage::Item(game_state)) {
            socket.send(Message::Text(message)).await.ok();
        }
    }
    if let Some(dungeon_state) = clone_dungeon_state(app_state.clone()) {
        if let Ok(message) =
            serde_json::to_string(&websocket::ServerMessage::Dungeon(dungeon_state.dungeons))
        {
            socket.send(Message::Text(message)).await.ok();
        }
    }
    if let Some(location_state) = clone_location_state(app_state.clone()) {
        if let Ok(message) =
            serde_json::to_string(&websocket::ServerMessage::Location(location_state.locations))
        {
            socket.send(Message::Text(message)).await.ok();
        }
    }

    let mut updates = app_state.update_sender.subscribe();

    while let Ok(update_type) = updates.recv().await {
        let update_message = match update_type {
            Update::Items => {
                clone_game_state(app_state.clone()).map(|gs| websocket::ServerMessage::Item(gs))
            }
            Update::Dungeons => {
                clone_dungeon_state(app_state.clone())
                    .map(|ds| websocket::ServerMessage::Dungeon(ds.dungeons))
            }
            Update::Locations => {
                clone_location_state(app_state.clone())
                    .map(|ls| websocket::ServerMessage::Location(ls.locations))
            }
        };

        if let Some(message) = update_message {
            if let Ok(string_message) = serde_json::to_string(&message) {
                socket.send(Message::Text(string_message)).await.ok();
            }
        }
    }
}

fn clone_game_state(app_state: Arc<AppState>) -> Option<GameState> {
    if let Ok(game_state) = app_state.game_state.read().map(|gs| gs.clone()) {
        Some(game_state)
    } else {
        None
    }
}

fn clone_dungeon_state(app_state: Arc<AppState>) -> Option<DungeonState> {
    if let Ok(dungeon_state) = app_state.dungeon_state.read().map(|ds| ds.clone()) {
        Some(dungeon_state)
    } else {
        None
    }
}

fn clone_location_state(app_state: Arc<AppState>) -> Option<LocationState> {
    if let Ok(location_state) = app_state.location_state.read().map(|ls| ls.clone()) {
        Some(location_state)
    } else {
        None
    }
}
