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

/// ListRumReplaySessionWatchersOptionalParams is a struct for passing parameters to the method [`RumReplayViewershipAPI::list_rum_replay_session_watchers`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRumReplaySessionWatchersOptionalParams {
    /// Number of items per page.
    pub page_size: Option<i32>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
}

impl ListRumReplaySessionWatchersOptionalParams {
    /// Number of items per page.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// ListRumReplayViewershipHistorySessionsOptionalParams is a struct for passing parameters to the method [`RumReplayViewershipAPI::list_rum_replay_viewership_history_sessions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRumReplayViewershipHistorySessionsOptionalParams {
    /// Start timestamp in milliseconds for watched_at filter.
    pub filter_watched_at_start: Option<i64>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Filter by user UUID. Defaults to current user if not specified.
    pub filter_created_by: Option<String>,
    /// End timestamp in milliseconds for watched_at filter.
    pub filter_watched_at_end: Option<i64>,
    /// Comma-separated list of session IDs to filter by.
    pub filter_session_ids: Option<String>,
    /// Number of items per page.
    pub page_size: Option<i32>,
    /// Filter by application ID.
    pub filter_application_id: Option<String>,
}

impl ListRumReplayViewershipHistorySessionsOptionalParams {
    /// Start timestamp in milliseconds for watched_at filter.
    pub fn filter_watched_at_start(mut self, value: i64) -> Self {
        self.filter_watched_at_start = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by user UUID. Defaults to current user if not specified.
    pub fn filter_created_by(mut self, value: String) -> Self {
        self.filter_created_by = Some(value);
        self
    }
    /// End timestamp in milliseconds for watched_at filter.
    pub fn filter_watched_at_end(mut self, value: i64) -> Self {
        self.filter_watched_at_end = Some(value);
        self
    }
    /// Comma-separated list of session IDs to filter by.
    pub fn filter_session_ids(mut self, value: String) -> Self {
        self.filter_session_ids = Some(value);
        self
    }
    /// Number of items per page.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Filter by application ID.
    pub fn filter_application_id(mut self, value: String) -> Self {
        self.filter_application_id = Some(value);
        self
    }
}

/// CreateRumReplaySessionWatchError is a struct for typed errors of method [`RumReplayViewershipAPI::create_rum_replay_session_watch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRumReplaySessionWatchError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteRumReplaySessionWatchError is a struct for typed errors of method [`RumReplayViewershipAPI::delete_rum_replay_session_watch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRumReplaySessionWatchError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRumReplaySessionWatchersError is a struct for typed errors of method [`RumReplayViewershipAPI::list_rum_replay_session_watchers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRumReplaySessionWatchersError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRumReplayViewershipHistorySessionsError is a struct for typed errors of method [`RumReplayViewershipAPI::list_rum_replay_viewership_history_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRumReplayViewershipHistorySessionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Track and manage RUM replay session viewership. Monitor who watches replay sessions and maintain watch history for audit and analytics purposes.
#[derive(Debug, Clone)]
pub struct RumReplayViewershipAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumReplayViewershipAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumReplayViewershipAPI {
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

    /// Record a session watch.
    pub async fn create_rum_replay_session_watch(
        &self,
        session_id: String,
        body: crate::datadogV2::model::Watch,
    ) -> Result<crate::datadogV2::model::Watch, datadog::Error<CreateRumReplaySessionWatchError>>
    {
        match self
            .create_rum_replay_session_watch_with_http_info(session_id, body)
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

    /// Record a session watch.
    pub async fn create_rum_replay_session_watch_with_http_info(
        &self,
        session_id: String,
        body: crate::datadogV2::model::Watch,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Watch>,
        datadog::Error<CreateRumReplaySessionWatchError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_rum_replay_session_watch";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/sessions/{session_id}/watches",
            local_configuration.get_operation_host(operation_id),
            session_id = datadog::urlencode(session_id)
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
            match serde_json::from_str::<crate::datadogV2::model::Watch>(&local_content) {
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
            let local_entity: Option<CreateRumReplaySessionWatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete session watch history.
    pub async fn delete_rum_replay_session_watch(
        &self,
        session_id: String,
    ) -> Result<(), datadog::Error<DeleteRumReplaySessionWatchError>> {
        match self
            .delete_rum_replay_session_watch_with_http_info(session_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete session watch history.
    pub async fn delete_rum_replay_session_watch_with_http_info(
        &self,
        session_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteRumReplaySessionWatchError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_rum_replay_session_watch";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/sessions/{session_id}/watches",
            local_configuration.get_operation_host(operation_id),
            session_id = datadog::urlencode(session_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteRumReplaySessionWatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List session watchers.
    pub async fn list_rum_replay_session_watchers(
        &self,
        session_id: String,
        params: ListRumReplaySessionWatchersOptionalParams,
    ) -> Result<
        crate::datadogV2::model::WatcherArray,
        datadog::Error<ListRumReplaySessionWatchersError>,
    > {
        match self
            .list_rum_replay_session_watchers_with_http_info(session_id, params)
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

    /// List session watchers.
    pub async fn list_rum_replay_session_watchers_with_http_info(
        &self,
        session_id: String,
        params: ListRumReplaySessionWatchersOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::WatcherArray>,
        datadog::Error<ListRumReplaySessionWatchersError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_rum_replay_session_watchers";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/sessions/{session_id}/watchers",
            local_configuration.get_operation_host(operation_id),
            session_id = datadog::urlencode(session_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::WatcherArray>(&local_content) {
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
            let local_entity: Option<ListRumReplaySessionWatchersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List watched sessions.
    pub async fn list_rum_replay_viewership_history_sessions(
        &self,
        params: ListRumReplayViewershipHistorySessionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ViewershipHistorySessionArray,
        datadog::Error<ListRumReplayViewershipHistorySessionsError>,
    > {
        match self
            .list_rum_replay_viewership_history_sessions_with_http_info(params)
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

    /// List watched sessions.
    pub async fn list_rum_replay_viewership_history_sessions_with_http_info(
        &self,
        params: ListRumReplayViewershipHistorySessionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ViewershipHistorySessionArray>,
        datadog::Error<ListRumReplayViewershipHistorySessionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_rum_replay_viewership_history_sessions";

        // unbox and build optional parameters
        let filter_watched_at_start = params.filter_watched_at_start;
        let page_number = params.page_number;
        let filter_created_by = params.filter_created_by;
        let filter_watched_at_end = params.filter_watched_at_end;
        let filter_session_ids = params.filter_session_ids;
        let page_size = params.page_size;
        let filter_application_id = params.filter_application_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/viewership-history/sessions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_watched_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[watched_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_by {
            local_req_builder =
                local_req_builder.query(&[("filter[created_by]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_watched_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[watched_at][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_session_ids {
            local_req_builder =
                local_req_builder.query(&[("filter[session_ids]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_application_id {
            local_req_builder = local_req_builder
                .query(&[("filter[application_id]", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::ViewershipHistorySessionArray>(
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
            let local_entity: Option<ListRumReplayViewershipHistorySessionsError> =
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
