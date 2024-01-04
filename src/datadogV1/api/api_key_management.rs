// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateAPIKeyParams is a struct for passing parameters to the method [`CreateAPIKey`]
#[derive(Clone, Debug)]
pub struct CreateAPIKeyParams {
    pub body: crate::datadogV1::model::ApiKey,
}

/// CreateApplicationKeyParams is a struct for passing parameters to the method [`CreateApplicationKey`]
#[derive(Clone, Debug)]
pub struct CreateApplicationKeyParams {
    pub body: crate::datadogV1::model::ApplicationKey,
}

/// DeleteAPIKeyParams is a struct for passing parameters to the method [`DeleteAPIKey`]
#[derive(Clone, Debug)]
pub struct DeleteAPIKeyParams {
    /// The specific API key you are working with.
    pub key: String,
}

/// DeleteApplicationKeyParams is a struct for passing parameters to the method [`DeleteApplicationKey`]
#[derive(Clone, Debug)]
pub struct DeleteApplicationKeyParams {
    /// The specific APP key you are working with.
    pub key: String,
}

/// GetAPIKeyParams is a struct for passing parameters to the method [`GetAPIKey`]
#[derive(Clone, Debug)]
pub struct GetAPIKeyParams {
    /// The specific API key you are working with.
    pub key: String,
}

/// GetApplicationKeyParams is a struct for passing parameters to the method [`GetApplicationKey`]
#[derive(Clone, Debug)]
pub struct GetApplicationKeyParams {
    /// The specific APP key you are working with.
    pub key: String,
}

/// UpdateAPIKeyParams is a struct for passing parameters to the method [`UpdateAPIKey`]
#[derive(Clone, Debug)]
pub struct UpdateAPIKeyParams {
    /// The specific API key you are working with.
    pub key: String,
    pub body: crate::datadogV1::model::ApiKey,
}

/// UpdateApplicationKeyParams is a struct for passing parameters to the method [`UpdateApplicationKey`]
#[derive(Clone, Debug)]
pub struct UpdateApplicationKeyParams {
    /// The specific APP key you are working with.
    pub key: String,
    pub body: crate::datadogV1::model::ApplicationKey,
}

/// CreateAPIKeyError is a struct for typed errors of method [`CreateAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAPIKeyError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateApplicationKeyError is a struct for typed errors of method [`CreateApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationKeyError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAPIKeyError is a struct for typed errors of method [`DeleteAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAPIKeyError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteApplicationKeyError is a struct for typed errors of method [`DeleteApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationKeyError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPIKeyError is a struct for typed errors of method [`GetAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPIKeyError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetApplicationKeyError is a struct for typed errors of method [`GetApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationKeyError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAPIKeysError is a struct for typed errors of method [`ListAPIKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAPIKeysError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListApplicationKeysError is a struct for typed errors of method [`ListApplicationKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationKeysError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAPIKeyError is a struct for typed errors of method [`UpdateAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPIKeyError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateApplicationKeyError is a struct for typed errors of method [`UpdateApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationKeyError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct KeyManagementAPI {
    config: configuration::Configuration,
}

