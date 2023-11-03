// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetMetricMetadataParams is a struct for passing parameters to the method [`GetMetricMetadata`]
#[derive(Clone, Debug, Default)]
pub struct GetMetricMetadataParams {
    /// Name of the metric for which to get metadata.
    pub metric_name: String,
}

/// ListActiveMetricsParams is a struct for passing parameters to the method [`ListActiveMetrics`]
#[derive(Clone, Debug, Default)]
pub struct ListActiveMetricsParams {
    /// Seconds since the Unix epoch.
    pub from: i64,
    /// Hostname for filtering the list of metrics returned.
    /// If set, metrics retrieved are those with the corresponding hostname tag.
    pub host: Option<String>,
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Cannot be combined with other filters.
    pub tag_filter: Option<String>,
}

/// ListMetricsParams is a struct for passing parameters to the method [`ListMetrics`]
#[derive(Clone, Debug, Default)]
pub struct ListMetricsParams {
    /// Query string to search metrics upon. Can optionally be prefixed with `metrics:`.
    pub q: String,
}

/// QueryMetricsParams is a struct for passing parameters to the method [`QueryMetrics`]
#[derive(Clone, Debug, Default)]
pub struct QueryMetricsParams {
    /// Start of the queried time period, seconds since the Unix epoch.
    pub from: i64,
    /// End of the queried time period, seconds since the Unix epoch.
    pub to: i64,
    /// Query string.
    pub query: String,
}

/// SubmitMetricsParams is a struct for passing parameters to the method [`SubmitMetrics`]
#[derive(Clone, Debug, Default)]
pub struct SubmitMetricsParams {
    pub body: crate::datadogV1::model::MetricsPayload,
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV1::model::MetricContentEncoding>,
}

/// UpdateMetricMetadataParams is a struct for passing parameters to the method [`UpdateMetricMetadata`]
#[derive(Clone, Debug, Default)]
pub struct UpdateMetricMetadataParams {
    /// Name of the metric for which to edit metadata.
    pub metric_name: String,
    /// New metadata.
    pub body: crate::datadogV1::model::MetricMetadata,
}

