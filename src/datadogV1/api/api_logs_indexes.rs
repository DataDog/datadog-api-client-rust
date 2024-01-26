// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateLogsIndexParams is a struct for passing parameters to the method [`LogsIndexesAPI::create_logs_index`]
#[derive(Clone, Debug)]
pub struct CreateLogsIndexParams {
    /// Object containing the new index.
    pub body: crate::datadogV1::model::LogsIndex,
}

/// GetLogsIndexParams is a struct for passing parameters to the method [`LogsIndexesAPI::get_logs_index`]
#[derive(Clone, Debug)]
pub struct GetLogsIndexParams {
    /// Name of the log index.
    pub name: String,
}

/// UpdateLogsIndexParams is a struct for passing parameters to the method [`LogsIndexesAPI::update_logs_index`]
#[derive(Clone, Debug)]
pub struct UpdateLogsIndexParams {
    /// Name of the log index.
    pub name: String,
    /// Object containing the new `LogsIndexUpdateRequest`.
    pub body: crate::datadogV1::model::LogsIndexUpdateRequest,
}

/// UpdateLogsIndexOrderParams is a struct for passing parameters to the method [`LogsIndexesAPI::update_logs_index_order`]
#[derive(Clone, Debug)]
pub struct UpdateLogsIndexOrderParams {
    /// Object containing the new ordered list of index names
    pub body: crate::datadogV1::model::LogsIndexesOrder,
}

/// CreateLogsIndexError is a struct for typed errors of method [`LogsIndexesAPI::create_logs_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsIndexError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsIndexError is a struct for typed errors of method [`LogsIndexesAPI::get_logs_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsIndexError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsIndexOrderError is a struct for typed errors of method [`LogsIndexesAPI::get_logs_index_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsIndexOrderError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogIndexesError is a struct for typed errors of method [`LogsIndexesAPI::list_log_indexes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogIndexesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsIndexError is a struct for typed errors of method [`LogsIndexesAPI::update_logs_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsIndexError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status429(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsIndexOrderError is a struct for typed errors of method [`LogsIndexesAPI::update_logs_index_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsIndexOrderError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct LogsIndexesAPI {
    config: configuration::Configuration,
}

impl Default for LogsIndexesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl LogsIndexesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates a new index. Returns the Index object passed in the request body when the request is successful.
    pub async fn create_logs_index(
        &self,
        params: CreateLogsIndexParams,
    ) -> Result<Option<crate::datadogV1::model::LogsIndex>, Error<CreateLogsIndexError>> {
        match self.create_logs_index_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates a new index. Returns the Index object passed in the request body when the request is successful.
    pub async fn create_logs_index_with_http_info(
        &self,
        params: CreateLogsIndexParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::LogsIndex>, Error<CreateLogsIndexError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/indexes",
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
            let local_entity: Option<crate::datadogV1::model::LogsIndex> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateLogsIndexError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get one log index from your organization. This endpoint takes no JSON arguments.
    pub async fn get_logs_index(
        &self,
        params: GetLogsIndexParams,
    ) -> Result<Option<crate::datadogV1::model::LogsIndex>, Error<GetLogsIndexError>> {
        match self.get_logs_index_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get one log index from your organization. This endpoint takes no JSON arguments.
    pub async fn get_logs_index_with_http_info(
        &self,
        params: GetLogsIndexParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::LogsIndex>, Error<GetLogsIndexError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let name = params.name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/indexes/{name}",
            local_configuration.base_path,
            name = urlencode(name)
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
            let local_entity: Option<crate::datadogV1::model::LogsIndex> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsIndexError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the current order of your log indexes. This endpoint takes no JSON arguments.
    pub async fn get_logs_index_order(
        &self,
    ) -> Result<Option<crate::datadogV1::model::LogsIndexesOrder>, Error<GetLogsIndexOrderError>>
    {
        match self.get_logs_index_order_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the current order of your log indexes. This endpoint takes no JSON arguments.
    pub async fn get_logs_index_order_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsIndexesOrder>,
        Error<GetLogsIndexOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/index-order",
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
            let local_entity: Option<crate::datadogV1::model::LogsIndexesOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsIndexOrderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The Index object describes the configuration of a log index.
    /// This endpoint returns an array of the `LogIndex` objects of your organization.
    pub async fn list_log_indexes(
        &self,
    ) -> Result<Option<crate::datadogV1::model::LogsIndexListResponse>, Error<ListLogIndexesError>>
    {
        match self.list_log_indexes_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The Index object describes the configuration of a log index.
    /// This endpoint returns an array of the `LogIndex` objects of your organization.
    pub async fn list_log_indexes_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsIndexListResponse>,
        Error<ListLogIndexesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/indexes",
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
            let local_entity: Option<crate::datadogV1::model::LogsIndexListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogIndexesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an index as identified by its name.
    /// Returns the Index object passed in the request body when the request is successful.
    ///
    /// Using the `PUT` method updates your index’s configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_index(
        &self,
        params: UpdateLogsIndexParams,
    ) -> Result<Option<crate::datadogV1::model::LogsIndex>, Error<UpdateLogsIndexError>> {
        match self.update_logs_index_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an index as identified by its name.
    /// Returns the Index object passed in the request body when the request is successful.
    ///
    /// Using the `PUT` method updates your index’s configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_index_with_http_info(
        &self,
        params: UpdateLogsIndexParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::LogsIndex>, Error<UpdateLogsIndexError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let name = params.name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/indexes/{name}",
            local_configuration.base_path,
            name = urlencode(name)
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
            let local_entity: Option<crate::datadogV1::model::LogsIndex> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsIndexError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint updates the index order of your organization.
    /// It returns the index order object passed in the request body when the request is successful.
    pub async fn update_logs_index_order(
        &self,
        params: UpdateLogsIndexOrderParams,
    ) -> Result<Option<crate::datadogV1::model::LogsIndexesOrder>, Error<UpdateLogsIndexOrderError>>
    {
        match self.update_logs_index_order_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint updates the index order of your organization.
    /// It returns the index order object passed in the request body when the request is successful.
    pub async fn update_logs_index_order_with_http_info(
        &self,
        params: UpdateLogsIndexOrderParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsIndexesOrder>,
        Error<UpdateLogsIndexOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/index-order",
            local_configuration.base_path
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
            let local_entity: Option<crate::datadogV1::model::LogsIndexesOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsIndexOrderError> =
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