impl Default for KeyManagementAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl KeyManagementAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates an API key with a given name.
    pub async fn create_api_key(
        &self,
        params: CreateAPIKeyParams,
    ) -> Result<Option<crate::datadogV1::model::ApiKeyResponse>, Error<CreateAPIKeyError>> {
        match self.create_api_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates an API key with a given name.
    pub async fn create_api_key_with_http_info(
        &self,
        params: CreateAPIKeyParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::ApiKeyResponse>, Error<CreateAPIKeyError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/api_key", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::ApiKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateAPIKeyError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an application key with a given name.
    pub async fn create_application_key(
        &self,
        params: CreateApplicationKeyParams,
    ) -> Result<
        Option<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<CreateApplicationKeyError>,
    > {
        match self.create_application_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an application key with a given name.
    pub async fn create_application_key_with_http_info(
        &self,
        params: CreateApplicationKeyParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<CreateApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/application_key", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given API key.
    pub async fn delete_api_key(
        &self,
        params: DeleteAPIKeyParams,
    ) -> Result<Option<crate::datadogV1::model::ApiKeyResponse>, Error<DeleteAPIKeyError>> {
        match self.delete_api_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given API key.
    pub async fn delete_api_key_with_http_info(
        &self,
        params: DeleteAPIKeyParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::ApiKeyResponse>, Error<DeleteAPIKeyError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/api_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
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
            let local_entity: Option<crate::datadogV1::model::ApiKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteAPIKeyError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given application key.
    pub async fn delete_application_key(
        &self,
        params: DeleteApplicationKeyParams,
    ) -> Result<
        Option<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<DeleteApplicationKeyError>,
    > {
        match self.delete_application_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given application key.
    pub async fn delete_application_key_with_http_info(
        &self,
        params: DeleteApplicationKeyParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<DeleteApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/application_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
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
            let local_entity: Option<crate::datadogV1::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a given API key.
    pub async fn get_api_key(
        &self,
        params: GetAPIKeyParams,
    ) -> Result<Option<crate::datadogV1::model::ApiKeyResponse>, Error<GetAPIKeyError>> {
        match self.get_api_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a given API key.
    pub async fn get_api_key_with_http_info(
        &self,
        params: GetAPIKeyParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::ApiKeyResponse>, Error<GetAPIKeyError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/api_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
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
            let local_entity: Option<crate::datadogV1::model::ApiKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetAPIKeyError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a given application key.
    pub async fn get_application_key(
        &self,
        params: GetApplicationKeyParams,
    ) -> Result<
        Option<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<GetApplicationKeyError>,
    > {
        match self.get_application_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a given application key.
    pub async fn get_application_key_with_http_info(
        &self,
        params: GetApplicationKeyParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<GetApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/application_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
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
            let local_entity: Option<crate::datadogV1::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all API keys available for your account.
    pub async fn list_api_keys(
        &self,
    ) -> Result<Option<crate::datadogV1::model::ApiKeyListResponse>, Error<ListAPIKeysError>> {
        match self.list_api_keys_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all API keys available for your account.
    pub async fn list_api_keys_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV1::model::ApiKeyListResponse>, Error<ListAPIKeysError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/api_key", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::ApiKeyListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAPIKeysError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all application keys available for your Datadog account.
    pub async fn list_application_keys(
        &self,
    ) -> Result<
        Option<crate::datadogV1::model::ApplicationKeyListResponse>,
        Error<ListApplicationKeysError>,
    > {
        match self.list_application_keys_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all application keys available for your Datadog account.
    pub async fn list_application_keys_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::ApplicationKeyListResponse>,
        Error<ListApplicationKeysError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/application_key", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::ApplicationKeyListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListApplicationKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit an API key name.
    pub async fn update_api_key(
        &self,
        params: UpdateAPIKeyParams,
    ) -> Result<Option<crate::datadogV1::model::ApiKeyResponse>, Error<UpdateAPIKeyError>> {
        match self.update_api_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an API key name.
    pub async fn update_api_key_with_http_info(
        &self,
        params: UpdateAPIKeyParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::ApiKeyResponse>, Error<UpdateAPIKeyError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/api_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV1::model::ApiKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateAPIKeyError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit an application key name.
    pub async fn update_application_key(
        &self,
        params: UpdateApplicationKeyParams,
    ) -> Result<
        Option<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<UpdateApplicationKeyError>,
    > {
        match self.update_application_key_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an application key name.
    pub async fn update_application_key_with_http_info(
        &self,
        params: UpdateApplicationKeyParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::ApplicationKeyResponse>,
        Error<UpdateApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let key = params.key;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/application_key/{key}",
            local_configuration.base_path,
            key = urlencode(key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV1::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateApplicationKeyError> =
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
