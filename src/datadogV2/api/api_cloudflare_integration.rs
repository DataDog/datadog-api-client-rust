// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateCloudflareAccountParams is a struct for passing parameters to the method [`CreateCloudflareAccount`]
#[derive(Clone, Debug, Default)]
pub struct CreateCloudflareAccountParams {
    pub body: crate::datadogV2::model::CloudflareAccountCreateRequest,
}

/// DeleteCloudflareAccountParams is a struct for passing parameters to the method [`DeleteCloudflareAccount`]
#[derive(Clone, Debug, Default)]
pub struct DeleteCloudflareAccountParams {
    /// None
    pub account_id: String,
}

/// GetCloudflareAccountParams is a struct for passing parameters to the method [`GetCloudflareAccount`]
#[derive(Clone, Debug, Default)]
pub struct GetCloudflareAccountParams {
    /// None
    pub account_id: String,
}

/// UpdateCloudflareAccountParams is a struct for passing parameters to the method [`UpdateCloudflareAccount`]
#[derive(Clone, Debug, Default)]
pub struct UpdateCloudflareAccountParams {
    /// None
    pub account_id: String,
    pub body: crate::datadogV2::model::CloudflareAccountUpdateRequest,
}

/// CreateCloudflareAccountError is a struct for typed errors of method [`CreateCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudflareAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteCloudflareAccountError is a struct for typed errors of method [`DeleteCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudflareAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetCloudflareAccountError is a struct for typed errors of method [`GetCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudflareAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListCloudflareAccountsError is a struct for typed errors of method [`ListCloudflareAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudflareAccountsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateCloudflareAccountError is a struct for typed errors of method [`UpdateCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCloudflareAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct CloudflareIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for CloudflareIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl CloudflareIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Cloudflare account.
    pub async fn create_cloudflare_account(
        &self,
        params: CreateCloudflareAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<CreateCloudflareAccountError>,
    > {
        match self.create_cloudflare_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Cloudflare account.
    pub async fn create_cloudflare_account_with_http_info(
        &self,
        params: CreateCloudflareAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<CreateCloudflareAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/cloudflare/accounts",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build parameters

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::CloudflareAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateCloudflareAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Cloudflare account.
    pub async fn delete_cloudflare_account(
        &self,
        params: DeleteCloudflareAccountParams,
    ) -> Result<Option<()>, Error<DeleteCloudflareAccountError>> {
        match self.delete_cloudflare_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Cloudflare account.
    pub async fn delete_cloudflare_account_with_http_info(
        &self,
        params: DeleteCloudflareAccountParams,
    ) -> Result<ResponseContent<()>, Error<DeleteCloudflareAccountError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/cloudflare/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build parameters

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<DeleteCloudflareAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Cloudflare account.
    pub async fn get_cloudflare_account(
        &self,
        params: GetCloudflareAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<GetCloudflareAccountError>,
    > {
        match self.get_cloudflare_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Cloudflare account.
    pub async fn get_cloudflare_account_with_http_info(
        &self,
        params: GetCloudflareAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<GetCloudflareAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/cloudflare/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::CloudflareAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetCloudflareAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Cloudflare accounts.
    pub async fn list_cloudflare_accounts(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::CloudflareAccountsResponse>,
        Error<ListCloudflareAccountsError>,
    > {
        match self.list_cloudflare_accounts_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List Cloudflare accounts.
    pub async fn list_cloudflare_accounts_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudflareAccountsResponse>,
        Error<ListCloudflareAccountsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/cloudflare/accounts",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::CloudflareAccountsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListCloudflareAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Cloudflare account.
    pub async fn update_cloudflare_account(
        &self,
        params: UpdateCloudflareAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<UpdateCloudflareAccountError>,
    > {
        match self.update_cloudflare_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Cloudflare account.
    pub async fn update_cloudflare_account_with_http_info(
        &self,
        params: UpdateCloudflareAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudflareAccountResponse>,
        Error<UpdateCloudflareAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/cloudflare/accounts/{account_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build parameters

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::CloudflareAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateCloudflareAccountError> =
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
