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

/// AddRumReplaySessionToPlaylistOptionalParams is a struct for passing parameters to the method [`RumReplayPlaylistsAPI::add_rum_replay_session_to_playlist`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct AddRumReplaySessionToPlaylistOptionalParams {
    /// Data source type. Valid values: 'rum' or 'product_analytics'. Defaults to 'rum'.
    pub data_source: Option<String>,
}

impl AddRumReplaySessionToPlaylistOptionalParams {
    /// Data source type. Valid values: 'rum' or 'product_analytics'. Defaults to 'rum'.
    pub fn data_source(mut self, value: String) -> Self {
        self.data_source = Some(value);
        self
    }
}

/// ListRumReplayPlaylistSessionsOptionalParams is a struct for passing parameters to the method [`RumReplayPlaylistsAPI::list_rum_replay_playlist_sessions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRumReplayPlaylistSessionsOptionalParams {
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Number of items per page.
    pub page_size: Option<i32>,
}

impl ListRumReplayPlaylistSessionsOptionalParams {
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of items per page.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// ListRumReplayPlaylistsOptionalParams is a struct for passing parameters to the method [`RumReplayPlaylistsAPI::list_rum_replay_playlists`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListRumReplayPlaylistsOptionalParams {
    /// Filter playlists by the UUID of the user who created them.
    pub filter_created_by_uuid: Option<String>,
    /// Search query to filter playlists by name.
    pub filter_query: Option<String>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Number of items per page.
    pub page_size: Option<i32>,
}

impl ListRumReplayPlaylistsOptionalParams {
    /// Filter playlists by the UUID of the user who created them.
    pub fn filter_created_by_uuid(mut self, value: String) -> Self {
        self.filter_created_by_uuid = Some(value);
        self
    }
    /// Search query to filter playlists by name.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of items per page.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// AddRumReplaySessionToPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::add_rum_replay_session_to_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRumReplaySessionToPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// BulkRemoveRumReplayPlaylistSessionsError is a struct for typed errors of method [`RumReplayPlaylistsAPI::bulk_remove_rum_replay_playlist_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkRemoveRumReplayPlaylistSessionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateRumReplayPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::create_rum_replay_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRumReplayPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteRumReplayPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::delete_rum_replay_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRumReplayPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRumReplayPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::get_rum_replay_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRumReplayPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRumReplayPlaylistSessionsError is a struct for typed errors of method [`RumReplayPlaylistsAPI::list_rum_replay_playlist_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRumReplayPlaylistSessionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRumReplayPlaylistsError is a struct for typed errors of method [`RumReplayPlaylistsAPI::list_rum_replay_playlists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRumReplayPlaylistsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RemoveRumReplaySessionFromPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::remove_rum_replay_session_from_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRumReplaySessionFromPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRumReplayPlaylistError is a struct for typed errors of method [`RumReplayPlaylistsAPI::update_rum_replay_playlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRumReplayPlaylistError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create and manage playlists of RUM replay sessions. Organize, categorize, and share collections of replay sessions for analysis and collaboration.
#[derive(Debug, Clone)]
pub struct RumReplayPlaylistsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumReplayPlaylistsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumReplayPlaylistsAPI {
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

