// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetDailyCustomReportsParams is a struct for passing parameters to the method [`GetDailyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetDailyCustomReportsParams {
    /// The number of files to return in the response. `[default=60]`.
    pub page_size: Option<i64>,
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub page_number: Option<i64>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_dir: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub sort: Option<crate::datadogV1::model::UsageSort>,
}

/// GetHourlyUsageAttributionParams is a struct for passing parameters to the method [`GetHourlyUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetHourlyUsageAttributionParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Usage type to retrieve.
    pub usage_type: crate::datadogV1::model::HourlyUsageAttributionUsageType,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
    /// Comma separated list of tags used to group usage. If no value is provided the usage will not be broken down by tags.
    ///
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub tag_breakdown_keys: Option<String>,
    /// Include child org usage in the response. Defaults to `true`.
    pub include_descendants: Option<bool>,
}

/// GetIncidentManagementParams is a struct for passing parameters to the method [`GetIncidentManagement`]
#[derive(Clone, Debug)]
pub struct GetIncidentManagementParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetIngestedSpansParams is a struct for passing parameters to the method [`GetIngestedSpans`]
#[derive(Clone, Debug)]
pub struct GetIngestedSpansParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetMonthlyCustomReportsParams is a struct for passing parameters to the method [`GetMonthlyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetMonthlyCustomReportsParams {
    /// The number of files to return in the response `[default=60].`
    pub page_size: Option<i64>,
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub page_number: Option<i64>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_dir: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub sort: Option<crate::datadogV1::model::UsageSort>,
}

/// GetMonthlyUsageAttributionParams is a struct for passing parameters to the method [`GetMonthlyUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetMonthlyUsageAttributionParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
    /// Maximum of 15 months ago.
    pub start_month: String,
    /// Comma-separated list of usage types to return, or `*` for all usage types.
    pub fields: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<String>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_direction: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by.
    pub sort_name: Option<crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics>,
    /// Comma separated list of tag keys used to group usage. If no value is provided the usage will not be broken down by tags.
    ///
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub tag_breakdown_keys: Option<String>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
    /// Include child org usage in the response. Defaults to `true`.
    pub include_descendants: Option<bool>,
}

/// GetSpecifiedDailyCustomReportsParams is a struct for passing parameters to the method [`GetSpecifiedDailyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetSpecifiedDailyCustomReportsParams {
    /// Date of the report in the format `YYYY-MM-DD`.
    pub report_id: String,
}

/// GetSpecifiedMonthlyCustomReportsParams is a struct for passing parameters to the method [`GetSpecifiedMonthlyCustomReports`]
#[derive(Clone, Debug)]
pub struct GetSpecifiedMonthlyCustomReportsParams {
    /// Date of the report in the format `YYYY-MM-DD`.
    pub report_id: String,
}

/// GetUsageAnalyzedLogsParams is a struct for passing parameters to the method [`GetUsageAnalyzedLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageAnalyzedLogsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageAttributionParams is a struct for passing parameters to the method [`GetUsageAttribution`]
#[derive(Clone, Debug)]
pub struct GetUsageAttributionParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
    /// Maximum of 15 months ago.
    pub start_month: String,
    /// Comma-separated list of usage types to return, or `*` for all usage types.
    pub fields: crate::datadogV1::model::UsageAttributionSupportedMetrics,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<String>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_direction: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by.
    pub sort_name: Option<crate::datadogV1::model::UsageAttributionSort>,
    /// Include child org usage in the response. Defaults to false.
    pub include_descendants: Option<bool>,
    /// Number of records to skip before beginning to return.
    pub offset: Option<i64>,
    /// Maximum number of records to be returned.
    pub limit: Option<i64>,
}

/// GetUsageAuditLogsParams is a struct for passing parameters to the method [`GetUsageAuditLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageAuditLogsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageBillableSummaryParams is a struct for passing parameters to the method [`GetUsageBillableSummary`]
#[derive(Clone, Debug)]
pub struct GetUsageBillableSummaryParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month.
    pub month: Option<String>,
}

