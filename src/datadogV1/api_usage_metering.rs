// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// GetDailyCustomReportsParams is a struct for passing parameters to the method [`GetDailyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetDailyCustomReportsParams {
    /* The number of files to return in the response. `[default=60]`. */
    pub page_size: i64,
    /* The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`. */
    pub page_number: i64,
    /* The direction to sort by: `[desc, asc]`. */
    pub sort_dir: UsageSortDirection,
    /* The field to sort by: `[computed_on, size, start_date, end_date]`. */
    pub sort: UsageSort,
}

// GetHourlyUsageAttributionParams is a struct for passing parameters to the method [`GetHourlyUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetHourlyUsageAttributionParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Usage type to retrieve. */
    pub usage_type: HourlyUsageAttributionUsageType,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
    /* List following results with a next_record_id provided in the previous query. */
    pub next_record_id: String,
    /* Comma separated list of tags used to group usage. If no value is provided the usage will not be broken down by tags.

To see which tags are available, look for the value of `tag_config_source` in the API response. */
    pub tag_breakdown_keys: String,
    /* Include child org usage in the response. Defaults to `true`. */
    pub include_descendants: bool,
}

// GetIncidentManagementParams is a struct for passing parameters to the method [`GetIncidentManagement`]
#[derive(Clone, Debug)]
pub struct GetIncidentManagementParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetIngestedSpansParams is a struct for passing parameters to the method [`GetIngestedSpans`]
#[derive(Clone, Debug)]
pub struct GetIngestedSpansParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetMonthlyCustomReportsParams is a struct for passing parameters to the method [`GetMonthlyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetMonthlyCustomReportsParams {
    /* The number of files to return in the response `[default=60].` */
    pub page_size: i64,
    /* The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`. */
    pub page_number: i64,
    /* The direction to sort by: `[desc, asc]`. */
    pub sort_dir: UsageSortDirection,
    /* The field to sort by: `[computed_on, size, start_date, end_date]`. */
    pub sort: UsageSort,
}

// GetMonthlyUsageAttributionParams is a struct for passing parameters to the method [`GetMonthlyUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetMonthlyUsageAttributionParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
Maximum of 15 months ago. */
    pub start_month: String,
    /* Comma-separated list of usage types to return, or `*` for all usage types. */
    pub fields: MonthlyUsageAttributionSupportedMetrics,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month. */
    pub end_month: String,
    /* The direction to sort by: `[desc, asc]`. */
    pub sort_direction: UsageSortDirection,
    /* The field to sort by. */
    pub sort_name: MonthlyUsageAttributionSupportedMetrics,
    /* Comma separated list of tag keys used to group usage. If no value is provided the usage will not be broken down by tags.

To see which tags are available, look for the value of `tag_config_source` in the API response. */
    pub tag_breakdown_keys: String,
    /* List following results with a next_record_id provided in the previous query. */
    pub next_record_id: String,
    /* Include child org usage in the response. Defaults to `true`. */
    pub include_descendants: bool,
}

// GetSpecifiedDailyCustomReportsParams is a struct for passing parameters to the method [`GetSpecifiedDailyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetSpecifiedDailyCustomReportsParams {
    /* Date of the report in the format `YYYY-MM-DD`. */
    pub report_id: String,
}

// GetSpecifiedMonthlyCustomReportsParams is a struct for passing parameters to the method [`GetSpecifiedMonthlyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetSpecifiedMonthlyCustomReportsParams {
    /* Date of the report in the format `YYYY-MM-DD`. */
    pub report_id: String,
}

// GetUsageAnalyzedLogsParams is a struct for passing parameters to the method [`GetUsageAnalyzedLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageAnalyzedLogsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageAttributionParams is a struct for passing parameters to the method [`GetUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetUsageAttributionParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
Maximum of 15 months ago. */
    pub start_month: String,
    /* Comma-separated list of usage types to return, or `*` for all usage types. */
    pub fields: UsageAttributionSupportedMetrics,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month. */
    pub end_month: String,
    /* The direction to sort by: `[desc, asc]`. */
    pub sort_direction: UsageSortDirection,
    /* The field to sort by. */
    pub sort_name: UsageAttributionSort,
    /* Include child org usage in the response. Defaults to false. */
    pub include_descendants: bool,
    /* Number of records to skip before beginning to return. */
    pub offset: i64,
    /* Maximum number of records to be returned. */
    pub limit: i64,
}

