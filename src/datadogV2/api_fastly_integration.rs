// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;

/// CreateFastlyAccountParams is a struct for passing parameters to the method [`CreateFastlyAccount`]
#[derive(Clone, Debug, Default)]
pub struct CreateFastlyAccountParams {
    pub body: crate::datadogV2::FastlyAccountCreateRequest,
}

/// CreateFastlyServiceParams is a struct for passing parameters to the method [`CreateFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct CreateFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    pub body: crate::datadogV2::FastlyServiceRequest,
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
    pub body: crate::datadogV2::FastlyAccountUpdateRequest,
}

/// UpdateFastlyServiceParams is a struct for passing parameters to the method [`UpdateFastlyService`]
#[derive(Clone, Debug, Default)]
pub struct UpdateFastlyServiceParams {
    /// Fastly Account id.
    pub account_id: String,
    /// Fastly Service ID.
    pub service_id: String,
    pub body: crate::datadogV2::FastlyServiceRequest,
}

/// CreateFastlyAccountError is a struct for typed errors of method [`CreateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyAccountError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFastlyServiceError is a struct for typed errors of method [`CreateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyServiceError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyAccountError is a struct for typed errors of method [`DeleteFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyAccountError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyServiceError is a struct for typed errors of method [`DeleteFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyServiceError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFastlyAccountError is a struct for typed errors of method [`GetFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyAccountError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFastlyServiceError is a struct for typed errors of method [`GetFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyServiceError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFastlyAccountsError is a struct for typed errors of method [`ListFastlyAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyAccountsError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFastlyServicesError is a struct for typed errors of method [`ListFastlyServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyServicesError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyAccountError is a struct for typed errors of method [`UpdateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyAccountError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyServiceError is a struct for typed errors of method [`UpdateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyServiceError {
    Status400(crate::datadogV2::APIErrorResponse),
    Status403(crate::datadogV2::APIErrorResponse),
    Status404(crate::datadogV2::APIErrorResponse),
    Status429(crate::datadogV2::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create a Fastly account.
pub async fn CreateFastlyAccount(
    configuration: &configuration::Configuration,
    params: CreateFastlyAccountParams,
) -> Result<crate::datadogV2::FastlyAccountResponse, Error<CreateFastlyAccountError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let body = params.body;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts",
        local_configuration.base_path
    );
    let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
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
pub async fn CreateFastlyService(
    configuration: &configuration::Configuration,
    params: CreateFastlyServiceParams,
) -> Result<crate::datadogV2::FastlyServiceResponse, Error<CreateFastlyServiceError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let body = params.body;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts/{account_id}/services",
        local_configuration.base_path,
        account_id = urlencode(account_id)
    );
    let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
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
pub async fn DeleteFastlyAccount(
    configuration: &configuration::Configuration,
    params: DeleteFastlyAccountParams,
) -> Result<(), Error<DeleteFastlyAccountError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts/{account_id}",
        local_configuration.base_path,
        account_id = urlencode(account_id)
    );
    let mut local_req_builder =
        local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
        Ok(())
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
pub async fn DeleteFastlyService(
    configuration: &configuration::Configuration,
    params: DeleteFastlyServiceParams,
) -> Result<(), Error<DeleteFastlyServiceError>> {
    let local_configuration = configuration;

    // unbox the parameters
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
        Ok(())
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
pub async fn GetFastlyAccount(
    configuration: &configuration::Configuration,
    params: GetFastlyAccountParams,
) -> Result<crate::datadogV2::FastlyAccountResponse, Error<GetFastlyAccountError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts/{account_id}",
        local_configuration.base_path,
        account_id = urlencode(account_id)
    );
    let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
    } else {
        let local_entity: Option<GetFastlyAccountError> = serde_json::from_str(&local_content).ok();
        let local_error = ResponseContent {
            status: local_status,
            content: local_content,
            entity: local_entity,
        };
        Err(Error::ResponseError(local_error))
    }
}

/// Get a Fastly service for an account.
pub async fn GetFastlyService(
    configuration: &configuration::Configuration,
    params: GetFastlyServiceParams,
) -> Result<crate::datadogV2::FastlyServiceResponse, Error<GetFastlyServiceError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let service_id = params.service_id;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts/{account_id}/services/{service_id}",
        local_configuration.base_path,
        account_id = urlencode(account_id),
        service_id = urlencode(service_id)
    );
    let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
    } else {
        let local_entity: Option<GetFastlyServiceError> = serde_json::from_str(&local_content).ok();
        let local_error = ResponseContent {
            status: local_status,
            content: local_content,
            entity: local_entity,
        };
        Err(Error::ResponseError(local_error))
    }
}

/// List Fastly accounts.
pub async fn ListFastlyAccounts(
    configuration: &configuration::Configuration,
) -> Result<crate::datadogV2::FastlyAccountsResponse, Error<ListFastlyAccountsError>> {
    let local_configuration = configuration;

    // unbox the parameters

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts",
        local_configuration.base_path
    );
    let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
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
pub async fn ListFastlyServices(
    configuration: &configuration::Configuration,
    params: ListFastlyServicesParams,
) -> Result<crate::datadogV2::FastlyServicesResponse, Error<ListFastlyServicesError>> {
    let local_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;

    let local_client = &local_configuration.client;

    let local_uri_str = format!(
        "{}/api/v2/integrations/fastly/accounts/{account_id}/services",
        local_configuration.base_path,
        account_id = urlencode(account_id)
    );
    let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
        serde_json::from_str(&local_content).map_err(Error::from)
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
pub async fn UpdateFastlyAccount(
    configuration: &configuration::Configuration,
    params: UpdateFastlyAccountParams,
) -> Result<crate::datadogV2::FastlyAccountResponse, Error<UpdateFastlyAccountError>> {
    let local_configuration = configuration;

    // unbox the parameters
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
        serde_json::from_str(&local_content).map_err(Error::from)
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
pub async fn UpdateFastlyService(
    configuration: &configuration::Configuration,
    params: UpdateFastlyServiceParams,
) -> Result<crate::datadogV2::FastlyServiceResponse, Error<UpdateFastlyServiceError>> {
    let local_configuration = configuration;

    // unbox the parameters
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
        serde_json::from_str(&local_content).map_err(Error::from)
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