/// GetUsageCIAppParams is a struct for passing parameters to the method [`GetUsageCIApp`]
#[derive(Clone, Debug)]
pub struct GetUsageCIAppParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageCWSParams is a struct for passing parameters to the method [`GetUsageCWS`]
#[derive(Clone, Debug)]
pub struct GetUsageCWSParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageCloudSecurityPostureManagementParams is a struct for passing parameters to the method [`GetUsageCloudSecurityPostureManagement`]
#[derive(Clone, Debug)]
pub struct GetUsageCloudSecurityPostureManagementParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageDBMParams is a struct for passing parameters to the method [`GetUsageDBM`]
#[derive(Clone, Debug)]
pub struct GetUsageDBMParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageFargateParams is a struct for passing parameters to the method [`GetUsageFargate`]
#[derive(Clone, Debug)]
pub struct GetUsageFargateParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageHostsParams is a struct for passing parameters to the method [`GetUsageHosts`]
#[derive(Clone, Debug)]
pub struct GetUsageHostsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageIndexedSpansParams is a struct for passing parameters to the method [`GetUsageIndexedSpans`]
#[derive(Clone, Debug)]
pub struct GetUsageIndexedSpansParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageInternetOfThingsParams is a struct for passing parameters to the method [`GetUsageInternetOfThings`]
#[derive(Clone, Debug)]
pub struct GetUsageInternetOfThingsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageLambdaParams is a struct for passing parameters to the method [`GetUsageLambda`]
#[derive(Clone, Debug)]
pub struct GetUsageLambdaParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageLogsParams is a struct for passing parameters to the method [`GetUsageLogs`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageLogsByIndexParams is a struct for passing parameters to the method [`GetUsageLogsByIndex`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsByIndexParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
    /// Comma-separated list of log index names.
    pub index_name: Option<Vec<String>>,
}

/// GetUsageLogsByRetentionParams is a struct for passing parameters to the method [`GetUsageLogsByRetention`]
#[derive(Clone, Debug)]
pub struct GetUsageLogsByRetentionParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageNetworkFlowsParams is a struct for passing parameters to the method [`GetUsageNetworkFlows`]
#[derive(Clone, Debug)]
pub struct GetUsageNetworkFlowsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageNetworkHostsParams is a struct for passing parameters to the method [`GetUsageNetworkHosts`]
#[derive(Clone, Debug)]
pub struct GetUsageNetworkHostsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageOnlineArchiveParams is a struct for passing parameters to the method [`GetUsageOnlineArchive`]
#[derive(Clone, Debug)]
pub struct GetUsageOnlineArchiveParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageProfilingParams is a struct for passing parameters to the method [`GetUsageProfiling`]
#[derive(Clone, Debug)]
pub struct GetUsageProfilingParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageRumSessionsParams is a struct for passing parameters to the method [`GetUsageRumSessions`]
#[derive(Clone, Debug)]
pub struct GetUsageRumSessionsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
    /// RUM type: `[browser, mobile]`. Defaults to `browser`.
    pub type_: Option<String>,
}

/// GetUsageRumUnitsParams is a struct for passing parameters to the method [`GetUsageRumUnits`]
#[derive(Clone, Debug)]
pub struct GetUsageRumUnitsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageSDSParams is a struct for passing parameters to the method [`GetUsageSDS`]
#[derive(Clone, Debug)]
pub struct GetUsageSDSParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageSNMPParams is a struct for passing parameters to the method [`GetUsageSNMP`]
#[derive(Clone, Debug)]
pub struct GetUsageSNMPParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageSummaryParams is a struct for passing parameters to the method [`GetUsageSummary`]
#[derive(Clone, Debug)]
pub struct GetUsageSummaryParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage beginning in this month.
    /// Maximum of 15 months ago.
    pub start_month: String,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<String>,
    /// Include usage summaries for each sub-org.
    pub include_org_details: Option<bool>,
}

/// GetUsageSyntheticsParams is a struct for passing parameters to the method [`GetUsageSynthetics`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageSyntheticsAPIParams is a struct for passing parameters to the method [`GetUsageSyntheticsAPI`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsAPIParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageSyntheticsBrowserParams is a struct for passing parameters to the method [`GetUsageSyntheticsBrowser`]
#[derive(Clone, Debug)]
pub struct GetUsageSyntheticsBrowserParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageTimeseriesParams is a struct for passing parameters to the method [`GetUsageTimeseries`]
#[derive(Clone, Debug)]
pub struct GetUsageTimeseriesParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageTopAvgMetricsParams is a struct for passing parameters to the method [`GetUsageTopAvgMetrics`]
#[derive(Clone, Debug)]
pub struct GetUsageTopAvgMetricsParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub month: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub day: Option<String>,
    /// Comma-separated list of metric names.
    pub names: Option<Vec<String>>,
    /// Maximum number of results to return (between 1 and 5000) - defaults to 500 results if limit not specified.
    pub limit: Option<i32>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
}

