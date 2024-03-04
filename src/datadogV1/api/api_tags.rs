// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

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
    pub fn source(&mut self, value: String) -> &mut Self {
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
    pub fn source(&mut self, value: String) -> &mut Self {
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
    pub fn source(&mut self, value: String) -> &mut Self {
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
    pub fn source(&mut self, value: String) -> &mut Self {
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
    pub fn source(&mut self, value: String) -> &mut Self {
        self.source = Some(value);
        self
    }
}

/// CreateHostTagsError is a struct for typed errors of method [`TagsAPI::create_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteHostTagsError is a struct for typed errors of method [`TagsAPI::delete_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHostTagsError is a struct for typed errors of method [`TagsAPI::get_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListHostTagsError is a struct for typed errors of method [`TagsAPI::list_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateHostTagsError is a struct for typed errors of method [`TagsAPI::update_host_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct TagsAPI {
    config: configuration::Configuration,
}

impl Default for TagsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl TagsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// This endpoint allows you to add new tags to a host,
    /// optionally specifying where these tags come from.
    pub async fn create_host_tags(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: CreateHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, Error<CreateHostTagsError>> {
        match self
            .create_host_tags_with_http_info(host_name, body, params)
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

    /// This endpoint allows you to add new tags to a host,
    /// optionally specifying where these tags come from.
    pub async fn create_host_tags_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: CreateHostTagsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<CreateHostTagsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.create_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<CreateHostTagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows you to remove all user-assigned tags
    /// for a single host.
    pub async fn delete_host_tags(
        &self,
        host_name: String,
        params: DeleteHostTagsOptionalParams,
    ) -> Result<(), Error<DeleteHostTagsError>> {
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
    ) -> Result<ResponseContent<()>, Error<DeleteHostTagsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteHostTagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Return the list of tags that apply to a given host.
    pub async fn get_host_tags(
        &self,
        host_name: String,
        params: GetHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, Error<GetHostTagsError>> {
        match self.get_host_tags_with_http_info(host_name, params).await {
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

    /// Return the list of tags that apply to a given host.
    pub async fn get_host_tags_with_http_info(
        &self,
        host_name: String,
        params: GetHostTagsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<GetHostTagsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<GetHostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Return a mapping of tags to hosts for your whole infrastructure.
    pub async fn list_host_tags(
        &self,
        params: ListHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::TagToHosts, Error<ListHostTagsError>> {
        match self.list_host_tags_with_http_info(params).await {
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

    /// Return a mapping of tags to hosts for your whole infrastructure.
    pub async fn list_host_tags_with_http_info(
        &self,
        params: ListHostTagsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::TagToHosts>, Error<ListHostTagsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.list_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &local_configuration.client;

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
            match serde_json::from_str::<crate::datadogV1::model::TagToHosts>(&local_content) {
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
            let local_entity: Option<ListHostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows you to update/replace all tags in
    /// an integration source with those supplied in the request.
    pub async fn update_host_tags(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: UpdateHostTagsOptionalParams,
    ) -> Result<crate::datadogV1::model::HostTags, Error<UpdateHostTagsError>> {
        match self
            .update_host_tags_with_http_info(host_name, body, params)
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

    /// This endpoint allows you to update/replace all tags in
    /// an integration source with those supplied in the request.
    pub async fn update_host_tags_with_http_info(
        &self,
        host_name: String,
        body: crate::datadogV1::model::HostTags,
        params: UpdateHostTagsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<UpdateHostTagsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.update_host_tags";

        // unbox and build optional parameters
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.get_operation_host(operation_id),
            host_name = urlencode(host_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = source {
            local_req_builder =
                local_req_builder.query(&[("source", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::HostTags>(&local_content) {
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
            let local_entity: Option<UpdateHostTagsError> =
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
