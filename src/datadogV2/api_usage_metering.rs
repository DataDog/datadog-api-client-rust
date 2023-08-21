// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// GetCostByOrgParams is a struct for passing parameters to the method [`GetCostByOrg`]
#[derive(Clone, Debug)]
pub struct GetCostByOrgParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. */
    pub start_month: String,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month. */
    pub end_month: String,
}

// GetEstimatedCostByOrgParams is a struct for passing parameters to the method [`GetEstimatedCostByOrg`]
#[derive(Clone, Debug)]
pub struct GetEstimatedCostByOrgParams {
    /* String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`. */
    pub view: String,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. Either start_month or start_date should be specified, but not both. (start_month cannot go beyond two months in the past). Provide an `end_month` to view month-over-month cost. */
    pub start_month: String,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month. */
    pub end_month: String,
    /* Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost beginning this day. Either start_month or start_date should be specified, but not both. (start_date cannot go beyond two months in the past). Provide an `end_date` to view day-over-day cumulative cost. */
    pub start_date: String,
    /* Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost ending this day. */
    pub end_date: String,
}

// GetHistoricalCostByOrgParams is a struct for passing parameters to the method [`GetHistoricalCostByOrg`]
#[derive(Clone, Debug)]
pub struct GetHistoricalCostByOrgParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. */
    pub start_month: String,
    /* String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`.  Defaults to `summary`. */
    pub view: String,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month. */
    pub end_month: String,
}

// GetHourlyUsageParams is a struct for passing parameters to the method [`GetHourlyUsage`]
#[derive(Clone, Debug)]
pub struct GetHourlyUsageParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub filter_timestamp_start: String,
    /* Comma separated list of product families to retrieve. Available families are `all`, `analyzed_logs`,
`application_security`, `audit_trail`, `serverless`, `ci_app`, `cloud_cost_management`,
`cspm`, `custom_events`, `cws`, `dbm`, `fargate`,
`infra_hosts`, `incident_management`, `indexed_logs`, `indexed_spans`, `ingested_spans`, `iot`,
`lambda_traced_invocations`, `logs`, `network_flows`, `network_hosts`, `observability_pipelines`,
`online_archive`, `profiling`, `rum`, `rum_browser_sessions`, `rum_mobile_sessions`, `sds`, `snmp`,
`synthetics_api`, `synthetics_browser`, `synthetics_parallel_testing`, and `timeseries`.
The following product family has been **deprecated**: `audit_logs`. */
    pub filter_product_families: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub filter_timestamp_end: String,
    /* Include child org usage in the response. Defaults to false. */
    pub filter_include_descendants: bool,
    /* Include breakdown of usage by subcategories where applicable (for product family logs only). Defaults to false. */
    pub filter_include_breakdown: bool,
    /* Comma separated list of product family versions to use in the format `product_family:version`. For example,
`infra_hosts:1.0.0`. If this parameter is not used, the API will use the latest version of each requested
product family. Currently all families have one version `1.0.0`. */
    pub filter_versions: String,
    /* Maximum number of results to return (between 1 and 500) - defaults to 500 if limit not specified. */
    pub page_limit: i32,
    /* List following results with a next_record_id provided in the previous query. */
    pub page_next_record_id: String,
}

// GetUsageApplicationSecurityMonitoringParams is a struct for passing parameters to the method [`GetUsageApplicationSecurityMonitoring`]
#[derive(Clone, Debug)]
pub struct GetUsageApplicationSecurityMonitoringParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageLambdaTracedInvocationsParams is a struct for passing parameters to the method [`GetUsageLambdaTracedInvocations`]
#[derive(Clone, Debug)]
pub struct GetUsageLambdaTracedInvocationsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageObservabilityPipelinesParams is a struct for passing parameters to the method [`GetUsageObservabilityPipelines`]
#[derive(Clone, Debug)]
pub struct GetUsageObservabilityPipelinesParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}




/// GetCostByOrgError is a struct for typed errors of method [`GetCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostByOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetEstimatedCostByOrgError is a struct for typed errors of method [`GetEstimatedCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEstimatedCostByOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHistoricalCostByOrgError is a struct for typed errors of method [`GetHistoricalCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoricalCostByOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageError is a struct for typed errors of method [`GetHourlyUsage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageApplicationSecurityMonitoringError is a struct for typed errors of method [`GetUsageApplicationSecurityMonitoring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageApplicationSecurityMonitoringError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaTracedInvocationsError is a struct for typed errors of method [`GetUsageLambdaTracedInvocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaTracedInvocationsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageObservabilityPipelinesError is a struct for typed errors of method [`GetUsageObservabilityPipelines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageObservabilityPipelinesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}