// GetUsageAuditLogsParams is a struct for passing parameters to the method [`GetUsageAuditLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageAuditLogsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageBillableSummaryParams is a struct for passing parameters to the method [`GetUsageBillableSummary`]
#[derive(Clone, Debug)]
pub struct GetUsageBillableSummaryParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month. */
    pub month: String,
}

// GetUsageCIAppParams is a struct for passing parameters to the method [`GetUsageCIApp`]
#[derive(Clone, Debug)]
pub struct GetUsageCIAppParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageCWSParams is a struct for passing parameters to the method [`GetUsageCWS`]
#[derive(Clone, Debug)]
pub struct GetUsageCWSParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageCloudSecurityPostureManagementParams is a struct for passing parameters to the method [`GetUsageCloudSecurityPostureManagement`]
#[derive(Clone, Debug)]
pub struct GetUsageCloudSecurityPostureManagementParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageDBMParams is a struct for passing parameters to the method [`GetUsageDBM`]
#[derive(Clone, Debug)]
pub struct GetUsageDBMParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageFargateParams is a struct for passing parameters to the method [`GetUsageFargate`]
#[derive(Clone, Debug)]
pub struct GetUsageFargateParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageHostsParams is a struct for passing parameters to the method [`GetUsageHosts`]
#[derive(Clone, Debug)]
pub struct GetUsageHostsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageIndexedSpansParams is a struct for passing parameters to the method [`GetUsageIndexedSpans`]
#[derive(Clone, Debug)]
pub struct GetUsageIndexedSpansParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageInternetOfThingsParams is a struct for passing parameters to the method [`GetUsageInternetOfThings`]
#[derive(Clone, Debug)]
pub struct GetUsageInternetOfThingsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageLambdaParams is a struct for passing parameters to the method [`GetUsageLambda`]
#[derive(Clone, Debug)]
pub struct GetUsageLambdaParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageLogsParams is a struct for passing parameters to the method [`GetUsageLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageLogsByIndexParams is a struct for passing parameters to the method [`GetUsageLogsByIndex`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsByIndexParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
    /* Comma-separated list of log index names. */
    pub index_name: Vec<String>,
}

// GetUsageLogsByRetentionParams is a struct for passing parameters to the method [`GetUsageLogsByRetention`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsByRetentionParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageNetworkFlowsParams is a struct for passing parameters to the method [`GetUsageNetworkFlows`]
#[derive(Clone, Debug)]
pub struct GetUsageNetworkFlowsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageNetworkHostsParams is a struct for passing parameters to the method [`GetUsageNetworkHosts`]
#[derive(Clone, Debug)]
pub struct GetUsageNetworkHostsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageOnlineArchiveParams is a struct for passing parameters to the method [`GetUsageOnlineArchive`]
#[derive(Clone, Debug)]
pub struct GetUsageOnlineArchiveParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageProfilingParams is a struct for passing parameters to the method [`GetUsageProfiling`]
#[derive(Clone, Debug)]
pub struct GetUsageProfilingParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageRumSessionsParams is a struct for passing parameters to the method [`GetUsageRumSessions`]
#[derive(Clone, Debug)]
pub struct GetUsageRumSessionsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
    /* RUM type: `[browser, mobile]`. Defaults to `browser`. */
    pub type_: String,
}

// GetUsageRumUnitsParams is a struct for passing parameters to the method [`GetUsageRumUnits`]
#[derive(Clone, Debug)]
pub struct GetUsageRumUnitsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageSDSParams is a struct for passing parameters to the method [`GetUsageSDS`]
#[derive(Clone, Debug)]
pub struct GetUsageSDSParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageSNMPParams is a struct for passing parameters to the method [`GetUsageSNMP`]
#[derive(Clone, Debug)]
pub struct GetUsageSNMPParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
**before** this hour. */
    pub end_hr: String,
}

// GetUsageSummaryParams is a struct for passing parameters to the method [`GetUsageSummary`]
#[derive(Clone, Debug)]
pub struct GetUsageSummaryParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
Maximum of 15 months ago. */
    pub start_month: String,
    /* Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month. */
    pub end_month: String,
    /* Include usage summaries for each sub-org. */
    pub include_org_details: bool,
}

// GetUsageSyntheticsParams is a struct for passing parameters to the method [`GetUsageSynthetics`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageSyntheticsAPIParams is a struct for passing parameters to the method [`GetUsageSyntheticsAPI`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsAPIParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageSyntheticsBrowserParams is a struct for passing parameters to the method [`GetUsageSyntheticsBrowser`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsBrowserParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageTimeseriesParams is a struct for passing parameters to the method [`GetUsageTimeseries`]
#[derive(Clone, Debug)]
pub struct GetUsageTimeseriesParams {
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour. */
    pub start_hr: String,
    /* Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour. */
    pub end_hr: String,
}

// GetUsageTopAvgMetricsParams is a struct for passing parameters to the method [`GetUsageTopAvgMetrics`]
#[derive(Clone, Debug)]
pub struct GetUsageTopAvgMetricsParams {
    /* Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both) */
    pub month: String,
    /* Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both) */
    pub day: String,
    /* Comma-separated list of metric names. */
    pub names: Vec<String>,
    /* Maximum number of results to return (between 1 and 5000) - defaults to 500 results if limit not specified. */
    pub limit: i32,
    /* List following results with a next_record_id provided in the previous query. */
    pub next_record_id: String,
}




/// GetDailyCustomReportsError is a struct for typed errors of method [`GetDailyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDailyCustomReportsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageAttributionError is a struct for typed errors of method [`GetHourlyUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageAttributionError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentManagementError is a struct for typed errors of method [`GetIncidentManagement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentManagementError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIngestedSpansError is a struct for typed errors of method [`GetIngestedSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIngestedSpansError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCustomReportsError is a struct for typed errors of method [`GetMonthlyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCustomReportsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyUsageAttributionError is a struct for typed errors of method [`GetMonthlyUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyUsageAttributionError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedDailyCustomReportsError is a struct for typed errors of method [`GetSpecifiedDailyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedDailyCustomReportsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedMonthlyCustomReportsError is a struct for typed errors of method [`GetSpecifiedMonthlyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedMonthlyCustomReportsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageAnalyzedLogsError is a struct for typed errors of method [`GetUsageAnalyzedLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyzedLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageAttributionError is a struct for typed errors of method [`GetUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAttributionError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageAuditLogsError is a struct for typed errors of method [`GetUsageAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAuditLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageBillableSummaryError is a struct for typed errors of method [`GetUsageBillableSummary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageBillableSummaryError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCIAppError is a struct for typed errors of method [`GetUsageCIApp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCIAppError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCWSError is a struct for typed errors of method [`GetUsageCWS`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCWSError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCloudSecurityPostureManagementError is a struct for typed errors of method [`GetUsageCloudSecurityPostureManagement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCloudSecurityPostureManagementError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageDBMError is a struct for typed errors of method [`GetUsageDBM`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageDBMError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageFargateError is a struct for typed errors of method [`GetUsageFargate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageFargateError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageHostsError is a struct for typed errors of method [`GetUsageHosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageHostsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageIndexedSpansError is a struct for typed errors of method [`GetUsageIndexedSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageIndexedSpansError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageInternetOfThingsError is a struct for typed errors of method [`GetUsageInternetOfThings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageInternetOfThingsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaError is a struct for typed errors of method [`GetUsageLambda`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsError is a struct for typed errors of method [`GetUsageLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByIndexError is a struct for typed errors of method [`GetUsageLogsByIndex`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByIndexError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByRetentionError is a struct for typed errors of method [`GetUsageLogsByRetention`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByRetentionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkFlowsError is a struct for typed errors of method [`GetUsageNetworkFlows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkFlowsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkHostsError is a struct for typed errors of method [`GetUsageNetworkHosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkHostsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageOnlineArchiveError is a struct for typed errors of method [`GetUsageOnlineArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageOnlineArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageProfilingError is a struct for typed errors of method [`GetUsageProfiling`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageProfilingError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumSessionsError is a struct for typed errors of method [`GetUsageRumSessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumSessionsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumUnitsError is a struct for typed errors of method [`GetUsageRumUnits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumUnitsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSDSError is a struct for typed errors of method [`GetUsageSDS`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSDSError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSNMPError is a struct for typed errors of method [`GetUsageSNMP`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSNMPError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSummaryError is a struct for typed errors of method [`GetUsageSummary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSummaryError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsError is a struct for typed errors of method [`GetUsageSynthetics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsAPIError is a struct for typed errors of method [`GetUsageSyntheticsAPI`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsAPIError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsBrowserError is a struct for typed errors of method [`GetUsageSyntheticsBrowser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsBrowserError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageTimeseriesError is a struct for typed errors of method [`GetUsageTimeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTimeseriesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageTopAvgMetricsError is a struct for typed errors of method [`GetUsageTopAvgMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTopAvgMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}