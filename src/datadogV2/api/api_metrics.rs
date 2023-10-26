// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// SubmitMetricsParams is a struct for passing parameters to the method [`SubmitMetrics`]
#[derive(Clone, Debug, Default)]
pub struct SubmitMetricsParams {
    pub body: crate::datadogV2::model::MetricPayload,
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV2::model::MetricContentEncoding>,
}

/// SubmitMetricsError is a struct for typed errors of method [`SubmitMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitMetricsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status408(Option<crate::datadogV2::model::APIErrorResponse>),
    Status413(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct MetricsAPI {
    config: configuration::Configuration,
}

impl Default for MetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl MetricsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards.
    /// The maximum payload size is 500 kilobytes (512000 bytes). Compressed payloads must have a decompressed size of less than 5 megabytes (5242880 bytes).
    ///
    /// If you’re submitting metrics directly to the Datadog API without using DogStatsD, expect:
    ///
    /// - 64 bits for the timestamp
    /// - 64 bits for the value
    /// - 20 bytes for the metric names
    /// - 50 bytes for the timeseries
    /// - The full payload is approximately 100 bytes.
    ///
    /// Host name is one of the resources in the Resources field.
    pub async fn submit_metrics(
        &self,
        params: SubmitMetricsParams,
    ) -> Result<Option<crate::datadogV2::model::IntakePayloadAccepted>, Error<SubmitMetricsError>> {
        match self.submit_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards.
    /// The maximum payload size is 500 kilobytes (512000 bytes). Compressed payloads must have a decompressed size of less than 5 megabytes (5242880 bytes).
    ///
    /// If you’re submitting metrics directly to the Datadog API without using DogStatsD, expect:
    ///
    /// - 64 bits for the timestamp
    /// - 64 bits for the value
    /// - 20 bytes for the metric names
    /// - 50 bytes for the timeseries
    /// - The full payload is approximately 100 bytes.
    ///
    /// Host name is one of the resources in the Resources field.
    pub async fn submit_metrics_with_http_info(
        &self,
        params: SubmitMetricsParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::IntakePayloadAccepted>, Error<SubmitMetricsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;
        let content_encoding = params.content_encoding;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/series", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::IntakePayloadAccepted> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SubmitMetricsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