/// GetDailyCustomReportsError is a struct for typed errors of method [`GetDailyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDailyCustomReportsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageAttributionError is a struct for typed errors of method [`GetHourlyUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentManagementError is a struct for typed errors of method [`GetIncidentManagement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentManagementError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIngestedSpansError is a struct for typed errors of method [`GetIngestedSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIngestedSpansError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCustomReportsError is a struct for typed errors of method [`GetMonthlyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCustomReportsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyUsageAttributionError is a struct for typed errors of method [`GetMonthlyUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedDailyCustomReportsError is a struct for typed errors of method [`GetSpecifiedDailyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedDailyCustomReportsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedMonthlyCustomReportsError is a struct for typed errors of method [`GetSpecifiedMonthlyCustomReports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedMonthlyCustomReportsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAnalyzedLogsError is a struct for typed errors of method [`GetUsageAnalyzedLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyzedLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAttributionError is a struct for typed errors of method [`GetUsageAttribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAuditLogsError is a struct for typed errors of method [`GetUsageAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAuditLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageBillableSummaryError is a struct for typed errors of method [`GetUsageBillableSummary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageBillableSummaryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCIAppError is a struct for typed errors of method [`GetUsageCIApp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCIAppError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCWSError is a struct for typed errors of method [`GetUsageCWS`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCWSError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCloudSecurityPostureManagementError is a struct for typed errors of method [`GetUsageCloudSecurityPostureManagement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCloudSecurityPostureManagementError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageDBMError is a struct for typed errors of method [`GetUsageDBM`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageDBMError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageFargateError is a struct for typed errors of method [`GetUsageFargate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageFargateError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageHostsError is a struct for typed errors of method [`GetUsageHosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageHostsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageIndexedSpansError is a struct for typed errors of method [`GetUsageIndexedSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageIndexedSpansError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageInternetOfThingsError is a struct for typed errors of method [`GetUsageInternetOfThings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageInternetOfThingsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaError is a struct for typed errors of method [`GetUsageLambda`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsError is a struct for typed errors of method [`GetUsageLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByIndexError is a struct for typed errors of method [`GetUsageLogsByIndex`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByIndexError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByRetentionError is a struct for typed errors of method [`GetUsageLogsByRetention`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByRetentionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkFlowsError is a struct for typed errors of method [`GetUsageNetworkFlows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkFlowsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkHostsError is a struct for typed errors of method [`GetUsageNetworkHosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkHostsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageOnlineArchiveError is a struct for typed errors of method [`GetUsageOnlineArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageOnlineArchiveError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageProfilingError is a struct for typed errors of method [`GetUsageProfiling`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageProfilingError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumSessionsError is a struct for typed errors of method [`GetUsageRumSessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumSessionsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumUnitsError is a struct for typed errors of method [`GetUsageRumUnits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumUnitsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSDSError is a struct for typed errors of method [`GetUsageSDS`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSDSError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSNMPError is a struct for typed errors of method [`GetUsageSNMP`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSNMPError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSummaryError is a struct for typed errors of method [`GetUsageSummary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSummaryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsError is a struct for typed errors of method [`GetUsageSynthetics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsAPIError is a struct for typed errors of method [`GetUsageSyntheticsAPI`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsAPIError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsBrowserError is a struct for typed errors of method [`GetUsageSyntheticsBrowser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsBrowserError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageTimeseriesError is a struct for typed errors of method [`GetUsageTimeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTimeseriesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageTopAvgMetricsError is a struct for typed errors of method [`GetUsageTopAvgMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTopAvgMetricsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct UsageMeteringAPI {
    config: configuration::Configuration,
}

