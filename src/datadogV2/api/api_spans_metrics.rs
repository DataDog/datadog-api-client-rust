// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSpansMetricError is a struct for typed errors of method [`SpansMetricsAPI::create_spans_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSpansMetricError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSpansMetricError is a struct for typed errors of method [`SpansMetricsAPI::delete_spans_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSpansMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpansMetricError is a struct for typed errors of method [`SpansMetricsAPI::get_spans_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpansMetricError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSpansMetricsError is a struct for typed errors of method [`SpansMetricsAPI::list_spans_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansMetricsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSpansMetricError is a struct for typed errors of method [`SpansMetricsAPI::update_spans_metric`]
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
        body: crate::datadogV2::model::SpansMetricCreateRequest,
    ) -> Result<crate::datadogV2::model::SpansMetricResponse, Error<CreateSpansMetricError>> {
        match self.create_spans_metric_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Create a metric based on your ingested spans in your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn create_spans_metric_with_http_info(
        &self,
        body: crate::datadogV2::model::SpansMetricCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SpansMetricResponse>,
        Error<CreateSpansMetricError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics",
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
            match serde_json::from_str::<crate::datadogV2::model::SpansMetricResponse>(
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
            let local_entity: Option<CreateSpansMetricError> =
                serde_json::from_str(&local_content).ok();
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
        metric_id: String,
    ) -> Result<(), Error<DeleteSpansMetricError>> {
        match self.delete_spans_metric_with_http_info(metric_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific span-based metric from your organization.
    pub async fn delete_spans_metric_with_http_info(
        &self,
        metric_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteSpansMetricError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
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
            let local_entity: Option<DeleteSpansMetricError> =
                serde_json::from_str(&local_content).ok();
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
        metric_id: String,
    ) -> Result<crate::datadogV2::model::SpansMetricResponse, Error<GetSpansMetricError>> {
        match self.get_spans_metric_with_http_info(metric_id).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get a specific span-based metric from your organization.
    pub async fn get_spans_metric_with_http_info(
        &self,
        metric_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SpansMetricResponse>,
        Error<GetSpansMetricError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
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
            match serde_json::from_str::<crate::datadogV2::model::SpansMetricResponse>(
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
            let local_entity: Option<GetSpansMetricError> =
                serde_json::from_str(&local_content).ok();
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
    ) -> Result<crate::datadogV2::model::SpansMetricsResponse, Error<ListSpansMetricsError>> {
        match self.list_spans_metrics_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured span-based metrics with their definitions.
    pub async fn list_spans_metrics_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SpansMetricsResponse>,
        Error<ListSpansMetricsError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics",
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
            match serde_json::from_str::<crate::datadogV2::model::SpansMetricsResponse>(
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
            let local_entity: Option<ListSpansMetricsError> =
                serde_json::from_str(&local_content).ok();
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
        metric_id: String,
        body: crate::datadogV2::model::SpansMetricUpdateRequest,
    ) -> Result<crate::datadogV2::model::SpansMetricResponse, Error<UpdateSpansMetricError>> {
        match self
            .update_spans_metric_with_http_info(metric_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Update a specific span-based metric from your organization.
    /// Returns the span-based metric object from the request body when the request is successful.
    pub async fn update_spans_metric_with_http_info(
        &self,
        metric_id: String,
        body: crate::datadogV2::model::SpansMetricUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SpansMetricResponse>,
        Error<UpdateSpansMetricError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/metrics/{metric_id}",
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
            match serde_json::from_str::<crate::datadogV2::model::SpansMetricResponse>(
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
            let local_entity: Option<UpdateSpansMetricError> =
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
