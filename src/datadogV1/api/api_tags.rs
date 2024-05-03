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

/// CreateHostTagsOptionalParams is a struct for passing parameters to the method [`TagsAPI::create_host_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateHostTagsOptionalParams {
    /// The source of the tags.
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>).
    pub source: Option<String>,
}

impl CreateHostTagsOptionalParams {
    /// The source of the tags.
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>).
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

/// DeleteHostTagsOptionalParams is a struct for passing parameters to the method [`TagsAPI::delete_host_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteHostTagsOptionalParams {
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>).
    pub source: Option<String>,
}

impl DeleteHostTagsOptionalParams {
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>).
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

/// GetHostTagsOptionalParams is a struct for passing parameters to the method [`TagsAPI::get_host_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHostTagsOptionalParams {
    /// Source to filter.
    pub source: Option<String>,
}

impl GetHostTagsOptionalParams {
    /// Source to filter.
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

/// ListHostTagsOptionalParams is a struct for passing parameters to the method [`TagsAPI::list_host_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListHostTagsOptionalParams {
    /// When specified, filters host list to those tags with the specified source.
    pub source: Option<String>,
}

impl ListHostTagsOptionalParams {
    /// When specified, filters host list to those tags with the specified source.
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

/// UpdateHostTagsOptionalParams is a struct for passing parameters to the method [`TagsAPI::update_host_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateHostTagsOptionalParams {
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>)
    pub source: Option<String>,
}

impl UpdateHostTagsOptionalParams {
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>)
    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }
}

/// CreateHostTagsError is a struct for typed errors of method [`TagsAPI::create_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHostTagsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteHostTagsError is a struct for typed errors of method [`TagsAPI::delete_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHostTagsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHostTagsError is a struct for typed errors of method [`TagsAPI::get_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTagsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListHostTagsError is a struct for typed errors of method [`TagsAPI::list_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostTagsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateHostTagsError is a struct for typed errors of method [`TagsAPI::update_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateHostTagsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The tag endpoint allows you to assign tags to hosts,
/// for example: `role:database`. Those tags are applied to
/// all metrics sent by the host. Refer to hosts by name
/// (`yourhost.example.com`) when fetching and applying
/// tags to a particular host.
///
/// The component of your infrastructure responsible for a tag is identified
/// by a source. For example, some valid sources include nagios, hudson, jenkins,
/// users, feed, chef, puppet, git, bitbucket, fabric, capistrano, etc.
///
/// Read more about tags on [Getting Started with Tags](<https://docs.datadoghq.com/getting_started/tagging/>).
#[derive(Debug, Clone)]
pub struct TagsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for TagsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl TagsAPI {
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

    /// This endpoint allows you to add new tags to a host,
    /// optionally specifying where these tags come from.
    pub async fn create_host_tags(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: CreateHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, datadog::Error<CreateHostTagsError>> {
        match self
            .create_host_tags_with_http_info(host_name, body, params)
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

    /// This endpoint allows you to add new tags to a host,
    /// optionally specifying where these tags come from.
    pub async fn create_host_tags_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: CreateHostTagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostTags>,
        datadog::Error<CreateHostTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<CreateHostTagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows you to remove all user-assigned tags
    /// for a single host.
    pub async fn delete_host_tags(
        &self,
        host_name: String,
        params: DeleteHostTagsOptionalParams,
    ) -> Result<(), datadog::Error<DeleteHostTagsError>> {
        match self
            .delete_host_tags_with_http_info(host_name, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to remove all user-assigned tags
    /// for a single host.
    pub async fn delete_host_tags_with_http_info(
        &self,
        host_name: String,
        params: DeleteHostTagsOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteHostTagsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<DeleteHostTagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Return the list of tags that apply to a given host.
    pub async fn get_host_tags(
        &self,
        host_name: String,
        params: GetHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, datadog::Error<GetHostTagsError>> {
        match self.get_host_tags_with_http_info(host_name, params).await {
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

    /// Return the list of tags that apply to a given host.
    pub async fn get_host_tags_with_http_info(
        &self,
        host_name: String,
        params: GetHostTagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostTags>,
        datadog::Error<GetHostTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<GetHostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Return a mapping of tags to hosts for your whole infrastructure.
    pub async fn list_host_tags(
        &self,
        params: ListHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::TagToHosts, datadog::Error<ListHostTagsError>> {
        match self.list_host_tags_with_http_info(params).await {
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

    /// Return a mapping of tags to hosts for your whole infrastructure.
    pub async fn list_host_tags_with_http_info(
        &self,
        params: ListHostTagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::TagToHosts>,
        datadog::Error<ListHostTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::TagToHosts>(&local_content) {
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
            let local_entity: Option<ListHostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows you to update/replace all tags in
    /// an integration source with those supplied in the request.
    pub async fn update_host_tags(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: UpdateHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, datadog::Error<UpdateHostTagsError>> {
        match self
            .update_host_tags_with_http_info(host_name, body, params)
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

    /// This endpoint allows you to update/replace all tags in
    /// an integration source with those supplied in the request.
    pub async fn update_host_tags_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: UpdateHostTagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HostTags>,
        datadog::Error<UpdateHostTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = datadog::urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<UpdateHostTagsError> =
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
