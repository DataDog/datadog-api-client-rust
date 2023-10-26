// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSpansMetricParams is a struct for passing parameters to the method [`CreateSpansMetric`]
#[derive(Clone, Debug, Default)]
pub struct CreateSpansMetricParams {
    /// The definition of the new span-based metric.
    pub body: crate::datadogV2::model::SpansMetricCreateRequest,
}

/// DeleteSpansMetricParams is a struct for passing parameters to the method [`DeleteSpansMetric`]
#[derive(Clone, Debug, Default)]
pub struct DeleteSpansMetricParams {
    /// The name of the span-based metric.
    pub metric_id: String,
}

/// GetSpansMetricParams is a struct for passing parameters to the method [`GetSpansMetric`]
#[derive(Clone, Debug, Default)]
pub struct GetSpansMetricParams {
    /// The name of the span-based metric.
    pub metric_id: String,
}

/// UpdateSpansMetricParams is a struct for passing parameters to the method [`UpdateSpansMetric`]
#[derive(Clone, Debug, Default)]
pub struct UpdateSpansMetricParams {
    /// The name of the span-based metric.
    pub metric_id: String,
    /// New definition of the span-based metric.
    pub body: crate::datadogV2::model::SpansMetricUpdateRequest,
}

/// CreateSpansMetricError is a struct for typed errors of method [`CreateSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSpansMetricError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSpansMetricError is a struct for typed errors of method [`DeleteSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSpansMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpansMetricError is a struct for typed errors of method [`GetSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpansMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSpansMetricsError is a struct for typed errors of method [`ListSpansMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansMetricsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSpansMetricError is a struct for typed errors of method [`UpdateSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpansMetricError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SpansMetricsAPI {
    config: configuration::Configuration,
}

impl Default for SpansMetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SpansMetricsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a metric based on your ingested spans in your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn create_spans_metric(
        &self,
        params: CreateSpansMetricParams,
    ) -> Result<Option<crate::datadogV2::model::SpansMetricResponse>, Error<CreateSpansMetricError>> {
        match self.create_spans_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a metric based on your ingested spans in your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn create_spans_metric_with_http_info(
        &self,
        params: CreateSpansMetricParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansMetricResponse>, Error<CreateSpansMetricError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/apm/config/metrics", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::SpansMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateSpansMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a specific span-based metric from your organization.
    pub async fn delete_spans_metric(
        &self,
        params: DeleteSpansMetricParams,
    ) -> Result<Option<()>, Error<DeleteSpansMetricError>> {
        match self.delete_spans_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific span-based metric from your organization.
    pub async fn delete_spans_metric_with_http_info(
        &self,
        params: DeleteSpansMetricParams,
    ) -> Result<ResponseContent<()>, Error<DeleteSpansMetricError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let metric_id = params.metric_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<DeleteSpansMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific span-based metric from your organization.
    pub async fn get_spans_metric(
        &self,
        params: GetSpansMetricParams,
    ) -> Result<Option<crate::datadogV2::model::SpansMetricResponse>, Error<GetSpansMetricError>> {
        match self.get_spans_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific span-based metric from your organization.
    pub async fn get_spans_metric_with_http_info(
        &self,
        params: GetSpansMetricParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansMetricResponse>, Error<GetSpansMetricError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let metric_id = params.metric_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::SpansMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSpansMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of configured span-based metrics with their definitions.
    pub async fn list_spans_metrics(
        &self,
    ) -> Result<Option<crate::datadogV2::model::SpansMetricsResponse>, Error<ListSpansMetricsError>> {
        match self.list_spans_metrics_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured span-based metrics with their definitions.
    pub async fn list_spans_metrics_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansMetricsResponse>, Error<ListSpansMetricsError>> {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/apm/config/metrics", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::SpansMetricsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSpansMetricsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a specific span-based metric from your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn update_spans_metric(
        &self,
        params: UpdateSpansMetricParams,
    ) -> Result<Option<crate::datadogV2::model::SpansMetricResponse>, Error<UpdateSpansMetricError>> {
        match self.update_spans_metric_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a specific span-based metric from your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn update_spans_metric_with_http_info(
        &self,
        params: UpdateSpansMetricParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::SpansMetricResponse>, Error<UpdateSpansMetricError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let metric_id = params.metric_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
            local_configuration.base_path,
            metric_id = urlencode(metric_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV2::model::SpansMetricResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateSpansMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
