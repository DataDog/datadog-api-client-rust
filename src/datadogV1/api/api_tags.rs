// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateHostTagsParams is a struct for passing parameters to the method [`CreateHostTags`]
#[derive(Clone, Debug, Default)]
pub struct CreateHostTagsParams {
    /// This endpoint allows you to add new tags to a host, optionally specifying where the tags came from.
    pub host_name: String,
    /// Update host tags request body.
    pub body: crate::datadogV1::model::HostTags,
    /// The source of the tags.
    /// [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    pub source: Option<String>,
}

/// DeleteHostTagsParams is a struct for passing parameters to the method [`DeleteHostTags`]
#[derive(Clone, Debug, Default)]
pub struct DeleteHostTagsParams {
    /// This endpoint allows you to remove all user-assigned tags for a single host.
    pub host_name: String,
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    pub source: Option<String>,
}

/// GetHostTagsParams is a struct for passing parameters to the method [`GetHostTags`]
#[derive(Clone, Debug, Default)]
pub struct GetHostTagsParams {
    /// When specified, filters list of tags to those tags with the specified source.
    pub host_name: String,
    /// Source to filter.
    pub source: Option<String>,
}

/// ListHostTagsParams is a struct for passing parameters to the method [`ListHostTags`]
#[derive(Clone, Debug, Default)]
pub struct ListHostTagsParams {
    /// When specified, filters host list to those tags with the specified source.
    pub source: Option<String>,
}

/// UpdateHostTagsParams is a struct for passing parameters to the method [`UpdateHostTags`]
#[derive(Clone, Debug, Default)]
pub struct UpdateHostTagsParams {
    /// This endpoint allows you to update/replace all in an integration source with those supplied in the request.
    pub host_name: String,
    /// Add tags to host
    pub body: crate::datadogV1::model::HostTags,
    /// The source of the tags (for example chef, puppet).
    /// [Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value)
    pub source: Option<String>,
}

/// CreateHostTagsError is a struct for typed errors of method [`CreateHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteHostTagsError is a struct for typed errors of method [`DeleteHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHostTagsError is a struct for typed errors of method [`GetHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListHostTagsError is a struct for typed errors of method [`ListHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostTagsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateHostTagsError is a struct for typed errors of method [`UpdateHostTags`]
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
        params: CreateHostTagsParams,
    ) -> Result<Option<crate::datadogV1::model::HostTags>, Error<CreateHostTagsError>> {
        match self.create_host_tags_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to add new tags to a host,
    /// optionally specifying where these tags come from.
    pub async fn create_host_tags_with_http_info(
        &self,
        params: CreateHostTagsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<CreateHostTagsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let host_name = params.host_name;
        let body = params.body;
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostTags> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateHostTagsError> = serde_json::from_str(&local_content).ok();
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
        params: DeleteHostTagsParams,
    ) -> Result<Option<()>, Error<DeleteHostTagsError>> {
        match self.delete_host_tags_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to remove all user-assigned tags
    /// for a single host.
    pub async fn delete_host_tags_with_http_info(
        &self,
        params: DeleteHostTagsParams,
    ) -> Result<ResponseContent<()>, Error<DeleteHostTagsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let host_name = params.host_name;
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteHostTagsError> = serde_json::from_str(&local_content).ok();
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
        params: GetHostTagsParams,
    ) -> Result<Option<crate::datadogV1::model::HostTags>, Error<GetHostTagsError>> {
        match self.get_host_tags_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Return the list of tags that apply to a given host.
    pub async fn get_host_tags_with_http_info(
        &self,
        params: GetHostTagsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<GetHostTagsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let host_name = params.host_name;
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::HostTags> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: ListHostTagsParams,
    ) -> Result<Option<crate::datadogV1::model::TagToHosts>, Error<ListHostTagsError>> {
        match self.list_host_tags_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Return a mapping of tags to hosts for your whole infrastructure.
    pub async fn list_host_tags_with_http_info(
        &self,
        params: ListHostTagsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::TagToHosts>, Error<ListHostTagsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/tags/hosts", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::TagToHosts> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateHostTagsParams,
    ) -> Result<Option<crate::datadogV1::model::HostTags>, Error<UpdateHostTagsError>> {
        match self.update_host_tags_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to update/replace all tags in
    /// an integration source with those supplied in the request.
    pub async fn update_host_tags_with_http_info(
        &self,
        params: UpdateHostTagsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::HostTags>, Error<UpdateHostTagsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let host_name = params.host_name;
        let body = params.body;
        let source = params.source;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/tags/hosts/{host_name}",
            local_configuration.base_path,
            host_name = urlencode(host_name)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::HostTags> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateHostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