/// GetMetricMetadataError is a struct for typed errors of method [`GetMetricMetadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricMetadataError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListActiveMetricsError is a struct for typed errors of method [`ListActiveMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMetricsError is a struct for typed errors of method [`ListMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// QueryMetricsError is a struct for typed errors of method [`QueryMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitMetricsError is a struct for typed errors of method [`SubmitMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status408(Option<crate::datadogV1::model::APIErrorResponse>),
    Status413(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateMetricMetadataError is a struct for typed errors of method [`UpdateMetricMetadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricMetadataError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
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

    /// Get metadata about a specific metric.
    pub async fn get_metric_metadata(
        &self,
        params: GetMetricMetadataParams,
    ) -> Result<Option<crate::datadogV1::model::MetricMetadata>, Error<GetMetricMetadataError>>
    {
        match self.get_metric_metadata_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get metadata about a specific metric.
    pub async fn get_metric_metadata_with_http_info(
        &self,
        params: GetMetricMetadataParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricMetadata>,
        Error<GetMetricMetadataError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let metric_name = params.metric_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters

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
            let local_entity: Option<crate::datadogV1::model::MetricMetadata> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetMetricMetadataError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of actively reporting metrics from a given time until now.
    pub async fn list_active_metrics(
        &self,
        params: ListActiveMetricsParams,
    ) -> Result<Option<crate::datadogV1::model::MetricsListResponse>, Error<ListActiveMetricsError>>
    {
        match self.list_active_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of actively reporting metrics from a given time until now.
    pub async fn list_active_metrics_with_http_info(
        &self,
        params: ListActiveMetricsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricsListResponse>,
        Error<ListActiveMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let from = params.from;
        let host = params.host;
        let tag_filter = params.tag_filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/metrics", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters
        local_req_builder = local_req_builder.query(&[("from", &from.to_string())]);
        if let Some(ref local_str) = host {
            local_req_builder = local_req_builder.query(&[("host", &local_str.to_string())]);
        };
        if let Some(ref local_str) = tag_filter {
            local_req_builder = local_req_builder.query(&[("tag_filter", &local_str.to_string())]);
        };

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
            let local_entity: Option<crate::datadogV1::model::MetricsListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListActiveMetricsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Search for metrics from the last 24 hours in Datadog.
    pub async fn list_metrics(
        &self,
        params: ListMetricsParams,
    ) -> Result<Option<crate::datadogV1::model::MetricSearchResponse>, Error<ListMetricsError>>
    {
        match self.list_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Search for metrics from the last 24 hours in Datadog.
    pub async fn list_metrics_with_http_info(
        &self,
        params: ListMetricsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricSearchResponse>,
        Error<ListMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let q = params.q;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/search", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters
        local_req_builder = local_req_builder.query(&[("q", &q.to_string())]);

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
            let local_entity: Option<crate::datadogV1::model::MetricSearchResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListMetricsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Query timeseries points.
    pub async fn query_metrics(
        &self,
        params: QueryMetricsParams,
    ) -> Result<Option<crate::datadogV1::model::MetricsQueryResponse>, Error<QueryMetricsError>>
    {
        match self.query_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Query timeseries points.
    pub async fn query_metrics_with_http_info(
        &self,
        params: QueryMetricsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricsQueryResponse>,
        Error<QueryMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let from = params.from;
        let to = params.to;
        let query = params.query;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/query", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build parameters
        local_req_builder = local_req_builder.query(&[("from", &from.to_string())]);
        local_req_builder = local_req_builder.query(&[("to", &to.to_string())]);
        local_req_builder = local_req_builder.query(&[("query", &query.to_string())]);

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
            let local_entity: Option<crate::datadogV1::model::MetricsQueryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<QueryMetricsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards.
    /// The maximum payload size is 3.2 megabytes (3200000 bytes). Compressed payloads must have a decompressed size of less than 62 megabytes (62914560 bytes).
    ///
    /// If you’re submitting metrics directly to the Datadog API without using DogStatsD, expect:
    ///
    /// - 64 bits for the timestamp
    /// - 64 bits for the value
    /// - 40 bytes for the metric names
    /// - 50 bytes for the timeseries
    /// - The full payload is approximately 100 bytes. However, with the DogStatsD API,
    /// compression is applied, which reduces the payload size.
    pub async fn submit_metrics(
        &self,
        params: SubmitMetricsParams,
    ) -> Result<Option<crate::datadogV1::model::IntakePayloadAccepted>, Error<SubmitMetricsError>>
    {
        match self.submit_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards.
    /// The maximum payload size is 3.2 megabytes (3200000 bytes). Compressed payloads must have a decompressed size of less than 62 megabytes (62914560 bytes).
    ///
    /// If you’re submitting metrics directly to the Datadog API without using DogStatsD, expect:
    ///
    /// - 64 bits for the timestamp
    /// - 64 bits for the value
    /// - 40 bytes for the metric names
    /// - 50 bytes for the timeseries
    /// - The full payload is approximately 100 bytes. However, with the DogStatsD API,
    /// compression is applied, which reduces the payload size.
    pub async fn submit_metrics_with_http_info(
        &self,
        params: SubmitMetricsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;
        let content_encoding = params.content_encoding;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/series", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build parameters
        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
        };

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
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
            let local_entity: Option<crate::datadogV1::model::IntakePayloadAccepted> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SubmitMetricsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit metadata of a specific metric. Find out more about [supported types](https://docs.datadoghq.com/developers/metrics).
    pub async fn update_metric_metadata(
        &self,
        params: UpdateMetricMetadataParams,
    ) -> Result<Option<crate::datadogV1::model::MetricMetadata>, Error<UpdateMetricMetadataError>>
    {
        match self.update_metric_metadata_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit metadata of a specific metric. Find out more about [supported types](https://docs.datadoghq.com/developers/metrics).
    pub async fn update_metric_metadata_with_http_info(
        &self,
        params: UpdateMetricMetadataParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricMetadata>,
        Error<UpdateMetricMetadataError>,
    > {
        let local_configuration = &self.config;

        // unbox the parameters
        let metric_name = params.metric_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build parameters

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
            let local_entity: Option<crate::datadogV1::model::MetricMetadata> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateMetricMetadataError> =
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
