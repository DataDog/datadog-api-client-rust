// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// EstimateMetricsOutputSeriesOptionalParams is a struct for passing parameters to the method [`MetricsAPI::estimate_metrics_output_series`]
#[derive(Clone, Default, Debug)]
pub struct EstimateMetricsOutputSeriesOptionalParams {
    /// Filtered tag keys that the metric is configured to query with.
    pub filter_groups: Option<String>,
    /// The number of hours of look back (from now) to estimate cardinality with.
    pub filter_hours_ago: Option<i32>,
    /// The number of aggregations that a `count`, `rate`, or `gauge` metric is configured to use. Max number of aggregation combos is 9.
    pub filter_num_aggregations: Option<i32>,
    /// A boolean, for distribution metrics only, to estimate cardinality if the metric includes additional percentile aggregators.
    pub filter_pct: Option<bool>,
    /// A window, in hours, from the look back to estimate cardinality with.
    pub filter_timespan_h: Option<i32>,
}

impl EstimateMetricsOutputSeriesOptionalParams {
    /// Filtered tag keys that the metric is configured to query with.
    pub fn filter_groups(&mut self, value: String) -> &mut Self {
        self.filter_groups = Some(value);
        self
    }
    /// The number of hours of look back (from now) to estimate cardinality with.
    pub fn filter_hours_ago(&mut self, value: i32) -> &mut Self {
        self.filter_hours_ago = Some(value);
        self
    }
    /// The number of aggregations that a `count`, `rate`, or `gauge` metric is configured to use. Max number of aggregation combos is 9.
    pub fn filter_num_aggregations(&mut self, value: i32) -> &mut Self {
        self.filter_num_aggregations = Some(value);
        self
    }
    /// A boolean, for distribution metrics only, to estimate cardinality if the metric includes additional percentile aggregators.
    pub fn filter_pct(&mut self, value: bool) -> &mut Self {
        self.filter_pct = Some(value);
        self
    }
    /// A window, in hours, from the look back to estimate cardinality with.
    pub fn filter_timespan_h(&mut self, value: i32) -> &mut Self {
        self.filter_timespan_h = Some(value);
        self
    }
}

/// ListActiveMetricConfigurationsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::list_active_metric_configurations`]
#[derive(Clone, Default, Debug)]
pub struct ListActiveMetricConfigurationsOptionalParams {
    /// The number of seconds of look back (from now).
    /// Default value is 604,800 (1 week), minimum value is 7200 (2 hours), maximum value is 2,630,000 (1 month).
    pub window_seconds: Option<i64>,
}

impl ListActiveMetricConfigurationsOptionalParams {
    /// The number of seconds of look back (from now).
    /// Default value is 604,800 (1 week), minimum value is 7200 (2 hours), maximum value is 2,630,000 (1 month).
    pub fn window_seconds(&mut self, value: i64) -> &mut Self {
        self.window_seconds = Some(value);
        self
    }
}

/// ListTagConfigurationsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::list_tag_configurations`]
#[derive(Clone, Default, Debug)]
pub struct ListTagConfigurationsOptionalParams {
    /// Filter custom metrics that have configured tags.
    pub filter_configured: Option<bool>,
    /// Filter tag configurations by configured tags.
    pub filter_tags_configured: Option<String>,
    /// Filter metrics by metric type.
    pub filter_metric_type: Option<crate::datadogV2::model::MetricTagConfigurationMetricTypes>,
    /// Filter distributions with additional percentile
    /// aggregations enabled or disabled.
    pub filter_include_percentiles: Option<bool>,
    /// (Beta) Filter custom metrics that have or have not been queried in the specified window[seconds].
    /// If no window is provided or the window is less than 2 hours, a default of 2 hours will be applied.
    pub filter_queried: Option<bool>,
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Can only be combined with the filter[queried] filter.
    pub filter_tags: Option<String>,
    /// The number of seconds of look back (from now) to apply to a filter[tag] or filter[queried] query.
    /// Default value is 3600 (1 hour), maximum value is 2,592,000 (30 days).
    pub window_seconds: Option<i64>,
}

