use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    handler::get,
    http::{HeaderMap, HeaderValue, StatusCode},
    AddExtensionLayer, Router,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{instrument, log::info};

mod engine;
mod pb;

// 声明 abi.rs
use pb::*;

use crate::engine::{Engine, Photon};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    // 构建路由
    let app = Router::new()
        // `GET /image` 会执行 generate 函数，并把 spec 和 url 传递过去
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );
    // 运行 web 服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: 处理图片
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    // 使用 image engine 处理
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());
    let mut headers = HeaderMap::new();

    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();

    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}
