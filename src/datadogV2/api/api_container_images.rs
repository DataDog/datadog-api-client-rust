// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use reqwest::header::HeaderMap;
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
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Comma-separated list of tags to group Container Images by.
    pub fn group_by(mut self, value: String) -> Self {
        self.group_by = Some(value);
        self
    }
    /// Attribute to sort Container Images by.
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ContainerImagesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl ContainerImagesAPI {
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

    /// Get all Container Images for your organization.
    pub async fn list_container_images(
        &self,
        params: ListContainerImagesOptionalParams,
    ) -> Result<crate::datadogV2::model::ContainerImagesResponse, Error<ListContainerImagesError>>
    {
        match self.list_container_images_with_http_info(params).await {
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

    pub fn list_container_images_with_pagination(
        &self,
        mut params: ListContainerImagesOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::ContainerImageItem, Error<ListContainerImagesError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 1000;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.list_container_images(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(pagination) = meta.pagination else { break };
                let Some(next_cursor) = pagination.next_cursor else { break };

                params.page_cursor = Some(next_cursor);
            }
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
        let operation_id = "v2.list_container_images";

        // unbox and build optional parameters
        let filter_tags = params.filter_tags;
        let group_by = params.group_by;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_cursor = params.page_cursor;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/container_images",
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
