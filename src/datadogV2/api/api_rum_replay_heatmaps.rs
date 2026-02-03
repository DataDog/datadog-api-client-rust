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

/// ListReplayHeatmapSnapshotsOptionalParams is a struct for passing parameters to the method [`RumReplayHeatmapsAPI::list_replay_heatmap_snapshots`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListReplayHeatmapSnapshotsOptionalParams {
    /// Device type to filter snapshots.
    pub filter_device_type: Option<String>,
    /// Maximum number of snapshots to return.
    pub page_limit: Option<i32>,
    /// Filter by application ID.
    pub filter_application_id: Option<String>,
}

impl ListReplayHeatmapSnapshotsOptionalParams {
    /// Device type to filter snapshots.
    pub fn filter_device_type(mut self, value: String) -> Self {
        self.filter_device_type = Some(value);
        self
    }
    /// Maximum number of snapshots to return.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Filter by application ID.
    pub fn filter_application_id(mut self, value: String) -> Self {
        self.filter_application_id = Some(value);
        self
    }
}

/// CreateReplayHeatmapSnapshotError is a struct for typed errors of method [`RumReplayHeatmapsAPI::create_replay_heatmap_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateReplayHeatmapSnapshotError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteReplayHeatmapSnapshotError is a struct for typed errors of method [`RumReplayHeatmapsAPI::delete_replay_heatmap_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteReplayHeatmapSnapshotError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListReplayHeatmapSnapshotsError is a struct for typed errors of method [`RumReplayHeatmapsAPI::list_replay_heatmap_snapshots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListReplayHeatmapSnapshotsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateReplayHeatmapSnapshotError is a struct for typed errors of method [`RumReplayHeatmapsAPI::update_replay_heatmap_snapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateReplayHeatmapSnapshotError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage heatmap snapshots for RUM replay sessions. Create, update, delete, and retrieve snapshots to visualize user interactions on specific views.
#[derive(Debug, Clone)]
pub struct RumReplayHeatmapsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumReplayHeatmapsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumReplayHeatmapsAPI {
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

    /// Create a heatmap snapshot.
    pub async fn create_replay_heatmap_snapshot(
        &self,
        body: crate::datadogV2::model::SnapshotCreateRequest,
    ) -> Result<crate::datadogV2::model::Snapshot, datadog::Error<CreateReplayHeatmapSnapshotError>>
    {
        match self
            .create_replay_heatmap_snapshot_with_http_info(body)
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

    /// Create a heatmap snapshot.
    pub async fn create_replay_heatmap_snapshot_with_http_info(
        &self,
        body: crate::datadogV2::model::SnapshotCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Snapshot>,
        datadog::Error<CreateReplayHeatmapSnapshotError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_replay_heatmap_snapshot";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/heatmap/snapshots",
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
            match serde_json::from_str::<crate::datadogV2::model::Snapshot>(&local_content) {
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
            let local_entity: Option<CreateReplayHeatmapSnapshotError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a heatmap snapshot.
    pub async fn delete_replay_heatmap_snapshot(
        &self,
        snapshot_id: String,
    ) -> Result<(), datadog::Error<DeleteReplayHeatmapSnapshotError>> {
        match self
            .delete_replay_heatmap_snapshot_with_http_info(snapshot_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a heatmap snapshot.
    pub async fn delete_replay_heatmap_snapshot_with_http_info(
        &self,
        snapshot_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteReplayHeatmapSnapshotError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_replay_heatmap_snapshot";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/heatmap/snapshots/{snapshot_id}",
            local_configuration.get_operation_host(operation_id),
            snapshot_id = datadog::urlencode(snapshot_id)
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
            let local_entity: Option<DeleteReplayHeatmapSnapshotError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List heatmap snapshots.
    pub async fn list_replay_heatmap_snapshots(
        &self,
        filter_view_name: String,
        params: ListReplayHeatmapSnapshotsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SnapshotArray,
        datadog::Error<ListReplayHeatmapSnapshotsError>,
    > {
        match self
            .list_replay_heatmap_snapshots_with_http_info(filter_view_name, params)
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

    /// List heatmap snapshots.
    pub async fn list_replay_heatmap_snapshots_with_http_info(
        &self,
        filter_view_name: String,
        params: ListReplayHeatmapSnapshotsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SnapshotArray>,
        datadog::Error<ListReplayHeatmapSnapshotsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_replay_heatmap_snapshots";

        // unbox and build optional parameters
        let filter_device_type = params.filter_device_type;
        let page_limit = params.page_limit;
        let filter_application_id = params.filter_application_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/heatmap/snapshots",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[view_name]", &filter_view_name.to_string())]);
        if let Some(ref local_query_param) = filter_device_type {
            local_req_builder =
                local_req_builder.query(&[("filter[device_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::SnapshotArray>(&local_content) {
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
            let local_entity: Option<ListReplayHeatmapSnapshotsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a heatmap snapshot.
    pub async fn update_replay_heatmap_snapshot(
        &self,
        snapshot_id: String,
        body: crate::datadogV2::model::SnapshotUpdateRequest,
    ) -> Result<crate::datadogV2::model::Snapshot, datadog::Error<UpdateReplayHeatmapSnapshotError>>
    {
        match self
            .update_replay_heatmap_snapshot_with_http_info(snapshot_id, body)
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

    /// Update a heatmap snapshot.
    pub async fn update_replay_heatmap_snapshot_with_http_info(
        &self,
        snapshot_id: String,
        body: crate::datadogV2::model::SnapshotUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Snapshot>,
        datadog::Error<UpdateReplayHeatmapSnapshotError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_replay_heatmap_snapshot";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/heatmap/snapshots/{snapshot_id}",
            local_configuration.get_operation_host(operation_id),
            snapshot_id = datadog::urlencode(snapshot_id)
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
            match serde_json::from_str::<crate::datadogV2::model::Snapshot>(&local_content) {
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
            let local_entity: Option<UpdateReplayHeatmapSnapshotError> =
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
