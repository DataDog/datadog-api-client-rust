// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListContainersOptionalParams is a struct for passing parameters to the method [`ContainersAPI::list_containers`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListContainersOptionalParams {
    /// Comma-separated list of tags to filter containers by.
    pub filter_tags: Option<String>,
    /// Comma-separated list of tags to group containers by.
    pub group_by: Option<String>,
    /// Attribute to sort containers by.
    pub sort: Option<String>,
    /// Maximum number of results returned.
    pub page_size: Option<i32>,
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.pagination.next_cursor`.
    pub page_cursor: Option<String>,
}

impl ListContainersOptionalParams {
    /// Comma-separated list of tags to filter containers by.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Comma-separated list of tags to group containers by.
    pub fn group_by(mut self, value: String) -> Self {
        self.group_by = Some(value);
        self
    }
    /// Attribute to sort containers by.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Maximum number of results returned.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.pagination.next_cursor`.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
}

/// ListContainersError is a struct for typed errors of method [`ContainersAPI::list_containers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListContainersError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ContainersAPI {
    config: configuration::Configuration,
}

impl Default for ContainersAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ContainersAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Get all containers for your organization.
    pub async fn list_containers(
        &self,
        params: ListContainersOptionalParams,
    ) -> Result<crate::datadogV2::model::ContainersResponse, Error<ListContainersError>> {
        match self.list_containers_with_http_info(params).await {
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

    /// Get all containers for your organization.
    pub async fn list_containers_with_http_info(
        &self,
        params: ListContainersOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ContainersResponse>,
        Error<ListContainersError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_containers";

        // unbox and build optional parameters
        let filter_tags = params.filter_tags;
        let group_by = params.group_by;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_cursor = params.page_cursor;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/containers",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = group_by {
            local_req_builder =
                local_req_builder.query(&[("group_by", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ContainersResponse>(
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
            let local_entity: Option<ListContainersError> =
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
