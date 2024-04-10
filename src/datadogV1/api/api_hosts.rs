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

/// GetHostTotalsOptionalParams is a struct for passing parameters to the method [`HostsAPI::get_host_totals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHostTotalsOptionalParams {
    /// Number of seconds from which you want to get total number of active hosts.
    pub from: Option<i64>,
}

impl GetHostTotalsOptionalParams {
    /// Number of seconds from which you want to get total number of active hosts.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
}

/// ListHostsOptionalParams is a struct for passing parameters to the method [`HostsAPI::list_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListHostsOptionalParams {
    /// String to filter search results.
    pub filter: Option<String>,
    /// Sort hosts by this field.
    pub sort_field: Option<String>,
    /// Direction of sort. Options include `asc` and `desc`.
    pub sort_dir: Option<String>,
    /// Host result to start search from.
    pub start: Option<i64>,
    /// Number of hosts to return. Max 1000.
    pub count: Option<i64>,
    /// Number of seconds since UNIX epoch from which you want to search your hosts.
    pub from: Option<i64>,
    /// Include information on the muted status of hosts and when the mute expires.
    pub include_muted_hosts_data: Option<bool>,
    /// Include additional metadata about the hosts (agent_version, machine, platform, processor, etc.).
    pub include_hosts_metadata: Option<bool>,
}

impl ListHostsOptionalParams {
    /// String to filter search results.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Sort hosts by this field.
    pub fn sort_field(mut self, value: String) -> Self {
        self.sort_field = Some(value);
        self
    }
    /// Direction of sort. Options include `asc` and `desc`.
    pub fn sort_dir(mut self, value: String) -> Self {
        self.sort_dir = Some(value);
        self
    }
    /// Host result to start search from.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
    /// Number of hosts to return. Max 1000.
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }
    /// Number of seconds since UNIX epoch from which you want to search your hosts.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
    /// Include information on the muted status of hosts and when the mute expires.
    pub fn include_muted_hosts_data(mut self, value: bool) -> Self {
        self.include_muted_hosts_data = Some(value);
        self
    }
    /// Include additional metadata about the hosts (agent_version, machine, platform, processor, etc.).
    pub fn include_hosts_metadata(mut self, value: bool) -> Self {
        self.include_hosts_metadata = Some(value);
        self
    }
}

/// GetHostTotalsError is a struct for typed errors of method [`HostsAPI::get_host_totals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTotalsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListHostsError is a struct for typed errors of method [`HostsAPI::list_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// MuteHostError is a struct for typed errors of method [`HostsAPI::mute_host`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MuteHostError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UnmuteHostError is a struct for typed errors of method [`HostsAPI::unmute_host`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnmuteHostError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct HostsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for HostsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl HostsAPI {
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

    /// This endpoint returns the total number of active and up hosts in your Datadog account.
    /// Active means the host has reported in the past hour, and up means it has reported in the past two hours.
    pub async fn get_host_totals(
        &self,
        params: GetHostTotalsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTotals, datadog::Error<GetHostTotalsError>> {
        match self.get_host_totals_with_http_info(params).await {
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

    /// This endpoint returns the total number of active and up hosts in your Datadog account.
    /// Active means the host has reported in the past hour, and up means it has reported in the past two hours.
    pub async fn get_host_totals_with_http_info(
        &self,
        params: GetHostTotalsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostTotals>,
        datadog::Error<GetHostTotalsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_host_totals";

        // unbox and build optional parameters
        let from = params.from;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/hosts/totals",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::HostTotals>(&local_content) {
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
            let local_entity: Option<GetHostTotalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows searching for hosts by name, alias, or tag.
    /// Hosts live within the past 3 hours are included by default.
    /// Retention is 7 days.
    /// Results are paginated with a max of 1000 results at a time.
    pub async fn list_hosts(
        &self,
        params: ListHostsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostListResponse, datadog::Error<ListHostsError>> {
        match self.list_hosts_with_http_info(params).await {
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

    /// This endpoint allows searching for hosts by name, alias, or tag.
    /// Hosts live within the past 3 hours are included by default.
    /// Retention is 7 days.
    /// Results are paginated with a max of 1000 results at a time.
    pub async fn list_hosts_with_http_info(
        &self,
        params: ListHostsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostListResponse>,
        datadog::Error<ListHostsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_hosts";

        // unbox and build optional parameters
        let filter = params.filter;
        let sort_field = params.sort_field;
        let sort_dir = params.sort_dir;
        let start = params.start;
        let count = params.count;
        let from = params.from;
        let include_muted_hosts_data = params.include_muted_hosts_data;
        let include_hosts_metadata = params.include_hosts_metadata;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/hosts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_field {
            local_req_builder =
                local_req_builder.query(&[("sort_field", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_dir {
            local_req_builder =
                local_req_builder.query(&[("sort_dir", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = count {
            local_req_builder =
                local_req_builder.query(&[("count", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_muted_hosts_data {
            local_req_builder = local_req_builder
                .query(&[("include_muted_hosts_data", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_hosts_metadata {
            local_req_builder = local_req_builder
                .query(&[("include_hosts_metadata", &local_query_param.to_string())]);
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::HostListResponse>(&local_content)
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
            let local_entity: Option<ListHostsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Mute a host. **Note:** This creates a [Downtime V2](<https://docs.datadoghq.com/api/latest/downtimes/#schedule-a-downtime>) for the host.
    pub async fn mute_host(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostMuteSettings,
    ) -> Result<crate::datadogV1::model::HostMuteResponse, datadog::Error<MuteHostError>> {
        match self.mute_host_with_http_info(host_name, body).await {
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

    /// Mute a host. **Note:** This creates a [Downtime V2](<https://docs.datadoghq.com/api/latest/downtimes/#schedule-a-downtime>) for the host.
    pub async fn mute_host_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostMuteSettings,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostMuteResponse>,
        datadog::Error<MuteHostError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.mute_host";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/host/{host_name}/mute",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::HostMuteResponse>(&local_content)
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
            let local_entity: Option<MuteHostError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Unmutes a host. This endpoint takes no JSON arguments.
    pub async fn unmute_host(
        &self,
        host_name: String,
    ) -> Result<crate::datadogV1::model::HostMuteResponse, datadog::Error<UnmuteHostError>> {
        match self.unmute_host_with_http_info(host_name).await {
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

    /// Unmutes a host. This endpoint takes no JSON arguments.
    pub async fn unmute_host_with_http_info(
        &self,
        host_name: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostMuteResponse>,
        datadog::Error<UnmuteHostError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.unmute_host";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/host/{host_name}/unmute",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::HostMuteResponse>(&local_content)
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
            let local_entity: Option<UnmuteHostError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
