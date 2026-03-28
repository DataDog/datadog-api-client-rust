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

/// SearchWidgetsOptionalParams is a struct for passing parameters to the method [`WidgetsAPI::search_widgets`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchWidgetsOptionalParams {
    /// Filter widgets by widget type.
    pub filter_widget_type: Option<crate::datadogV2::model::WidgetType>,
    /// Filter widgets by the email handle of the creator.
    pub filter_creator_handle: Option<String>,
    /// Filter to only widgets favorited by the current user.
    pub filter_is_favorited: Option<bool>,
    /// Filter widgets by title (substring match).
    pub filter_title: Option<String>,
    /// Filter widgets by tags. Format as bracket-delimited CSV, e.g. `[tag1,tag2]`.
    pub filter_tags: Option<String>,
    /// Sort field for the results. Prefix with `-` for descending order.
    /// Allowed values: `title`, `created_at`, `modified_at`.
    pub sort: Option<String>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Number of widgets per page.
    pub page_size: Option<i32>,
}

impl SearchWidgetsOptionalParams {
    /// Filter widgets by widget type.
    pub fn filter_widget_type(mut self, value: crate::datadogV2::model::WidgetType) -> Self {
        self.filter_widget_type = Some(value);
        self
    }
    /// Filter widgets by the email handle of the creator.
    pub fn filter_creator_handle(mut self, value: String) -> Self {
        self.filter_creator_handle = Some(value);
        self
    }
    /// Filter to only widgets favorited by the current user.
    pub fn filter_is_favorited(mut self, value: bool) -> Self {
        self.filter_is_favorited = Some(value);
        self
    }
    /// Filter widgets by title (substring match).
    pub fn filter_title(mut self, value: String) -> Self {
        self.filter_title = Some(value);
        self
    }
    /// Filter widgets by tags. Format as bracket-delimited CSV, e.g. `[tag1,tag2]`.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Sort field for the results. Prefix with `-` for descending order.
    /// Allowed values: `title`, `created_at`, `modified_at`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of widgets per page.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// CreateWidgetError is a struct for typed errors of method [`WidgetsAPI::create_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWidgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteWidgetError is a struct for typed errors of method [`WidgetsAPI::delete_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWidgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetWidgetError is a struct for typed errors of method [`WidgetsAPI::get_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWidgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchWidgetsError is a struct for typed errors of method [`WidgetsAPI::search_widgets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchWidgetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateWidgetError is a struct for typed errors of method [`WidgetsAPI::update_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWidgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create, read, update, and delete saved widgets. Widgets are reusable
/// visualization components stored independently from any dashboard or notebook,
/// partitioned by experience type and identified by a UUID.
#[derive(Debug, Clone)]
pub struct WidgetsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for WidgetsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl WidgetsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        #[allow(unused_mut)]
        let mut reqwest_client_builder = reqwest::Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        #[allow(unused_mut)]
        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        #[cfg(feature = "retry")]
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

    /// Create a new widget for a given experience type.
    pub async fn create_widget(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        body: crate::datadogV2::model::CreateOrUpdateWidgetRequest,
    ) -> Result<crate::datadogV2::model::WidgetResponse, datadog::Error<CreateWidgetError>> {
        match self
            .create_widget_with_http_info(experience_type, body)
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

    /// Create a new widget for a given experience type.
    pub async fn create_widget_with_http_info(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        body: crate::datadogV2::model::CreateOrUpdateWidgetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::WidgetResponse>,
        datadog::Error<CreateWidgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_widget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/widgets/{experience_type}",
            local_configuration.get_operation_host(operation_id),
            experience_type = datadog::urlencode(experience_type.to_string())
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::WidgetResponse>(&local_content) {
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
            let local_entity: Option<CreateWidgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Soft-delete a widget by its UUID for a given experience type.
    pub async fn delete_widget(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteWidgetError>> {
        match self
            .delete_widget_with_http_info(experience_type, uuid)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Soft-delete a widget by its UUID for a given experience type.
    pub async fn delete_widget_with_http_info(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteWidgetError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_widget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/widgets/{experience_type}/{uuid}",
            local_configuration.get_operation_host(operation_id),
            experience_type = datadog::urlencode(experience_type.to_string()),
            uuid = datadog::urlencode(uuid.to_string())
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
            let local_entity: Option<DeleteWidgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a widget by its UUID for a given experience type.
    pub async fn get_widget(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::WidgetResponse, datadog::Error<GetWidgetError>> {
        match self.get_widget_with_http_info(experience_type, uuid).await {
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

    /// Retrieve a widget by its UUID for a given experience type.
    pub async fn get_widget_with_http_info(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::WidgetResponse>,
        datadog::Error<GetWidgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_widget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/widgets/{experience_type}/{uuid}",
            local_configuration.get_operation_host(operation_id),
            experience_type = datadog::urlencode(experience_type.to_string()),
            uuid = datadog::urlencode(uuid.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::WidgetResponse>(&local_content) {
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
            let local_entity: Option<GetWidgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search and list widgets for a given experience type. Supports filtering by widget type, creator, title, and tags, as well as sorting and pagination.
    pub async fn search_widgets(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        params: SearchWidgetsOptionalParams,
    ) -> Result<crate::datadogV2::model::WidgetListResponse, datadog::Error<SearchWidgetsError>>
    {
        match self
            .search_widgets_with_http_info(experience_type, params)
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

    /// Search and list widgets for a given experience type. Supports filtering by widget type, creator, title, and tags, as well as sorting and pagination.
    pub async fn search_widgets_with_http_info(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        params: SearchWidgetsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::WidgetListResponse>,
        datadog::Error<SearchWidgetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_widgets";

        // unbox and build optional parameters
        let filter_widget_type = params.filter_widget_type;
        let filter_creator_handle = params.filter_creator_handle;
        let filter_is_favorited = params.filter_is_favorited;
        let filter_title = params.filter_title;
        let filter_tags = params.filter_tags;
        let sort = params.sort;
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/widgets/{experience_type}",
            local_configuration.get_operation_host(operation_id),
            experience_type = datadog::urlencode(experience_type.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_widget_type {
            local_req_builder =
                local_req_builder.query(&[("filter[widgetType]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_creator_handle {
            local_req_builder = local_req_builder
                .query(&[("filter[creatorHandle]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_is_favorited {
            local_req_builder =
                local_req_builder.query(&[("filter[isFavorited]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_title {
            local_req_builder =
                local_req_builder.query(&[("filter[title]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::WidgetListResponse>(
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
            let local_entity: Option<SearchWidgetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a widget by its UUID for a given experience type. This performs a full replacement of the widget definition.
    pub async fn update_widget(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
        body: crate::datadogV2::model::CreateOrUpdateWidgetRequest,
    ) -> Result<crate::datadogV2::model::WidgetResponse, datadog::Error<UpdateWidgetError>> {
        match self
            .update_widget_with_http_info(experience_type, uuid, body)
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

    /// Update a widget by its UUID for a given experience type. This performs a full replacement of the widget definition.
    pub async fn update_widget_with_http_info(
        &self,
        experience_type: crate::datadogV2::model::WidgetExperienceType,
        uuid: uuid::Uuid,
        body: crate::datadogV2::model::CreateOrUpdateWidgetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::WidgetResponse>,
        datadog::Error<UpdateWidgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_widget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/widgets/{experience_type}/{uuid}",
            local_configuration.get_operation_host(operation_id),
            experience_type = datadog::urlencode(experience_type.to_string()),
            uuid = datadog::urlencode(uuid.to_string())
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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::WidgetResponse>(&local_content) {
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
            let local_entity: Option<UpdateWidgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
