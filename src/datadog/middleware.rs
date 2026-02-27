// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

//! Bearer token middleware for OAuth2 authentication.
//!
//! Since the generated API structs only support API key auth natively, this
//! middleware allows injecting an OAuth2 bearer token at the HTTP layer when
//! using [`with_client_and_config`](crate::datadogV2::api_logs::LogsAPI::with_client_and_config).

use async_trait::async_trait;
use reqwest_middleware::{Middleware, Next, Result};
use task_local_extensions::Extensions;

/// Injects `Authorization: Bearer <token>` into every outgoing request.
///
/// Because the DD API client adds `DD-API-KEY` / `DD-APPLICATION-KEY` headers
/// inside each `_with_http_info` method, this middleware runs *after* those
/// headers are set. The `Authorization` header is distinct so there is no
/// conflict — both sets of headers end up on the request.
pub struct BearerTokenMiddleware {
    pub token: String,
}

#[async_trait]
impl Middleware for BearerTokenMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<reqwest::Response> {
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.token)
                .parse()
                .expect("bearer token is valid header value"),
        );
        next.run(req, extensions).await
    }
}
