// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
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

impl GetHourlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetIncidentManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetIngestedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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

impl GetMonthlyUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub fn end_month(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageAnalyzedLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageAttributionOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_attribution`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageAttributionOptionalParams {
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

impl GetUsageAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub fn end_month(mut self, value: String) -> Self {
        self.end_month = Some(value);
        self
    }
    /// The direction to sort by: `[desc, asc]`.
    pub fn sort_direction(mut self, value: crate::datadogV1::model::UsageSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }
    /// The field to sort by.
    pub fn sort_name(mut self, value: crate::datadogV1::model::UsageAttributionSort) -> Self {
        self.sort_name = Some(value);
        self
    }
    /// Include child org usage in the response. Defaults to false.
    pub fn include_descendants(mut self, value: bool) -> Self {
        self.include_descendants = Some(value);
        self
    }
    /// Number of records to skip before beginning to return.
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Maximum number of records to be returned.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
}

/// GetUsageAuditLogsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_audit_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageAuditLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageAuditLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageBillableSummaryOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_billable_summary`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageBillableSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month.
    pub month: Option<String>,
}

impl GetUsageBillableSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage starting this month.
    pub fn month(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageCIAppOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageCWSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageCloudSecurityPostureManagementOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageDBMOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageFargateOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_fargate`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageFargateOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageFargateOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageHostsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageIndexedSpansOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_indexed_spans`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageIndexedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageIndexedSpansOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageInternetOfThingsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLambdaOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_lambda`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLambdaOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageLambdaOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLogsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_logs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageLogsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLogsByIndexOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_logs_by_index`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLogsByIndexOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
    /// Comma-separated list of log index names.
    pub index_name: Option<Vec<String>>,
}

impl GetUsageLogsByIndexOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageLogsByRetentionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageNetworkFlowsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageNetworkHostsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_network_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageNetworkHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageNetworkHostsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageOnlineArchiveOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageProfilingOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageRumSessionsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_rum_sessions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageRumSessionsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
    /// RUM type: `[browser, mobile]`. Defaults to `browser`.
    pub type_: Option<String>,
}

