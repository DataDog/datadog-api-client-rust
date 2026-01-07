// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// SubmitProductAnalyticsEventError is a struct for typed errors of method [`ProductAnalyticsAPI::submit_product_analytics_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitProductAnalyticsEventError {
    ProductAnalyticsServerSideEventErrors(
        crate::datadogV2::model::ProductAnalyticsServerSideEventErrors,
    ),
    UnknownValue(serde_json::Value),
}

/// Send server-side events to Product Analytics. Server-Side Events Ingestion allows you to collect custom events
/// from any server-side source, and retains events for 15 months. Server-side events are helpful for understanding
/// causes of a funnel drop-off which are external to the client-side (for example, payment processing error).
/// See the [Product Analytics page](<https://docs.datadoghq.com/product_analytics/>) for more information.
#[derive(Debug, Clone)]
pub struct ProductAnalyticsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ProductAnalyticsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ProductAnalyticsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Send server-side events to Product Analytics. Server-side events are retained for 15 months.
    ///
    /// Server-Side events in Product Analytics are helpful for tracking events that occur on the server,
    /// as opposed to client-side events, which are captured by Real User Monitoring (RUM) SDKs.
    /// This allows for a more comprehensive view of the user journey by including actions that happen on the server.
    /// Typical examples could be `checkout.completed` or `payment.processed`.
    ///
    /// Ingested server-side events are integrated into Product Analytics to allow users to select and filter
    /// these events in the event picker, similar to how views or actions are handled.
    ///
    /// **Requirements:**
    /// - At least one of `usr`, `account`, or `session` must be provided with a valid ID.
    /// - The `application.id` must reference a Product Analytics-enabled application.
    ///
    /// **Custom Attributes:**
    /// Any additional fields in the payload are flattened and searchable as facets.
    /// For example, a payload with `{"customer": {"tier": "premium"}}` is searchable with
    /// the syntax `@customer.tier:premium` in Datadog.
    ///
    /// The status codes answered by the HTTP API are:
    /// - 202: Accepted: The request has been accepted for processing
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 401: Unauthorized (likely a missing API Key)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 408: Request Timeout, request should be retried after some time
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 429: Too Many Requests, request should be retried after some time
    /// - 500: Internal Server Error, the server encountered an unexpected condition that prevented it from fulfilling the request, request should be retried after some time
    /// - 503: Service Unavailable, the server is not ready to handle the request probably because it is overloaded, request should be retried after some time
    pub async fn submit_product_analytics_event(
        &self,
        body: crate::datadogV2::model::ProductAnalyticsServerSideEventItem,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        datadog::Error<SubmitProductAnalyticsEventError>,
    > {
        match self
            .submit_product_analytics_event_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Send server-side events to Product Analytics. Server-side events are retained for 15 months.
    ///
    /// Server-Side events in Product Analytics are helpful for tracking events that occur on the server,
    /// as opposed to client-side events, which are captured by Real User Monitoring (RUM) SDKs.
    /// This allows for a more comprehensive view of the user journey by including actions that happen on the server.
    /// Typical examples could be `checkout.completed` or `payment.processed`.
    ///
    /// Ingested server-side events are integrated into Product Analytics to allow users to select and filter
    /// these events in the event picker, similar to how views or actions are handled.
    ///
    /// **Requirements:**
    /// - At least one of `usr`, `account`, or `session` must be provided with a valid ID.
    /// - The `application.id` must reference a Product Analytics-enabled application.
    ///
    /// **Custom Attributes:**
    /// Any additional fields in the payload are flattened and searchable as facets.
    /// For example, a payload with `{"customer": {"tier": "premium"}}` is searchable with
    /// the syntax `@customer.tier:premium` in Datadog.
    ///
    /// The status codes answered by the HTTP API are:
    /// - 202: Accepted: The request has been accepted for processing
    /// - 400: Bad request (likely an issue in the payload formatting)
    /// - 401: Unauthorized (likely a missing API Key)
    /// - 403: Permission issue (likely using an invalid API Key)
    /// - 408: Request Timeout, request should be retried after some time
    /// - 413: Payload too large (batch is above 5MB uncompressed)
    /// - 429: Too Many Requests, request should be retried after some time
    /// - 500: Internal Server Error, the server encountered an unexpected condition that prevented it from fulfilling the request, request should be retried after some time
    /// - 503: Service Unavailable, the server is not ready to handle the request probably because it is overloaded, request should be retried after some time
    pub async fn submit_product_analytics_event_with_http_info(
        &self,
        body: crate::datadogV2::model::ProductAnalyticsServerSideEventItem,
    ) -> Result<
        datadog::ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        datadog::Error<SubmitProductAnalyticsEventError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.submit_product_analytics_event";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/prodlytics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SubmitProductAnalyticsEventError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
