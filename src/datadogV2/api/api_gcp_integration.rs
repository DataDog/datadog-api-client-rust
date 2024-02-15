// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// MakeGCPSTSDelegateOptionalParams is a struct for passing parameters to the method [`GCPIntegrationAPI::make_gcpsts_delegate`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct MakeGCPSTSDelegateOptionalParams {
    /// Create a delegate service account within Datadog.
    pub body: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl MakeGCPSTSDelegateOptionalParams {
    /// Create a delegate service account within Datadog.
    pub fn body(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.body = Some(value);
        self
    }
}

/// CreateGCPSTSAccountError is a struct for typed errors of method [`GCPIntegrationAPI::create_gcpsts_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGCPSTSAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteGCPSTSAccountError is a struct for typed errors of method [`GCPIntegrationAPI::delete_gcpsts_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGCPSTSAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetGCPSTSDelegateError is a struct for typed errors of method [`GCPIntegrationAPI::get_gcpsts_delegate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGCPSTSDelegateError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListGCPSTSAccountsError is a struct for typed errors of method [`GCPIntegrationAPI::list_gcpsts_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGCPSTSAccountsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// MakeGCPSTSDelegateError is a struct for typed errors of method [`GCPIntegrationAPI::make_gcpsts_delegate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MakeGCPSTSDelegateError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateGCPSTSAccountError is a struct for typed errors of method [`GCPIntegrationAPI::update_gcpsts_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGCPSTSAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct GCPIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for GCPIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl GCPIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a new entry within Datadog for your STS enabled service account.
    pub async fn create_gcpsts_account(
        &self,
        body: crate::datadogV2::model::GCPSTSServiceAccountCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::GCPSTSServiceAccountResponse>,
        Error<CreateGCPSTSAccountError>,
    > {
        match self.create_gcpsts_account_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new entry within Datadog for your STS enabled service account.
    pub async fn create_gcpsts_account_with_http_info(
        &self,
        body: crate::datadogV2::model::GCPSTSServiceAccountCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::GCPSTSServiceAccountResponse>,
        Error<CreateGCPSTSAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_gcpsts_account";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/accounts",
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
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
            let local_entity: Option<crate::datadogV2::model::GCPSTSServiceAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateGCPSTSAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an STS enabled GCP account from within Datadog.
    pub async fn delete_gcpsts_account(
        &self,
        account_id: String,
    ) -> Result<Option<()>, Error<DeleteGCPSTSAccountError>> {
        match self.delete_gcpsts_account_with_http_info(account_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an STS enabled GCP account from within Datadog.
    pub async fn delete_gcpsts_account_with_http_info(
        &self,
        account_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteGCPSTSAccountError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_gcpsts_account";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/accounts/{account_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
            let local_entity: Option<DeleteGCPSTSAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List your Datadog-GCP STS delegate account configured in your Datadog account.
    pub async fn get_gcpsts_delegate(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::GCPSTSDelegateAccountResponse>,
        Error<GetGCPSTSDelegateError>,
    > {
        match self.get_gcpsts_delegate_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List your Datadog-GCP STS delegate account configured in your Datadog account.
    pub async fn get_gcpsts_delegate_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::GCPSTSDelegateAccountResponse>,
        Error<GetGCPSTSDelegateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_gcpsts_delegate";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/sts_delegate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::GCPSTSDelegateAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetGCPSTSDelegateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all GCP STS-enabled service accounts configured in your Datadog account.
    pub async fn list_gcpsts_accounts(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::GCPSTSServiceAccountsResponse>,
        Error<ListGCPSTSAccountsError>,
    > {
        match self.list_gcpsts_accounts_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all GCP STS-enabled service accounts configured in your Datadog account.
    pub async fn list_gcpsts_accounts_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::GCPSTSServiceAccountsResponse>,
        Error<ListGCPSTSAccountsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_gcpsts_accounts";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/accounts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::GCPSTSServiceAccountsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListGCPSTSAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Datadog GCP principal.
    pub async fn make_gcpsts_delegate(
        &self,
        params: MakeGCPSTSDelegateOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::GCPSTSDelegateAccountResponse>,
        Error<MakeGCPSTSDelegateError>,
    > {
        match self.make_gcpsts_delegate_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Datadog GCP principal.
    pub async fn make_gcpsts_delegate_with_http_info(
        &self,
        params: MakeGCPSTSDelegateOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::GCPSTSDelegateAccountResponse>,
        Error<MakeGCPSTSDelegateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.make_gcpsts_delegate";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/sts_delegate",
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
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
            let local_entity: Option<crate::datadogV2::model::GCPSTSDelegateAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<MakeGCPSTSDelegateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an STS enabled service account.
    pub async fn update_gcpsts_account(
        &self,
        account_id: String,
        body: crate::datadogV2::model::GCPSTSServiceAccountUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::GCPSTSServiceAccountResponse>,
        Error<UpdateGCPSTSAccountError>,
    > {
        match self
            .update_gcpsts_account_with_http_info(account_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an STS enabled service account.
    pub async fn update_gcpsts_account_with_http_info(
        &self,
        account_id: String,
        body: crate::datadogV2::model::GCPSTSServiceAccountUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::GCPSTSServiceAccountResponse>,
        Error<UpdateGCPSTSAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_gcpsts_account";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/gcp/accounts/{account_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
            let local_entity: Option<crate::datadogV2::model::GCPSTSServiceAccountResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateGCPSTSAccountError> =
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