    /// Add a session to a playlist.
    pub async fn add_rum_replay_session_to_playlist(
        &self,
        ts: i64,
        playlist_id: i32,
        session_id: String,
        params: AddRumReplaySessionToPlaylistOptionalParams,
    ) -> Result<
        crate::datadogV2::model::PlaylistsSession,
        datadog::Error<AddRumReplaySessionToPlaylistError>,
    > {
        match self
            .add_rum_replay_session_to_playlist_with_http_info(ts, playlist_id, session_id, params)
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

    /// Add a session to a playlist.
    pub async fn add_rum_replay_session_to_playlist_with_http_info(
        &self,
        ts: i64,
        playlist_id: i32,
        session_id: String,
        params: AddRumReplaySessionToPlaylistOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PlaylistsSession>,
        datadog::Error<AddRumReplaySessionToPlaylistError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.add_rum_replay_session_to_playlist";

        // unbox and build optional parameters
        let data_source = params.data_source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}/sessions/{session_id}",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id,
            session_id = datadog::urlencode(session_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("ts", &ts.to_string())]);
        if let Some(ref local_query_param) = data_source {
            local_req_builder =
                local_req_builder.query(&[("data_source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::PlaylistsSession>(&local_content)
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
            let local_entity: Option<AddRumReplaySessionToPlaylistError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Remove sessions from a playlist.
    pub async fn bulk_remove_rum_replay_playlist_sessions(
        &self,
        playlist_id: i32,
        body: crate::datadogV2::model::SessionIdArray,
    ) -> Result<(), datadog::Error<BulkRemoveRumReplayPlaylistSessionsError>> {
        match self
            .bulk_remove_rum_replay_playlist_sessions_with_http_info(playlist_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove sessions from a playlist.
    pub async fn bulk_remove_rum_replay_playlist_sessions_with_http_info(
        &self,
        playlist_id: i32,
        body: crate::datadogV2::model::SessionIdArray,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<BulkRemoveRumReplayPlaylistSessionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.bulk_remove_rum_replay_playlist_sessions";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}/sessions",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<BulkRemoveRumReplayPlaylistSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a playlist.
    pub async fn create_rum_replay_playlist(
        &self,
        body: crate::datadogV2::model::Playlist,
    ) -> Result<crate::datadogV2::model::Playlist, datadog::Error<CreateRumReplayPlaylistError>>
    {
        match self.create_rum_replay_playlist_with_http_info(body).await {
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

    /// Create a playlist.
    pub async fn create_rum_replay_playlist_with_http_info(
        &self,
        body: crate::datadogV2::model::Playlist,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Playlist>,
        datadog::Error<CreateRumReplayPlaylistError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_rum_replay_playlist";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists",
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
            match serde_json::from_str::<crate::datadogV2::model::Playlist>(&local_content) {
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
            let local_entity: Option<CreateRumReplayPlaylistError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a playlist.
    pub async fn delete_rum_replay_playlist(
        &self,
        playlist_id: i32,
    ) -> Result<(), datadog::Error<DeleteRumReplayPlaylistError>> {
        match self
            .delete_rum_replay_playlist_with_http_info(playlist_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a playlist.
    pub async fn delete_rum_replay_playlist_with_http_info(
        &self,
        playlist_id: i32,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteRumReplayPlaylistError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_rum_replay_playlist";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id
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
            let local_entity: Option<DeleteRumReplayPlaylistError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a playlist.
    pub async fn get_rum_replay_playlist(
        &self,
        playlist_id: i32,
    ) -> Result<crate::datadogV2::model::Playlist, datadog::Error<GetRumReplayPlaylistError>> {
        match self
            .get_rum_replay_playlist_with_http_info(playlist_id)
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

    /// Get a playlist.
    pub async fn get_rum_replay_playlist_with_http_info(
        &self,
        playlist_id: i32,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Playlist>,
        datadog::Error<GetRumReplayPlaylistError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_rum_replay_playlist";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id
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
            match serde_json::from_str::<crate::datadogV2::model::Playlist>(&local_content) {
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
            let local_entity: Option<GetRumReplayPlaylistError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List sessions in a playlist.
    pub async fn list_rum_replay_playlist_sessions(
        &self,
        playlist_id: i32,
        params: ListRumReplayPlaylistSessionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::PlaylistsSessionArray,
        datadog::Error<ListRumReplayPlaylistSessionsError>,
    > {
        match self
            .list_rum_replay_playlist_sessions_with_http_info(playlist_id, params)
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

    /// List sessions in a playlist.
    pub async fn list_rum_replay_playlist_sessions_with_http_info(
        &self,
        playlist_id: i32,
        params: ListRumReplayPlaylistSessionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PlaylistsSessionArray>,
        datadog::Error<ListRumReplayPlaylistSessionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_rum_replay_playlist_sessions";

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}/sessions",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::PlaylistsSessionArray>(
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
            let local_entity: Option<ListRumReplayPlaylistSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List playlists.
    pub async fn list_rum_replay_playlists(
        &self,
        params: ListRumReplayPlaylistsOptionalParams,
    ) -> Result<crate::datadogV2::model::PlaylistArray, datadog::Error<ListRumReplayPlaylistsError>>
    {
        match self.list_rum_replay_playlists_with_http_info(params).await {
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

    /// List playlists.
    pub async fn list_rum_replay_playlists_with_http_info(
        &self,
        params: ListRumReplayPlaylistsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PlaylistArray>,
        datadog::Error<ListRumReplayPlaylistsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_rum_replay_playlists";

        // unbox and build optional parameters
        let filter_created_by_uuid = params.filter_created_by_uuid;
        let filter_query = params.filter_query;
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_created_by_uuid {
            local_req_builder = local_req_builder
                .query(&[("filter[created_by_uuid]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::PlaylistArray>(&local_content) {
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
            let local_entity: Option<ListRumReplayPlaylistsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Remove a session from a playlist.
    pub async fn remove_rum_replay_session_from_playlist(
        &self,
        playlist_id: i32,
        session_id: String,
    ) -> Result<(), datadog::Error<RemoveRumReplaySessionFromPlaylistError>> {
        match self
            .remove_rum_replay_session_from_playlist_with_http_info(playlist_id, session_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove a session from a playlist.
    pub async fn remove_rum_replay_session_from_playlist_with_http_info(
        &self,
        playlist_id: i32,
        session_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<RemoveRumReplaySessionFromPlaylistError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.remove_rum_replay_session_from_playlist";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}/sessions/{session_id}",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id,
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
            let local_entity: Option<RemoveRumReplaySessionFromPlaylistError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a playlist.
    pub async fn update_rum_replay_playlist(
        &self,
        playlist_id: i32,
        body: crate::datadogV2::model::Playlist,
    ) -> Result<crate::datadogV2::model::Playlist, datadog::Error<UpdateRumReplayPlaylistError>>
    {
        match self
            .update_rum_replay_playlist_with_http_info(playlist_id, body)
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

    /// Update a playlist.
    pub async fn update_rum_replay_playlist_with_http_info(
        &self,
        playlist_id: i32,
        body: crate::datadogV2::model::Playlist,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Playlist>,
        datadog::Error<UpdateRumReplayPlaylistError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_rum_replay_playlist";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rum/replay/playlists/{playlist_id}",
            local_configuration.get_operation_host(operation_id),
            playlist_id = playlist_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::Playlist>(&local_content) {
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
            let local_entity: Option<UpdateRumReplayPlaylistError> =
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
