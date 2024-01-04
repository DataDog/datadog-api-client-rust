// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateOktaAccountParams is a struct for passing parameters to the method [`CreateOktaAccount`]
#[derive(Clone, Debug)]
pub struct CreateOktaAccountParams {
    pub body: crate::datadogV2::model::OktaAccountRequest,
}

/// DeleteOktaAccountParams is a struct for passing parameters to the method [`DeleteOktaAccount`]
#[derive(Clone, Debug)]
pub struct DeleteOktaAccountParams {
    /// None
    pub account_id: String,
}

/// GetOktaAccountParams is a struct for passing parameters to the method [`GetOktaAccount`]
#[derive(Clone, Debug)]
pub struct GetOktaAccountParams {
    /// None
    pub account_id: String,
}

/// UpdateOktaAccountParams is a struct for passing parameters to the method [`UpdateOktaAccount`]
#[derive(Clone, Debug)]
pub struct UpdateOktaAccountParams {
    /// None
    pub account_id: String,
    pub body: crate::datadogV2::model::OktaAccountUpdateRequest,
}

/// CreateOktaAccountError is a struct for typed errors of method [`CreateOktaAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOktaAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteOktaAccountError is a struct for typed errors of method [`DeleteOktaAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOktaAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOktaAccountError is a struct for typed errors of method [`GetOktaAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOktaAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListOktaAccountsError is a struct for typed errors of method [`ListOktaAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOktaAccountsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOktaAccountError is a struct for typed errors of method [`UpdateOktaAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOktaAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct OktaIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for OktaIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl OktaIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create an Okta account.
    pub async fn create_okta_account(
        &self,
        params: CreateOktaAccountParams,
    ) -> Result<Option<crate::datadogV2::model::OktaAccountResponse>, Error<CreateOktaAccountError>>
    {
        match self.create_okta_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an Okta account.
    pub async fn create_okta_account_with_http_info(
        &self,
        params: CreateOktaAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OktaAccountResponse>,
        Error<CreateOktaAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/okta/accounts",
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
            let local_entity: Option<crate::datadogV2::model::OktaAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateOktaAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an Okta account.
    pub async fn delete_okta_account(
        &self,
        params: DeleteOktaAccountParams,
    ) -> Result<Option<()>, Error<DeleteOktaAccountError>> {
        match self.delete_okta_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an Okta account.
    pub async fn delete_okta_account_with_http_info(
        &self,
        params: DeleteOktaAccountParams,
    ) -> Result<ResponseContent<()>, Error<DeleteOktaAccountError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/okta/accounts/{account_id}",
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
            let local_entity: Option<DeleteOktaAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an Okta account.
    pub async fn get_okta_account(
        &self,
        params: GetOktaAccountParams,
    ) -> Result<Option<crate::datadogV2::model::OktaAccountResponse>, Error<GetOktaAccountError>>
    {
        match self.get_okta_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an Okta account.
    pub async fn get_okta_account_with_http_info(
        &self,
        params: GetOktaAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OktaAccountResponse>,
        Error<GetOktaAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/okta/accounts/{account_id}",
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
            let local_entity: Option<crate::datadogV2::model::OktaAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetOktaAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Okta accounts.
    pub async fn list_okta_accounts(
        &self,
    ) -> Result<Option<crate::datadogV2::model::OktaAccountsResponse>, Error<ListOktaAccountsError>>
    {
        match self.list_okta_accounts_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List Okta accounts.
    pub async fn list_okta_accounts_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OktaAccountsResponse>,
        Error<ListOktaAccountsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/okta/accounts",
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
            let local_entity: Option<crate::datadogV2::model::OktaAccountsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListOktaAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an Okta account.
    pub async fn update_okta_account(
        &self,
        params: UpdateOktaAccountParams,
    ) -> Result<Option<crate::datadogV2::model::OktaAccountResponse>, Error<UpdateOktaAccountError>>
    {
        match self.update_okta_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an Okta account.
    pub async fn update_okta_account_with_http_info(
        &self,
        params: UpdateOktaAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OktaAccountResponse>,
        Error<UpdateOktaAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/okta/accounts/{account_id}",
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
            let local_entity: Option<crate::datadogV2::model::OktaAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateOktaAccountError> =
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