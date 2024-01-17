// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateAuthNMappingParams is a struct for passing parameters to the method [`CreateAuthNMapping`]
#[derive(Clone, Debug)]
pub struct CreateAuthNMappingParams {
    pub body: crate::datadogV2::model::AuthNMappingCreateRequest,
}

/// DeleteAuthNMappingParams is a struct for passing parameters to the method [`DeleteAuthNMapping`]
#[derive(Clone, Debug)]
pub struct DeleteAuthNMappingParams {
    /// The UUID of the AuthN Mapping.
    pub authn_mapping_id: String,
}

/// GetAuthNMappingParams is a struct for passing parameters to the method [`GetAuthNMapping`]
#[derive(Clone, Debug)]
pub struct GetAuthNMappingParams {
    /// The UUID of the AuthN Mapping.
    pub authn_mapping_id: String,
}

/// ListAuthNMappingsParams is a struct for passing parameters to the method [`ListAuthNMappings`]
#[derive(Clone, Debug)]
pub struct ListAuthNMappingsParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Sort AuthN Mappings depending on the given field.
    pub sort: Option<crate::datadogV2::model::AuthNMappingsSort>,
    /// Filter all mappings by the given string.
    pub filter: Option<String>,
}

/// UpdateAuthNMappingParams is a struct for passing parameters to the method [`UpdateAuthNMapping`]
#[derive(Clone, Debug)]
pub struct UpdateAuthNMappingParams {
    /// The UUID of the AuthN Mapping.
    pub authn_mapping_id: String,
    pub body: crate::datadogV2::model::AuthNMappingUpdateRequest,
}

/// CreateAuthNMappingError is a struct for typed errors of method [`CreateAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAuthNMappingError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAuthNMappingError is a struct for typed errors of method [`DeleteAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAuthNMappingError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAuthNMappingError is a struct for typed errors of method [`GetAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAuthNMappingError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAuthNMappingsError is a struct for typed errors of method [`ListAuthNMappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuthNMappingsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAuthNMappingError is a struct for typed errors of method [`UpdateAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAuthNMappingError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct AuthNMappingsAPI {
    config: configuration::Configuration,
}

impl Default for AuthNMappingsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AuthNMappingsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create an AuthN Mapping.
    pub async fn create_auth_n_mapping(
        &self,
        params: CreateAuthNMappingParams,
    ) -> Result<Option<crate::datadogV2::model::AuthNMappingResponse>, Error<CreateAuthNMappingError>>
    {
        match self.create_auth_n_mapping_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an AuthN Mapping.
    pub async fn create_auth_n_mapping_with_http_info(
        &self,
        params: CreateAuthNMappingParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuthNMappingResponse>,
        Error<CreateAuthNMappingError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/authn_mappings", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::AuthNMappingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateAuthNMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an AuthN Mapping specified by AuthN Mapping UUID.
    pub async fn delete_auth_n_mapping(
        &self,
        params: DeleteAuthNMappingParams,
    ) -> Result<Option<()>, Error<DeleteAuthNMappingError>> {
        match self.delete_auth_n_mapping_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an AuthN Mapping specified by AuthN Mapping UUID.
    pub async fn delete_auth_n_mapping_with_http_info(
        &self,
        params: DeleteAuthNMappingParams,
    ) -> Result<ResponseContent<()>, Error<DeleteAuthNMappingError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let authn_mapping_id = params.authn_mapping_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/authn_mappings/{authn_mapping_id}",
            local_configuration.base_path,
            authn_mapping_id = urlencode(authn_mapping_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteAuthNMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an AuthN Mapping specified by the AuthN Mapping UUID.
    pub async fn get_auth_n_mapping(
        &self,
        params: GetAuthNMappingParams,
    ) -> Result<Option<crate::datadogV2::model::AuthNMappingResponse>, Error<GetAuthNMappingError>>
    {
        match self.get_auth_n_mapping_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an AuthN Mapping specified by the AuthN Mapping UUID.
    pub async fn get_auth_n_mapping_with_http_info(
        &self,
        params: GetAuthNMappingParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuthNMappingResponse>,
        Error<GetAuthNMappingError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let authn_mapping_id = params.authn_mapping_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/authn_mappings/{authn_mapping_id}",
            local_configuration.base_path,
            authn_mapping_id = urlencode(authn_mapping_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::AuthNMappingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetAuthNMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all AuthN Mappings in the org.
    pub async fn list_auth_n_mappings(
        &self,
        params: ListAuthNMappingsParams,
    ) -> Result<Option<crate::datadogV2::model::AuthNMappingsResponse>, Error<ListAuthNMappingsError>>
    {
        match self.list_auth_n_mappings_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all AuthN Mappings in the org.
    pub async fn list_auth_n_mappings_with_http_info(
        &self,
        params: ListAuthNMappingsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuthNMappingsResponse>,
        Error<ListAuthNMappingsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/authn_mappings", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter {
            local_req_builder = local_req_builder.query(&[("filter", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::AuthNMappingsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAuthNMappingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit an AuthN Mapping.
    pub async fn update_auth_n_mapping(
        &self,
        params: UpdateAuthNMappingParams,
    ) -> Result<Option<crate::datadogV2::model::AuthNMappingResponse>, Error<UpdateAuthNMappingError>>
    {
        match self.update_auth_n_mapping_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an AuthN Mapping.
    pub async fn update_auth_n_mapping_with_http_info(
        &self,
        params: UpdateAuthNMappingParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::AuthNMappingResponse>,
        Error<UpdateAuthNMappingError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let authn_mapping_id = params.authn_mapping_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/authn_mappings/{authn_mapping_id}",
            local_configuration.base_path,
            authn_mapping_id = urlencode(authn_mapping_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::AuthNMappingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateAuthNMappingError> =
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
