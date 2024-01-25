// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreatePowerpackParams is a struct for passing parameters to the method [`PowerpackAPI::create_powerpack`]
#[derive(Clone, Debug)]
pub struct CreatePowerpackParams {
    /// Create a powerpack request body.
    pub body: crate::datadogV2::model::Powerpack,
}

/// DeletePowerpackParams is a struct for passing parameters to the method [`PowerpackAPI::delete_powerpack`]
#[derive(Clone, Debug)]
pub struct DeletePowerpackParams {
    /// Powerpack id
    pub powerpack_id: String,
}

/// GetPowerpackParams is a struct for passing parameters to the method [`PowerpackAPI::get_powerpack`]
#[derive(Clone, Debug)]
pub struct GetPowerpackParams {
    /// ID of the powerpack.
    pub powerpack_id: String,
}

/// ListPowerpacksParams is a struct for passing parameters to the method [`PowerpackAPI::list_powerpacks`]
#[derive(Clone, Debug)]
pub struct ListPowerpacksParams {
    /// Maximum number of powerpacks in the response.
    pub page_limit: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

/// UpdatePowerpackParams is a struct for passing parameters to the method [`PowerpackAPI::update_powerpack`]
#[derive(Clone, Debug)]
pub struct UpdatePowerpackParams {
    /// ID of the powerpack.
    pub powerpack_id: String,
    /// Update a powerpack request body.
    pub body: crate::datadogV2::model::Powerpack,
}

/// CreatePowerpackError is a struct for typed errors of method [`PowerpackAPI::create_powerpack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePowerpackError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePowerpackError is a struct for typed errors of method [`PowerpackAPI::delete_powerpack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePowerpackError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPowerpackError is a struct for typed errors of method [`PowerpackAPI::get_powerpack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPowerpackError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListPowerpacksError is a struct for typed errors of method [`PowerpackAPI::list_powerpacks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPowerpacksError {
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePowerpackError is a struct for typed errors of method [`PowerpackAPI::update_powerpack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePowerpackError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct PowerpackAPI {
    config: configuration::Configuration,
}

impl Default for PowerpackAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl PowerpackAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a powerpack.
    pub async fn create_powerpack(
        &self,
        params: CreatePowerpackParams,
    ) -> Result<Option<crate::datadogV2::model::PowerpackResponse>, Error<CreatePowerpackError>>
    {
        match self.create_powerpack_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a powerpack.
    pub async fn create_powerpack_with_http_info(
        &self,
        params: CreatePowerpackParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PowerpackResponse>,
        Error<CreatePowerpackError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/powerpacks", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::PowerpackResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreatePowerpackError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a powerpack.
    pub async fn delete_powerpack(
        &self,
        params: DeletePowerpackParams,
    ) -> Result<Option<()>, Error<DeletePowerpackError>> {
        match self.delete_powerpack_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a powerpack.
    pub async fn delete_powerpack_with_http_info(
        &self,
        params: DeletePowerpackParams,
    ) -> Result<ResponseContent<()>, Error<DeletePowerpackError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let powerpack_id = params.powerpack_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.base_path,
            powerpack_id = urlencode(powerpack_id)
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
            let local_entity: Option<DeletePowerpackError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a powerpack.
    pub async fn get_powerpack(
        &self,
        params: GetPowerpackParams,
    ) -> Result<Option<crate::datadogV2::model::PowerpackResponse>, Error<GetPowerpackError>> {
        match self.get_powerpack_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a powerpack.
    pub async fn get_powerpack_with_http_info(
        &self,
        params: GetPowerpackParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::PowerpackResponse>, Error<GetPowerpackError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let powerpack_id = params.powerpack_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.base_path,
            powerpack_id = urlencode(powerpack_id)
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
            let local_entity: Option<crate::datadogV2::model::PowerpackResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetPowerpackError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of all powerpacks.
    pub async fn list_powerpacks(
        &self,
        params: ListPowerpacksParams,
    ) -> Result<Option<crate::datadogV2::model::ListPowerpacksResponse>, Error<ListPowerpacksError>>
    {
        match self.list_powerpacks_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of all powerpacks.
    pub async fn list_powerpacks_with_http_info(
        &self,
        params: ListPowerpacksParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListPowerpacksResponse>,
        Error<ListPowerpacksError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/powerpacks", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_limit {
            local_req_builder = local_req_builder.query(&[("page[limit]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ListPowerpacksResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListPowerpacksError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a powerpack.
    pub async fn update_powerpack(
        &self,
        params: UpdatePowerpackParams,
    ) -> Result<Option<crate::datadogV2::model::PowerpackResponse>, Error<UpdatePowerpackError>>
    {
        match self.update_powerpack_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a powerpack.
    pub async fn update_powerpack_with_http_info(
        &self,
        params: UpdatePowerpackParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PowerpackResponse>,
        Error<UpdatePowerpackError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let powerpack_id = params.powerpack_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.base_path,
            powerpack_id = urlencode(powerpack_id)
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
            let local_entity: Option<crate::datadogV2::model::PowerpackResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdatePowerpackError> =
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
