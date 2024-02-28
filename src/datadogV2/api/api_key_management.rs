// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetAPIKeyOptionalParams is a struct for passing parameters to the method [`KeyManagementAPI::get_api_key`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAPIKeyOptionalParams {
    /// Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`.
    pub include: Option<String>,
}

impl GetAPIKeyOptionalParams {
    /// Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// GetApplicationKeyOptionalParams is a struct for passing parameters to the method [`KeyManagementAPI::get_application_key`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetApplicationKeyOptionalParams {
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub include: Option<String>,
}

impl GetApplicationKeyOptionalParams {
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// ListAPIKeysOptionalParams is a struct for passing parameters to the method [`KeyManagementAPI::list_api_keys`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAPIKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// API key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub sort: Option<crate::datadogV2::model::APIKeysSort>,
    /// Filter API keys by the specified string.
    pub filter: Option<String>,
    /// Only include API keys created on or after the specified date.
    pub filter_created_at_start: Option<String>,
    /// Only include API keys created on or before the specified date.
    pub filter_created_at_end: Option<String>,
    /// Only include API keys modified on or after the specified date.
    pub filter_modified_at_start: Option<String>,
    /// Only include API keys modified on or before the specified date.
    pub filter_modified_at_end: Option<String>,
    /// Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`.
    pub include: Option<String>,
    /// Filter API keys by remote config read enabled status.
    pub filter_remote_config_read_enabled: Option<bool>,
    /// Filter API keys by category.
    pub filter_category: Option<String>,
}

impl ListAPIKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// API key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub fn sort(&mut self, value: crate::datadogV2::model::APIKeysSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Filter API keys by the specified string.
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
    /// Only include API keys created on or after the specified date.
    pub fn filter_created_at_start(&mut self, value: String) -> &mut Self {
        self.filter_created_at_start = Some(value);
        self
    }
    /// Only include API keys created on or before the specified date.
    pub fn filter_created_at_end(&mut self, value: String) -> &mut Self {
        self.filter_created_at_end = Some(value);
        self
    }
    /// Only include API keys modified on or after the specified date.
    pub fn filter_modified_at_start(&mut self, value: String) -> &mut Self {
        self.filter_modified_at_start = Some(value);
        self
    }
    /// Only include API keys modified on or before the specified date.
    pub fn filter_modified_at_end(&mut self, value: String) -> &mut Self {
        self.filter_modified_at_end = Some(value);
        self
    }
    /// Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
    /// Filter API keys by remote config read enabled status.
    pub fn filter_remote_config_read_enabled(&mut self, value: bool) -> &mut Self {
        self.filter_remote_config_read_enabled = Some(value);
        self
    }
    /// Filter API keys by category.
    pub fn filter_category(&mut self, value: String) -> &mut Self {
        self.filter_category = Some(value);
        self
    }
}

/// ListApplicationKeysOptionalParams is a struct for passing parameters to the method [`KeyManagementAPI::list_application_keys`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub sort: Option<crate::datadogV2::model::ApplicationKeysSort>,
    /// Filter application keys by the specified string.
    pub filter: Option<String>,
    /// Only include application keys created on or after the specified date.
    pub filter_created_at_start: Option<String>,
    /// Only include application keys created on or before the specified date.
    pub filter_created_at_end: Option<String>,
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub include: Option<String>,
}

impl ListApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub fn sort(&mut self, value: crate::datadogV2::model::ApplicationKeysSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Filter application keys by the specified string.
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
    /// Only include application keys created on or after the specified date.
    pub fn filter_created_at_start(&mut self, value: String) -> &mut Self {
        self.filter_created_at_start = Some(value);
        self
    }
    /// Only include application keys created on or before the specified date.
    pub fn filter_created_at_end(&mut self, value: String) -> &mut Self {
        self.filter_created_at_end = Some(value);
        self
    }
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// ListCurrentUserApplicationKeysOptionalParams is a struct for passing parameters to the method [`KeyManagementAPI::list_current_user_application_keys`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCurrentUserApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub sort: Option<crate::datadogV2::model::ApplicationKeysSort>,
    /// Filter application keys by the specified string.
    pub filter: Option<String>,
    /// Only include application keys created on or after the specified date.
    pub filter_created_at_start: Option<String>,
    /// Only include application keys created on or before the specified date.
    pub filter_created_at_end: Option<String>,
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub include: Option<String>,
}

