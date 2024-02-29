// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListPowerpacksOptionalParams is a struct for passing parameters to the method [`PowerpackAPI::list_powerpacks`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListPowerpacksOptionalParams {
    /// Maximum number of powerpacks in the response.
    pub page_limit: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

impl ListPowerpacksOptionalParams {
    /// Maximum number of powerpacks in the response.
    pub fn page_limit(&mut self, value: i64) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(&mut self, value: i64) -> &mut Self {
        self.page_offset = Some(value);
        self
    }
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
        body: crate::datadogV2::model::Powerpack,
    ) -> Result<crate::datadogV2::model::PowerpackResponse, Error<CreatePowerpackError>> {
        match self.create_powerpack_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a powerpack.
    pub async fn create_powerpack_with_http_info(
        &self,
        body: crate::datadogV2::model::Powerpack,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PowerpackResponse>,
        Error<CreatePowerpackError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_powerpack";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks",
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
            match serde_json::from_str::<crate::datadogV2::model::PowerpackResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        powerpack_id: String,
    ) -> Result<(), Error<DeletePowerpackError>> {
        match self.delete_powerpack_with_http_info(powerpack_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a powerpack.
    pub async fn delete_powerpack_with_http_info(
        &self,
        powerpack_id: String,
    ) -> Result<ResponseContent<()>, Error<DeletePowerpackError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_powerpack";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.get_operation_host(operation_id),
            powerpack_id = urlencode(powerpack_id)
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
        powerpack_id: String,
    ) -> Result<crate::datadogV2::model::PowerpackResponse, Error<GetPowerpackError>> {
        match self.get_powerpack_with_http_info(powerpack_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a powerpack.
    pub async fn get_powerpack_with_http_info(
        &self,
        powerpack_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::PowerpackResponse>, Error<GetPowerpackError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_powerpack";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.get_operation_host(operation_id),
            powerpack_id = urlencode(powerpack_id)
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
            match serde_json::from_str::<crate::datadogV2::model::PowerpackResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        params: ListPowerpacksOptionalParams,
    ) -> Result<crate::datadogV2::model::ListPowerpacksResponse, Error<ListPowerpacksError>> {
        match self.list_powerpacks_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a list of all powerpacks.
    pub async fn list_powerpacks_with_http_info(
        &self,
        params: ListPowerpacksOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListPowerpacksResponse>,
        Error<ListPowerpacksError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_powerpacks";

        // unbox and build optional parameters
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ListPowerpacksResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        powerpack_id: String,
        body: crate::datadogV2::model::Powerpack,
    ) -> Result<crate::datadogV2::model::PowerpackResponse, Error<UpdatePowerpackError>> {
        match self
            .update_powerpack_with_http_info(powerpack_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a powerpack.
    pub async fn update_powerpack_with_http_info(
        &self,
        powerpack_id: String,
        body: crate::datadogV2::model::Powerpack,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PowerpackResponse>,
        Error<UpdatePowerpackError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_powerpack";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/powerpacks/{powerpack_id}",
            local_configuration.get_operation_host(operation_id),
            powerpack_id = urlencode(powerpack_id)
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
            match serde_json::from_str::<crate::datadogV2::model::PowerpackResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