impl ListTagConfigurationsOptionalParams {
    /// Filter custom metrics that have configured tags.
    pub fn filter_configured(&mut self, value: bool) -> &mut Self {
        self.filter_configured = Some(value);
        self
    }
    /// Filter tag configurations by configured tags.
    pub fn filter_tags_configured(&mut self, value: String) -> &mut Self {
        self.filter_tags_configured = Some(value);
        self
    }
    /// Filter metrics by metric type.
    pub fn filter_metric_type(
        &mut self,
        value: crate::datadogV2::model::MetricTagConfigurationMetricTypes,
    ) -> &mut Self {
        self.filter_metric_type = Some(value);
        self
    }
    /// Filter distributions with additional percentile
    /// aggregations enabled or disabled.
    pub fn filter_include_percentiles(&mut self, value: bool) -> &mut Self {
        self.filter_include_percentiles = Some(value);
        self
    }
    /// (Beta) Filter custom metrics that have or have not been queried in the specified window[seconds].
    /// If no window is provided or the window is less than 2 hours, a default of 2 hours will be applied.
    pub fn filter_queried(&mut self, value: bool) -> &mut Self {
        self.filter_queried = Some(value);
        self
    }
    /// Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
    /// Can only be combined with the filter[queried] filter.
    pub fn filter_tags(&mut self, value: String) -> &mut Self {
        self.filter_tags = Some(value);
        self
    }
    /// The number of seconds of look back (from now) to apply to a filter[tag] or filter[queried] query.
    /// Default value is 3600 (1 hour), maximum value is 2,592,000 (30 days).
    pub fn window_seconds(&mut self, value: i64) -> &mut Self {
        self.window_seconds = Some(value);
        self
    }
}

/// SubmitMetricsOptionalParams is a struct for passing parameters to the method [`MetricsAPI::submit_metrics`]
#[derive(Clone, Default, Debug)]
pub struct SubmitMetricsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub content_encoding: Option<crate::datadogV2::model::MetricContentEncoding>,
}

impl SubmitMetricsOptionalParams {
    /// HTTP header used to compress the media-type.
    pub fn content_encoding(
        &mut self,
        value: crate::datadogV2::model::MetricContentEncoding,
    ) -> &mut Self {
        self.content_encoding = Some(value);
        self
    }
}

/// CreateBulkTagsMetricsConfigurationError is a struct for typed errors of method [`MetricsAPI::create_bulk_tags_metrics_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateBulkTagsMetricsConfigurationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateTagConfigurationError is a struct for typed errors of method [`MetricsAPI::create_tag_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagConfigurationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteBulkTagsMetricsConfigurationError is a struct for typed errors of method [`MetricsAPI::delete_bulk_tags_metrics_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBulkTagsMetricsConfigurationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTagConfigurationError is a struct for typed errors of method [`MetricsAPI::delete_tag_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagConfigurationError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EstimateMetricsOutputSeriesError is a struct for typed errors of method [`MetricsAPI::estimate_metrics_output_series`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateMetricsOutputSeriesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListActiveMetricConfigurationsError is a struct for typed errors of method [`MetricsAPI::list_active_metric_configurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricConfigurationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTagConfigurationByNameError is a struct for typed errors of method [`MetricsAPI::list_tag_configuration_by_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagConfigurationByNameError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTagConfigurationsError is a struct for typed errors of method [`MetricsAPI::list_tag_configurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagConfigurationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTagsByMetricNameError is a struct for typed errors of method [`MetricsAPI::list_tags_by_metric_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagsByMetricNameError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListVolumesByMetricNameError is a struct for typed errors of method [`MetricsAPI::list_volumes_by_metric_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVolumesByMetricNameError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// QueryScalarDataError is a struct for typed errors of method [`MetricsAPI::query_scalar_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryScalarDataError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// QueryTimeseriesDataError is a struct for typed errors of method [`MetricsAPI::query_timeseries_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryTimeseriesDataError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SubmitMetricsError is a struct for typed errors of method [`MetricsAPI::submit_metrics`]
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