impl ListCurrentUserApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub fn sort(&mut self, value: crate::datadogV2::model::ApplicationKeysSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Filter application keys by the specified string.
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
    /// Only include application keys created on or after the specified date.
    pub fn filter_created_at_start(&mut self, value: String) -> &mut Self {
        self.filter_created_at_start = Some(value);
        self
    }
    /// Only include application keys created on or before the specified date.
    pub fn filter_created_at_end(&mut self, value: String) -> &mut Self {
        self.filter_created_at_end = Some(value);
        self
    }
    /// Resource path for related resources to include in the response. Only `owned_by` is supported.
    pub fn include(&mut self, value: String) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// CreateAPIKeyError is a struct for typed errors of method [`KeyManagementAPI::create_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAPIKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateCurrentUserApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::create_current_user_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAPIKeyError is a struct for typed errors of method [`KeyManagementAPI::delete_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAPIKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::delete_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteCurrentUserApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::delete_current_user_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCurrentUserApplicationKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPIKeyError is a struct for typed errors of method [`KeyManagementAPI::get_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPIKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::get_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetCurrentUserApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::get_current_user_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentUserApplicationKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAPIKeysError is a struct for typed errors of method [`KeyManagementAPI::list_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAPIKeysError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListApplicationKeysError is a struct for typed errors of method [`KeyManagementAPI::list_application_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationKeysError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListCurrentUserApplicationKeysError is a struct for typed errors of method [`KeyManagementAPI::list_current_user_application_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCurrentUserApplicationKeysError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAPIKeyError is a struct for typed errors of method [`KeyManagementAPI::update_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPIKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::update_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateCurrentUserApplicationKeyError is a struct for typed errors of method [`KeyManagementAPI::update_current_user_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCurrentUserApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
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

    /// Create an API key.
    pub async fn create_api_key(
        &self,
        body: crate::datadogV2::model::APIKeyCreateRequest,
    ) -> Result<Option<crate::datadogV2::model::APIKeyResponse>, Error<CreateAPIKeyError>> {
        match self.create_api_key_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an API key.
    pub async fn create_api_key_with_http_info(
        &self,
        body: crate::datadogV2::model::APIKeyCreateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::APIKeyResponse>, Error<CreateAPIKeyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.create_api_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/api_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::APIKeyResponse> =
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

    /// Create an application key for current user
    pub async fn create_current_user_application_key(
        &self,
        body: crate::datadogV2::model::ApplicationKeyCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<CreateCurrentUserApplicationKeyError>,
    > {
        match self
            .create_current_user_application_key_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an application key for current user
    pub async fn create_current_user_application_key_with_http_info(
        &self,
        body: crate::datadogV2::model::ApplicationKeyCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<CreateCurrentUserApplicationKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_current_user_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/current_user/application_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateCurrentUserApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an API key.
    pub async fn delete_api_key(
        &self,
        api_key_id: String,
    ) -> Result<Option<()>, Error<DeleteAPIKeyError>> {
        match self.delete_api_key_with_http_info(api_key_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an API key.
    pub async fn delete_api_key_with_http_info(
        &self,
        api_key_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteAPIKeyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_api_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/api_keys/{api_key_id}",
            local_configuration.get_operation_host(operation_id),
            api_key_id = urlencode(api_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            let local_entity: Option<DeleteAPIKeyError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an application key
    pub async fn delete_application_key(
        &self,
        app_key_id: String,
    ) -> Result<Option<()>, Error<DeleteApplicationKeyError>> {
        match self.delete_application_key_with_http_info(app_key_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an application key
    pub async fn delete_application_key_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteApplicationKeyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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

    /// Delete an application key owned by current user
    pub async fn delete_current_user_application_key(
        &self,
        app_key_id: String,
    ) -> Result<Option<()>, Error<DeleteCurrentUserApplicationKeyError>> {
        match self
            .delete_current_user_application_key_with_http_info(app_key_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an application key owned by current user
    pub async fn delete_current_user_application_key_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteCurrentUserApplicationKeyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_current_user_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/current_user/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            let local_entity: Option<DeleteCurrentUserApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an API key.
    pub async fn get_api_key(
        &self,
        api_key_id: String,
        params: GetAPIKeyOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::APIKeyResponse>, Error<GetAPIKeyError>> {
        match self.get_api_key_with_http_info(api_key_id, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an API key.
    pub async fn get_api_key_with_http_info(
        &self,
        api_key_id: String,
        params: GetAPIKeyOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::APIKeyResponse>, Error<GetAPIKeyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_api_key";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/api_keys/{api_key_id}",
            local_configuration.get_operation_host(operation_id),
            api_key_id = urlencode(api_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::APIKeyResponse> =
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

    /// Get an application key for your org.
    pub async fn get_application_key(
        &self,
        app_key_id: String,
        params: GetApplicationKeyOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<GetApplicationKeyError>,
    > {
        match self
            .get_application_key_with_http_info(app_key_id, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an application key for your org.
    pub async fn get_application_key_with_http_info(
        &self,
        app_key_id: String,
        params: GetApplicationKeyOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<GetApplicationKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_application_key";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
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

    /// Get an application key owned by current user
    pub async fn get_current_user_application_key(
        &self,
        app_key_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<GetCurrentUserApplicationKeyError>,
    > {
        match self
            .get_current_user_application_key_with_http_info(app_key_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an application key owned by current user
    pub async fn get_current_user_application_key_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<GetCurrentUserApplicationKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_current_user_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/current_user/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetCurrentUserApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all API keys available for your account.
    pub async fn list_api_keys(
        &self,
        params: ListAPIKeysOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::APIKeysResponse>, Error<ListAPIKeysError>> {
        match self.list_api_keys_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all API keys available for your account.
    pub async fn list_api_keys_with_http_info(
        &self,
        params: ListAPIKeysOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::APIKeysResponse>, Error<ListAPIKeysError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.list_api_keys";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_created_at_start = params.filter_created_at_start;
        let filter_created_at_end = params.filter_created_at_end;
        let filter_modified_at_start = params.filter_modified_at_start;
        let filter_modified_at_end = params.filter_modified_at_end;
        let include = params.include;
        let filter_remote_config_read_enabled = params.filter_remote_config_read_enabled;
        let filter_category = params.filter_category;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/api_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_modified_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[modified_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_modified_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[modified_at][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_remote_config_read_enabled {
            local_req_builder = local_req_builder.query(&[(
                "filter[remote_config_read_enabled]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_category {
            local_req_builder =
                local_req_builder.query(&[("filter[category]", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::APIKeysResponse> =
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

    /// List all application keys available for your org
    pub async fn list_application_keys(
        &self,
        params: ListApplicationKeysOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListApplicationKeysError>,
    > {
        match self.list_application_keys_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all application keys available for your org
    pub async fn list_application_keys_with_http_info(
        &self,
        params: ListApplicationKeysOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListApplicationKeysError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_application_keys";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_created_at_start = params.filter_created_at_start;
        let filter_created_at_end = params.filter_created_at_end;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/application_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ListApplicationKeysResponse> =
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

    /// List all application keys available for current user
    pub async fn list_current_user_application_keys(
        &self,
        params: ListCurrentUserApplicationKeysOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListCurrentUserApplicationKeysError>,
    > {
        match self
            .list_current_user_application_keys_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all application keys available for current user
    pub async fn list_current_user_application_keys_with_http_info(
        &self,
        params: ListCurrentUserApplicationKeysOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListCurrentUserApplicationKeysError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_current_user_application_keys";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_created_at_start = params.filter_created_at_start;
        let filter_created_at_end = params.filter_created_at_end;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/current_user/application_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ListApplicationKeysResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListCurrentUserApplicationKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an API key.
    pub async fn update_api_key(
        &self,
        api_key_id: String,
        body: crate::datadogV2::model::APIKeyUpdateRequest,
    ) -> Result<Option<crate::datadogV2::model::APIKeyResponse>, Error<UpdateAPIKeyError>> {
        match self.update_api_key_with_http_info(api_key_id, body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an API key.
    pub async fn update_api_key_with_http_info(
        &self,
        api_key_id: String,
        body: crate::datadogV2::model::APIKeyUpdateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::APIKeyResponse>, Error<UpdateAPIKeyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.update_api_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/api_keys/{api_key_id}",
            local_configuration.get_operation_host(operation_id),
            api_key_id = urlencode(api_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::APIKeyResponse> =
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

    /// Edit an application key
    pub async fn update_application_key(
        &self,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<UpdateApplicationKeyError>,
    > {
        match self
            .update_application_key_with_http_info(app_key_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an application key
    pub async fn update_application_key_with_http_info(
        &self,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<UpdateApplicationKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
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

    /// Edit an application key owned by current user
    pub async fn update_current_user_application_key(
        &self,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<UpdateCurrentUserApplicationKeyError>,
    > {
        match self
            .update_current_user_application_key_with_http_info(app_key_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an application key owned by current user
    pub async fn update_current_user_application_key_with_http_info(
        &self,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<UpdateCurrentUserApplicationKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_current_user_application_key";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/current_user/application_keys/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateCurrentUserApplicationKeyError> =
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