impl Default for UsageMeteringAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl UsageMeteringAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_daily_custom_reports(
        &self,
        params: GetDailyCustomReportsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetDailyCustomReportsError>,
    > {
        match self.get_daily_custom_reports_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_daily_custom_reports_with_http_info(
        &self,
        params: GetDailyCustomReportsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetDailyCustomReportsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort_dir = params.sort_dir;
        let sort = params.sort;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/daily_custom_reports",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_dir {
            local_req_builder = local_req_builder.query(&[("sort_dir", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageCustomReportsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetDailyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage attribution. Multi-region data is available starting March 1, 2023.
    ///
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    ///
    /// ```
    /// response := GetHourlyUsageAttribution(start_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetHourlyUsageAttribution(start_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    pub async fn get_hourly_usage_attribution(
        &self,
        params: GetHourlyUsageAttributionParams,
    ) -> Result<
        Option<crate::datadogV1::model::HourlyUsageAttributionResponse>,
        Error<GetHourlyUsageAttributionError>,
    > {
        match self
            .get_hourly_usage_attribution_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage attribution. Multi-region data is available starting March 1, 2023.
    ///
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    ///
    /// ```
    /// response := GetHourlyUsageAttribution(start_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetHourlyUsageAttribution(start_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    pub async fn get_hourly_usage_attribution_with_http_info(
        &self,
        params: GetHourlyUsageAttributionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::HourlyUsageAttributionResponse>,
        Error<GetHourlyUsageAttributionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let usage_type = params.usage_type;
        let end_hr = params.end_hr;
        let next_record_id = params.next_record_id;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let include_descendants = params.include_descendants;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/hourly-attribution",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        local_req_builder = local_req_builder.query(&[("usage_type", &usage_type.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
        };
        if let Some(ref local_str) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_str.to_string())]);
        };
        if let Some(ref local_str) = tag_breakdown_keys {
            local_req_builder =
                local_req_builder.query(&[("tag_breakdown_keys", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::HourlyUsageAttributionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetHourlyUsageAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for incident management.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_incident_management(
        &self,
        params: GetIncidentManagementParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageIncidentManagementResponse>,
        Error<GetIncidentManagementError>,
    > {
        match self.get_incident_management_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for incident management.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_incident_management_with_http_info(
        &self,
        params: GetIncidentManagementParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIncidentManagementResponse>,
        Error<GetIncidentManagementError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/incident-management",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageIncidentManagementResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentManagementError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for ingested spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_ingested_spans(
        &self,
        params: GetIngestedSpansParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageIngestedSpansResponse>,
        Error<GetIngestedSpansError>,
    > {
        match self.get_ingested_spans_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for ingested spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_ingested_spans_with_http_info(
        &self,
        params: GetIngestedSpansParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIngestedSpansResponse>,
        Error<GetIngestedSpansError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/ingested-spans",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageIngestedSpansResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIngestedSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_monthly_custom_reports(
        &self,
        params: GetMonthlyCustomReportsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetMonthlyCustomReportsError>,
    > {
        match self.get_monthly_custom_reports_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_monthly_custom_reports_with_http_info(
        &self,
        params: GetMonthlyCustomReportsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetMonthlyCustomReportsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort_dir = params.sort_dir;
        let sort = params.sort;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monthly_custom_reports",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_dir {
            local_req_builder = local_req_builder.query(&[("sort_dir", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageCustomReportsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetMonthlyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get monthly usage attribution. Multi-region data is available starting March 1, 2023.
    ///
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    ///
    /// ```
    /// response := GetMonthlyUsageAttribution(start_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetMonthlyUsageAttribution(start_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    pub async fn get_monthly_usage_attribution(
        &self,
        params: GetMonthlyUsageAttributionParams,
    ) -> Result<
        Option<crate::datadogV1::model::MonthlyUsageAttributionResponse>,
        Error<GetMonthlyUsageAttributionError>,
    > {
        match self
            .get_monthly_usage_attribution_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get monthly usage attribution. Multi-region data is available starting March 1, 2023.
    ///
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    ///
    /// ```
    /// response := GetMonthlyUsageAttribution(start_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetMonthlyUsageAttribution(start_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    pub async fn get_monthly_usage_attribution_with_http_info(
        &self,
        params: GetMonthlyUsageAttributionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MonthlyUsageAttributionResponse>,
        Error<GetMonthlyUsageAttributionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_month = params.start_month;
        let fields = params.fields;
        let end_month = params.end_month;
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let next_record_id = params.next_record_id;
        let include_descendants = params.include_descendants;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/monthly-attribution",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_direction {
            local_req_builder =
                local_req_builder.query(&[("sort_direction", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_name {
            local_req_builder = local_req_builder.query(&[("sort_name", &local_str.to_string())]);
        };
        if let Some(ref local_str) = tag_breakdown_keys {
            local_req_builder =
                local_req_builder.query(&[("tag_breakdown_keys", &local_str.to_string())]);
        };
        if let Some(ref local_str) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::MonthlyUsageAttributionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetMonthlyUsageAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get specified daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports(
        &self,
        params: GetSpecifiedDailyCustomReportsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedDailyCustomReportsError>,
    > {
        match self
            .get_specified_daily_custom_reports_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get specified daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports_with_http_info(
        &self,
        params: GetSpecifiedDailyCustomReportsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedDailyCustomReportsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let report_id = params.report_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/daily_custom_reports/{report_id}",
            local_configuration.base_path,
            report_id = urlencode(report_id)
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
            let local_entity: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSpecifiedDailyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get specified monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports(
        &self,
        params: GetSpecifiedMonthlyCustomReportsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        match self
            .get_specified_monthly_custom_reports_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get specified monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports_with_http_info(
        &self,
        params: GetSpecifiedMonthlyCustomReportsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let report_id = params.report_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monthly_custom_reports/{report_id}",
            local_configuration.base_path,
            report_id = urlencode(report_id)
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
            let local_entity: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSpecifiedMonthlyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for analyzed logs (Security Monitoring).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_analyzed_logs(
        &self,
        params: GetUsageAnalyzedLogsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageAnalyzedLogsResponse>,
        Error<GetUsageAnalyzedLogsError>,
    > {
        match self.get_usage_analyzed_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for analyzed logs (Security Monitoring).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_analyzed_logs_with_http_info(
        &self,
        params: GetUsageAnalyzedLogsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAnalyzedLogsResponse>,
        Error<GetUsageAnalyzedLogsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/analyzed_logs",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageAnalyzedLogsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageAnalyzedLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get usage attribution.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_usage_attribution(
        &self,
        params: GetUsageAttributionParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageAttributionResponse>,
        Error<GetUsageAttributionError>,
    > {
        match self.get_usage_attribution_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get usage attribution.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/) for the associated migration guide.
    pub async fn get_usage_attribution_with_http_info(
        &self,
        params: GetUsageAttributionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAttributionResponse>,
        Error<GetUsageAttributionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_month = params.start_month;
        let fields = params.fields;
        let end_month = params.end_month;
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let include_descendants = params.include_descendants;
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/attribution", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_direction {
            local_req_builder =
                local_req_builder.query(&[("sort_direction", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_name {
            local_req_builder = local_req_builder.query(&[("sort_name", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_str.to_string())]);
        };
        if let Some(ref local_str) = offset {
            local_req_builder = local_req_builder.query(&[("offset", &local_str.to_string())]);
        };
        if let Some(ref local_str) = limit {
            local_req_builder = local_req_builder.query(&[("limit", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageAttributionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for audit logs.
    /// **Note:** This endpoint has been deprecated.
    pub async fn get_usage_audit_logs(
        &self,
        params: GetUsageAuditLogsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageAuditLogsResponse>,
        Error<GetUsageAuditLogsError>,
    > {
        match self.get_usage_audit_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for audit logs.
    /// **Note:** This endpoint has been deprecated.
    pub async fn get_usage_audit_logs_with_http_info(
        &self,
        params: GetUsageAuditLogsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAuditLogsResponse>,
        Error<GetUsageAuditLogsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/audit_logs", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageAuditLogsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageAuditLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get billable usage across your account.
    pub async fn get_usage_billable_summary(
        &self,
        params: GetUsageBillableSummaryParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageBillableSummaryResponse>,
        Error<GetUsageBillableSummaryError>,
    > {
        match self.get_usage_billable_summary_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get billable usage across your account.
    pub async fn get_usage_billable_summary_with_http_info(
        &self,
        params: GetUsageBillableSummaryParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageBillableSummaryResponse>,
        Error<GetUsageBillableSummaryError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let month = params.month;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/billable-summary",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = month {
            local_req_builder = local_req_builder.query(&[("month", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageBillableSummaryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageBillableSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for CI visibility (tests, pipeline, and spans).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_ci_app(
        &self,
        params: GetUsageCIAppParams,
    ) -> Result<Option<crate::datadogV1::model::UsageCIVisibilityResponse>, Error<GetUsageCIAppError>>
    {
        match self.get_usage_ci_app_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for CI visibility (tests, pipeline, and spans).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_ci_app_with_http_info(
        &self,
        params: GetUsageCIAppParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCIVisibilityResponse>,
        Error<GetUsageCIAppError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/ci-app", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageCIVisibilityResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageCIAppError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for cloud workload security.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_cws(
        &self,
        params: GetUsageCWSParams,
    ) -> Result<Option<crate::datadogV1::model::UsageCWSResponse>, Error<GetUsageCWSError>> {
        match self.get_usage_cws_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for cloud workload security.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_cws_with_http_info(
        &self,
        params: GetUsageCWSParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageCWSResponse>, Error<GetUsageCWSError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/cws", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageCWSResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageCWSError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for cloud security management (CSM) pro.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management(
        &self,
        params: GetUsageCloudSecurityPostureManagementParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse>,
        Error<GetUsageCloudSecurityPostureManagementError>,
    > {
        match self
            .get_usage_cloud_security_posture_management_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for cloud security management (CSM) pro.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management_with_http_info(
        &self,
        params: GetUsageCloudSecurityPostureManagementParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse>,
        Error<GetUsageCloudSecurityPostureManagementError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/cspm", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
                crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageCloudSecurityPostureManagementError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for database monitoring
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_dbm(
        &self,
        params: GetUsageDBMParams,
    ) -> Result<Option<crate::datadogV1::model::UsageDBMResponse>, Error<GetUsageDBMError>> {
        match self.get_usage_dbm_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for database monitoring
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_dbm_with_http_info(
        &self,
        params: GetUsageDBMParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageDBMResponse>, Error<GetUsageDBMError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/dbm", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageDBMResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageDBMError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [Fargate](https://docs.datadoghq.com/integrations/ecs_fargate/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_fargate(
        &self,
        params: GetUsageFargateParams,
    ) -> Result<Option<crate::datadogV1::model::UsageFargateResponse>, Error<GetUsageFargateError>>
    {
        match self.get_usage_fargate_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [Fargate](https://docs.datadoghq.com/integrations/ecs_fargate/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_fargate_with_http_info(
        &self,
        params: GetUsageFargateParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageFargateResponse>,
        Error<GetUsageFargateError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/fargate", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageFargateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageFargateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for hosts and containers.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_hosts(
        &self,
        params: GetUsageHostsParams,
    ) -> Result<Option<crate::datadogV1::model::UsageHostsResponse>, Error<GetUsageHostsError>>
    {
        match self.get_usage_hosts_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for hosts and containers.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_hosts_with_http_info(
        &self,
        params: GetUsageHostsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageHostsResponse>,
        Error<GetUsageHostsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/hosts", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageHostsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageHostsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for indexed spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_indexed_spans(
        &self,
        params: GetUsageIndexedSpansParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageIndexedSpansResponse>,
        Error<GetUsageIndexedSpansError>,
    > {
        match self.get_usage_indexed_spans_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for indexed spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_indexed_spans_with_http_info(
        &self,
        params: GetUsageIndexedSpansParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIndexedSpansResponse>,
        Error<GetUsageIndexedSpansError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/indexed-spans",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageIndexedSpansResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageIndexedSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for IoT.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_internet_of_things(
        &self,
        params: GetUsageInternetOfThingsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageIoTResponse>,
        Error<GetUsageInternetOfThingsError>,
    > {
        match self
            .get_usage_internet_of_things_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for IoT.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_internet_of_things_with_http_info(
        &self,
        params: GetUsageInternetOfThingsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIoTResponse>,
        Error<GetUsageInternetOfThingsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/iot", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageIoTResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageInternetOfThingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for lambda.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_lambda(
        &self,
        params: GetUsageLambdaParams,
    ) -> Result<Option<crate::datadogV1::model::UsageLambdaResponse>, Error<GetUsageLambdaError>>
    {
        match self.get_usage_lambda_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for lambda.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_lambda_with_http_info(
        &self,
        params: GetUsageLambdaParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLambdaResponse>,
        Error<GetUsageLambdaError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/aws_lambda", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageLambdaResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageLambdaError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for logs.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_logs(
        &self,
        params: GetUsageLogsParams,
    ) -> Result<Option<crate::datadogV1::model::UsageLogsResponse>, Error<GetUsageLogsError>> {
        match self.get_usage_logs_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for logs.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_logs_with_http_info(
        &self,
        params: GetUsageLogsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageLogsResponse>, Error<GetUsageLogsError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/logs", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageLogsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageLogsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for logs by index.
    pub async fn get_usage_logs_by_index(
        &self,
        params: GetUsageLogsByIndexParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageLogsByIndexResponse>,
        Error<GetUsageLogsByIndexError>,
    > {
        match self.get_usage_logs_by_index_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for logs by index.
    pub async fn get_usage_logs_by_index_with_http_info(
        &self,
        params: GetUsageLogsByIndexParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLogsByIndexResponse>,
        Error<GetUsageLogsByIndexError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;
        let index_name = params.index_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/logs_by_index",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
        };
        if let Some(ref local) = index_name {
            local_req_builder = local_req_builder.query(&[(
                "index_name",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            let local_entity: Option<crate::datadogV1::model::UsageLogsByIndexResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageLogsByIndexError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for indexed logs by retention period.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_logs_by_retention(
        &self,
        params: GetUsageLogsByRetentionParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageLogsByRetentionResponse>,
        Error<GetUsageLogsByRetentionError>,
    > {
        match self
            .get_usage_logs_by_retention_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for indexed logs by retention period.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_logs_by_retention_with_http_info(
        &self,
        params: GetUsageLogsByRetentionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLogsByRetentionResponse>,
        Error<GetUsageLogsByRetentionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/logs-by-retention",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageLogsByRetentionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageLogsByRetentionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for network flows.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_network_flows(
        &self,
        params: GetUsageNetworkFlowsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageNetworkFlowsResponse>,
        Error<GetUsageNetworkFlowsError>,
    > {
        match self.get_usage_network_flows_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for network flows.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_network_flows_with_http_info(
        &self,
        params: GetUsageNetworkFlowsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageNetworkFlowsResponse>,
        Error<GetUsageNetworkFlowsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/network_flows",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageNetworkFlowsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageNetworkFlowsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for network hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_network_hosts(
        &self,
        params: GetUsageNetworkHostsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageNetworkHostsResponse>,
        Error<GetUsageNetworkHostsError>,
    > {
        match self.get_usage_network_hosts_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for network hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_network_hosts_with_http_info(
        &self,
        params: GetUsageNetworkHostsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageNetworkHostsResponse>,
        Error<GetUsageNetworkHostsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/network_hosts",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageNetworkHostsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageNetworkHostsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for online archive.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_online_archive(
        &self,
        params: GetUsageOnlineArchiveParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageOnlineArchiveResponse>,
        Error<GetUsageOnlineArchiveError>,
    > {
        match self.get_usage_online_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for online archive.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_online_archive_with_http_info(
        &self,
        params: GetUsageOnlineArchiveParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageOnlineArchiveResponse>,
        Error<GetUsageOnlineArchiveError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/online-archive",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageOnlineArchiveResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageOnlineArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for profiled hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_profiling(
        &self,
        params: GetUsageProfilingParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageProfilingResponse>,
        Error<GetUsageProfilingError>,
    > {
        match self.get_usage_profiling_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for profiled hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_profiling_with_http_info(
        &self,
        params: GetUsageProfilingParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageProfilingResponse>,
        Error<GetUsageProfilingError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/profiling", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageProfilingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageProfilingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [RUM](https://docs.datadoghq.com/real_user_monitoring/) Sessions.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_rum_sessions(
        &self,
        params: GetUsageRumSessionsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageRumSessionsResponse>,
        Error<GetUsageRumSessionsError>,
    > {
        match self.get_usage_rum_sessions_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [RUM](https://docs.datadoghq.com/real_user_monitoring/) Sessions.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_rum_sessions_with_http_info(
        &self,
        params: GetUsageRumSessionsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageRumSessionsResponse>,
        Error<GetUsageRumSessionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;
        let type_ = params.type_;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/rum_sessions",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
        };
        if let Some(ref local_str) = type_ {
            local_req_builder = local_req_builder.query(&[("type", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageRumSessionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageRumSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [RUM](https://docs.datadoghq.com/real_user_monitoring/) Units.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_rum_units(
        &self,
        params: GetUsageRumUnitsParams,
    ) -> Result<Option<crate::datadogV1::model::UsageRumUnitsResponse>, Error<GetUsageRumUnitsError>>
    {
        match self.get_usage_rum_units_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [RUM](https://docs.datadoghq.com/real_user_monitoring/) Units.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_rum_units_with_http_info(
        &self,
        params: GetUsageRumUnitsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageRumUnitsResponse>,
        Error<GetUsageRumUnitsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/rum", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageRumUnitsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageRumUnitsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for sensitive data scanner.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_sds(
        &self,
        params: GetUsageSDSParams,
    ) -> Result<Option<crate::datadogV1::model::UsageSDSResponse>, Error<GetUsageSDSError>> {
        match self.get_usage_sds_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for sensitive data scanner.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_sds_with_http_info(
        &self,
        params: GetUsageSDSParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageSDSResponse>, Error<GetUsageSDSError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/sds", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSDSResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSDSError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for SNMP devices.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_snmp(
        &self,
        params: GetUsageSNMPParams,
    ) -> Result<Option<crate::datadogV1::model::UsageSNMPResponse>, Error<GetUsageSNMPError>> {
        match self.get_usage_snmp_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for SNMP devices.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_snmp_with_http_info(
        &self,
        params: GetUsageSNMPParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageSNMPResponse>, Error<GetUsageSNMPError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/snmp", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSNMPResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSNMPError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all usage across your account.
    pub async fn get_usage_summary(
        &self,
        params: GetUsageSummaryParams,
    ) -> Result<Option<crate::datadogV1::model::UsageSummaryResponse>, Error<GetUsageSummaryError>>
    {
        match self.get_usage_summary_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all usage across your account.
    pub async fn get_usage_summary_with_http_info(
        &self,
        params: GetUsageSummaryParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSummaryResponse>,
        Error<GetUsageSummaryError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_month = params.start_month;
        let end_month = params.end_month;
        let include_org_details = params.include_org_details;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/summary", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_org_details {
            local_req_builder =
                local_req_builder.query(&[("include_org_details", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSummaryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [synthetics checks](https://docs.datadoghq.com/synthetics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics(
        &self,
        params: GetUsageSyntheticsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageSyntheticsResponse>,
        Error<GetUsageSyntheticsError>,
    > {
        match self.get_usage_synthetics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [synthetics checks](https://docs.datadoghq.com/synthetics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics_with_http_info(
        &self,
        params: GetUsageSyntheticsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsResponse>,
        Error<GetUsageSyntheticsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/synthetics", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSyntheticsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSyntheticsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [synthetics API checks](https://docs.datadoghq.com/synthetics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics_api(
        &self,
        params: GetUsageSyntheticsAPIParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageSyntheticsAPIResponse>,
        Error<GetUsageSyntheticsAPIError>,
    > {
        match self.get_usage_synthetics_api_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [synthetics API checks](https://docs.datadoghq.com/synthetics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics_api_with_http_info(
        &self,
        params: GetUsageSyntheticsAPIParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsAPIResponse>,
        Error<GetUsageSyntheticsAPIError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/synthetics_api",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSyntheticsAPIResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSyntheticsAPIError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for synthetics browser checks.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics_browser(
        &self,
        params: GetUsageSyntheticsBrowserParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageSyntheticsBrowserResponse>,
        Error<GetUsageSyntheticsBrowserError>,
    > {
        match self
            .get_usage_synthetics_browser_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for synthetics browser checks.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_synthetics_browser_with_http_info(
        &self,
        params: GetUsageSyntheticsBrowserParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsBrowserResponse>,
        Error<GetUsageSyntheticsBrowserError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/synthetics_browser",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageSyntheticsBrowserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageSyntheticsBrowserError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_timeseries(
        &self,
        params: GetUsageTimeseriesParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageTimeseriesResponse>,
        Error<GetUsageTimeseriesError>,
    > {
        match self.get_usage_timeseries_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family). Refer to [Migrating from the V1 Hourly Usage APIs to V2](https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/) for the associated migration guide.
    pub async fn get_usage_timeseries_with_http_info(
        &self,
        params: GetUsageTimeseriesParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageTimeseriesResponse>,
        Error<GetUsageTimeseriesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/usage/timeseries", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_str) = end_hr {
            local_req_builder = local_req_builder.query(&[("end_hr", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageTimeseriesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageTimeseriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics(
        &self,
        params: GetUsageTopAvgMetricsParams,
    ) -> Result<
        Option<crate::datadogV1::model::UsageTopAvgMetricsResponse>,
        Error<GetUsageTopAvgMetricsError>,
    > {
        match self.get_usage_top_avg_metrics_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all [custom metrics](https://docs.datadoghq.com/developers/metrics/custom_metrics/) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics_with_http_info(
        &self,
        params: GetUsageTopAvgMetricsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageTopAvgMetricsResponse>,
        Error<GetUsageTopAvgMetricsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let month = params.month;
        let day = params.day;
        let names = params.names;
        let limit = params.limit;
        let next_record_id = params.next_record_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/top_avg_metrics",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = month {
            local_req_builder = local_req_builder.query(&[("month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = day {
            local_req_builder = local_req_builder.query(&[("day", &local_str.to_string())]);
        };
        if let Some(ref local) = names {
            local_req_builder = local_req_builder.query(&[(
                "names",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_str) = limit {
            local_req_builder = local_req_builder.query(&[("limit", &local_str.to_string())]);
        };
        if let Some(ref local_str) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::UsageTopAvgMetricsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUsageTopAvgMetricsError> =
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