impl GetUsageRumSessionsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageRumUnitsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageSDSOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageSNMPOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSummaryOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_summary`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub end_month: Option<String>,
    /// Include usage summaries for each sub-org.
    pub include_org_details: Option<bool>,
}

impl GetUsageSummaryOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for usage ending this month.
    pub fn end_month(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageSyntheticsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSyntheticsAPIOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_synthetics_api`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSyntheticsAPIOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageSyntheticsAPIOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageSyntheticsBrowserOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_synthetics_browser`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageSyntheticsBrowserOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageSyntheticsBrowserOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageTimeseriesOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_timeseries`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageTimeseriesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageTimeseriesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageTopAvgMetricsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_top_avg_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageTopAvgMetricsOptionalParams {
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

impl GetUsageTopAvgMetricsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub fn month(mut self, value: String) -> Self {
        self.month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: [YYYY-MM-DD] for usage beginning at this hour. (Either month or day should be specified, but not both)
    pub fn day(mut self, value: String) -> Self {
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
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_hourly_usage_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentManagementError is a struct for typed errors of method [`UsageMeteringAPI::get_incident_management`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentManagementError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIngestedSpansError is a struct for typed errors of method [`UsageMeteringAPI::get_ingested_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIngestedSpansError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCustomReportsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyUsageAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_usage_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedDailyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_specified_daily_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedDailyCustomReportsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSpecifiedMonthlyCustomReportsError is a struct for typed errors of method [`UsageMeteringAPI::get_specified_monthly_custom_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecifiedMonthlyCustomReportsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAnalyzedLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_analyzed_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAnalyzedLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAttributionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageAuditLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_audit_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageAuditLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageBillableSummaryError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_billable_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageBillableSummaryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCIAppError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_ci_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCIAppError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCWSError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_cws`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCWSError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageCloudSecurityPostureManagementError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_cloud_security_posture_management`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageCloudSecurityPostureManagementError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageDBMError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_dbm`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageDBMError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageFargateError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_fargate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageFargateError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageHostsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageHostsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageIndexedSpansError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_indexed_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageIndexedSpansError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageInternetOfThingsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_internet_of_things`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageInternetOfThingsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_lambda`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByIndexError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs_by_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByIndexError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLogsByRetentionError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_logs_by_retention`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLogsByRetentionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkFlowsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_network_flows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkFlowsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageNetworkHostsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_network_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageNetworkHostsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageOnlineArchiveError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_online_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageOnlineArchiveError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageProfilingError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_profiling`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageProfilingError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumSessionsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_rum_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumSessionsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageRumUnitsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_rum_units`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageRumUnitsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSDSError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_sds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSDSError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSNMPError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_snmp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSNMPError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSummaryError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSummaryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsAPIError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsAPIError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageSyntheticsBrowserError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_synthetics_browser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageSyntheticsBrowserError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageTimeseriesError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageTimeseriesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageTopAvgMetricsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_top_avg_metrics`]
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for UsageMeteringAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl UsageMeteringAPI {
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

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_daily_custom_reports(
        &self,
        params: GetDailyCustomReportsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCustomReportsResponse,
        Error<GetDailyCustomReportsError>,
    > {
        match self.get_daily_custom_reports_with_http_info(params).await {
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

    /// Get daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_daily_custom_reports_with_http_info(
        &self,
        params: GetDailyCustomReportsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetDailyCustomReportsError>,
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
            match serde_json::from_str::<crate::datadogV1::model::UsageCustomReportsResponse>(
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
        start_hr: String,
        usage_type: crate::datadogV1::model::HourlyUsageAttributionUsageType,
        params: GetHourlyUsageAttributionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::HourlyUsageAttributionResponse,
        Error<GetHourlyUsageAttributionError>,
    > {
        match self
            .get_hourly_usage_attribution_with_http_info(start_hr, usage_type, params)
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
        start_hr: String,
        usage_type: crate::datadogV1::model::HourlyUsageAttributionUsageType,
        params: GetHourlyUsageAttributionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::HourlyUsageAttributionResponse>,
        Error<GetHourlyUsageAttributionError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        local_req_builder = local_req_builder.query(&[("usage_type", &usage_type.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::HourlyUsageAttributionResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_incident_management(
        &self,
        start_hr: String,
        params: GetIncidentManagementOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageIncidentManagementResponse,
        Error<GetIncidentManagementError>,
    > {
        match self
            .get_incident_management_with_http_info(start_hr, params)
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

    /// Get hourly usage for incident management.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_incident_management_with_http_info(
        &self,
        start_hr: String,
        params: GetIncidentManagementOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIncidentManagementResponse>,
        Error<GetIncidentManagementError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageIncidentManagementResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_ingested_spans(
        &self,
        start_hr: String,
        params: GetIngestedSpansOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageIngestedSpansResponse, Error<GetIngestedSpansError>>
    {
        match self
            .get_ingested_spans_with_http_info(start_hr, params)
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

    /// Get hourly usage for ingested spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_ingested_spans_with_http_info(
        &self,
        start_hr: String,
        params: GetIngestedSpansOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIngestedSpansResponse>,
        Error<GetIngestedSpansError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageIngestedSpansResponse>(
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
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_monthly_custom_reports(
        &self,
        params: GetMonthlyCustomReportsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCustomReportsResponse,
        Error<GetMonthlyCustomReportsError>,
    > {
        match self.get_monthly_custom_reports_with_http_info(params).await {
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

    /// Get monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_monthly_custom_reports_with_http_info(
        &self,
        params: GetMonthlyCustomReportsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCustomReportsResponse>,
        Error<GetMonthlyCustomReportsError>,
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
            match serde_json::from_str::<crate::datadogV1::model::UsageCustomReportsResponse>(
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
        start_month: String,
        fields: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
        params: GetMonthlyUsageAttributionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::MonthlyUsageAttributionResponse,
        Error<GetMonthlyUsageAttributionError>,
    > {
        match self
            .get_monthly_usage_attribution_with_http_info(start_month, fields, params)
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
        start_month: String,
        fields: crate::datadogV1::model::MonthlyUsageAttributionSupportedMetrics,
        params: GetMonthlyUsageAttributionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MonthlyUsageAttributionResponse>,
        Error<GetMonthlyUsageAttributionError>,
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

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::MonthlyUsageAttributionResponse>(
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
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports(
        &self,
        report_id: String,
    ) -> Result<
        crate::datadogV1::model::UsageSpecifiedCustomReportsResponse,
        Error<GetSpecifiedDailyCustomReportsError>,
    > {
        match self
            .get_specified_daily_custom_reports_with_http_info(report_id)
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

    /// Get specified daily custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_daily_custom_reports_with_http_info(
        &self,
        report_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedDailyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_specified_daily_custom_reports";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/daily_custom_reports/{report_id}",
            local_configuration.get_operation_host(operation_id),
            report_id = urlencode(report_id)
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>(
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
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports(
        &self,
        report_id: String,
    ) -> Result<
        crate::datadogV1::model::UsageSpecifiedCustomReportsResponse,
        Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        match self
            .get_specified_monthly_custom_reports_with_http_info(report_id)
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

    /// Get specified monthly custom reports.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_specified_monthly_custom_reports_with_http_info(
        &self,
        report_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>,
        Error<GetSpecifiedMonthlyCustomReportsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_specified_monthly_custom_reports";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/monthly_custom_reports/{report_id}",
            local_configuration.get_operation_host(operation_id),
            report_id = urlencode(report_id)
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSpecifiedCustomReportsResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_analyzed_logs(
        &self,
        start_hr: String,
        params: GetUsageAnalyzedLogsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageAnalyzedLogsResponse, Error<GetUsageAnalyzedLogsError>>
    {
        match self
            .get_usage_analyzed_logs_with_http_info(start_hr, params)
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

    /// Get hourly usage for analyzed logs (Security Monitoring).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_analyzed_logs_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageAnalyzedLogsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAnalyzedLogsResponse>,
        Error<GetUsageAnalyzedLogsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageAnalyzedLogsResponse>(
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
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_usage_attribution(
        &self,
        start_month: String,
        fields: crate::datadogV1::model::UsageAttributionSupportedMetrics,
        params: GetUsageAttributionOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageAttributionResponse, Error<GetUsageAttributionError>>
    {
        match self
            .get_usage_attribution_with_http_info(start_month, fields, params)
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

    /// Get usage attribution.
    /// **Note:** This endpoint will be fully deprecated on December 1, 2022.
    /// Refer to [Migrating from v1 to v2 of the Usage Attribution API](<https://docs.datadoghq.com/account_management/guide/usage-attribution-migration/>) for the associated migration guide.
    pub async fn get_usage_attribution_with_http_info(
        &self,
        start_month: String,
        fields: crate::datadogV1::model::UsageAttributionSupportedMetrics,
        params: GetUsageAttributionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAttributionResponse>,
        Error<GetUsageAttributionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_usage_attribution";

        // unbox and build optional parameters
        let end_month = params.end_month;
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let include_descendants = params.include_descendants;
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/usage/attribution",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_direction {
            local_req_builder =
                local_req_builder.query(&[("sort_direction", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_name {
            local_req_builder =
                local_req_builder.query(&[("sort_name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_descendants {
            local_req_builder =
                local_req_builder.query(&[("include_descendants", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageAttributionResponse>(
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
        start_hr: String,
        params: GetUsageAuditLogsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageAuditLogsResponse, Error<GetUsageAuditLogsError>>
    {
        match self
            .get_usage_audit_logs_with_http_info(start_hr, params)
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

    /// Get hourly usage for audit logs.
    /// **Note:** This endpoint has been deprecated.
    pub async fn get_usage_audit_logs_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageAuditLogsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageAuditLogsResponse>,
        Error<GetUsageAuditLogsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageAuditLogsResponse>(
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
        params: GetUsageBillableSummaryOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageBillableSummaryResponse,
        Error<GetUsageBillableSummaryError>,
    > {
        match self.get_usage_billable_summary_with_http_info(params).await {
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

    /// Get billable usage across your account.
    pub async fn get_usage_billable_summary_with_http_info(
        &self,
        params: GetUsageBillableSummaryOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageBillableSummaryResponse>,
        Error<GetUsageBillableSummaryError>,
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
            local_req_builder =
                local_req_builder.query(&[("month", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageBillableSummaryResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_ci_app(
        &self,
        start_hr: String,
        params: GetUsageCIAppOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageCIVisibilityResponse, Error<GetUsageCIAppError>> {
        match self.get_usage_ci_app_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for CI visibility (tests, pipeline, and spans).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_ci_app_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageCIAppOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCIVisibilityResponse>,
        Error<GetUsageCIAppError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageCIVisibilityResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cws(
        &self,
        start_hr: String,
        params: GetUsageCWSOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageCWSResponse, Error<GetUsageCWSError>> {
        match self.get_usage_cws_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for cloud workload security.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cws_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageCWSOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageCWSResponse>, Error<GetUsageCWSError>>
    {
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageCWSResponse>(&local_content)
            {
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management(
        &self,
        start_hr: String,
        params: GetUsageCloudSecurityPostureManagementOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
        Error<GetUsageCloudSecurityPostureManagementError>,
    > {
        match self
            .get_usage_cloud_security_posture_management_with_http_info(start_hr, params)
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

    /// Get hourly usage for cloud security management (CSM) pro.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_cloud_security_posture_management_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageCloudSecurityPostureManagementOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse>,
        Error<GetUsageCloudSecurityPostureManagementError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<
                crate::datadogV1::model::UsageCloudSecurityPostureManagementResponse,
            >(&local_content)
            {
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_dbm(
        &self,
        start_hr: String,
        params: GetUsageDBMOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageDBMResponse, Error<GetUsageDBMError>> {
        match self.get_usage_dbm_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for database monitoring
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_dbm_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageDBMOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageDBMResponse>, Error<GetUsageDBMError>>
    {
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageDBMResponse>(&local_content)
            {
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
            let local_entity: Option<GetUsageDBMError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for [Fargate](<https://docs.datadoghq.com/integrations/ecs_fargate/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_fargate(
        &self,
        start_hr: String,
        params: GetUsageFargateOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageFargateResponse, Error<GetUsageFargateError>> {
        match self
            .get_usage_fargate_with_http_info(start_hr, params)
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

    /// Get hourly usage for [Fargate](<https://docs.datadoghq.com/integrations/ecs_fargate/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_fargate_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageFargateOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageFargateResponse>,
        Error<GetUsageFargateError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageFargateResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_hosts(
        &self,
        start_hr: String,
        params: GetUsageHostsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageHostsResponse, Error<GetUsageHostsError>> {
        match self.get_usage_hosts_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for hosts and containers.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_hosts_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageHostsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageHostsResponse>,
        Error<GetUsageHostsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageHostsResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_indexed_spans(
        &self,
        start_hr: String,
        params: GetUsageIndexedSpansOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageIndexedSpansResponse, Error<GetUsageIndexedSpansError>>
    {
        match self
            .get_usage_indexed_spans_with_http_info(start_hr, params)
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

    /// Get hourly usage for indexed spans.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_indexed_spans_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageIndexedSpansOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIndexedSpansResponse>,
        Error<GetUsageIndexedSpansError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageIndexedSpansResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_internet_of_things(
        &self,
        start_hr: String,
        params: GetUsageInternetOfThingsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageIoTResponse, Error<GetUsageInternetOfThingsError>>
    {
        match self
            .get_usage_internet_of_things_with_http_info(start_hr, params)
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

    /// Get hourly usage for IoT.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_internet_of_things_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageInternetOfThingsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageIoTResponse>,
        Error<GetUsageInternetOfThingsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageIoTResponse>(&local_content)
            {
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

    /// Get hourly usage for Lambda.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_lambda(
        &self,
        start_hr: String,
        params: GetUsageLambdaOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageLambdaResponse, Error<GetUsageLambdaError>> {
        match self.get_usage_lambda_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for Lambda.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_lambda_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageLambdaOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLambdaResponse>,
        Error<GetUsageLambdaError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageLambdaResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs(
        &self,
        start_hr: String,
        params: GetUsageLogsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageLogsResponse, Error<GetUsageLogsError>> {
        match self.get_usage_logs_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for logs.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageLogsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageLogsResponse>, Error<GetUsageLogsError>>
    {
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsResponse>(&local_content)
            {
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
        start_hr: String,
        params: GetUsageLogsByIndexOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageLogsByIndexResponse, Error<GetUsageLogsByIndexError>>
    {
        match self
            .get_usage_logs_by_index_with_http_info(start_hr, params)
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

    /// Get hourly usage for logs by index.
    pub async fn get_usage_logs_by_index_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageLogsByIndexOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLogsByIndexResponse>,
        Error<GetUsageLogsByIndexError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsByIndexResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_by_retention(
        &self,
        start_hr: String,
        params: GetUsageLogsByRetentionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageLogsByRetentionResponse,
        Error<GetUsageLogsByRetentionError>,
    > {
        match self
            .get_usage_logs_by_retention_with_http_info(start_hr, params)
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

    /// Get hourly usage for indexed logs by retention period.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_logs_by_retention_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageLogsByRetentionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageLogsByRetentionResponse>,
        Error<GetUsageLogsByRetentionError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageLogsByRetentionResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_flows(
        &self,
        start_hr: String,
        params: GetUsageNetworkFlowsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageNetworkFlowsResponse, Error<GetUsageNetworkFlowsError>>
    {
        match self
            .get_usage_network_flows_with_http_info(start_hr, params)
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

    /// Get hourly usage for network flows.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_flows_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageNetworkFlowsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageNetworkFlowsResponse>,
        Error<GetUsageNetworkFlowsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageNetworkFlowsResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_hosts(
        &self,
        start_hr: String,
        params: GetUsageNetworkHostsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageNetworkHostsResponse, Error<GetUsageNetworkHostsError>>
    {
        match self
            .get_usage_network_hosts_with_http_info(start_hr, params)
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

    /// Get hourly usage for network hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_network_hosts_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageNetworkHostsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageNetworkHostsResponse>,
        Error<GetUsageNetworkHostsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageNetworkHostsResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_online_archive(
        &self,
        start_hr: String,
        params: GetUsageOnlineArchiveOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageOnlineArchiveResponse,
        Error<GetUsageOnlineArchiveError>,
    > {
        match self
            .get_usage_online_archive_with_http_info(start_hr, params)
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

    /// Get hourly usage for online archive.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_online_archive_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageOnlineArchiveOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageOnlineArchiveResponse>,
        Error<GetUsageOnlineArchiveError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageOnlineArchiveResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_profiling(
        &self,
        start_hr: String,
        params: GetUsageProfilingOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageProfilingResponse, Error<GetUsageProfilingError>>
    {
        match self
            .get_usage_profiling_with_http_info(start_hr, params)
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

    /// Get hourly usage for profiled hosts.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_profiling_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageProfilingOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageProfilingResponse>,
        Error<GetUsageProfilingError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageProfilingResponse>(
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

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Sessions.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_sessions(
        &self,
        start_hr: String,
        params: GetUsageRumSessionsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageRumSessionsResponse, Error<GetUsageRumSessionsError>>
    {
        match self
            .get_usage_rum_sessions_with_http_info(start_hr, params)
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

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Sessions.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_sessions_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageRumSessionsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageRumSessionsResponse>,
        Error<GetUsageRumSessionsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = type_ {
            local_req_builder =
                local_req_builder.query(&[("type", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageRumSessionsResponse>(
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

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Units.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_units(
        &self,
        start_hr: String,
        params: GetUsageRumUnitsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageRumUnitsResponse, Error<GetUsageRumUnitsError>> {
        match self
            .get_usage_rum_units_with_http_info(start_hr, params)
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

    /// Get hourly usage for [RUM](<https://docs.datadoghq.com/real_user_monitoring/>) Units.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_rum_units_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageRumUnitsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageRumUnitsResponse>,
        Error<GetUsageRumUnitsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageRumUnitsResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_sds(
        &self,
        start_hr: String,
        params: GetUsageSDSOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSDSResponse, Error<GetUsageSDSError>> {
        match self.get_usage_sds_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for sensitive data scanner.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_sds_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageSDSOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageSDSResponse>, Error<GetUsageSDSError>>
    {
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSDSResponse>(&local_content)
            {
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_snmp(
        &self,
        start_hr: String,
        params: GetUsageSNMPOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSNMPResponse, Error<GetUsageSNMPError>> {
        match self.get_usage_snmp_with_http_info(start_hr, params).await {
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

    /// Get hourly usage for SNMP devices.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_snmp_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageSNMPOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::UsageSNMPResponse>, Error<GetUsageSNMPError>>
    {
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSNMPResponse>(&local_content)
            {
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
        start_month: String,
        params: GetUsageSummaryOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSummaryResponse, Error<GetUsageSummaryError>> {
        match self
            .get_usage_summary_with_http_info(start_month, params)
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

    /// Get all usage across your account.
    pub async fn get_usage_summary_with_http_info(
        &self,
        start_month: String,
        params: GetUsageSummaryOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSummaryResponse>,
        Error<GetUsageSummaryError>,
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

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_org_details {
            local_req_builder =
                local_req_builder.query(&[("include_org_details", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSummaryResponse>(
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

    /// Get hourly usage for [synthetics checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageSyntheticsResponse, Error<GetUsageSyntheticsError>>
    {
        match self
            .get_usage_synthetics_with_http_info(start_hr, params)
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

    /// Get hourly usage for [synthetics checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsResponse>,
        Error<GetUsageSyntheticsError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsResponse>(
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

    /// Get hourly usage for [synthetics API checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_api(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsAPIOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageSyntheticsAPIResponse,
        Error<GetUsageSyntheticsAPIError>,
    > {
        match self
            .get_usage_synthetics_api_with_http_info(start_hr, params)
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

    /// Get hourly usage for [synthetics API checks](<https://docs.datadoghq.com/synthetics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_api_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsAPIOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsAPIResponse>,
        Error<GetUsageSyntheticsAPIError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsAPIResponse>(
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_browser(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsBrowserOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageSyntheticsBrowserResponse,
        Error<GetUsageSyntheticsBrowserError>,
    > {
        match self
            .get_usage_synthetics_browser_with_http_info(start_hr, params)
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

    /// Get hourly usage for synthetics browser checks.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_synthetics_browser_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageSyntheticsBrowserOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageSyntheticsBrowserResponse>,
        Error<GetUsageSyntheticsBrowserError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageSyntheticsBrowserResponse>(
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

    /// Get hourly usage for [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_timeseries(
        &self,
        start_hr: String,
        params: GetUsageTimeseriesOptionalParams,
    ) -> Result<crate::datadogV1::model::UsageTimeseriesResponse, Error<GetUsageTimeseriesError>>
    {
        match self
            .get_usage_timeseries_with_http_info(start_hr, params)
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

    /// Get hourly usage for [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>).
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>). Refer to [Migrating from the V1 Hourly Usage APIs to V2](<https://docs.datadoghq.com/account_management/guide/hourly-usage-migration/>) for the associated migration guide.
    pub async fn get_usage_timeseries_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageTimeseriesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageTimeseriesResponse>,
        Error<GetUsageTimeseriesError>,
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

        local_req_builder = local_req_builder.query(&[("start_hr", &start_hr.to_string())]);
        if let Some(ref local_query_param) = end_hr {
            local_req_builder =
                local_req_builder.query(&[("end_hr", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageTimeseriesResponse>(
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

    /// Get all [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics(
        &self,
        params: GetUsageTopAvgMetricsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::UsageTopAvgMetricsResponse,
        Error<GetUsageTopAvgMetricsError>,
    > {
        match self.get_usage_top_avg_metrics_with_http_info(params).await {
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

    /// Get all [custom metrics](<https://docs.datadoghq.com/developers/metrics/custom_metrics/>) by hourly average. Use the month parameter to get a month-to-date data resolution or use the day parameter to get a daily resolution. One of the two is required, and only one of the two is allowed.
    pub async fn get_usage_top_avg_metrics_with_http_info(
        &self,
        params: GetUsageTopAvgMetricsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::UsageTopAvgMetricsResponse>,
        Error<GetUsageTopAvgMetricsError>,
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
            local_req_builder =
                local_req_builder.query(&[("month", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = day {
            local_req_builder = local_req_builder.query(&[("day", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV1::model::UsageTopAvgMetricsResponse>(
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
