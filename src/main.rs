// Copyright 2022 Zinc Labs Inc. and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{middleware, web, App, HttpServer};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

mod api;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));
    log::info!("Starting ZincObserve Test");

    // HTTP server
    let thread_id = Arc::new(AtomicU8::new(0));
    let haddr: SocketAddr = format!("0.0.0.0:{}", "5080").parse()?;

    HttpServer::new(move || {
        let local_id = thread_id.load(Ordering::SeqCst) as usize;
        log::info!(
            "starting HTTP server at: {}, thread_id: {}",
            haddr,
            local_id
        );

        let app = App::new().service(
            web::scope("/api")
                .service(api::org_es_index)
                .service(api::org_es_license)
                .service(api::org_es_xpack)
                .service(api::org_es_index_template)
                .service(api::org_es_index_template_create)
                .service(api::org_es_data_stream)
                .service(api::org_es_data_stream_create),
        );
        app.wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new(
                r#"%a "%r" %s %b "%{Content-Length}i" "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
    })
    .bind(haddr)?
    .run()
    .await?;

    log::info!("server stopped");

    Ok(())
}
