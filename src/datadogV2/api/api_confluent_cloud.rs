// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateConfluentAccountParams is a struct for passing parameters to the method [`CreateConfluentAccount`]
#[derive(Clone, Debug, Default)]
pub struct CreateConfluentAccountParams {
    /// Confluent payload
    pub body: crate::datadogV2::model::ConfluentAccountCreateRequest,
}

/// CreateConfluentResourceParams is a struct for passing parameters to the method [`CreateConfluentResource`]
#[derive(Clone, Debug, Default)]
pub struct CreateConfluentResourceParams {
    /// Confluent Account id.
    pub account_id: String,
    /// Confluent payload
    pub body: crate::datadogV2::model::ConfluentResourceRequest,
}

/// DeleteConfluentAccountParams is a struct for passing parameters to the method [`DeleteConfluentAccount`]
#[derive(Clone, Debug, Default)]
pub struct DeleteConfluentAccountParams {
    /// Confluent Account id.
    pub account_id: String,
}

/// DeleteConfluentResourceParams is a struct for passing parameters to the method [`DeleteConfluentResource`]
#[derive(Clone, Debug, Default)]
pub struct DeleteConfluentResourceParams {
    /// Confluent Account id.
    pub account_id: String,
    /// Confluent Account Resource ID.
    pub resource_id: String,
}

/// GetConfluentAccountParams is a struct for passing parameters to the method [`GetConfluentAccount`]
#[derive(Clone, Debug, Default)]
pub struct GetConfluentAccountParams {
    /// Confluent Account id.
    pub account_id: String,
}

/// GetConfluentResourceParams is a struct for passing parameters to the method [`GetConfluentResource`]
#[derive(Clone, Debug, Default)]
pub struct GetConfluentResourceParams {
    /// Confluent Account id.
    pub account_id: String,
    /// Confluent Account Resource ID.
    pub resource_id: String,
}

/// ListConfluentResourceParams is a struct for passing parameters to the method [`ListConfluentResource`]
#[derive(Clone, Debug, Default)]
pub struct ListConfluentResourceParams {
    /// Confluent Account id.
    pub account_id: String,
}

/// UpdateConfluentAccountParams is a struct for passing parameters to the method [`UpdateConfluentAccount`]
#[derive(Clone, Debug, Default)]
pub struct UpdateConfluentAccountParams {
    /// Confluent Account id.
    pub account_id: String,
    /// Confluent payload
    pub body: crate::datadogV2::model::ConfluentAccountUpdateRequest,
}

/// UpdateConfluentResourceParams is a struct for passing parameters to the method [`UpdateConfluentResource`]
#[derive(Clone, Debug, Default)]
pub struct UpdateConfluentResourceParams {
    /// Confluent Account id.
    pub account_id: String,
    /// Confluent Account Resource ID.
    pub resource_id: String,
    /// Confluent payload
    pub body: crate::datadogV2::model::ConfluentResourceRequest,
}

/// CreateConfluentAccountError is a struct for typed errors of method [`CreateConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateConfluentResourceError is a struct for typed errors of method [`CreateConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentAccountError is a struct for typed errors of method [`DeleteConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentResourceError is a struct for typed errors of method [`DeleteConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetConfluentAccountError is a struct for typed errors of method [`GetConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetConfluentResourceError is a struct for typed errors of method [`GetConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListConfluentAccountError is a struct for typed errors of method [`ListConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListConfluentResourceError is a struct for typed errors of method [`ListConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentAccountError is a struct for typed errors of method [`UpdateConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentResourceError is a struct for typed errors of method [`UpdateConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ConfluentCloudAPI {
    config: configuration::Configuration,
}

