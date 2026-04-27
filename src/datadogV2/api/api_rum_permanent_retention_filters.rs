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

/// GetPermanentRetentionFilterError is a struct for typed errors of method [`RumPermanentRetentionFiltersAPI::get_permanent_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermanentRetentionFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListPermanentRetentionFiltersError is a struct for typed errors of method [`RumPermanentRetentionFiltersAPI::list_permanent_retention_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPermanentRetentionFiltersError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdatePermanentRetentionFilterError is a struct for typed errors of method [`RumPermanentRetentionFiltersAPI::update_permanent_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePermanentRetentionFilterError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage permanent retention filters through [Manage Applications](<https://app.datadoghq.com/rum/list>) in RUM for your organization.
///
/// Permanent retention filters are preconfigured filters that ship with each RUM application.
/// They cannot be created, deleted, or reordered. Their identity (`name`, `event_type`, `query`, `sample_rate`, `enabled`)
/// and cross-product enabled flags (`session_replay_enabled`, `trace_enabled`) are fixed.
/// You can adjust only the cross-product sample rates (`session_replay_sample_rate`, `trace_sample_rate`),
/// and only when the corresponding editability flag is `true`.
#[derive(Debug, Clone)]
pub struct RumPermanentRetentionFiltersAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumPermanentRetentionFiltersAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumPermanentRetentionFiltersAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
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

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Get a single permanent retention filter for a RUM application.
    pub async fn get_permanent_retention_filter(
        &self,
        app_id: String,
        rf_id: String,
    ) -> Result<
        crate::datadogV2::model::RumPermanentRetentionFilterResponse,
        datadog::Error<GetPermanentRetentionFilterError>,
    > {
        match self
            .get_permanent_retention_filter_with_http_info(app_id, rf_id)
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

    /// Get a single permanent retention filter for a RUM application.
    pub async fn get_permanent_retention_filter_with_http_info(
        &self,
        app_id: String,
        rf_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumPermanentRetentionFilterResponse>,
        datadog::Error<GetPermanentRetentionFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_permanent_retention_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{app_id}/permanent_retention_filters/{rf_id}",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id),
            rf_id = datadog::urlencode(rf_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::RumPermanentRetentionFilterResponse>(
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
            let local_entity: Option<GetPermanentRetentionFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of permanent retention filters for a RUM application.
    pub async fn list_permanent_retention_filters(
        &self,
        app_id: String,
    ) -> Result<
        crate::datadogV2::model::RumPermanentRetentionFiltersResponse,
        datadog::Error<ListPermanentRetentionFiltersError>,
    > {
        match self
            .list_permanent_retention_filters_with_http_info(app_id)
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

    /// Get the list of permanent retention filters for a RUM application.
    pub async fn list_permanent_retention_filters_with_http_info(
        &self,
        app_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumPermanentRetentionFiltersResponse>,
        datadog::Error<ListPermanentRetentionFiltersError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_permanent_retention_filters";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{app_id}/permanent_retention_filters",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::RumPermanentRetentionFiltersResponse,
            >(&local_content)
            {
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
            let local_entity: Option<ListPermanentRetentionFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the cross-product sample rates of a permanent retention filter for a RUM application.
    /// Only `cross_product_sampling.session_replay_sample_rate` and `cross_product_sampling.trace_sample_rate` can be updated,
    /// and only when the matching flag in `cross_product_sampling_editability` is `true` on the filter.
    /// Any other field — including `name`, `event_type`, `query`, `sample_rate`, `enabled`, `session_replay_enabled`,
    /// `trace_enabled`, and `cross_product_sampling_editability` — is read-only and cannot be sent in the payload.
    /// Returns the updated permanent retention filter when the request is successful.
    pub async fn update_permanent_retention_filter(
        &self,
        app_id: String,
        rf_id: String,
        body: crate::datadogV2::model::RumPermanentRetentionFilterUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::RumPermanentRetentionFilterResponse,
        datadog::Error<UpdatePermanentRetentionFilterError>,
    > {
        match self
            .update_permanent_retention_filter_with_http_info(app_id, rf_id, body)
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

    /// Update the cross-product sample rates of a permanent retention filter for a RUM application.
    /// Only `cross_product_sampling.session_replay_sample_rate` and `cross_product_sampling.trace_sample_rate` can be updated,
    /// and only when the matching flag in `cross_product_sampling_editability` is `true` on the filter.
    /// Any other field — including `name`, `event_type`, `query`, `sample_rate`, `enabled`, `session_replay_enabled`,
    /// `trace_enabled`, and `cross_product_sampling_editability` — is read-only and cannot be sent in the payload.
    /// Returns the updated permanent retention filter when the request is successful.
    pub async fn update_permanent_retention_filter_with_http_info(
        &self,
        app_id: String,
        rf_id: String,
        body: crate::datadogV2::model::RumPermanentRetentionFilterUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RumPermanentRetentionFilterResponse>,
        datadog::Error<UpdatePermanentRetentionFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_permanent_retention_filter";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/applications/{app_id}/permanent_retention_filters/{rf_id}",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id),
            rf_id = datadog::urlencode(rf_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::RumPermanentRetentionFilterResponse>(
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
            let local_entity: Option<UpdatePermanentRetentionFilterError> =
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
