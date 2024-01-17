// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateLogsMetricParams is a struct for passing parameters to the method [`CreateLogsMetric`]
#[derive(Clone, Debug)]
pub struct CreateLogsMetricParams {
    /// The definition of the new log-based metric.
    pub body: crate::datadogV2::model::LogsMetricCreateRequest,
}

/// DeleteLogsMetricParams is a struct for passing parameters to the method [`DeleteLogsMetric`]
#[derive(Clone, Debug)]
pub struct DeleteLogsMetricParams {
    /// The name of the log-based metric.
    pub metric_id: String,
}

/// GetLogsMetricParams is a struct for passing parameters to the method [`GetLogsMetric`]
#[derive(Clone, Debug)]
pub struct GetLogsMetricParams {
    /// The name of the log-based metric.
    pub metric_id: String,
}

/// UpdateLogsMetricParams is a struct for passing parameters to the method [`UpdateLogsMetric`]
#[derive(Clone, Debug)]
pub struct UpdateLogsMetricParams {
    /// The name of the log-based metric.
    pub metric_id: String,
    /// New definition of the log-based metric.
    pub body: crate::datadogV2::model::LogsMetricUpdateRequest,
}

/// CreateLogsMetricError is a struct for typed errors of method [`CreateLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsMetricError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsMetricError is a struct for typed errors of method [`DeleteLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsMetricError is a struct for typed errors of method [`GetLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsMetricsError is a struct for typed errors of method [`ListLogsMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsMetricsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsMetricError is a struct for typed errors of method [`UpdateLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsMetricError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct LogsMetricsAPI {
    config: configuration::Configuration,
}

impl Default for LogsMetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl LogsMetricsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a metric based on your ingested logs in your organization.
    /// Returns the log-based metric object from the request body when the request is successful.
    pub async fn create_logs_metric(
        &self,
        params: CreateLogsMetricParams,
    ) -> Result<Option<crate::datadogV2::model::LogsMetricResponse>, Error<CreateLogsMetricError>>
    {
        match self.create_logs_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a metric based on your ingested logs in your organization.
    /// Returns the log-based metric object from the request body when the request is successful.
    pub async fn create_logs_metric_with_http_info(
        &self,
        params: CreateLogsMetricParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsMetricResponse>,
        Error<CreateLogsMetricError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/metrics",
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
            let local_entity: Option<crate::datadogV2::model::LogsMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateLogsMetricError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a specific log-based metric from your organization.
    pub async fn delete_logs_metric(
        &self,
        params: DeleteLogsMetricParams,
    ) -> Result<Option<()>, Error<DeleteLogsMetricError>> {
        match self.delete_logs_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific log-based metric from your organization.
    pub async fn delete_logs_metric_with_http_info(
        &self,
        params: DeleteLogsMetricParams,
    ) -> Result<ResponseContent<()>, Error<DeleteLogsMetricError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let metric_id = params.metric_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
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
            let local_entity: Option<DeleteLogsMetricError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific log-based metric from your organization.
    pub async fn get_logs_metric(
        &self,
        params: GetLogsMetricParams,
    ) -> Result<Option<crate::datadogV2::model::LogsMetricResponse>, Error<GetLogsMetricError>>
    {
        match self.get_logs_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific log-based metric from your organization.
    pub async fn get_logs_metric_with_http_info(
        &self,
        params: GetLogsMetricParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsMetricResponse>,
        Error<GetLogsMetricError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let metric_id = params.metric_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
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
            let local_entity: Option<crate::datadogV2::model::LogsMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsMetricError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of configured log-based metrics with their definitions.
    pub async fn list_logs_metrics(
        &self,
    ) -> Result<Option<crate::datadogV2::model::LogsMetricsResponse>, Error<ListLogsMetricsError>>
    {
        match self.list_logs_metrics_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured log-based metrics with their definitions.
    pub async fn list_logs_metrics_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsMetricsResponse>,
        Error<ListLogsMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/metrics",
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
            let local_entity: Option<crate::datadogV2::model::LogsMetricsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogsMetricsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a specific log-based metric from your organization.
    /// Returns the log-based metric object from the request body when the request is successful.
    pub async fn update_logs_metric(
        &self,
        params: UpdateLogsMetricParams,
    ) -> Result<Option<crate::datadogV2::model::LogsMetricResponse>, Error<UpdateLogsMetricError>>
    {
        match self.update_logs_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a specific log-based metric from your organization.
    /// Returns the log-based metric object from the request body when the request is successful.
    pub async fn update_logs_metric_with_http_info(
        &self,
        params: UpdateLogsMetricParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsMetricResponse>,
        Error<UpdateLogsMetricError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let metric_id = params.metric_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
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
            let local_entity: Option<crate::datadogV2::model::LogsMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsMetricError> =
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
