// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateLogsPipelineParams is a struct for passing parameters to the method [`LogsPipelinesAPI::create_logs_pipeline`]
#[derive(Clone, Debug)]
pub struct CreateLogsPipelineParams {
    /// Definition of the new pipeline.
    pub body: crate::datadogV1::model::LogsPipeline,
}

/// DeleteLogsPipelineParams is a struct for passing parameters to the method [`LogsPipelinesAPI::delete_logs_pipeline`]
#[derive(Clone, Debug)]
pub struct DeleteLogsPipelineParams {
    /// ID of the pipeline to delete.
    pub pipeline_id: String,
}

/// GetLogsPipelineParams is a struct for passing parameters to the method [`LogsPipelinesAPI::get_logs_pipeline`]
#[derive(Clone, Debug)]
pub struct GetLogsPipelineParams {
    /// ID of the pipeline to get.
    pub pipeline_id: String,
}

/// UpdateLogsPipelineParams is a struct for passing parameters to the method [`LogsPipelinesAPI::update_logs_pipeline`]
#[derive(Clone, Debug)]
pub struct UpdateLogsPipelineParams {
    /// ID of the pipeline to delete.
    pub pipeline_id: String,
    /// New definition of the pipeline.
    pub body: crate::datadogV1::model::LogsPipeline,
}

/// UpdateLogsPipelineOrderParams is a struct for passing parameters to the method [`LogsPipelinesAPI::update_logs_pipeline_order`]
#[derive(Clone, Debug)]
pub struct UpdateLogsPipelineOrderParams {
    /// Object containing the new ordered list of pipeline IDs.
    pub body: crate::datadogV1::model::LogsPipelinesOrder,
}