/// UpdateTagConfigurationError is a struct for typed errors of method [`MetricsAPI::update_tag_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTagConfigurationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
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

    /// Create and define a list of queryable tag keys for a set of existing count, gauge, rate, and distribution metrics.
    /// Metrics are selected by passing a metric name prefix. Use the Delete method of this API path to remove tag configurations.
    /// Results can be sent to a set of account email addresses, just like the same operation in the Datadog web app.
    /// If multiple calls include the same metric, the last configuration applied (not by submit order) is used, do not
    /// expect deterministic ordering of concurrent calls. The `exclude_tags_mode` value will set all metrics that match the prefix to
    /// the same exclusion state, metric tag configurations do not support mixed inclusion and exclusion for tags on the same metric.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn create_bulk_tags_metrics_configuration(
        &self,
        body: crate::datadogV2::model::MetricBulkTagConfigCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::MetricBulkTagConfigResponse>,
        Error<CreateBulkTagsMetricsConfigurationError>,
    > {
        match self
            .create_bulk_tags_metrics_configuration_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create and define a list of queryable tag keys for a set of existing count, gauge, rate, and distribution metrics.
    /// Metrics are selected by passing a metric name prefix. Use the Delete method of this API path to remove tag configurations.
    /// Results can be sent to a set of account email addresses, just like the same operation in the Datadog web app.
    /// If multiple calls include the same metric, the last configuration applied (not by submit order) is used, do not
    /// expect deterministic ordering of concurrent calls. The `exclude_tags_mode` value will set all metrics that match the prefix to
    /// the same exclusion state, metric tag configurations do not support mixed inclusion and exclusion for tags on the same metric.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn create_bulk_tags_metrics_configuration_with_http_info(
        &self,
        body: crate::datadogV2::model::MetricBulkTagConfigCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricBulkTagConfigResponse>,
        Error<CreateBulkTagsMetricsConfigurationError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/config/bulk-tags",
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
            let local_entity: Option<crate::datadogV2::model::MetricBulkTagConfigResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateBulkTagsMetricsConfigurationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create and define a list of queryable tag keys for an existing count/gauge/rate/distribution metric.
    /// Optionally, include percentile aggregations on any distribution metric or configure custom aggregations
    /// on any count, rate, or gauge metric. By setting `exclude_tags_mode` to true the behavior is changed
    /// from an allow-list to a deny-list, and tags in the defined list will not be queryable.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn create_tag_configuration(
        &self,
        metric_name: String,
        body: crate::datadogV2::model::MetricTagConfigurationCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<CreateTagConfigurationError>,
    > {
        match self
            .create_tag_configuration_with_http_info(metric_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create and define a list of queryable tag keys for an existing count/gauge/rate/distribution metric.
    /// Optionally, include percentile aggregations on any distribution metric or configure custom aggregations
    /// on any count, rate, or gauge metric. By setting `exclude_tags_mode` to true the behavior is changed
    /// from an allow-list to a deny-list, and tags in the defined list will not be queryable.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn create_tag_configuration_with_http_info(
        &self,
        metric_name: String,
        body: crate::datadogV2::model::MetricTagConfigurationCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<CreateTagConfigurationError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/tags",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<crate::datadogV2::model::MetricTagConfigurationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateTagConfigurationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete all custom lists of queryable tag keys for a set of existing count, gauge, rate, and distribution metrics.
    /// Metrics are selected by passing a metric name prefix.
    /// Results can be sent to a set of account email addresses, just like the same operation in the Datadog web app.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn delete_bulk_tags_metrics_configuration(
        &self,
        body: crate::datadogV2::model::MetricBulkTagConfigDeleteRequest,
    ) -> Result<
        Option<crate::datadogV2::model::MetricBulkTagConfigResponse>,
        Error<DeleteBulkTagsMetricsConfigurationError>,
    > {
        match self
            .delete_bulk_tags_metrics_configuration_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete all custom lists of queryable tag keys for a set of existing count, gauge, rate, and distribution metrics.
    /// Metrics are selected by passing a metric name prefix.
    /// Results can be sent to a set of account email addresses, just like the same operation in the Datadog web app.
    /// Can only be used with application keys of users with the `Manage Tags for Metrics` permission.
    pub async fn delete_bulk_tags_metrics_configuration_with_http_info(
        &self,
        body: crate::datadogV2::model::MetricBulkTagConfigDeleteRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricBulkTagConfigResponse>,
        Error<DeleteBulkTagsMetricsConfigurationError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/config/bulk-tags",
            local_configuration.base_path
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
            let local_entity: Option<crate::datadogV2::model::MetricBulkTagConfigResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteBulkTagsMetricsConfigurationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes a metric's tag configuration. Can only be used with application
    /// keys from users with the `Manage Tags for Metrics` permission.
    pub async fn delete_tag_configuration(
        &self,
        metric_name: String,
    ) -> Result<Option<()>, Error<DeleteTagConfigurationError>> {
        match self
            .delete_tag_configuration_with_http_info(metric_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes a metric's tag configuration. Can only be used with application
    /// keys from users with the `Manage Tags for Metrics` permission.
    pub async fn delete_tag_configuration_with_http_info(
        &self,
        metric_name: String,
    ) -> Result<ResponseContent<()>, Error<DeleteTagConfigurationError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/tags",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<DeleteTagConfigurationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns the estimated cardinality for a metric with a given tag, percentile and number of aggregations configuration using Metrics without Limits&trade;.
    pub async fn estimate_metrics_output_series(
        &self,
        metric_name: String,
        params: EstimateMetricsOutputSeriesOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::MetricEstimateResponse>,
        Error<EstimateMetricsOutputSeriesError>,
    > {
        match self
            .estimate_metrics_output_series_with_http_info(metric_name, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns the estimated cardinality for a metric with a given tag, percentile and number of aggregations configuration using Metrics without Limits&trade;.
    pub async fn estimate_metrics_output_series_with_http_info(
        &self,
        metric_name: String,
        params: EstimateMetricsOutputSeriesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricEstimateResponse>,
        Error<EstimateMetricsOutputSeriesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_groups = params.filter_groups;
        let filter_hours_ago = params.filter_hours_ago;
        let filter_num_aggregations = params.filter_num_aggregations;
        let filter_pct = params.filter_pct;
        let filter_timespan_h = params.filter_timespan_h;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/estimate",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_groups {
            local_req_builder =
                local_req_builder.query(&[("filter[groups]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_hours_ago {
            local_req_builder =
                local_req_builder.query(&[("filter[hours_ago]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_num_aggregations {
            local_req_builder = local_req_builder
                .query(&[("filter[num_aggregations]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_pct {
            local_req_builder =
                local_req_builder.query(&[("filter[pct]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_timespan_h {
            local_req_builder =
                local_req_builder.query(&[("filter[timespan_h]", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<crate::datadogV2::model::MetricEstimateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EstimateMetricsOutputSeriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List tags and aggregations that are actively queried on dashboards and monitors for a given metric name.
    pub async fn list_active_metric_configurations(
        &self,
        metric_name: String,
        params: ListActiveMetricConfigurationsOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::MetricSuggestedTagsAndAggregationsResponse>,
        Error<ListActiveMetricConfigurationsError>,
    > {
        match self
            .list_active_metric_configurations_with_http_info(metric_name, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List tags and aggregations that are actively queried on dashboards and monitors for a given metric name.
    pub async fn list_active_metric_configurations_with_http_info(
        &self,
        metric_name: String,
        params: ListActiveMetricConfigurationsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricSuggestedTagsAndAggregationsResponse>,
        Error<ListActiveMetricConfigurationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let window_seconds = params.window_seconds;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/active-configurations",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = window_seconds {
            local_req_builder =
                local_req_builder.query(&[("window[seconds]", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<
                crate::datadogV2::model::MetricSuggestedTagsAndAggregationsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListActiveMetricConfigurationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns the tag configuration for the given metric name.
    pub async fn list_tag_configuration_by_name(
        &self,
        metric_name: String,
    ) -> Result<
        Option<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<ListTagConfigurationByNameError>,
    > {
        match self
            .list_tag_configuration_by_name_with_http_info(metric_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns the tag configuration for the given metric name.
    pub async fn list_tag_configuration_by_name_with_http_info(
        &self,
        metric_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<ListTagConfigurationByNameError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/tags",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<crate::datadogV2::model::MetricTagConfigurationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListTagConfigurationByNameError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns all metrics that can be configured in the Metrics Summary page or with Metrics without Limits™ (matching additional filters if specified).
    pub async fn list_tag_configurations(
        &self,
        params: ListTagConfigurationsOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::MetricsAndMetricTagConfigurationsResponse>,
        Error<ListTagConfigurationsError>,
    > {
        match self.list_tag_configurations_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns all metrics that can be configured in the Metrics Summary page or with Metrics without Limits™ (matching additional filters if specified).
    pub async fn list_tag_configurations_with_http_info(
        &self,
        params: ListTagConfigurationsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricsAndMetricTagConfigurationsResponse>,
        Error<ListTagConfigurationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let filter_configured = params.filter_configured;
        let filter_tags_configured = params.filter_tags_configured;
        let filter_metric_type = params.filter_metric_type;
        let filter_include_percentiles = params.filter_include_percentiles;
        let filter_queried = params.filter_queried;
        let filter_tags = params.filter_tags;
        let window_seconds = params.window_seconds;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/metrics", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_configured {
            local_req_builder =
                local_req_builder.query(&[("filter[configured]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags_configured {
            local_req_builder = local_req_builder
                .query(&[("filter[tags_configured]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_metric_type {
            local_req_builder =
                local_req_builder.query(&[("filter[metric_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_include_percentiles {
            local_req_builder = local_req_builder.query(&[(
                "filter[include_percentiles]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_queried {
            local_req_builder =
                local_req_builder.query(&[("filter[queried]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = window_seconds {
            local_req_builder =
                local_req_builder.query(&[("window[seconds]", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<
                crate::datadogV2::model::MetricsAndMetricTagConfigurationsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListTagConfigurationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// View indexed tag key-value pairs for a given metric name.
    pub async fn list_tags_by_metric_name(
        &self,
        metric_name: String,
    ) -> Result<
        Option<crate::datadogV2::model::MetricAllTagsResponse>,
        Error<ListTagsByMetricNameError>,
    > {
        match self
            .list_tags_by_metric_name_with_http_info(metric_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// View indexed tag key-value pairs for a given metric name.
    pub async fn list_tags_by_metric_name_with_http_info(
        &self,
        metric_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricAllTagsResponse>,
        Error<ListTagsByMetricNameError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/all-tags",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<crate::datadogV2::model::MetricAllTagsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListTagsByMetricNameError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// View distinct metrics volumes for the given metric name.
    ///
    /// Custom metrics generated in-app from other products will return `null` for ingested volumes.
    pub async fn list_volumes_by_metric_name(
        &self,
        metric_name: String,
    ) -> Result<
        Option<crate::datadogV2::model::MetricVolumesResponse>,
        Error<ListVolumesByMetricNameError>,
    > {
        match self
            .list_volumes_by_metric_name_with_http_info(metric_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// View distinct metrics volumes for the given metric name.
    ///
    /// Custom metrics generated in-app from other products will return `null` for ingested volumes.
    pub async fn list_volumes_by_metric_name_with_http_info(
        &self,
        metric_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricVolumesResponse>,
        Error<ListVolumesByMetricNameError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/volumes",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<crate::datadogV2::model::MetricVolumesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListVolumesByMetricNameError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Query scalar values (as seen on Query Value, Table, and Toplist widgets).
    /// Multiple data sources are supported with the ability to
    /// process the data using formulas and functions.
    pub async fn query_scalar_data(
        &self,
        body: crate::datadogV2::model::ScalarFormulaQueryRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ScalarFormulaQueryResponse>,
        Error<QueryScalarDataError>,
    > {
        match self.query_scalar_data_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Query scalar values (as seen on Query Value, Table, and Toplist widgets).
    /// Multiple data sources are supported with the ability to
    /// process the data using formulas and functions.
    pub async fn query_scalar_data_with_http_info(
        &self,
        body: crate::datadogV2::model::ScalarFormulaQueryRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ScalarFormulaQueryResponse>,
        Error<QueryScalarDataError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/query/scalar", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::ScalarFormulaQueryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<QueryScalarDataError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Query timeseries data across various data sources and
    /// process the data by applying formulas and functions.
    pub async fn query_timeseries_data(
        &self,
        body: crate::datadogV2::model::TimeseriesFormulaQueryRequest,
    ) -> Result<
        Option<crate::datadogV2::model::TimeseriesFormulaQueryResponse>,
        Error<QueryTimeseriesDataError>,
    > {
        match self.query_timeseries_data_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Query timeseries data across various data sources and
    /// process the data by applying formulas and functions.
    pub async fn query_timeseries_data_with_http_info(
        &self,
        body: crate::datadogV2::model::TimeseriesFormulaQueryRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::TimeseriesFormulaQueryResponse>,
        Error<QueryTimeseriesDataError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/query/timeseries", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::TimeseriesFormulaQueryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<QueryTimeseriesDataError> =
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
        body: crate::datadogV2::model::MetricPayload,
        params: SubmitMetricsOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::IntakePayloadAccepted>, Error<SubmitMetricsError>>
    {
        match self.submit_metrics_with_http_info(body, params).await {
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
        body: crate::datadogV2::model::MetricPayload,
        params: SubmitMetricsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IntakePayloadAccepted>,
        Error<SubmitMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/series", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local) = content_encoding {
            local_req_builder = local_req_builder.header("Content-Encoding", &local.to_string());
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV2::model::IntakePayloadAccepted> =
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

    /// Update the tag configuration of a metric or percentile aggregations of a distribution metric or custom aggregations
    /// of a count, rate, or gauge metric. By setting `exclude_tags_mode` to true the behavior is changed
    /// from an allow-list to a deny-list, and tags in the defined list will not be queryable.
    /// Can only be used with application keys from users with the `Manage Tags for Metrics` permission. This endpoint requires
    /// a tag configuration to be created first.
    pub async fn update_tag_configuration(
        &self,
        metric_name: String,
        body: crate::datadogV2::model::MetricTagConfigurationUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<UpdateTagConfigurationError>,
    > {
        match self
            .update_tag_configuration_with_http_info(metric_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the tag configuration of a metric or percentile aggregations of a distribution metric or custom aggregations
    /// of a count, rate, or gauge metric. By setting `exclude_tags_mode` to true the behavior is changed
    /// from an allow-list to a deny-list, and tags in the defined list will not be queryable.
    /// Can only be used with application keys from users with the `Manage Tags for Metrics` permission. This endpoint requires
    /// a tag configuration to be created first.
    pub async fn update_tag_configuration_with_http_info(
        &self,
        metric_name: String,
        body: crate::datadogV2::model::MetricTagConfigurationUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MetricTagConfigurationResponse>,
        Error<UpdateTagConfigurationError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/metrics/{metric_name}/tags",
            local_configuration.base_path,
            metric_name = urlencode(metric_name)
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
            let local_entity: Option<crate::datadogV2::model::MetricTagConfigurationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateTagConfigurationError> =
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
