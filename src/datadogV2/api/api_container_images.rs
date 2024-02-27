// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListContainerImagesOptionalParams is a struct for passing parameters to the method [`ContainerImagesAPI::list_container_images`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListContainerImagesOptionalParams {
    /// Comma-separated list of tags to filter Container Images by.
    pub filter_tags: Option<String>,
    /// Comma-separated list of tags to group Container Images by.
    pub group_by: Option<String>,
    /// Attribute to sort Container Images by.
    pub sort: Option<String>,
    /// Maximum number of results returned.
    pub page_size: Option<i32>,
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.pagination.next_cursor`.
    pub page_cursor: Option<String>,
}

impl ListContainerImagesOptionalParams {
    /// Comma-separated list of tags to filter Container Images by.
    pub fn filter_tags(&mut self, value: String) -> &mut Self {
        self.filter_tags = Some(value);
        self
    }
    /// Comma-separated list of tags to group Container Images by.
    pub fn group_by(&mut self, value: String) -> &mut Self {
        self.group_by = Some(value);
        self
    }
    /// Attribute to sort Container Images by.
    pub fn sort(&mut self, value: String) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Maximum number of results returned.
    pub fn page_size(&mut self, value: i32) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// String to query the next page of results.
    /// This key is provided with each valid response from the API in `meta.pagination.next_cursor`.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
}

/// ListContainerImagesError is a struct for typed errors of method [`ContainerImagesAPI::list_container_images`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListContainerImagesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ContainerImagesAPI {
    config: configuration::Configuration,
}

impl Default for ContainerImagesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ContainerImagesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Get all Container Images for your organization.
    pub async fn list_container_images(
        &self,
        params: ListContainerImagesOptionalParams,
    ) -> Result<crate::datadogV2::model::ContainerImagesResponse, Error<ListContainerImagesError>>
    {
        match self.list_container_images_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get all Container Images for your organization.
    pub async fn list_container_images_with_http_info(
        &self,
        params: ListContainerImagesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ContainerImagesResponse>,
        Error<ListContainerImagesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_tags = params.filter_tags;
        let group_by = params.group_by;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_cursor = params.page_cursor;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/container_images", local_configuration.base_path);
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
            match serde_json::from_str::<crate::datadogV2::model::ContainerImagesResponse>(
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
            let local_entity: Option<ListContainerImagesError> =
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
