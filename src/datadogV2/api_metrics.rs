// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateBulkTagsMetricsConfigurationParams is a struct for passing parameters to the method [`CreateBulkTagsMetricsConfiguration`]
#[derive(Clone, Debug)]
pub struct CreateBulkTagsMetricsConfigurationParams {
    pub body: MetricBulkTagConfigCreateRequest,
}

// CreateTagConfigurationParams is a struct for passing parameters to the method [`CreateTagConfiguration`]
#[derive(Clone, Debug)]
pub struct CreateTagConfigurationParams {
    /* The name of the metric. */
    pub metric_name: String,
    pub body: MetricTagConfigurationCreateRequest,
}

// DeleteBulkTagsMetricsConfigurationParams is a struct for passing parameters to the method [`DeleteBulkTagsMetricsConfiguration`]
#[derive(Clone, Debug)]
pub struct DeleteBulkTagsMetricsConfigurationParams {
    pub body: MetricBulkTagConfigDeleteRequest,
}

// DeleteTagConfigurationParams is a struct for passing parameters to the method [`DeleteTagConfiguration`]
#[derive(Clone, Debug)]
pub struct DeleteTagConfigurationParams {
    /* The name of the metric. */
    pub metric_name: String,
}

// EstimateMetricsOutputSeriesParams is a struct for passing parameters to the method [`EstimateMetricsOutputSeries`]
#[derive(Clone, Debug)]
pub struct EstimateMetricsOutputSeriesParams {
    /* The name of the metric. */
    pub metric_name: String,
    /* Filtered tag keys that the metric is configured to query with. */
    pub filter_groups: String,
    /* The number of hours of look back (from now) to estimate cardinality with. */
    pub filter_hours_ago: i32,
    /* The number of aggregations that a `count`, `rate`, or `gauge` metric is configured to use. Max number of aggregation combos is 9. */
    pub filter_num_aggregations: i32,
    /* A boolean, for distribution metrics only, to estimate cardinality if the metric includes additional percentile aggregators. */
    pub filter_pct: bool,
    /* A window, in hours, from the look back to estimate cardinality with. */
    pub filter_timespan_h: i32,
}

// ListActiveMetricConfigurationsParams is a struct for passing parameters to the method [`ListActiveMetricConfigurations`]
#[derive(Clone, Debug)]
pub struct ListActiveMetricConfigurationsParams {
    /* The name of the metric. */
    pub metric_name: String,
    /* The number of seconds of look back (from now).
Default value is 604,800 (1 week), minimum value is 7200 (2 hours), maximum value is 2,630,000 (1 month). */
    pub window_seconds: i64,
}

// ListTagConfigurationByNameParams is a struct for passing parameters to the method [`ListTagConfigurationByName`]
#[derive(Clone, Debug)]
pub struct ListTagConfigurationByNameParams {
    /* The name of the metric. */
    pub metric_name: String,
}

// ListTagConfigurationsParams is a struct for passing parameters to the method [`ListTagConfigurations`]
#[derive(Clone, Debug)]
pub struct ListTagConfigurationsParams {
    /* Filter custom metrics that have configured tags. */
    pub filter_configured: bool,
    /* Filter tag configurations by configured tags. */
    pub filter_tags_configured: String,
    /* Filter metrics by metric type. */
    pub filter_metric_type: MetricTagConfigurationMetricTypes,
    /* Filter distributions with additional percentile
aggregations enabled or disabled. */
    pub filter_include_percentiles: bool,
    /* Filter custom metrics that have or have not been queried in the specified window[seconds].
If no window is provided or the window is less than 2 hours, a default of 2 hours will be applied. */
    pub filter_queried: bool,
    /* Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
Can only be combined with the filter[queried] filter. */
    pub filter_tags: String,
    /* The number of seconds of look back (from now) to apply to a filter[tag] or filter[queried] query.
Default value is 3600 (1 hour), maximum value is 2,592,000 (30 days). */
    pub window_seconds: i64,
}

// ListTagsByMetricNameParams is a struct for passing parameters to the method [`ListTagsByMetricName`]
#[derive(Clone, Debug)]
pub struct ListTagsByMetricNameParams {
    /* The name of the metric. */
    pub metric_name: String,
}

// ListVolumesByMetricNameParams is a struct for passing parameters to the method [`ListVolumesByMetricName`]
#[derive(Clone, Debug)]
pub struct ListVolumesByMetricNameParams {
    /* The name of the metric. */
    pub metric_name: String,
}

// QueryScalarDataParams is a struct for passing parameters to the method [`QueryScalarData`]
#[derive(Clone, Debug)]
pub struct QueryScalarDataParams {
    pub body: ScalarFormulaQueryRequest,
}

// QueryTimeseriesDataParams is a struct for passing parameters to the method [`QueryTimeseriesData`]
#[derive(Clone, Debug)]
pub struct QueryTimeseriesDataParams {
    pub body: TimeseriesFormulaQueryRequest,
}

// SubmitMetricsParams is a struct for passing parameters to the method [`SubmitMetrics`]
#[derive(Clone, Debug)]
pub struct SubmitMetricsParams {
    pub body: MetricPayload,
    /* HTTP header used to compress the media-type. */
    pub content_encoding: MetricContentEncoding,
}

// UpdateTagConfigurationParams is a struct for passing parameters to the method [`UpdateTagConfiguration`]
#[derive(Clone, Debug)]
pub struct UpdateTagConfigurationParams {
    /* The name of the metric. */
    pub metric_name: String,
    pub body: MetricTagConfigurationUpdateRequest,
}




/// CreateBulkTagsMetricsConfigurationError is a struct for typed errors of method [`CreateBulkTagsMetricsConfiguration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateBulkTagsMetricsConfigurationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateTagConfigurationError is a struct for typed errors of method [`CreateTagConfiguration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagConfigurationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteBulkTagsMetricsConfigurationError is a struct for typed errors of method [`DeleteBulkTagsMetricsConfiguration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBulkTagsMetricsConfigurationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTagConfigurationError is a struct for typed errors of method [`DeleteTagConfiguration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagConfigurationError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EstimateMetricsOutputSeriesError is a struct for typed errors of method [`EstimateMetricsOutputSeries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateMetricsOutputSeriesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListActiveMetricConfigurationsError is a struct for typed errors of method [`ListActiveMetricConfigurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricConfigurationsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagConfigurationByNameError is a struct for typed errors of method [`ListTagConfigurationByName`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagConfigurationByNameError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagConfigurationsError is a struct for typed errors of method [`ListTagConfigurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagConfigurationsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagsByMetricNameError is a struct for typed errors of method [`ListTagsByMetricName`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagsByMetricNameError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListVolumesByMetricNameError is a struct for typed errors of method [`ListVolumesByMetricName`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVolumesByMetricNameError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// QueryScalarDataError is a struct for typed errors of method [`QueryScalarData`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryScalarDataError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// QueryTimeseriesDataError is a struct for typed errors of method [`QueryTimeseriesData`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryTimeseriesDataError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SubmitMetricsError is a struct for typed errors of method [`SubmitMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status408(APIErrorResponse),
    Status413(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTagConfigurationError is a struct for typed errors of method [`UpdateTagConfiguration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTagConfigurationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}