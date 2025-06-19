use std::{net::SocketAddr, time::Duration};

use axum::{extract::{DefaultBodyLimit, Request}, Router};
use bytesize::ByteSize;
use tokio::net::TcpListener;

use tower_http::cors::{self, CorsLayer}; 
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use crate::{app::AppState, config::server::ServerConfig};


pub struct Server  {
    config: &'static ServerConfig
}

impl Server {

    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub  async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let route = self.build_router(state, router);
        let port = self.config.port();

        let  listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        tracing::info!("Listening on {}", listener.local_addr()?);

        axum::serve(
            listener, 
            route.into_make_service_with_connect_info::<SocketAddr>()
        ).await?;

        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {

        let timeout = TimeoutLayer::new(
            // 限制请求超时时间
            Duration::from_secs(120)
        );
        let limit = DefaultBodyLimit::max(
            // mib 表示 MB (限制请求体的大小)
            ByteSize::mib(10).as_u64() as usize
        );
        let coross = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600));

        // 处理请求路径，将末尾的 / 去掉
        let normal_path = NormalizePathLayer::trim_trailing_slash();

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
        // 请求从下往上走，先到 state -> layer -> router
        Router::new()
            .merge(router)
            .layer(timeout)
            .layer(limit)
            .layer(trace_layer)
            .layer(coross)
            .layer(normal_path)
            .with_state(state)

    }
}
