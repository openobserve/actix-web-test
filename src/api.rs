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

use axum::{
    extract::{Path, TypedHeader},
    headers::UserAgent,
    http::StatusCode,
    response::IntoResponse,
};

pub async fn org_es_index(
    Path(_org_id): Path<String>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse {
    // eg.1: User-Agent:[elastic-transport-ruby/8.0.1 (RUBY_VERSION: 3.1.2; linux x86_64; Faraday v1.10.0)]
    // eg.2: Elastic-filebeat/7.17.1 (linux; arm64; 1d05ba86138cfc9a5ae5c0acc64a57b8d81678ff; 2022-02-24 01:00:19 +0000 UTC)
    let mut version = "7.17.1";
    let user_agent = user_agent.as_str();
    if user_agent.to_lowercase().contains("elastic") {
        let re = regex::Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
        version = match re.captures(user_agent) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            None => "8.1.0",
        };
    }
    let es_info = r#"{"name":"opensearch","cluster_name":"opensearch-cluster","cluster_uuid":"h3nGzoJ1R12fZz","version":{"number":"0.0.0","build_flavor":"default","build_hash":"0","build_date":"0","build_snapshot":false,"lucene_version":"8.9.0","minimum_wire_version":"7.10.0","minimum_index_compatibility":"8.1.0"},"tagline":"You Know, for Search"}"#;
    let es_info = es_info.replace("0.0.0", version);

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_license(Path(_org_id): Path<String>) -> impl IntoResponse {
    let es_info = r#"{"status":"active"}"#;

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_xpack(Path(_org_id): Path<String>) -> impl IntoResponse {
    let es_info = r#"{"build":{},"features":{},"license":{"status":"active"}}"#;

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_index_template(
    Path((_org_id, name)): Path<(String, String)>,
) -> impl IntoResponse {
    let es_info = r#"{"index_patterns":["log-*"],"name":"logs","priority":1,"template":{"mappings":{"properties":{"_timestamp":{"aggregatable":false,"highlightable":false,"index":true,"sortable":false,"store":false,"type":"date"}}},"settings":{"number_of_replicas":1,"number_of_shards":3}}}"#;
    let es_info = es_info.replace("logs", &name);

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_index_template_create(
    Path((_org_id, name)): Path<(String, String)>,
) -> impl IntoResponse {
    let es_info = r#"{"name":"logs","message":"ok"}"#;
    let es_info = es_info.replace("logs", &name);

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_data_stream(
    Path((_org_id, name)): Path<(String, String)>,
) -> impl IntoResponse {
    let es_info = r#"{"data_streams":{"name":"logs","timestamp_field":{"name":"_timestamp"}}}"#;
    let es_info = es_info.replace("logs", &name);

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}

pub async fn org_es_data_stream_create(
    Path((_org_id, name)): Path<(String, String)>,
) -> impl IntoResponse {
    let es_info = r#"{"name":"logs","message":"ok"}"#;
    let es_info = es_info.replace("logs", &name);

    (
        StatusCode::OK,
        [
            ("Content-Type", "application/json"),
            ("X-Elastic-Product", "Elasticsearch"),
        ],
        es_info,
    )
}
