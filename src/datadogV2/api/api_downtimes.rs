// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetDowntimeOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::get_downtime`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetDowntimeOptionalParams {
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub include: Option<String>,
}

impl GetDowntimeOptionalParams {
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListDowntimesOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::list_downtimes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListDowntimesOptionalParams {
    /// Only return downtimes that are active when the request is made.
    pub current_only: Option<bool>,
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub include: Option<String>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of downtimes in the response.
    pub page_limit: Option<i64>,
}

impl ListDowntimesOptionalParams {
    /// Only return downtimes that are active when the request is made.
    pub fn current_only(mut self, value: bool) -> Self {
        self.current_only = Some(value);
        self
    }
    /// Comma-separated list of resource paths for related resources to include in the response. Supported resource
    /// paths are `created_by` and `monitor`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of downtimes in the response.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListMonitorDowntimesOptionalParams is a struct for passing parameters to the method [`DowntimesAPI::list_monitor_downtimes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMonitorDowntimesOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of downtimes in the response.
    pub page_limit: Option<i64>,
}

impl ListMonitorDowntimesOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of downtimes in the response.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// CancelDowntimeError is a struct for typed errors of method [`DowntimesAPI::cancel_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimeError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateDowntimeError is a struct for typed errors of method [`DowntimesAPI::create_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDowntimeError is a struct for typed errors of method [`DowntimesAPI::get_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDowntimesError is a struct for typed errors of method [`DowntimesAPI::list_downtimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDowntimesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMonitorDowntimesError is a struct for typed errors of method [`DowntimesAPI::list_monitor_downtimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorDowntimesError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDowntimeError is a struct for typed errors of method [`DowntimesAPI::update_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDowntimeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DowntimesAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for DowntimesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl DowntimesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
        let client = middleware_client_builder.build();
        Self { config, client }
    }

    pub fn with_client_and_config(
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime(
        &self,
        downtime_id: String,
    ) -> Result<(), Error<CancelDowntimeError>> {
        match self.cancel_downtime_with_http_info(downtime_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime_with_http_info(
        &self,
        downtime_id: String,
    ) -> Result<ResponseContent<()>, Error<CancelDowntimeError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.cancel_downtime";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            "*/*".parse().expect("failed to parse Accept header"),
        );

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<CancelDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime(
        &self,
        body: crate::datadogV2::model::DowntimeCreateRequest,
    ) -> Result<crate::datadogV2::model::DowntimeResponse, Error<CreateDowntimeError>> {
        match self.create_downtime_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime_with_http_info(
        &self,
        body: crate::datadogV2::model::DowntimeCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DowntimeResponse>,
        Error<CreateDowntimeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_downtime";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
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
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DowntimeResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime(
        &self,
        downtime_id: String,
        params: GetDowntimeOptionalParams,
    ) -> Result<crate::datadogV2::model::DowntimeResponse, Error<GetDowntimeError>> {
        match self.get_downtime_with_http_info(downtime_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime_with_http_info(
        &self,
        downtime_id: String,
        params: GetDowntimeOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::DowntimeResponse>, Error<GetDowntimeError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_downtime";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DowntimeResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetDowntimeError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes(
        &self,
        params: ListDowntimesOptionalParams,
    ) -> Result<crate::datadogV2::model::ListDowntimesResponse, Error<ListDowntimesError>> {
        match self.list_downtimes_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_downtimes_with_pagination(
        &self,
        mut params: ListDowntimesOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::DowntimeResponseData, Error<ListDowntimesError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 30;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_downtimes(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes_with_http_info(
        &self,
        params: ListDowntimesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListDowntimesResponse>,
        Error<ListDowntimesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_downtimes";

        // unbox and build optional parameters
        let current_only = params.current_only;
        let include = params.include;
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = current_only {
            local_req_builder =
                local_req_builder.query(&[("current_only", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListDowntimesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes(
        &self,
        monitor_id: i64,
        params: ListMonitorDowntimesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::MonitorDowntimeMatchResponse,
        Error<ListMonitorDowntimesError>,
    > {
        match self
            .list_monitor_downtimes_with_http_info(monitor_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_monitor_downtimes_with_pagination(
        &self,
        monitor_id: i64,
        mut params: ListMonitorDowntimesOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::MonitorDowntimeMatchResponseData,
            Error<ListMonitorDowntimesError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 30;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_monitor_downtimes( monitor_id.clone(),params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes_with_http_info(
        &self,
        monitor_id: i64,
        params: ListMonitorDowntimesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MonitorDowntimeMatchResponse>,
        Error<ListMonitorDowntimesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_monitor_downtimes";

        // unbox and build optional parameters
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/monitor/{monitor_id}/downtime_matches",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::MonitorDowntimeMatchResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListMonitorDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a downtime by `downtime_id`.
    pub async fn update_downtime(
        &self,
        downtime_id: String,
        body: crate::datadogV2::model::DowntimeUpdateRequest,
    ) -> Result<crate::datadogV2::model::DowntimeResponse, Error<UpdateDowntimeError>> {
        match self.update_downtime_with_http_info(downtime_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a downtime by `downtime_id`.
    pub async fn update_downtime_with_http_info(
        &self,
        downtime_id: String,
        body: crate::datadogV2::model::DowntimeUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DowntimeResponse>,
        Error<UpdateDowntimeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_downtime";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/downtime/{downtime_id}",
            local_configuration.get_operation_host(operation_id),
            downtime_id = urlencode(downtime_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
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
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DowntimeResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
