// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetDailyCustomReportsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_daily_custom_reports`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetDailyCustomReportsOptionalParams {
    /// The number of files to return in the response. `[default=60]`.
    pub page_size: Option<i64>,
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub page_number: Option<i64>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_dir: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub sort: Option<crate::datadogV1::model::UsageSort>,
}

impl GetDailyCustomReportsOptionalParams {
    /// The number of files to return in the response. `[default=60]`.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// The direction to sort by: `[desc, asc]`.
    pub fn sort_dir(mut self, value: crate::datadogV1::model::UsageSortDirection) -> Self {
        self.sort_dir = Some(value);
        self
    }
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub fn sort(mut self, value: crate::datadogV1::model::UsageSort) -> Self {
        self.sort = Some(value);
        self
    }
}

/// GetHourlyUsageAttributionOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_hourly_usage_attribution`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHourlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
    /// Comma separated list of tags used to group usage. If no value is provided the usage will not be broken down by tags.
    ///
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub tag_breakdown_keys: Option<String>,
    /// Include child org usage in the response. Defaults to `true`.
    pub include_descendants: Option<bool>,
}

impl GetHourlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
    /// List following results with a next_record_id provided in the previous query.
    pub fn next_record_id(mut self, value: String) -> Self {
        self.next_record_id = Some(value);
        self
    }
    /// Comma separated list of tags used to group usage. If no value is provided the usage will not be broken down by tags.
    ///
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub fn tag_breakdown_keys(mut self, value: String) -> Self {
        self.tag_breakdown_keys = Some(value);
        self
    }
    /// Include child org usage in the response. Defaults to `true`.
    pub fn include_descendants(mut self, value: bool) -> Self {
        self.include_descendants = Some(value);
        self
    }
}

/// GetIncidentManagementOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_incident_management`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetIncidentManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetIncidentManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetIngestedSpansOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_ingested_spans`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetIngestedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetIngestedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetMonthlyCustomReportsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_monthly_custom_reports`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMonthlyCustomReportsOptionalParams {
    /// The number of files to return in the response `[default=60].`
    pub page_size: Option<i64>,
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub page_number: Option<i64>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_dir: Option<crate::datadogV1::model::UsageSortDirection>,
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub sort: Option<crate::datadogV1::model::UsageSort>,
}

impl GetMonthlyCustomReportsOptionalParams {
    /// The number of files to return in the response `[default=60].`
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// The direction to sort by: `[desc, asc]`.
    pub fn sort_dir(mut self, value: crate::datadogV1::model::UsageSortDirection) -> Self {
        self.sort_dir = Some(value);
        self
    }
    /// The field to sort by: `[computed_on, size, start_date, end_date]`.
    pub fn sort(mut self, value: crate::datadogV1::model::UsageSort) -> Self {
        self.sort = Some(value);
        self
    }
}

/// GetMonthlyUsageAttributionOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_monthly_usage_attribution`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMonthlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
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

impl GetMonthlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
    /// The direction to sort by: `[desc, asc]`.
    pub fn sort_direction(mut self, value: crate::datadogV1::model::UsageSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }
    /// The field to sort by.
    pub fn sort_name(
        mut self,
        value: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
    ) -> Self {
        self.sort_name = Some(value);
        self
    }
    /// Comma separated list of tag keys used to group usage. If no value is provided the usage will not be broken down by tags.
    ///
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub fn tag_breakdown_keys(mut self, value: String) -> Self {
        self.tag_breakdown_keys = Some(value);
        self
    }
    /// List following results with a next_record_id provided in the previous query.
    pub fn next_record_id(mut self, value: String) -> Self {
        self.next_record_id = Some(value);
        self
    }
    /// Include child org usage in the response. Defaults to `true`.
    pub fn include_descendants(mut self, value: bool) -> Self {
        self.include_descendants = Some(value);
        self
    }
}

/// GetUsageAnalyzedLogsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_analyzed_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageAnalyzedLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageAnalyzedLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageAuditLogsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_audit_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageAuditLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageAuditLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageBillableSummaryOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_billable_summary`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageBillableSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month.
    pub month: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageBillableSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month.
    pub fn month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.month = Some(value);
        self
    }
}

/// GetUsageCIAppOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_ci_app`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageCIAppOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageCIAppOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageCWSOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_cws`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageCWSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageCWSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageCloudSecurityPostureManagementOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_cloud_security_posture_management`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageCloudSecurityPostureManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageCloudSecurityPostureManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageDBMOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_dbm`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageDBMOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageDBMOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageFargateOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_fargate`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageFargateOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageFargateOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageHostsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageIndexedSpansOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_indexed_spans`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageIndexedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageIndexedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageInternetOfThingsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_internet_of_things`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageInternetOfThingsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageInternetOfThingsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLambdaOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_lambda`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLambdaOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageLambdaOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLogsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLogsByIndexOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_logs_by_index`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLogsByIndexOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
    /// Comma-separated list of log index names.
    pub index_name: Option<Vec<String>>,
}

impl GetUsageLogsByIndexOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
    /// Comma-separated list of log index names.
    pub fn index_name(mut self, value: Vec<String>) -> Self {
        self.index_name = Some(value);
        self
    }
}

/// GetUsageLogsByRetentionOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_logs_by_retention`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLogsByRetentionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageLogsByRetentionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageNetworkFlowsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_network_flows`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageNetworkFlowsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageNetworkFlowsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageNetworkHostsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_network_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageNetworkHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageNetworkHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageOnlineArchiveOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_online_archive`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageOnlineArchiveOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageOnlineArchiveOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageProfilingOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_profiling`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageProfilingOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageProfilingOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageRumSessionsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_rum_sessions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageRumSessionsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
    /// RUM type: `[browser, mobile]`. Defaults to `browser`.
    pub type_: Option<String>,
}

impl GetUsageRumSessionsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
    /// RUM type: `[browser, mobile]`. Defaults to `browser`.
    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

/// GetUsageRumUnitsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_rum_units`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageRumUnitsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageRumUnitsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSDSOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_sds`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSDSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageSDSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSNMPOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_snmp`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSNMPOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageSNMPOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSummaryOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_summary`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
    /// Include usage summaries for each sub-org.
    pub include_org_details: Option<bool>,
}

impl GetUsageSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
    /// Include usage summaries for each sub-org.
    pub fn include_org_details(mut self, value: bool) -> Self {
        self.include_org_details = Some(value);
        self
    }
}

/// GetUsageSyntheticsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_synthetics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSyntheticsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageSyntheticsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSyntheticsAPIOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_synthetics_api`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSyntheticsAPIOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageSyntheticsAPIOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSyntheticsBrowserOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_synthetics_browser`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSyntheticsBrowserOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageSyntheticsBrowserOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageTimeseriesOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_timeseries`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageTimeseriesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageTimeseriesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageTopAvgMetricsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_top_avg_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageTopAvgMetricsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub month: Option<chrono::DateTime<chrono::Utc>>,
    /// Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub day: Option<chrono::DateTime<chrono::Utc>>,
    /// Comma-separated list of metric names.
    pub names: Option<Vec<String>>,
    /// Maximum number of results to return (between 1 and 5000) - defaults to 500 results if limit not specified.
    pub limit: Option<i32>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
}

impl GetUsageTopAvgMetricsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub fn month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub fn day(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.day = Some(value);
        self
    }
    /// Comma-separated list of metric names.
    pub fn names(mut self, value: Vec<String>) -> Self {
        self.names = Some(value);
        self
    }
    /// Maximum number of results to return (between 1 and 5000) - defaults to 500 results if limit not specified.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
    /// List following results with a next_record_id provided in the previous query.
    pub fn next_record_id(mut self, value: String) -> Self {
        self.next_record_id = Some(value);
        self
    }
}

/// GetDailyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_daily_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDailyCustomReportsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_hourly_usage_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageAttributionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentManagementError is a struct for typed errors of method [`UsageMeteringAPI::get_incident_management`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentManagementError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIngestedSpansError is a struct for typed errors of method [`UsageMeteringAPI::get_ingested_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIngestedSpansError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCustomReportsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyUsageAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_usage_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyUsageAttributionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedDailyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_specified_daily_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedDailyCustomReportsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedMonthlyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_specified_monthly_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedMonthlyCustomReportsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageAnalyzedLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_analyzed_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyzedLogsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageAuditLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_audit_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAuditLogsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageBillableSummaryError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_billable_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageBillableSummaryError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCIAppError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_ci_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCIAppError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCWSError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_cws`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCWSError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageCloudSecurityPostureManagementError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_cloud_security_posture_management`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCloudSecurityPostureManagementError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageDBMError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_dbm`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageDBMError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageFargateError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_fargate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageFargateError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageHostsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageHostsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageIndexedSpansError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_indexed_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageIndexedSpansError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageInternetOfThingsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_internet_of_things`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageInternetOfThingsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_lambda`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByIndexError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs_by_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByIndexError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByRetentionError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs_by_retention`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByRetentionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkFlowsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_network_flows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkFlowsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkHostsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_network_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkHostsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageOnlineArchiveError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_online_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageOnlineArchiveError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageProfilingError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_profiling`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageProfilingError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumSessionsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_rum_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumSessionsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumUnitsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_rum_units`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumUnitsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSDSError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_sds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSDSError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSNMPError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_snmp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSNMPError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSummaryError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSummaryError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsAPIError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsAPIError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsBrowserError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics_browser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsBrowserError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageTimeseriesError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTimeseriesError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageTopAvgMetricsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_top_avg_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTopAvgMetricsError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The usage metering API allows you to get hourly, daily, and
/// monthly usage across multiple facets of Datadog.
/// This API is available to all Pro and Enterprise customers.
///
/// **Note**: Usage data is delayed by up to 72 hours from when it was incurred.
/// It is retained for 15 months.
///
/// You can retrieve up to 24 hours of hourly usage data for multiple organizations,
/// and up to two months of hourly usage data for a single organization in one request.
/// Learn more on the [usage details documentation](<https://docs.datadoghq.com/account_management/billing/usage_details/>).
#[derive(Debug, Clone)]
pub struct UsageMeteringAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for UsageMeteringAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl UsageMeteringAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_daily_custom_reports(
        &self,
        params: GetDailyCustomReportsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCustomReportsResponse,
        datadog::Error<GetDailyCustomReportsError>,
    > {
        match self.get_daily_custom_reports_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_daily_custom_reports_with_http_info(
        &self,
        params: GetDailyCustomReportsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        datadog::Error<GetDailyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_daily_custom_reports";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort_dir = params.sort_dir;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/daily_custom_reports",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_dir {
            local_req_builder =
                local_req_builder.query(&[("sort_dir", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageCustomReportsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetDailyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
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
        start_hr: chrono::DateTime<chrono::Utc>,
        usage_type: crate::datadogV1::model::HourlyUsageAttributionUsageType,
        params: GetHourlyUsageAttributionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::HourlyUsageAttributionResponse,
        datadog::Error<GetHourlyUsageAttributionError>,
    > {
        match self
            .get_hourly_usage_attribution_with_http_info(start_hr, usage_type, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        start_hr: chrono::DateTime<chrono::Utc>,
        usage_type: crate::datadogV1::model::HourlyUsageAttributionUsageType,
        params: GetHourlyUsageAttributionOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::HourlyUsageAttributionResponse>,
        datadog::Error<GetHourlyUsageAttributionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_hourly_usage_attribution";

        // unbox and build optional parameters
        let end_hr = params.end_hr;
        let next_record_id = params.next_record_id;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let include_descendants = params.include_descendants;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/hourly-attribution",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        local_req_builder = local_req_builder.query(&[("usage_type", &usage_type.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tag_breakdown_keys {
            local_req_builder =
                local_req_builder.query(&[("tag_breakdown_keys", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::HourlyUsageAttributionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetHourlyUsageAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for incident management.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_incident_management(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetIncidentManagementOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageIncidentManagementResponse,
        datadog::Error<GetIncidentManagementError>,
    > {
        match self
            .get_incident_management_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for incident management.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_incident_management_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetIncidentManagementOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageIncidentManagementResponse>,
        datadog::Error<GetIncidentManagementError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_incident_management";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/incident-management",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageIncidentManagementResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetIncidentManagementError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for ingested spans.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_ingested_spans(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetIngestedSpansOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageIngestedSpansResponse,
        datadog::Error<GetIngestedSpansError>,
    > {
        match self
            .get_ingested_spans_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for ingested spans.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_ingested_spans_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetIngestedSpansOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageIngestedSpansResponse>,
        datadog::Error<GetIngestedSpansError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_ingested_spans";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/ingested-spans",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageIngestedSpansResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetIngestedSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_monthly_custom_reports(
        &self,
        params: GetMonthlyCustomReportsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCustomReportsResponse,
        datadog::Error<GetMonthlyCustomReportsError>,
    > {
        match self.get_monthly_custom_reports_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_monthly_custom_reports_with_http_info(
        &self,
        params: GetMonthlyCustomReportsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        datadog::Error<GetMonthlyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_monthly_custom_reports";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort_dir = params.sort_dir;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/monthly_custom_reports",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_dir {
            local_req_builder =
                local_req_builder.query(&[("sort_dir", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageCustomReportsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMonthlyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
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
        start_month: chrono::DateTime<chrono::Utc>,
        fields: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
        params: GetMonthlyUsageAttributionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::MonthlyUsageAttributionResponse,
        datadog::Error<GetMonthlyUsageAttributionError>,
    > {
        match self
            .get_monthly_usage_attribution_with_http_info(start_month, fields, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        start_month: chrono::DateTime<chrono::Utc>,
        fields: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
        params: GetMonthlyUsageAttributionOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::MonthlyUsageAttributionResponse>,
        datadog::Error<GetMonthlyUsageAttributionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_monthly_usage_attribution";

        // unbox and build optional parameters
        let end_month = params.end_month;
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let next_record_id = params.next_record_id;
        let include_descendants = params.include_descendants;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/monthly-attribution",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_month",
            &start_month.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder = local_req_builder.query(&[(
                "end_month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = sort_direction {
            local_req_builder =
                local_req_builder.query(&[("sort_direction", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_name {
            local_req_builder =
                local_req_builder.query(&[("sort_name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tag_breakdown_keys {
            local_req_builder =
                local_req_builder.query(&[("tag_breakdown_keys", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MonthlyUsageAttributionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMonthlyUsageAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get specified daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports(
        &self,
        report_id: String,
    ) -> Result<
        crate::datadogV1::model::UsageSpecifiedCustomReportsResponse,
        datadog::Error<GetSpecifiedDailyCustomReportsError>,
    > {
        match self
            .get_specified_daily_custom_reports_with_http_info(report_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get specified daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports_with_http_info(
        &self,
        report_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        datadog::Error<GetSpecifiedDailyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_specified_daily_custom_reports";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/daily_custom_reports/{report_id}",
            local_configuration.get_operation_host(operation_id),
            report_id = datadog::urlencode(report_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSpecifiedDailyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get specified monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports(
        &self,
        report_id: String,
    ) -> Result<
        crate::datadogV1::model::UsageSpecifiedCustomReportsResponse,
        datadog::Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        match self
            .get_specified_monthly_custom_reports_with_http_info(report_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get specified monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports_with_http_info(
        &self,
        report_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        datadog::Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_specified_monthly_custom_reports";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/monthly_custom_reports/{report_id}",
            local_configuration.get_operation_host(operation_id),
            report_id = datadog::urlencode(report_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSpecifiedMonthlyCustomReportsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for analyzed logs (Security Monitoring).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_analyzed_logs(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageAnalyzedLogsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageAnalyzedLogsResponse,
        datadog::Error<GetUsageAnalyzedLogsError>,
    > {
        match self
            .get_usage_analyzed_logs_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for analyzed logs (Security Monitoring).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_analyzed_logs_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageAnalyzedLogsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageAnalyzedLogsResponse>,
        datadog::Error<GetUsageAnalyzedLogsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_analyzed_logs";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/analyzed_logs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageAnalyzedLogsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageAnalyzedLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for audit logs.
    /// **Note:** This endpoint has been deprecated.
    pub async fn get_usage_audit_logs(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageAuditLogsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageAuditLogsResponse,
        datadog::Error<GetUsageAuditLogsError>,
    > {
        match self
            .get_usage_audit_logs_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for audit logs.
    /// **Note:** This endpoint has been deprecated.
    pub async fn get_usage_audit_logs_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageAuditLogsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageAuditLogsResponse>,
        datadog::Error<GetUsageAuditLogsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_audit_logs";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/audit_logs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageAuditLogsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageAuditLogsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get billable usage across your account.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_usage_billable_summary(
        &self,
        params: GetUsageBillableSummaryOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageBillableSummaryResponse,
        datadog::Error<GetUsageBillableSummaryError>,
    > {
        match self.get_usage_billable_summary_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get billable usage across your account.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_usage_billable_summary_with_http_info(
        &self,
        params: GetUsageBillableSummaryOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageBillableSummaryResponse>,
        datadog::Error<GetUsageBillableSummaryError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_billable_summary";

        // unbox and build optional parameters
        let month = params.month;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/billable-summary",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = month {
            local_req_builder = local_req_builder.query(&[(
                "month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageBillableSummaryResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageBillableSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for CI visibility (tests, pipeline, and spans).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_ci_app(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCIAppOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCIVisibilityResponse,
        datadog::Error<GetUsageCIAppError>,
    > {
        match self.get_usage_ci_app_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for CI visibility (tests, pipeline, and spans).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_ci_app_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCIAppOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageCIVisibilityResponse>,
        datadog::Error<GetUsageCIAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_ci_app";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/ci-app",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageCIVisibilityResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageCIAppError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for cloud workload security.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cws(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCWSOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageCWSResponse, datadog::Error<GetUsageCWSError>> {
        match self.get_usage_cws_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for cloud workload security.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cws_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCWSOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageCWSResponse>,
        datadog::Error<GetUsageCWSError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_cws";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/cws",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageCWSResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageCWSError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for cloud security management (CSM) pro.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCloudSecurityPostureManagementOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
        datadog::Error<GetUsageCloudSecurityPostureManagementError>,
    > {
        match self
            .get_usage_cloud_security_posture_management_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for cloud security management (CSM) pro.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageCloudSecurityPostureManagementOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
        >,
        datadog::Error<GetUsageCloudSecurityPostureManagementError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_cloud_security_posture_management";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/cspm",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageCloudSecurityPostureManagementError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for database monitoring
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_dbm(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageDBMOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageDBMResponse, datadog::Error<GetUsageDBMError>> {
        match self.get_usage_dbm_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for database monitoring
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_dbm_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageDBMOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageDBMResponse>,
        datadog::Error<GetUsageDBMError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_dbm";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/dbm",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageDBMResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageDBMError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [Fargate](<https://docs.datadoghq.com/integrations/ecs_fargate/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_fargate(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageFargateOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageFargateResponse, datadog::Error<GetUsageFargateError>>
    {
        match self
            .get_usage_fargate_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [Fargate](<https://docs.datadoghq.com/integrations/ecs_fargate/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_fargate_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageFargateOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageFargateResponse>,
        datadog::Error<GetUsageFargateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_fargate";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/fargate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageFargateResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageFargateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for hosts and containers.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_hosts(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageHostsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageHostsResponse, datadog::Error<GetUsageHostsError>>
    {
        match self.get_usage_hosts_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for hosts and containers.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_hosts_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageHostsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageHostsResponse>,
        datadog::Error<GetUsageHostsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_hosts";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/hosts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageHostsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageHostsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for indexed spans.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_indexed_spans(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageIndexedSpansOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageIndexedSpansResponse,
        datadog::Error<GetUsageIndexedSpansError>,
    > {
        match self
            .get_usage_indexed_spans_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for indexed spans.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_indexed_spans_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageIndexedSpansOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageIndexedSpansResponse>,
        datadog::Error<GetUsageIndexedSpansError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_indexed_spans";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/indexed-spans",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageIndexedSpansResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageIndexedSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for IoT.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_internet_of_things(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageInternetOfThingsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageIoTResponse,
        datadog::Error<GetUsageInternetOfThingsError>,
    > {
        match self
            .get_usage_internet_of_things_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for IoT.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_internet_of_things_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageInternetOfThingsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageIoTResponse>,
        datadog::Error<GetUsageInternetOfThingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_internet_of_things";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/iot",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageIoTResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageInternetOfThingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for Lambda.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_lambda(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLambdaOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageLambdaResponse, datadog::Error<GetUsageLambdaError>>
    {
        match self.get_usage_lambda_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for Lambda.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_lambda_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLambdaOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageLambdaResponse>,
        datadog::Error<GetUsageLambdaError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_lambda";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/aws_lambda",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageLambdaResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageLambdaError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for logs.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageLogsResponse, datadog::Error<GetUsageLogsError>> {
        match self.get_usage_logs_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for logs.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageLogsResponse>,
        datadog::Error<GetUsageLogsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_logs";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/logs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageLogsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for logs by index.
    pub async fn get_usage_logs_by_index(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsByIndexOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageLogsByIndexResponse,
        datadog::Error<GetUsageLogsByIndexError>,
    > {
        match self
            .get_usage_logs_by_index_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for logs by index.
    pub async fn get_usage_logs_by_index_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsByIndexOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageLogsByIndexResponse>,
        datadog::Error<GetUsageLogsByIndexError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_logs_by_index";

        // unbox and build optional parameters
        let end_hr = params.end_hr;
        let index_name = params.index_name;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/logs_by_index",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local) = index_name {
            local_req_builder = local_req_builder.query(&[(
                "index_name",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsByIndexResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageLogsByIndexError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for indexed logs by retention period.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_by_retention(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsByRetentionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageLogsByRetentionResponse,
        datadog::Error<GetUsageLogsByRetentionError>,
    > {
        match self
            .get_usage_logs_by_retention_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for indexed logs by retention period.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_by_retention_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLogsByRetentionOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageLogsByRetentionResponse>,
        datadog::Error<GetUsageLogsByRetentionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_logs_by_retention";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/logs-by-retention",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsByRetentionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageLogsByRetentionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for network flows.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_flows(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageNetworkFlowsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageNetworkFlowsResponse,
        datadog::Error<GetUsageNetworkFlowsError>,
    > {
        match self
            .get_usage_network_flows_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for network flows.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_flows_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageNetworkFlowsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageNetworkFlowsResponse>,
        datadog::Error<GetUsageNetworkFlowsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_network_flows";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/network_flows",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageNetworkFlowsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageNetworkFlowsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for network hosts.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_hosts(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageNetworkHostsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageNetworkHostsResponse,
        datadog::Error<GetUsageNetworkHostsError>,
    > {
        match self
            .get_usage_network_hosts_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for network hosts.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_hosts_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageNetworkHostsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageNetworkHostsResponse>,
        datadog::Error<GetUsageNetworkHostsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_network_hosts";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/network_hosts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageNetworkHostsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageNetworkHostsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for online archive.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_online_archive(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageOnlineArchiveOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageOnlineArchiveResponse,
        datadog::Error<GetUsageOnlineArchiveError>,
    > {
        match self
            .get_usage_online_archive_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for online archive.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_online_archive_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageOnlineArchiveOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageOnlineArchiveResponse>,
        datadog::Error<GetUsageOnlineArchiveError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_online_archive";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/online-archive",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageOnlineArchiveResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageOnlineArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for profiled hosts.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_profiling(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageProfilingOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageProfilingResponse,
        datadog::Error<GetUsageProfilingError>,
    > {
        match self
            .get_usage_profiling_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for profiled hosts.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_profiling_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageProfilingOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageProfilingResponse>,
        datadog::Error<GetUsageProfilingError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_profiling";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/profiling",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageProfilingResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageProfilingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Sessions.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_sessions(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageRumSessionsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageRumSessionsResponse,
        datadog::Error<GetUsageRumSessionsError>,
    > {
        match self
            .get_usage_rum_sessions_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Sessions.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_sessions_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageRumSessionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageRumSessionsResponse>,
        datadog::Error<GetUsageRumSessionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_rum_sessions";

        // unbox and build optional parameters
        let end_hr = params.end_hr;
        let type_ = params.type_;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/rum_sessions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = type_ {
            local_req_builder =
                local_req_builder.query(&[("type", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageRumSessionsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageRumSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Units.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_units(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageRumUnitsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageRumUnitsResponse, datadog::Error<GetUsageRumUnitsError>>
    {
        match self
            .get_usage_rum_units_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Units.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_units_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageRumUnitsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageRumUnitsResponse>,
        datadog::Error<GetUsageRumUnitsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_rum_units";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/rum",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageRumUnitsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageRumUnitsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for sensitive data scanner.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_sds(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSDSOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSDSResponse, datadog::Error<GetUsageSDSError>> {
        match self.get_usage_sds_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for sensitive data scanner.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_sds_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSDSOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSDSResponse>,
        datadog::Error<GetUsageSDSError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_sds";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/sds",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSDSResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSDSError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for SNMP devices.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_snmp(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSNMPOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSNMPResponse, datadog::Error<GetUsageSNMPError>> {
        match self.get_usage_snmp_with_http_info(start_hr, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for SNMP devices.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_snmp_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSNMPOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSNMPResponse>,
        datadog::Error<GetUsageSNMPError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_snmp";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/snmp",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSNMPResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSNMPError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get all usage across your account.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_usage_summary(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetUsageSummaryOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSummaryResponse, datadog::Error<GetUsageSummaryError>>
    {
        match self
            .get_usage_summary_with_http_info(start_month, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get all usage across your account.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_usage_summary_with_http_info(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetUsageSummaryOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSummaryResponse>,
        datadog::Error<GetUsageSummaryError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_summary";

        // unbox and build optional parameters
        let end_month = params.end_month;
        let include_org_details = params.include_org_details;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/summary",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_month",
            &start_month.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder = local_req_builder.query(&[(
                "end_month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = include_org_details {
            local_req_builder =
                local_req_builder.query(&[("include_org_details", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSummaryResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [synthetics checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageSyntheticsResponse,
        datadog::Error<GetUsageSyntheticsError>,
    > {
        match self
            .get_usage_synthetics_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [synthetics checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSyntheticsResponse>,
        datadog::Error<GetUsageSyntheticsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_synthetics";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/synthetics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSyntheticsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [synthetics API checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_api(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsAPIOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageSyntheticsAPIResponse,
        datadog::Error<GetUsageSyntheticsAPIError>,
    > {
        match self
            .get_usage_synthetics_api_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [synthetics API checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_api_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsAPIOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSyntheticsAPIResponse>,
        datadog::Error<GetUsageSyntheticsAPIError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_synthetics_api";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/synthetics_api",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsAPIResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSyntheticsAPIError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for synthetics browser checks.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_browser(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsBrowserOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageSyntheticsBrowserResponse,
        datadog::Error<GetUsageSyntheticsBrowserError>,
    > {
        match self
            .get_usage_synthetics_browser_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for synthetics browser checks.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_browser_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageSyntheticsBrowserOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageSyntheticsBrowserResponse>,
        datadog::Error<GetUsageSyntheticsBrowserError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_synthetics_browser";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/synthetics_browser",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsBrowserResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageSyntheticsBrowserError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_timeseries(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageTimeseriesOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageTimeseriesResponse,
        datadog::Error<GetUsageTimeseriesError>,
    > {
        match self
            .get_usage_timeseries_with_http_info(start_hr, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>).
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_timeseries_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageTimeseriesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageTimeseriesResponse>,
        datadog::Error<GetUsageTimeseriesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_timeseries";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/timeseries",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_hr",
            &start_hr.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder = local_req_builder.query(&[(
                "end_hr",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageTimeseriesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageTimeseriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get all [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics(
        &self,
        params: GetUsageTopAvgMetricsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageTopAvgMetricsResponse,
        datadog::Error<GetUsageTopAvgMetricsError>,
    > {
        match self.get_usage_top_avg_metrics_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get all [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics_with_http_info(
        &self,
        params: GetUsageTopAvgMetricsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::UsageTopAvgMetricsResponse>,
        datadog::Error<GetUsageTopAvgMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_top_avg_metrics";

        // unbox and build optional parameters
        let month = params.month;
        let day = params.day;
        let names = params.names;
        let limit = params.limit;
        let next_record_id = params.next_record_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/top_avg_metrics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = month {
            local_req_builder = local_req_builder.query(&[(
                "month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = day {
            local_req_builder = local_req_builder.query(&[(
                "day",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local) = names {
            local_req_builder = local_req_builder.query(&[(
                "names",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = next_record_id {
            local_req_builder =
                local_req_builder.query(&[("next_record_id", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json;datetime-format=rfc3339"),
        );

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::UsageTopAvgMetricsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetUsageTopAvgMetricsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
