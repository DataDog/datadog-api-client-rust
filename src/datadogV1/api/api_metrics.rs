// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use flate2::{
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
use reqwest;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Cannot be combined with other filters.
    pub fn tag_filter(mut self, value: String) -> Self {
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
        mut self,
        value: crate::datadogV1::model::DistributionPointsContentEncoding,
    ) -> Self {
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
        mut self,
        value: crate::datadogV1::model::MetricContentEncoding,
    ) -> Self {
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for MetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl MetricsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
        let client = middleware_client_builder.build();
        Self { config, client }
    }

    pub fn with_client_and_config(
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Get metadata about a specific metric.
    pub async fn get_metric_metadata(
        &self,
        metric_name: String,
    ) -> Result<crate::datadogV1::model::MetricMetadata, Error<GetMetricMetadataError>> {
        match self.get_metric_metadata_with_http_info(metric_name).await {
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.get_operation_host(operation_id),
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MetricMetadata>(&local_content) {
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
    ) -> Result<crate::datadogV1::model::MetricsListResponse, Error<ListActiveMetricsError>> {
        match self.list_active_metrics_with_http_info(from, params).await {
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

        let local_client = &self.client;

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

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MetricsListResponse>(
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
    ) -> Result<crate::datadogV1::model::MetricSearchResponse, Error<ListMetricsError>> {
        match self.list_metrics_with_http_info(q).await {
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("q", &q.to_string())]);

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MetricSearchResponse>(
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
    ) -> Result<crate::datadogV1::model::MetricsQueryResponse, Error<QueryMetricsError>> {
        match self.query_metrics_with_http_info(from, to, query).await {
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/query",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("from", &from.to_string())]);
        local_req_builder = local_req_builder.query(&[("to", &to.to_string())]);
        local_req_builder = local_req_builder.query(&[("query", &query.to_string())]);

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MetricsQueryResponse>(
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
    ) -> Result<crate::datadogV1::model::IntakePayloadAccepted, Error<SubmitDistributionPointsError>>
    {
        match self
            .submit_distribution_points_with_http_info(body, params)
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/distribution_points",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "text/json"
                .parse()
                .expect("failed to parse Content-Type header"),
        );
        headers.insert("Accept", "application/json".parse().unwrap());

        if let Some(ref local) = content_encoding {
            headers.insert(
                "Content-Encoding",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse Content-Encoding header"),
            );
        }

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::IntakePayloadAccepted>(
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
    ) -> Result<crate::datadogV1::model::IntakePayloadAccepted, Error<SubmitMetricsError>> {
        match self.submit_metrics_with_http_info(body, params).await {
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/series",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "text/json"
                .parse()
                .expect("failed to parse Content-Type header"),
        );
        headers.insert("Accept", "application/json".parse().unwrap());

        if let Some(ref local) = content_encoding {
            headers.insert(
                "Content-Encoding",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse Content-Encoding header"),
            );
        }

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::IntakePayloadAccepted>(
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
    ) -> Result<crate::datadogV1::model::MetricMetadata, Error<UpdateMetricMetadataError>> {
        match self
            .update_metric_metadata_with_http_info(metric_name, body)
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/metrics/{metric_name}",
            local_configuration.get_operation_host(operation_id),
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        // build user agent
        headers.insert(
            reqwest::header::USER_AGENT,
            local_configuration
                .user_agent
                .clone()
                .parse()
                .expect("failed to parse User Agent header"),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => panic!(
                        "Unsupported content encoding: {}",
                        content_encoding
                            .to_str()
                            .expect("non-ascii content encoding header value")
                    ),
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MetricMetadata>(&local_content) {
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
