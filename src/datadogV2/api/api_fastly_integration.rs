// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateFastlyAccountParams is a struct for passing parameters to the method [`CreateFastlyAccount`]
#[derive(Clone, Debug, Default)]
pub struct CreateFastlyAccountParams {
    pub body: crate::datadogV2::model::FastlyAccountCreateRequest,
}

/// CreateFastlyServiceParams is a struct for passing parameters to the method [`CreateFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct CreateFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    pub body: crate::datadogV2::model::FastlyServiceRequest,
}

/// DeleteFastlyAccountParams is a struct for passing parameters to the method [`DeleteFastlyAccount`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFastlyAccountParams {
    /// Fastly Account id.
    pub account_id: String,
}

/// DeleteFastlyServiceParams is a struct for passing parameters to the method [`DeleteFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    /// Fastly Service ID.
    pub service_id: String,
}

/// GetFastlyAccountParams is a struct for passing parameters to the method [`GetFastlyAccount`]
#[derive(Clone, Debug, Default)]
pub struct GetFastlyAccountParams {
    /// Fastly Account id.
    pub account_id: String,
}

/// GetFastlyServiceParams is a struct for passing parameters to the method [`GetFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct GetFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    /// Fastly Service ID.
    pub service_id: String,
}

/// ListFastlyServicesParams is a struct for passing parameters to the method [`ListFastlyServices`]
#[derive(Clone, Debug, Default)]
pub struct ListFastlyServicesParams {
    /// Fastly Account id.
    pub account_id: String,
}

/// UpdateFastlyAccountParams is a struct for passing parameters to the method [`UpdateFastlyAccount`]
#[derive(Clone, Debug, Default)]
pub struct UpdateFastlyAccountParams {
    /// Fastly Account id.
    pub account_id: String,
    pub body: crate::datadogV2::model::FastlyAccountUpdateRequest,
}

/// UpdateFastlyServiceParams is a struct for passing parameters to the method [`UpdateFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct UpdateFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    /// Fastly Service ID.
    pub service_id: String,
    pub body: crate::datadogV2::model::FastlyServiceRequest,
}