impl Default for ConfluentCloudAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ConfluentCloudAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Confluent account.
    pub async fn create_confluent_account(
        &self,
        params: CreateConfluentAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<CreateConfluentAccountError>,
    > {
        match self.create_confluent_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Confluent account.
    pub async fn create_confluent_account_with_http_info(
        &self,
        params: CreateConfluentAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<CreateConfluentAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts",
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::ConfluentAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Confluent resource for the account associated with the provided ID.
    pub async fn create_confluent_resource(
        &self,
        params: CreateConfluentResourceParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<CreateConfluentResourceError>,
    > {
        match self.create_confluent_resource_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Confluent resource for the account associated with the provided ID.
    pub async fn create_confluent_resource_with_http_info(
        &self,
        params: CreateConfluentResourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<CreateConfluentResourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources",
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::ConfluentResourceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Confluent account with the provided account ID.
    pub async fn delete_confluent_account(
        &self,
        params: DeleteConfluentAccountParams,
    ) -> Result<Option<()>, Error<DeleteConfluentAccountError>> {
        match self.delete_confluent_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Confluent account with the provided account ID.
    pub async fn delete_confluent_account_with_http_info(
        &self,
        params: DeleteConfluentAccountParams,
    ) -> Result<ResponseContent<()>, Error<DeleteConfluentAccountError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
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
            let local_entity: Option<DeleteConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn delete_confluent_resource(
        &self,
        params: DeleteConfluentResourceParams,
    ) -> Result<Option<()>, Error<DeleteConfluentResourceError>> {
        match self.delete_confluent_resource_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn delete_confluent_resource_with_http_info(
        &self,
        params: DeleteConfluentResourceParams,
    ) -> Result<ResponseContent<()>, Error<DeleteConfluentResourceError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let resource_id = params.resource_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
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
            let local_entity: Option<DeleteConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the Confluent account with the provided account ID.
    pub async fn get_confluent_account(
        &self,
        params: GetConfluentAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<GetConfluentAccountError>,
    > {
        match self.get_confluent_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the Confluent account with the provided account ID.
    pub async fn get_confluent_account_with_http_info(
        &self,
        params: GetConfluentAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<GetConfluentAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
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
            let local_entity: Option<crate::datadogV2::model::ConfluentAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn get_confluent_resource(
        &self,
        params: GetConfluentResourceParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<GetConfluentResourceError>,
    > {
        match self.get_confluent_resource_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn get_confluent_resource_with_http_info(
        &self,
        params: GetConfluentResourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<GetConfluentResourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let resource_id = params.resource_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
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
            let local_entity: Option<crate::datadogV2::model::ConfluentResourceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Confluent accounts.
    pub async fn list_confluent_account(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentAccountsResponse>,
        Error<ListConfluentAccountError>,
    > {
        match self.list_confluent_account_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List Confluent accounts.
    pub async fn list_confluent_account_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountsResponse>,
        Error<ListConfluentAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts",
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
            let local_entity: Option<crate::datadogV2::model::ConfluentAccountsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Confluent resource for the account associated with the provided ID.
    pub async fn list_confluent_resource(
        &self,
        params: ListConfluentResourceParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentResourcesResponse>,
        Error<ListConfluentResourceError>,
    > {
        match self.list_confluent_resource_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Confluent resource for the account associated with the provided ID.
    pub async fn list_confluent_resource_with_http_info(
        &self,
        params: ListConfluentResourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourcesResponse>,
        Error<ListConfluentResourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources",
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
            let local_entity: Option<crate::datadogV2::model::ConfluentResourcesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the Confluent account with the provided account ID.
    pub async fn update_confluent_account(
        &self,
        params: UpdateConfluentAccountParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<UpdateConfluentAccountError>,
    > {
        match self.update_confluent_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the Confluent account with the provided account ID.
    pub async fn update_confluent_account_with_http_info(
        &self,
        params: UpdateConfluentAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<UpdateConfluentAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::ConfluentAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn update_confluent_resource(
        &self,
        params: UpdateConfluentResourceParams,
    ) -> Result<
        Option<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<UpdateConfluentResourceError>,
    > {
        match self.update_confluent_resource_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn update_confluent_resource_with_http_info(
        &self,
        params: UpdateConfluentResourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<UpdateConfluentResourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let resource_id = params.resource_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.base_path,
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::ConfluentResourceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateConfluentResourceError> =
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
