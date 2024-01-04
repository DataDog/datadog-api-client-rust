// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListContainersParams is a struct for passing parameters to the method [`ListContainers`]
#[derive(Clone, Debug)]
pub struct ListContainersParams {
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

/// ListContainersError is a struct for typed errors of method [`ListContainers`]
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
        params: ListContainersParams,
    ) -> Result<Option<crate::datadogV2::model::ContainersResponse>, Error<ListContainersError>>
    {
        match self.list_containers_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all containers for your organization.
    pub async fn list_containers_with_http_info(
        &self,
        params: ListContainersParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ContainersResponse>,
        Error<ListContainersError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_tags = params.filter_tags;
        let group_by = params.group_by;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_cursor = params.page_cursor;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/containers", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = group_by {
            local_req_builder = local_req_builder.query(&[("group_by", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ContainersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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