/// CreateFastlyAccountError is a struct for typed errors of method [`CreateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateFastlyServiceError is a struct for typed errors of method [`CreateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyAccountError is a struct for typed errors of method [`DeleteFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyServiceError is a struct for typed errors of method [`DeleteFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetFastlyAccountError is a struct for typed errors of method [`GetFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetFastlyServiceError is a struct for typed errors of method [`GetFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListFastlyAccountsError is a struct for typed errors of method [`ListFastlyAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyAccountsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListFastlyServicesError is a struct for typed errors of method [`ListFastlyServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyServicesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyAccountError is a struct for typed errors of method [`UpdateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyServiceError is a struct for typed errors of method [`UpdateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct FastlyIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for FastlyIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl FastlyIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Fastly account.
    pub async fn create_fastly_account(
        &self,
        params: CreateFastlyAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyAccountResponse>,
        Error<CreateFastlyAccountError>,
    > {
        match self.create_fastly_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Fastly account.
    pub async fn create_fastly_account_with_http_info(
        &self,
        params: CreateFastlyAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyAccountResponse>,
        Error<CreateFastlyAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts",
            local_configuration.base_path
        );
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
            let local_entity: Option<crate::datadogV2::model::FastlyAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateFastlyAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Fastly service for an account.
    pub async fn create_fastly_service(
        &self,
        params: CreateFastlyServiceParams,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyServiceResponse>,
        Error<CreateFastlyServiceError>,
    > {
        match self.create_fastly_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Fastly service for an account.
    pub async fn create_fastly_service_with_http_info(
        &self,
        params: CreateFastlyServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyServiceResponse>,
        Error<CreateFastlyServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}/services",
            local_configuration.base_path,
            account_id = urlencode(account_id)
        );
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
            let local_entity: Option<crate::datadogV2::model::FastlyServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateFastlyServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Fastly account.
    pub async fn delete_fastly_account(
        &self,
        params: DeleteFastlyAccountParams,
    ) -> Result<Option<()>, Error<DeleteFastlyAccountError>> {
        match self.delete_fastly_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Fastly account.
    pub async fn delete_fastly_account_with_http_info(
        &self,
        params: DeleteFastlyAccountParams,
    ) -> Result<ResponseContent<()>, Error<DeleteFastlyAccountError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
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
            let local_entity: Option<DeleteFastlyAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Fastly service for an account.
    pub async fn delete_fastly_service(
        &self,
        params: DeleteFastlyServiceParams,
    ) -> Result<Option<()>, Error<DeleteFastlyServiceError>> {
        match self.delete_fastly_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Fastly service for an account.
    pub async fn delete_fastly_service_with_http_info(
        &self,
        params: DeleteFastlyServiceParams,
    ) -> Result<ResponseContent<()>, Error<DeleteFastlyServiceError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let service_id = params.service_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}/services/{service_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            service_id = urlencode(service_id)
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
            let local_entity: Option<DeleteFastlyServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Fastly account.
    pub async fn get_fastly_account(
        &self,
        params: GetFastlyAccountParams,
    ) -> Result<Option<crate::datadogV2::model::FastlyAccountResponse>, Error<GetFastlyAccountError>>
    {
        match self.get_fastly_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Fastly account.
    pub async fn get_fastly_account_with_http_info(
        &self,
        params: GetFastlyAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyAccountResponse>,
        Error<GetFastlyAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
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
            let local_entity: Option<crate::datadogV2::model::FastlyAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetFastlyAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Fastly service for an account.
    pub async fn get_fastly_service(
        &self,
        params: GetFastlyServiceParams,
    ) -> Result<Option<crate::datadogV2::model::FastlyServiceResponse>, Error<GetFastlyServiceError>>
    {
        match self.get_fastly_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Fastly service for an account.
    pub async fn get_fastly_service_with_http_info(
        &self,
        params: GetFastlyServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyServiceResponse>,
        Error<GetFastlyServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let service_id = params.service_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}/services/{service_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            service_id = urlencode(service_id)
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
            let local_entity: Option<crate::datadogV2::model::FastlyServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetFastlyServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Fastly accounts.
    pub async fn list_fastly_accounts(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyAccountsResponse>,
        Error<ListFastlyAccountsError>,
    > {
        match self.list_fastly_accounts_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List Fastly accounts.
    pub async fn list_fastly_accounts_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyAccountsResponse>,
        Error<ListFastlyAccountsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts",
            local_configuration.base_path
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
            let local_entity: Option<crate::datadogV2::model::FastlyAccountsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListFastlyAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Fastly services for an account.
    pub async fn list_fastly_services(
        &self,
        params: ListFastlyServicesParams,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyServicesResponse>,
        Error<ListFastlyServicesError>,
    > {
        match self.list_fastly_services_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List Fastly services for an account.
    pub async fn list_fastly_services_with_http_info(
        &self,
        params: ListFastlyServicesParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyServicesResponse>,
        Error<ListFastlyServicesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}/services",
            local_configuration.base_path,
            account_id = urlencode(account_id)
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
            let local_entity: Option<crate::datadogV2::model::FastlyServicesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListFastlyServicesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Fastly account.
    pub async fn update_fastly_account(
        &self,
        params: UpdateFastlyAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyAccountResponse>,
        Error<UpdateFastlyAccountError>,
    > {
        match self.update_fastly_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Fastly account.
    pub async fn update_fastly_account_with_http_info(
        &self,
        params: UpdateFastlyAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyAccountResponse>,
        Error<UpdateFastlyAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
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
            let local_entity: Option<crate::datadogV2::model::FastlyAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateFastlyAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Fastly service for an account.
    pub async fn update_fastly_service(
        &self,
        params: UpdateFastlyServiceParams,
    ) -> Result<
        Option<crate::datadogV2::model::FastlyServiceResponse>,
        Error<UpdateFastlyServiceError>,
    > {
        match self.update_fastly_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Fastly service for an account.
    pub async fn update_fastly_service_with_http_info(
        &self,
        params: UpdateFastlyServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::FastlyServiceResponse>,
        Error<UpdateFastlyServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let service_id = params.service_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/fastly/accounts/{account_id}/services/{service_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            service_id = urlencode(service_id)
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
            let local_entity: Option<crate::datadogV2::model::FastlyServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateFastlyServiceError> =
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
