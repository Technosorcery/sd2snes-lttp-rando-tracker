mod api;

use crate::lttp::AppState;

use axum::{
    extract::Path,
    http::{
        header::{
            HeaderValue,
            CONTENT_LENGTH,
            CONTENT_TYPE,
        },
        status::StatusCode,
        HeaderMap,
    },
    response::{
        IntoResponse,
        Response,
    },
    routing::get,
    Router,
};
use hyper::body::HttpBody;
use once_cell::sync;
use std::{
    collections::HashMap,
    path::{
        self,
        PathBuf,
    },
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::{
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

#[iftree::include_file_tree("paths = '/ui/dist/**'")]
#[derive(Debug)]
pub struct UiAsset {
    relative_path: &'static str,
    contents_str:  &'static str,
    get_str:       fn() -> std::borrow::Cow<'static, str>,
}

static UI_ASSET_MAP: sync::Lazy<HashMap<&str, &UiAsset>> =
    sync::Lazy::new(|| ASSETS.iter().map(|asset| (asset.relative_path, asset)).collect());

pub fn build(app_state: Arc<AppState>) -> Router {
    Router::new().nest("/api", api::build(app_state)).route("/ui/*path", get(get_ui_file)).layer(
        ServiceBuilder::new()
            .layer(SetResponseHeaderLayer::overriding(CONTENT_LENGTH, content_length_from_response))
            .layer(TraceLayer::new_for_http()),
    )
}

fn content_length_from_response<B>(response: &Response<B>) -> Option<HeaderValue>
where
    B: HttpBody,
{
    response
        .body()
        .size_hint()
        .exact()
        .map(|size| HeaderValue::from_str(&size.to_string()).unwrap())
}

#[allow(clippy::unused_async)]
async fn get_ui_file(Path(file_name): Path<String>) -> impl IntoResponse {
    let file = PathBuf::from(file_name.trim_start_matches('/'));
    let mut path = path::Path::new("ui/dist/").join(file);
    let mut path_str = match path.to_str() {
        Some(s) => s,
        None => return Err("Invalid UI file name".to_string()),
    };

    let file = if let Some(f) = UI_ASSET_MAP.get(&path_str) {
        f
    } else {
        path = path.join("index.html");
        path_str = match path.to_str() {
            Some(s) => s,
            None => return Err("Invalid UI file name".to_string()),
        };

        match UI_ASSET_MAP.get(&path_str) {
            Some(f) => f,
            None => return Ok((StatusCode::NOT_FOUND, HeaderMap::new(), "".to_owned())),
        }
    };

    let mut headers = HeaderMap::new();
    // TODO: Set a real CONTENT_TYPE: https://github.com/evolutics/iftree/blob/5a178ee040baa37184981d778ba9a82f8e71e3da/examples/scenario_media_type.rs
    headers.insert(CONTENT_TYPE, "text/html".parse().unwrap());

    Ok((StatusCode::OK, headers, (file.get_str)().into_owned()))
}
