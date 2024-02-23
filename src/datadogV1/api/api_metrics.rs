// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListActiveMetricsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::list_active_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListActiveMetricsOptionalParams {
    /// Hostname for filtering the list of metrics returned.
    /// If set, metrics retrieved are those with the corresponding hostname tag.
    pub host: Option<String>,
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Cannot be combined with other filters.
    pub tag_filter: Option<String>,
}

impl ListActiveMetricsOptionalParams {
    /// Hostname for filtering the list of metrics returned.
    /// If set, metrics retrieved are those with the corresponding hostname tag.
    pub fn host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Cannot be combined with other filters.
    pub fn tag_filter(&mut self, value: String) -> &mut Self {
        self.tag_filter = Some(value);
        self
    }
}

/// SubmitDistributionPointsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::submit_distribution_points`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SubmitDistributionPointsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV1::model::DistributionPointsContentEncoding>,
}

impl SubmitDistributionPointsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub fn content_encoding(
        &mut self,
        value: crate::datadogV1::model::DistributionPointsContentEncoding,
    ) -> &mut Self {
        self.content_encoding = Some(value);
        self
    }
}

/// SubmitMetricsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::submit_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SubmitMetricsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV1::model::MetricContentEncoding>,
}

impl SubmitMetricsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub fn content_encoding(
        &mut self,
        value: crate::datadogV1::model::MetricContentEncoding,
    ) -> &mut Self {
        self.content_encoding = Some(value);
        self
    }
}

/// GetMetricMetadataError is a struct for typed errors of method [`MetricsAPI::get_metric_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricMetadataError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListActiveMetricsError is a struct for typed errors of method [`MetricsAPI::list_active_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMetricsError is a struct for typed errors of method [`MetricsAPI::list_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// QueryMetricsError is a struct for typed errors of method [`MetricsAPI::query_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitDistributionPointsError is a struct for typed errors of method [`MetricsAPI::submit_distribution_points`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitDistributionPointsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status408(Option<crate::datadogV1::model::APIErrorResponse>),
    Status413(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitMetricsError is a struct for typed errors of method [`MetricsAPI::submit_metrics`]
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

/// UpdateMetricMetadataError is a struct for typed errors of method [`MetricsAPI::update_metric_metadata`]
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
        metric_name: String,
    ) -> Result<Option<crate::datadogV1::model::MetricMetadata>, Error<GetMetricMetadataError>>
    {
        match self.get_metric_metadata_with_http_info(metric_name).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get metadata about a specific metric.
    pub async fn get_metric_metadata_with_http_info(
        &self,
        metric_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricMetadata>,
        Error<GetMetricMetadataError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_metric_metadata";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.get_operation_host(operation_id),
            metric_name = urlencode(metric_name)
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
        from: i64,
        params: ListActiveMetricsOptionalParams,
    ) -> Result<Option<crate::datadogV1::model::MetricsListResponse>, Error<ListActiveMetricsError>>
    {
        match self.list_active_metrics_with_http_info(from, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of actively reporting metrics from a given time until now.
    pub async fn list_active_metrics_with_http_info(
        &self,
        from: i64,
        params: ListActiveMetricsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricsListResponse>,
        Error<ListActiveMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_active_metrics";

        // unbox and build optional parameters
        let host = params.host;
        let tag_filter = params.tag_filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("from", &from.to_string())]);
        if let Some(ref local_query_param) = host {
            local_req_builder =
                local_req_builder.query(&[("host", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tag_filter {
            local_req_builder =
                local_req_builder.query(&[("tag_filter", &local_query_param.to_string())]);
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
        q: String,
    ) -> Result<Option<crate::datadogV1::model::MetricSearchResponse>, Error<ListMetricsError>>
    {
        match self.list_metrics_with_http_info(q).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Search for metrics from the last 24 hours in Datadog.
    pub async fn list_metrics_with_http_info(
        &self,
        q: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricSearchResponse>,
        Error<ListMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_metrics";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("q", &q.to_string())]);

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
        from: i64,
        to: i64,
        query: String,
    ) -> Result<Option<crate::datadogV1::model::MetricsQueryResponse>, Error<QueryMetricsError>>
    {
        match self.query_metrics_with_http_info(from, to, query).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Query timeseries points.
    pub async fn query_metrics_with_http_info(
        &self,
        from: i64,
        to: i64,
        query: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricsQueryResponse>,
        Error<QueryMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.query_metrics";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/query",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("from", &from.to_string())]);
        local_req_builder = local_req_builder.query(&[("to", &to.to_string())]);
        local_req_builder = local_req_builder.query(&[("query", &query.to_string())]);

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

    /// The distribution points end-point allows you to post distribution data that can be graphed on Datadog’s dashboards.
    pub async fn submit_distribution_points(
        &self,
        body: crate::datadogV1::model::DistributionPointsPayload,
        params: SubmitDistributionPointsOptionalParams,
    ) -> Result<
        Option<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitDistributionPointsError>,
    > {
        match self
            .submit_distribution_points_with_http_info(body, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The distribution points end-point allows you to post distribution data that can be graphed on Datadog’s dashboards.
    pub async fn submit_distribution_points_with_http_info(
        &self,
        body: crate::datadogV1::model::DistributionPointsPayload,
        params: SubmitDistributionPointsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitDistributionPointsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.submit_distribution_points";

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/distribution_points",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
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
            let local_entity: Option<crate::datadogV1::model::IntakePayloadAccepted> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SubmitDistributionPointsError> =
                serde_json::from_str(&local_content).ok();
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
        body: crate::datadogV1::model::MetricsPayload,
        params: SubmitMetricsOptionalParams,
    ) -> Result<Option<crate::datadogV1::model::IntakePayloadAccepted>, Error<SubmitMetricsError>>
    {
        match self.submit_metrics_with_http_info(body, params).await {
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
        body: crate::datadogV1::model::MetricsPayload,
        params: SubmitMetricsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.submit_metrics";

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/series",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
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

    /// Edit metadata of a specific metric. Find out more about [supported types](<https://docs.datadoghq.com/developers/metrics>).
    pub async fn update_metric_metadata(
        &self,
        metric_name: String,
        body: crate::datadogV1::model::MetricMetadata,
    ) -> Result<Option<crate::datadogV1::model::MetricMetadata>, Error<UpdateMetricMetadataError>>
    {
        match self
            .update_metric_metadata_with_http_info(metric_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit metadata of a specific metric. Find out more about [supported types](<https://docs.datadoghq.com/developers/metrics>).
    pub async fn update_metric_metadata_with_http_info(
        &self,
        metric_name: String,
        body: crate::datadogV1::model::MetricMetadata,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MetricMetadata>,
        Error<UpdateMetricMetadataError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_metric_metadata";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.get_operation_host(operation_id),
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