/// CreateLogsPipelineError is a struct for typed errors of method [`LogsPipelinesAPI::create_logs_pipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsPipelineError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsPipelineError is a struct for typed errors of method [`LogsPipelinesAPI::delete_logs_pipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsPipelineError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsPipelineError is a struct for typed errors of method [`LogsPipelinesAPI::get_logs_pipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsPipelineError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsPipelineOrderError is a struct for typed errors of method [`LogsPipelinesAPI::get_logs_pipeline_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsPipelineOrderError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsPipelinesError is a struct for typed errors of method [`LogsPipelinesAPI::list_logs_pipelines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsPipelinesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsPipelineError is a struct for typed errors of method [`LogsPipelinesAPI::update_logs_pipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsPipelineError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsPipelineOrderError is a struct for typed errors of method [`LogsPipelinesAPI::update_logs_pipeline_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsPipelineOrderError {
    Status400(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status422(Option<crate::datadogV1::model::LogsAPIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct LogsPipelinesAPI {
    config: configuration::Configuration,
}

impl Default for LogsPipelinesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl LogsPipelinesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a pipeline in your organization.
    pub async fn create_logs_pipeline(
        &self,
        params: CreateLogsPipelineParams,
    ) -> Result<Option<crate::datadogV1::model::LogsPipeline>, Error<CreateLogsPipelineError>> {
        match self.create_logs_pipeline_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a pipeline in your organization.
    pub async fn create_logs_pipeline_with_http_info(
        &self,
        params: CreateLogsPipelineParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsPipeline>,
        Error<CreateLogsPipelineError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipelines",
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
            let local_entity: Option<crate::datadogV1::model::LogsPipeline> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateLogsPipelineError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given pipeline from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn delete_logs_pipeline(
        &self,
        params: DeleteLogsPipelineParams,
    ) -> Result<Option<()>, Error<DeleteLogsPipelineError>> {
        match self.delete_logs_pipeline_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given pipeline from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn delete_logs_pipeline_with_http_info(
        &self,
        params: DeleteLogsPipelineParams,
    ) -> Result<ResponseContent<()>, Error<DeleteLogsPipelineError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let pipeline_id = params.pipeline_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipelines/{pipeline_id}",
            local_configuration.base_path,
            pipeline_id = urlencode(pipeline_id)
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
            let local_entity: Option<DeleteLogsPipelineError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific pipeline from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_pipeline(
        &self,
        params: GetLogsPipelineParams,
    ) -> Result<Option<crate::datadogV1::model::LogsPipeline>, Error<GetLogsPipelineError>> {
        match self.get_logs_pipeline_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific pipeline from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_pipeline_with_http_info(
        &self,
        params: GetLogsPipelineParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::LogsPipeline>, Error<GetLogsPipelineError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let pipeline_id = params.pipeline_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipelines/{pipeline_id}",
            local_configuration.base_path,
            pipeline_id = urlencode(pipeline_id)
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
            let local_entity: Option<crate::datadogV1::model::LogsPipeline> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsPipelineError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the current order of your pipelines.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_pipeline_order(
        &self,
    ) -> Result<Option<crate::datadogV1::model::LogsPipelinesOrder>, Error<GetLogsPipelineOrderError>>
    {
        match self.get_logs_pipeline_order_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the current order of your pipelines.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_pipeline_order_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsPipelinesOrder>,
        Error<GetLogsPipelineOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipeline-order",
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
            let local_entity: Option<crate::datadogV1::model::LogsPipelinesOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsPipelineOrderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all pipelines from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn list_logs_pipelines(
        &self,
    ) -> Result<Option<Vec<crate::datadogV1::model::LogsPipeline>>, Error<ListLogsPipelinesError>>
    {
        match self.list_logs_pipelines_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all pipelines from your organization.
    /// This endpoint takes no JSON arguments.
    pub async fn list_logs_pipelines_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::LogsPipeline>>,
        Error<ListLogsPipelinesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipelines",
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
            let local_entity: Option<Vec<crate::datadogV1::model::LogsPipeline>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogsPipelinesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a given pipeline configuration to change it’s processors or their order.
    ///
    /// **Note**: Using this method updates your pipeline configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_pipeline(
        &self,
        params: UpdateLogsPipelineParams,
    ) -> Result<Option<crate::datadogV1::model::LogsPipeline>, Error<UpdateLogsPipelineError>> {
        match self.update_logs_pipeline_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a given pipeline configuration to change it’s processors or their order.
    ///
    /// **Note**: Using this method updates your pipeline configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_pipeline_with_http_info(
        &self,
        params: UpdateLogsPipelineParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsPipeline>,
        Error<UpdateLogsPipelineError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let pipeline_id = params.pipeline_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipelines/{pipeline_id}",
            local_configuration.base_path,
            pipeline_id = urlencode(pipeline_id)
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
            let local_entity: Option<crate::datadogV1::model::LogsPipeline> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsPipelineError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the order of your pipelines. Since logs are processed sequentially, reordering a pipeline may change
    /// the structure and content of the data processed by other pipelines and their processors.
    ///
    /// **Note**: Using the `PUT` method updates your pipeline order by replacing your current order
    /// with the new one sent to your Datadog organization.
    pub async fn update_logs_pipeline_order(
        &self,
        params: UpdateLogsPipelineOrderParams,
    ) -> Result<
        Option<crate::datadogV1::model::LogsPipelinesOrder>,
        Error<UpdateLogsPipelineOrderError>,
    > {
        match self.update_logs_pipeline_order_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the order of your pipelines. Since logs are processed sequentially, reordering a pipeline may change
    /// the structure and content of the data processed by other pipelines and their processors.
    ///
    /// **Note**: Using the `PUT` method updates your pipeline order by replacing your current order
    /// with the new one sent to your Datadog organization.
    pub async fn update_logs_pipeline_order_with_http_info(
        &self,
        params: UpdateLogsPipelineOrderParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::LogsPipelinesOrder>,
        Error<UpdateLogsPipelineOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/logs/config/pipeline-order",
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
            let local_entity: Option<crate::datadogV1::model::LogsPipelinesOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsPipelineOrderError> =
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
