// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetHostTotalsOptionalParams is a struct for passing parameters to the method [`HostsAPI::get_host_totals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHostTotalsOptionalParams {
    /// Number of seconds from which you want to get total number of active hosts.
    pub from: Option<i64>,
}

impl GetHostTotalsOptionalParams {
    /// Number of seconds from which you want to get total number of active hosts.
    pub fn from(&mut self, value: i64) -> &mut Self {
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
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
    /// Sort hosts by this field.
    pub fn sort_field(&mut self, value: String) -> &mut Self {
        self.sort_field = Some(value);
        self
    }
    /// Direction of sort. Options include `asc` and `desc`.
    pub fn sort_dir(&mut self, value: String) -> &mut Self {
        self.sort_dir = Some(value);
        self
    }
    /// Host result to start search from.
    pub fn start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }
    /// Number of hosts to return. Max 1000.
    pub fn count(&mut self, value: i64) -> &mut Self {
        self.count = Some(value);
        self
    }
    /// Number of seconds since UNIX epoch from which you want to search your hosts.
    pub fn from(&mut self, value: i64) -> &mut Self {
        self.from = Some(value);
        self
    }
    /// Include information on the muted status of hosts and when the mute expires.
    pub fn include_muted_hosts_data(&mut self, value: bool) -> &mut Self {
        self.include_muted_hosts_data = Some(value);
        self
    }
    /// Include additional metadata about the hosts (agent_version, machine, platform, processor, etc.).
    pub fn include_hosts_metadata(&mut self, value: bool) -> &mut Self {
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
    config: configuration::Configuration,
}

impl Default for HostsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl HostsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// This endpoint returns the total number of active and up hosts in your Datadog account.
    /// Active means the host has reported in the past hour, and up means it has reported in the past two hours.
    pub async fn get_host_totals(
        &self,
        params: GetHostTotalsOptionalParams,
    ) -> Result<Option<crate::datadogV1::model::HostTotals>, Error<GetHostTotalsError>> {
        match self.get_host_totals_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint returns the total number of active and up hosts in your Datadog account.
    /// Active means the host has reported in the past hour, and up means it has reported in the past two hours.
    pub async fn get_host_totals_with_http_info(
        &self,
        params: GetHostTotalsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTotals>, Error<GetHostTotalsError>>
    {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let from = params.from;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/hosts/totals", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostTotals> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetHostTotalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows searching for hosts by name, alias, or tag.
    /// Hosts live within the past 3 hours are included by default.
    /// Retention is 7 days.
    /// Results are paginated with a max of 1000 results at a time.
    pub async fn list_hosts(
        &self,
        params: ListHostsOptionalParams,
    ) -> Result<Option<crate::datadogV1::model::HostListResponse>, Error<ListHostsError>> {
        match self.list_hosts_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
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
    ) -> Result<ResponseContent<crate::datadogV1::model::HostListResponse>, Error<ListHostsError>>
    {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter = params.filter;
        let sort_field = params.sort_field;
        let sort_dir = params.sort_dir;
        let start = params.start;
        let count = params.count;
        let from = params.from;
        let include_muted_hosts_data = params.include_muted_hosts_data;
        let include_hosts_metadata = params.include_hosts_metadata;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/hosts", local_configuration.base_path);
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

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListHostsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Mute a host. **Note:** This creates a [Downtime V2](<https://docs.datadoghq.com/api/latest/downtimes/#schedule-a-downtime>) for the host.
    pub async fn mute_host(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostMuteSettings,
    ) -> Result<Option<crate::datadogV1::model::HostMuteResponse>, Error<MuteHostError>> {
        match self.mute_host_with_http_info(host_name, body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Mute a host. **Note:** This creates a [Downtime V2](<https://docs.datadoghq.com/api/latest/downtimes/#schedule-a-downtime>) for the host.
    pub async fn mute_host_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostMuteSettings,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostMuteResponse>, Error<MuteHostError>>
    {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/host/{host_name}/mute",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostMuteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<MuteHostError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Unmutes a host. This endpoint takes no JSON arguments.
    pub async fn unmute_host(
        &self,
        host_name: String,
    ) -> Result<Option<crate::datadogV1::model::HostMuteResponse>, Error<UnmuteHostError>> {
        match self.unmute_host_with_http_info(host_name).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Unmutes a host. This endpoint takes no JSON arguments.
    pub async fn unmute_host_with_http_info(
        &self,
        host_name: String,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostMuteResponse>, Error<UnmuteHostError>>
    {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/host/{host_name}/unmute",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostMuteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UnmuteHostError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
