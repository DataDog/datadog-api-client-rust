// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// DeleteCostTagDescriptionByKeyOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::delete_cost_tag_description_by_key`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteCostTagDescriptionByKeyOptionalParams {
    /// Cloud provider to scope the deletion to (for example, `aws`). Omit to delete every description for the tag key.
    pub cloud: Option<String>,
}

impl DeleteCostTagDescriptionByKeyOptionalParams {
    /// Cloud provider to scope the deletion to (for example, `aws`). Omit to delete every description for the tag key.
    pub fn cloud(mut self, value: String) -> Self {
        self.cloud = Some(value);
        self
    }
}

/// GetBudgetOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_budget`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetBudgetOptionalParams {
    /// When `true`, includes actual cost data in the response.
    pub actual: Option<bool>,
    /// When `true`, includes forecast cost data in the response, including `ootb_forecast` and `custom_forecast` per entry.
    pub forecast: Option<bool>,
    /// Start of the cost window in milliseconds since epoch. Must be used together with `end`.
    pub start: Option<i64>,
    /// End of the cost window in milliseconds since epoch. Must be used together with `start`.
    pub end: Option<i64>,
}

impl GetBudgetOptionalParams {
    /// When `true`, includes actual cost data in the response.
    pub fn actual(mut self, value: bool) -> Self {
        self.actual = Some(value);
        self
    }
    /// When `true`, includes forecast cost data in the response, including `ootb_forecast` and `custom_forecast` per entry.
    pub fn forecast(mut self, value: bool) -> Self {
        self.forecast = Some(value);
        self
    }
    /// Start of the cost window in milliseconds since epoch. Must be used together with `end`.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
    /// End of the cost window in milliseconds since epoch. Must be used together with `start`.
    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }
}

/// GetCommitmentsCommitmentListOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_commitment_list`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsCommitmentListOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub commitment_type: Option<crate::datadogV2::model::CommitmentsCommitmentType>,
}

impl GetCommitmentsCommitmentListOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub fn commitment_type(
        mut self,
        value: crate::datadogV2::model::CommitmentsCommitmentType,
    ) -> Self {
        self.commitment_type = Some(value);
        self
    }
}

/// GetCommitmentsCoverageScalarOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_coverage_scalar`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsCoverageScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
}

impl GetCommitmentsCoverageScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
}

/// GetCommitmentsCoverageTimeseriesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_coverage_timeseries`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsCoverageTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
}

impl GetCommitmentsCoverageTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
}

/// GetCommitmentsOnDemandHotspotsScalarOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_on_demand_hotspots_scalar`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsOnDemandHotspotsScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
}

impl GetCommitmentsOnDemandHotspotsScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
}

/// GetCommitmentsSavingsScalarOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_savings_scalar`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsSavingsScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
}

impl GetCommitmentsSavingsScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
}

/// GetCommitmentsSavingsTimeseriesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_savings_timeseries`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsSavingsTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
}

impl GetCommitmentsSavingsTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
}

/// GetCommitmentsUtilizationScalarOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_utilization_scalar`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsUtilizationScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub commitment_type: Option<crate::datadogV2::model::CommitmentsCommitmentType>,
}

impl GetCommitmentsUtilizationScalarOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub fn commitment_type(
        mut self,
        value: crate::datadogV2::model::CommitmentsCommitmentType,
    ) -> Self {
        self.commitment_type = Some(value);
        self
    }
}

/// GetCommitmentsUtilizationTimeseriesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_commitments_utilization_timeseries`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCommitmentsUtilizationTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub filter_by: Option<String>,
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub commitment_type: Option<crate::datadogV2::model::CommitmentsCommitmentType>,
}

impl GetCommitmentsUtilizationTimeseriesOptionalParams {
    /// Optional filter expression to narrow down results.
    pub fn filter_by(mut self, value: String) -> Self {
        self.filter_by = Some(value);
        self
    }
    /// Type of commitment to query. ri for Reserved Instances, sp for Savings Plans. Defaults to ri.
    pub fn commitment_type(
        mut self,
        value: crate::datadogV2::model::CommitmentsCommitmentType,
    ) -> Self {
        self.commitment_type = Some(value);
        self
    }
}

/// GetCostTagDescriptionByKeyOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_cost_tag_description_by_key`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCostTagDescriptionByKeyOptionalParams {
    /// Cloud provider to scope the lookup to (for example, `aws`). Omit to use the resolved fallback.
    pub filter_cloud: Option<String>,
}

impl GetCostTagDescriptionByKeyOptionalParams {
    /// Cloud provider to scope the lookup to (for example, `aws`). Omit to use the resolved fallback.
    pub fn filter_cloud(mut self, value: String) -> Self {
        self.filter_cloud = Some(value);
        self
    }
}

/// GetCostTagKeyOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_cost_tag_key`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCostTagKeyOptionalParams {
    /// The Cloud Cost Management metric to scope the tag key details to. When omitted, returns details across all metrics.
    pub filter_metric: Option<String>,
    /// Controls the size of the internal tag value search scope. This does **not** restrict the number of example tag values returned in the response. Defaults to 50, maximum 10000.
    pub page_size: Option<i32>,
}

impl GetCostTagKeyOptionalParams {
    /// The Cloud Cost Management metric to scope the tag key details to. When omitted, returns details across all metrics.
    pub fn filter_metric(mut self, value: String) -> Self {
        self.filter_metric = Some(value);
        self
    }
    /// Controls the size of the internal tag value search scope. This does **not** restrict the number of example tag values returned in the response. Defaults to 50, maximum 10000.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// GetCostTagMetadataCurrencyOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::get_cost_tag_metadata_currency`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCostTagMetadataCurrencyOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub filter_provider: Option<String>,
}

impl GetCostTagMetadataCurrencyOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub fn filter_provider(mut self, value: String) -> Self {
        self.filter_provider = Some(value);
        self
    }
}

/// ListCostAnomaliesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_anomalies`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostAnomaliesOptionalParams {
    /// Start time as Unix milliseconds. Defaults to the start of the latest stable seven-day window.
    pub start: Option<i64>,
    /// End time as Unix milliseconds. Defaults to the end of the latest stable seven-day window.
    pub end: Option<i64>,
    /// Optional JSON object mapping cost tag keys to allowed values, for example `{"team":["payments"],"env":["prod"]}`. Filters match anomaly dimensions or correlated tags.
    pub filter: Option<String>,
    /// Minimum absolute anomalous cost change to include. Numeric value; defaults to `1`.
    pub min_anomalous_threshold: Option<String>,
    /// Minimum absolute actual cost to include. Numeric value; defaults to `0`.
    pub min_cost_threshold: Option<String>,
    /// Filter by resolution state. Use `none` for unresolved anomalies, `all` or `*` for resolved anomalies, or a comma-separated list of causes.
    pub dismissal_cause: Option<String>,
    /// Sort field. One of `start_date`, `end_date`, `duration`, `max_cost`, `anomalous_cost`, or `dismissal_date`. Defaults to `anomalous_cost`.
    pub order_by: Option<String>,
    /// Sort direction. One of `asc` or `desc`. Defaults to `desc`.
    pub order: Option<String>,
    /// Maximum number of anomalies to return. Defaults to `200`.
    pub limit: Option<i64>,
    /// Pagination offset. Defaults to `0`.
    pub offset: Option<i64>,
    /// Optional repeated cloud or SaaS provider filters, such as `aws`, `gcp`, `azure`, `Oracle`, `datadog`, `OpenAI`, or `Anthropic`.
    pub provider_ids: Option<Vec<String>>,
}

impl ListCostAnomaliesOptionalParams {
    /// Start time as Unix milliseconds. Defaults to the start of the latest stable seven-day window.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
    /// End time as Unix milliseconds. Defaults to the end of the latest stable seven-day window.
    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }
    /// Optional JSON object mapping cost tag keys to allowed values, for example `{"team":["payments"],"env":["prod"]}`. Filters match anomaly dimensions or correlated tags.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Minimum absolute anomalous cost change to include. Numeric value; defaults to `1`.
    pub fn min_anomalous_threshold(mut self, value: String) -> Self {
        self.min_anomalous_threshold = Some(value);
        self
    }
    /// Minimum absolute actual cost to include. Numeric value; defaults to `0`.
    pub fn min_cost_threshold(mut self, value: String) -> Self {
        self.min_cost_threshold = Some(value);
        self
    }
    /// Filter by resolution state. Use `none` for unresolved anomalies, `all` or `*` for resolved anomalies, or a comma-separated list of causes.
    pub fn dismissal_cause(mut self, value: String) -> Self {
        self.dismissal_cause = Some(value);
        self
    }
    /// Sort field. One of `start_date`, `end_date`, `duration`, `max_cost`, `anomalous_cost`, or `dismissal_date`. Defaults to `anomalous_cost`.
    pub fn order_by(mut self, value: String) -> Self {
        self.order_by = Some(value);
        self
    }
    /// Sort direction. One of `asc` or `desc`. Defaults to `desc`.
    pub fn order(mut self, value: String) -> Self {
        self.order = Some(value);
        self
    }
    /// Maximum number of anomalies to return. Defaults to `200`.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Pagination offset. Defaults to `0`.
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Optional repeated cloud or SaaS provider filters, such as `aws`, `gcp`, `azure`, `Oracle`, `datadog`, `OpenAI`, or `Anthropic`.
    pub fn provider_ids(mut self, value: Vec<String>) -> Self {
        self.provider_ids = Some(value);
        self
    }
}

/// ListCostTagDescriptionsOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_descriptions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagDescriptionsOptionalParams {
    /// Filter descriptions to a specific cloud provider (for example, `aws`). Omit to return descriptions across all clouds.
    pub filter_cloud: Option<String>,
}

impl ListCostTagDescriptionsOptionalParams {
    /// Filter descriptions to a specific cloud provider (for example, `aws`). Omit to return descriptions across all clouds.
    pub fn filter_cloud(mut self, value: String) -> Self {
        self.filter_cloud = Some(value);
        self
    }
}

/// ListCostTagKeySourcesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_key_sources`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagKeySourcesOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub filter_provider: Option<String>,
    /// Filter results to tag keys that have data for a specific Cloud Cost Management metric (for example, `aws.cost.net.amortized`). When omitted, all tag keys for the requested period are returned.
    pub filter_metric: Option<String>,
}

impl ListCostTagKeySourcesOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub fn filter_provider(mut self, value: String) -> Self {
        self.filter_provider = Some(value);
        self
    }
    /// Filter results to tag keys that have data for a specific Cloud Cost Management metric (for example, `aws.cost.net.amortized`). When omitted, all tag keys for the requested period are returned.
    pub fn filter_metric(mut self, value: String) -> Self {
        self.filter_metric = Some(value);
        self
    }
}

/// ListCostTagKeysOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_keys`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagKeysOptionalParams {
    /// The Cloud Cost Management metric to scope the tag keys to. When omitted, returns tag keys across all metrics.
    pub filter_metric: Option<String>,
    /// Filter to return only tag keys that appear with the given `key:value` tag values. For example, `filter[tags]=providername:aws` returns tag keys found on the same cost data, such as `is_aws_ec2_compute` and `aws_instance_type`.
    pub filter_tags: Option<Vec<String>>,
}

impl ListCostTagKeysOptionalParams {
    /// The Cloud Cost Management metric to scope the tag keys to. When omitted, returns tag keys across all metrics.
    pub fn filter_metric(mut self, value: String) -> Self {
        self.filter_metric = Some(value);
        self
    }
    /// Filter to return only tag keys that appear with the given `key:value` tag values. For example, `filter[tags]=providername:aws` returns tag keys found on the same cost data, such as `is_aws_ec2_compute` and `aws_instance_type`.
    pub fn filter_tags(mut self, value: Vec<String>) -> Self {
        self.filter_tags = Some(value);
        self
    }
}

/// ListCostTagMetadataOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_metadata`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagMetadataOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub filter_provider: Option<String>,
    /// Filter results to a specific Cloud Cost Management metric (for example, `aws.cost.net.amortized`). When omitted, every available metric for the requested period is returned.
    pub filter_metric: Option<String>,
    /// Restrict results to a single tag key.
    pub filter_tag_key: Option<String>,
    /// When `true`, return one row per day with the day in the `date` attribute. Defaults to the monthly roll-up when omitted.
    pub filter_daily: Option<crate::datadogV2::model::CostTagMetadataDailyFilter>,
}

impl ListCostTagMetadataOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub fn filter_provider(mut self, value: String) -> Self {
        self.filter_provider = Some(value);
        self
    }
    /// Filter results to a specific Cloud Cost Management metric (for example, `aws.cost.net.amortized`). When omitted, every available metric for the requested period is returned.
    pub fn filter_metric(mut self, value: String) -> Self {
        self.filter_metric = Some(value);
        self
    }
    /// Restrict results to a single tag key.
    pub fn filter_tag_key(mut self, value: String) -> Self {
        self.filter_tag_key = Some(value);
        self
    }
    /// When `true`, return one row per day with the day in the `date` attribute. Defaults to the monthly roll-up when omitted.
    pub fn filter_daily(
        mut self,
        value: crate::datadogV2::model::CostTagMetadataDailyFilter,
    ) -> Self {
        self.filter_daily = Some(value);
        self
    }
}

/// ListCostTagMetadataMetricsOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_metadata_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagMetadataMetricsOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub filter_provider: Option<String>,
    /// When `true`, only return metrics for currently enabled accounts. When omitted or `false`, return all metrics present in tag metadata. Metrics not recognized by Cloud Cost Management are always excluded.
    pub filter_enabled_metrics_only: Option<bool>,
}

impl ListCostTagMetadataMetricsOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub fn filter_provider(mut self, value: String) -> Self {
        self.filter_provider = Some(value);
        self
    }
    /// When `true`, only return metrics for currently enabled accounts. When omitted or `false`, return all metrics present in tag metadata. Metrics not recognized by Cloud Cost Management are always excluded.
    pub fn filter_enabled_metrics_only(mut self, value: bool) -> Self {
        self.filter_enabled_metrics_only = Some(value);
        self
    }
}

/// ListCostTagMetadataOrchestratorsOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tag_metadata_orchestrators`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagMetadataOrchestratorsOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub filter_provider: Option<String>,
}

impl ListCostTagMetadataOrchestratorsOptionalParams {
    /// Filter results to a specific provider. Common cloud values are `aws`, `azure`, `gcp`, `Oracle` (OCI), and `custom`. SaaS billing integrations (for example, `Snowflake`, `MongoDB`, `Databricks`) are also accepted using their display-name string. Values are case-sensitive.
    pub fn filter_provider(mut self, value: String) -> Self {
        self.filter_provider = Some(value);
        self
    }
}

/// ListCostTagsOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_cost_tags`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCostTagsOptionalParams {
    /// The Cloud Cost Management metric to scope the tags to. When omitted, returns tags across all metrics.
    pub filter_metric: Option<String>,
    /// A substring used to filter the returned tags by name.
    pub filter_match: Option<String>,
    /// Filter to return only tags that appear with the given `key:value` tag values. For example, `filter[tags]=providername:aws` returns tags found on the same cost data, such as `aws_instance_type:t3.micro` and `aws_instance_type:m5.large`.
    pub filter_tags: Option<Vec<String>>,
    /// Restrict the returned tags to those whose key matches one of the given tag keys.
    pub filter_tag_keys: Option<Vec<String>>,
    /// Controls the size of the internal tag search scope. This does **not** restrict the number of tags returned in the response. Defaults to 50, maximum 10000.
    pub page_size: Option<i32>,
}

impl ListCostTagsOptionalParams {
    /// The Cloud Cost Management metric to scope the tags to. When omitted, returns tags across all metrics.
    pub fn filter_metric(mut self, value: String) -> Self {
        self.filter_metric = Some(value);
        self
    }
    /// A substring used to filter the returned tags by name.
    pub fn filter_match(mut self, value: String) -> Self {
        self.filter_match = Some(value);
        self
    }
    /// Filter to return only tags that appear with the given `key:value` tag values. For example, `filter[tags]=providername:aws` returns tags found on the same cost data, such as `aws_instance_type:t3.micro` and `aws_instance_type:m5.large`.
    pub fn filter_tags(mut self, value: Vec<String>) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Restrict the returned tags to those whose key matches one of the given tag keys.
    pub fn filter_tag_keys(mut self, value: Vec<String>) -> Self {
        self.filter_tag_keys = Some(value);
        self
    }
    /// Controls the size of the internal tag search scope. This does **not** restrict the number of tags returned in the response. Defaults to 50, maximum 10000.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// ListCustomCostsFilesOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::list_custom_costs_files`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCustomCostsFilesOptionalParams {
    /// Page number for pagination
    pub page_number: Option<i64>,
    /// Page size for pagination
    pub page_size: Option<i64>,
    /// Filter by file status
    pub filter_status: Option<String>,
    /// Filter files by name with case-insensitive substring matching.
    pub filter_name: Option<String>,
    /// Filter by provider.
    pub filter_provider: Option<Vec<String>>,
    /// Sort key with optional descending prefix
    pub sort: Option<String>,
}

impl ListCustomCostsFilesOptionalParams {
    /// Page number for pagination
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Page size for pagination
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Filter by file status
    pub fn filter_status(mut self, value: String) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Filter files by name with case-insensitive substring matching.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter by provider.
    pub fn filter_provider(mut self, value: Vec<String>) -> Self {
        self.filter_provider = Some(value);
        self
    }
    /// Sort key with optional descending prefix
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// SearchCostRecommendationsOptionalParams is a struct for passing parameters to the method [`CloudCostManagementAPI::search_cost_recommendations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchCostRecommendationsOptionalParams {
    /// Number of results per page (1–10000).
    pub page_size: Option<String>,
    /// Pagination token from a previous response.
    pub page_token: Option<String>,
}

impl SearchCostRecommendationsOptionalParams {
    /// Number of results per page (1–10000).
    pub fn page_size(mut self, value: String) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Pagination token from a previous response.
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
}

/// CreateCostAWSCURConfigError is a struct for typed errors of method [`CloudCostManagementAPI::create_cost_awscur_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCostAWSCURConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCostAzureUCConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::create_cost_azure_uc_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCostAzureUCConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCostGCPUsageCostConfigError is a struct for typed errors of method [`CloudCostManagementAPI::create_cost_gcp_usage_cost_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCostGCPUsageCostConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCustomAllocationRuleError is a struct for typed errors of method [`CloudCostManagementAPI::create_custom_allocation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCustomAllocationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateTagPipelinesRulesetError is a struct for typed errors of method [`CloudCostManagementAPI::create_tag_pipelines_ruleset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagPipelinesRulesetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteBudgetError is a struct for typed errors of method [`CloudCostManagementAPI::delete_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCostAWSCURConfigError is a struct for typed errors of method [`CloudCostManagementAPI::delete_cost_awscur_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCostAWSCURConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCostAzureUCConfigError is a struct for typed errors of method [`CloudCostManagementAPI::delete_cost_azure_uc_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCostAzureUCConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCostGCPUsageCostConfigError is a struct for typed errors of method [`CloudCostManagementAPI::delete_cost_gcp_usage_cost_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCostGCPUsageCostConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCostTagDescriptionByKeyError is a struct for typed errors of method [`CloudCostManagementAPI::delete_cost_tag_description_by_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCostTagDescriptionByKeyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCustomAllocationRuleError is a struct for typed errors of method [`CloudCostManagementAPI::delete_custom_allocation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCustomAllocationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCustomCostsFileError is a struct for typed errors of method [`CloudCostManagementAPI::delete_custom_costs_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCustomCostsFileError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCustomForecastError is a struct for typed errors of method [`CloudCostManagementAPI::delete_custom_forecast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCustomForecastError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTagPipelinesRulesetError is a struct for typed errors of method [`CloudCostManagementAPI::delete_tag_pipelines_ruleset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagPipelinesRulesetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GenerateCostTagDescriptionByKeyError is a struct for typed errors of method [`CloudCostManagementAPI::generate_cost_tag_description_by_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateCostTagDescriptionByKeyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBudgetError is a struct for typed errors of method [`CloudCostManagementAPI::get_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsCommitmentListError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_commitment_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsCommitmentListError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsCoverageScalarError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_coverage_scalar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsCoverageScalarError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsCoverageTimeseriesError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_coverage_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsCoverageTimeseriesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsOnDemandHotspotsScalarError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_on_demand_hotspots_scalar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsOnDemandHotspotsScalarError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsSavingsScalarError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_savings_scalar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsSavingsScalarError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsSavingsTimeseriesError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_savings_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsSavingsTimeseriesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsUtilizationScalarError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_utilization_scalar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsUtilizationScalarError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCommitmentsUtilizationTimeseriesError is a struct for typed errors of method [`CloudCostManagementAPI::get_commitments_utilization_timeseries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommitmentsUtilizationTimeseriesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostAWSCURConfigError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_awscur_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostAWSCURConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostAnomalyError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_anomaly`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostAnomalyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostAzureUCConfigError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_azure_uc_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostAzureUCConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostGCPUsageCostConfigError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_gcp_usage_cost_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostGCPUsageCostConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostTagDescriptionByKeyError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_tag_description_by_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostTagDescriptionByKeyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostTagKeyError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_tag_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostTagKeyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostTagMetadataCurrencyError is a struct for typed errors of method [`CloudCostManagementAPI::get_cost_tag_metadata_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostTagMetadataCurrencyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCustomAllocationRuleError is a struct for typed errors of method [`CloudCostManagementAPI::get_custom_allocation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomAllocationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCustomCostsFileError is a struct for typed errors of method [`CloudCostManagementAPI::get_custom_costs_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomCostsFileError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTagPipelinesRulesetError is a struct for typed errors of method [`CloudCostManagementAPI::get_tag_pipelines_ruleset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagPipelinesRulesetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListBudgetsError is a struct for typed errors of method [`CloudCostManagementAPI::list_budgets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBudgetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostAWSCURConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_awscur_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostAWSCURConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostAnomaliesError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_anomalies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostAnomaliesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostAzureUCConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_azure_uc_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostAzureUCConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostGCPUsageCostConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_gcp_usage_cost_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostGCPUsageCostConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostOCIConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_oci_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostOCIConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagDescriptionsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_descriptions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagDescriptionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagKeySourcesError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_key_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagKeySourcesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagKeysError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagKeysError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagMetadataError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagMetadataError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagMetadataMetricsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_metadata_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagMetadataMetricsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagMetadataMonthsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_metadata_months`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagMetadataMonthsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagMetadataOrchestratorsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tag_metadata_orchestrators`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagMetadataOrchestratorsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCostTagsError is a struct for typed errors of method [`CloudCostManagementAPI::list_cost_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCostTagsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCustomAllocationRulesError is a struct for typed errors of method [`CloudCostManagementAPI::list_custom_allocation_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCustomAllocationRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCustomAllocationRulesStatusError is a struct for typed errors of method [`CloudCostManagementAPI::list_custom_allocation_rules_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCustomAllocationRulesStatusError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCustomCostsFilesError is a struct for typed errors of method [`CloudCostManagementAPI::list_custom_costs_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCustomCostsFilesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagPipelinesRulesetsError is a struct for typed errors of method [`CloudCostManagementAPI::list_tag_pipelines_rulesets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagPipelinesRulesetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagPipelinesRulesetsStatusError is a struct for typed errors of method [`CloudCostManagementAPI::list_tag_pipelines_rulesets_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagPipelinesRulesetsStatusError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ReorderCustomAllocationRulesError is a struct for typed errors of method [`CloudCostManagementAPI::reorder_custom_allocation_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderCustomAllocationRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ReorderTagPipelinesRulesetsError is a struct for typed errors of method [`CloudCostManagementAPI::reorder_tag_pipelines_rulesets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderTagPipelinesRulesetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchCostRecommendationsError is a struct for typed errors of method [`CloudCostManagementAPI::search_cost_recommendations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCostRecommendationsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCostAWSCURConfigError is a struct for typed errors of method [`CloudCostManagementAPI::update_cost_awscur_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCostAWSCURConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCostAzureUCConfigsError is a struct for typed errors of method [`CloudCostManagementAPI::update_cost_azure_uc_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCostAzureUCConfigsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCostGCPUsageCostConfigError is a struct for typed errors of method [`CloudCostManagementAPI::update_cost_gcp_usage_cost_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCostGCPUsageCostConfigError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCustomAllocationRuleError is a struct for typed errors of method [`CloudCostManagementAPI::update_custom_allocation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCustomAllocationRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTagPipelinesRulesetError is a struct for typed errors of method [`CloudCostManagementAPI::update_tag_pipelines_ruleset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTagPipelinesRulesetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UploadCustomCostsFileError is a struct for typed errors of method [`CloudCostManagementAPI::upload_custom_costs_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadCustomCostsFileError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpsertBudgetError is a struct for typed errors of method [`CloudCostManagementAPI::upsert_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpsertCostTagDescriptionByKeyError is a struct for typed errors of method [`CloudCostManagementAPI::upsert_cost_tag_description_by_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertCostTagDescriptionByKeyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpsertCustomForecastError is a struct for typed errors of method [`CloudCostManagementAPI::upsert_custom_forecast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertCustomForecastError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateBudgetError is a struct for typed errors of method [`CloudCostManagementAPI::validate_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateCsvBudgetError is a struct for typed errors of method [`CloudCostManagementAPI::validate_csv_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateCsvBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateQueryError is a struct for typed errors of method [`CloudCostManagementAPI::validate_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateQueryError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The Cloud Cost Management API allows you to set up, edit, and delete Cloud Cost Management accounts for AWS, Azure, and Google Cloud. You can query your cost data by using the [Metrics endpoint](<https://docs.datadoghq.com/api/latest/metrics/#query-timeseries-data-across-multiple-products>) and the `cloud_cost` data source. For more information, see the [Cloud Cost Management documentation](<https://docs.datadoghq.com/cloud_cost_management/>).
#[derive(Debug, Clone)]
pub struct CloudCostManagementAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CloudCostManagementAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CloudCostManagementAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
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

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a Cloud Cost Management account for an AWS CUR config.
    pub async fn create_cost_awscur_config(
        &self,
        body: crate::datadogV2::model::AwsCURConfigPostRequest,
    ) -> Result<
        crate::datadogV2::model::AwsCurConfigResponse,
        datadog::Error<CreateCostAWSCURConfigError>,
    > {
        match self.create_cost_awscur_config_with_http_info(body).await {
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

    /// Create a Cloud Cost Management account for an AWS CUR config.
    pub async fn create_cost_awscur_config_with_http_info(
        &self,
        body: crate::datadogV2::model::AwsCURConfigPostRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AwsCurConfigResponse>,
        datadog::Error<CreateCostAWSCURConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_cost_awscur_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/aws_cur_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AwsCurConfigResponse>(
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
            let local_entity: Option<CreateCostAWSCURConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a Cloud Cost Management account for an Azure config.
    pub async fn create_cost_azure_uc_configs(
        &self,
        body: crate::datadogV2::model::AzureUCConfigPostRequest,
    ) -> Result<
        crate::datadogV2::model::AzureUCConfigPairsResponse,
        datadog::Error<CreateCostAzureUCConfigsError>,
    > {
        match self.create_cost_azure_uc_configs_with_http_info(body).await {
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

    /// Create a Cloud Cost Management account for an Azure config.
    pub async fn create_cost_azure_uc_configs_with_http_info(
        &self,
        body: crate::datadogV2::model::AzureUCConfigPostRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AzureUCConfigPairsResponse>,
        datadog::Error<CreateCostAzureUCConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_cost_azure_uc_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/azure_uc_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AzureUCConfigPairsResponse>(
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
            let local_entity: Option<CreateCostAzureUCConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a Cloud Cost Management account for an Google Cloud Usage Cost config.
    pub async fn create_cost_gcp_usage_cost_config(
        &self,
        body: crate::datadogV2::model::GCPUsageCostConfigPostRequest,
    ) -> Result<
        crate::datadogV2::model::GCPUsageCostConfigResponse,
        datadog::Error<CreateCostGCPUsageCostConfigError>,
    > {
        match self
            .create_cost_gcp_usage_cost_config_with_http_info(body)
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

    /// Create a Cloud Cost Management account for an Google Cloud Usage Cost config.
    pub async fn create_cost_gcp_usage_cost_config_with_http_info(
        &self,
        body: crate::datadogV2::model::GCPUsageCostConfigPostRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GCPUsageCostConfigResponse>,
        datadog::Error<CreateCostGCPUsageCostConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_cost_gcp_usage_cost_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/gcp_uc_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GCPUsageCostConfigResponse>(
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
            let local_entity: Option<CreateCostGCPUsageCostConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new custom allocation rule with the specified filters and allocation strategy.
    ///
    /// **Strategy Methods:**
    /// - **PROPORTIONAL/EVEN**: Allocates costs proportionally/evenly based on existing costs. Requires: granularity, allocated_by_tag_keys. Optional: based_on_costs, allocated_by_filters, evaluate_grouped_by_tag_keys, evaluate_grouped_by_filters.
    /// - **PROPORTIONAL_TIMESERIES/EVEN_TIMESERIES**: Allocates based on timeseries data. Requires: granularity, based_on_timeseries. Optional: evaluate_grouped_by_tag_keys.
    /// - **PERCENT**: Allocates fixed percentages to specific tags. Requires: allocated_by (array of percentage allocations).
    ///
    /// **Filter Conditions:**
    /// - Use **value** for single-value conditions: "is", "is not", "contains", "=", "!=", "like", "not like"
    /// - Use **values** for multi-value conditions: "in", "not in"
    /// - Cannot use both value and values simultaneously.
    ///
    /// **Supported operators**: is, is not, contains, in, not in, =, !=, like, not like
    pub async fn create_custom_allocation_rule(
        &self,
        body: crate::datadogV2::model::ArbitraryCostUpsertRequest,
    ) -> Result<
        crate::datadogV2::model::ArbitraryRuleResponse,
        datadog::Error<CreateCustomAllocationRuleError>,
    > {
        match self
            .create_custom_allocation_rule_with_http_info(body)
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

    /// Create a new custom allocation rule with the specified filters and allocation strategy.
    ///
    /// **Strategy Methods:**
    /// - **PROPORTIONAL/EVEN**: Allocates costs proportionally/evenly based on existing costs. Requires: granularity, allocated_by_tag_keys. Optional: based_on_costs, allocated_by_filters, evaluate_grouped_by_tag_keys, evaluate_grouped_by_filters.
    /// - **PROPORTIONAL_TIMESERIES/EVEN_TIMESERIES**: Allocates based on timeseries data. Requires: granularity, based_on_timeseries. Optional: evaluate_grouped_by_tag_keys.
    /// - **PERCENT**: Allocates fixed percentages to specific tags. Requires: allocated_by (array of percentage allocations).
    ///
    /// **Filter Conditions:**
    /// - Use **value** for single-value conditions: "is", "is not", "contains", "=", "!=", "like", "not like"
    /// - Use **values** for multi-value conditions: "in", "not in"
    /// - Cannot use both value and values simultaneously.
    ///
    /// **Supported operators**: is, is not, contains, in, not in, =, !=, like, not like
    pub async fn create_custom_allocation_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::ArbitraryCostUpsertRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ArbitraryRuleResponse>,
        datadog::Error<CreateCustomAllocationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_custom_allocation_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ArbitraryRuleResponse>(
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
            let local_entity: Option<CreateCustomAllocationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new tag pipeline ruleset with the specified rules and configuration
    pub async fn create_tag_pipelines_ruleset(
        &self,
        body: crate::datadogV2::model::CreateRulesetRequest,
    ) -> Result<crate::datadogV2::model::RulesetResp, datadog::Error<CreateTagPipelinesRulesetError>>
    {
        match self.create_tag_pipelines_ruleset_with_http_info(body).await {
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

    /// Create a new tag pipeline ruleset with the specified rules and configuration
    pub async fn create_tag_pipelines_ruleset_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateRulesetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesetResp>,
        datadog::Error<CreateTagPipelinesRulesetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_tag_pipelines_ruleset";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::RulesetResp>(&local_content) {
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
            let local_entity: Option<CreateTagPipelinesRulesetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a budget
    pub async fn delete_budget(
        &self,
        budget_id: String,
    ) -> Result<(), datadog::Error<DeleteBudgetError>> {
        match self.delete_budget_with_http_info(budget_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a budget
    pub async fn delete_budget_with_http_info(
        &self,
        budget_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteBudgetError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_budget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/{budget_id}",
            local_configuration.get_operation_host(operation_id),
            budget_id = datadog::urlencode(budget_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteBudgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Archive a Cloud Cost Management Account.
    pub async fn delete_cost_awscur_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<(), datadog::Error<DeleteCostAWSCURConfigError>> {
        match self
            .delete_cost_awscur_config_with_http_info(cloud_account_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive a Cloud Cost Management Account.
    pub async fn delete_cost_awscur_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCostAWSCURConfigError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_cost_awscur_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/aws_cur_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCostAWSCURConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Archive a Cloud Cost Management Account.
    pub async fn delete_cost_azure_uc_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<(), datadog::Error<DeleteCostAzureUCConfigError>> {
        match self
            .delete_cost_azure_uc_config_with_http_info(cloud_account_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive a Cloud Cost Management Account.
    pub async fn delete_cost_azure_uc_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCostAzureUCConfigError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_cost_azure_uc_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/azure_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCostAzureUCConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Archive a Cloud Cost Management account.
    pub async fn delete_cost_gcp_usage_cost_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<(), datadog::Error<DeleteCostGCPUsageCostConfigError>> {
        match self
            .delete_cost_gcp_usage_cost_config_with_http_info(cloud_account_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive a Cloud Cost Management account.
    pub async fn delete_cost_gcp_usage_cost_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCostGCPUsageCostConfigError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_cost_gcp_usage_cost_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/gcp_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCostGCPUsageCostConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a Cloud Cost Management tag key description. When `cloud` is omitted, deletes every description for the tag key, falling back to Datadog's global default when available. When `cloud` is provided, deletes only the description scoped to that cloud provider.
    pub async fn delete_cost_tag_description_by_key(
        &self,
        tag_key: String,
        params: DeleteCostTagDescriptionByKeyOptionalParams,
    ) -> Result<(), datadog::Error<DeleteCostTagDescriptionByKeyError>> {
        match self
            .delete_cost_tag_description_by_key_with_http_info(tag_key, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Cloud Cost Management tag key description. When `cloud` is omitted, deletes every description for the tag key, falling back to Datadog's global default when available. When `cloud` is provided, deletes only the description scoped to that cloud provider.
    pub async fn delete_cost_tag_description_by_key_with_http_info(
        &self,
        tag_key: String,
        params: DeleteCostTagDescriptionByKeyOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCostTagDescriptionByKeyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_cost_tag_description_by_key";

        // unbox and build optional parameters
        let cloud = params.cloud;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_descriptions/{tag_key}",
            local_configuration.get_operation_host(operation_id),
            tag_key = datadog::urlencode(tag_key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = cloud {
            local_req_builder =
                local_req_builder.query(&[("cloud", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCostTagDescriptionByKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a custom allocation rule - Delete an existing custom allocation rule by its ID
    pub async fn delete_custom_allocation_rule(
        &self,
        rule_id: i64,
    ) -> Result<(), datadog::Error<DeleteCustomAllocationRuleError>> {
        match self
            .delete_custom_allocation_rule_with_http_info(rule_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a custom allocation rule - Delete an existing custom allocation rule by its ID
    pub async fn delete_custom_allocation_rule_with_http_info(
        &self,
        rule_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCustomAllocationRuleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_custom_allocation_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = rule_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCustomAllocationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete the specified Custom Costs file.
    pub async fn delete_custom_costs_file(
        &self,
        file_id: String,
    ) -> Result<(), datadog::Error<DeleteCustomCostsFileError>> {
        match self.delete_custom_costs_file_with_http_info(file_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete the specified Custom Costs file.
    pub async fn delete_custom_costs_file_with_http_info(
        &self,
        file_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCustomCostsFileError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_custom_costs_file";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/custom_costs/{file_id}",
            local_configuration.get_operation_host(operation_id),
            file_id = datadog::urlencode(file_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCustomCostsFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete the custom forecast for a budget.
    pub async fn delete_custom_forecast(
        &self,
        budget_id: String,
    ) -> Result<(), datadog::Error<DeleteCustomForecastError>> {
        match self.delete_custom_forecast_with_http_info(budget_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete the custom forecast for a budget.
    pub async fn delete_custom_forecast_with_http_info(
        &self,
        budget_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCustomForecastError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_custom_forecast";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_custom_forecast' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/{budget_id}/custom-forecast",
            local_configuration.get_operation_host(operation_id),
            budget_id = datadog::urlencode(budget_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCustomForecastError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a tag pipeline ruleset - Delete an existing tag pipeline ruleset by its ID
    pub async fn delete_tag_pipelines_ruleset(
        &self,
        ruleset_id: String,
    ) -> Result<(), datadog::Error<DeleteTagPipelinesRulesetError>> {
        match self
            .delete_tag_pipelines_ruleset_with_http_info(ruleset_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a tag pipeline ruleset - Delete an existing tag pipeline ruleset by its ID
    pub async fn delete_tag_pipelines_ruleset_with_http_info(
        &self,
        ruleset_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteTagPipelinesRulesetError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_tag_pipelines_ruleset";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/{ruleset_id}",
            local_configuration.get_operation_host(operation_id),
            ruleset_id = datadog::urlencode(ruleset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteTagPipelinesRulesetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Use AI to draft a Cloud Cost Management tag key description based on associated cost data. The generated description is returned in the response and is not persisted by this endpoint; follow up with `UpsertCostTagDescriptionByKey` to save it.
    pub async fn generate_cost_tag_description_by_key(
        &self,
        tag_key: String,
    ) -> Result<
        crate::datadogV2::model::GenerateCostTagDescriptionResponse,
        datadog::Error<GenerateCostTagDescriptionByKeyError>,
    > {
        match self
            .generate_cost_tag_description_by_key_with_http_info(tag_key)
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

    /// Use AI to draft a Cloud Cost Management tag key description based on associated cost data. The generated description is returned in the response and is not persisted by this endpoint; follow up with `UpsertCostTagDescriptionByKey` to save it.
    pub async fn generate_cost_tag_description_by_key_with_http_info(
        &self,
        tag_key: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GenerateCostTagDescriptionResponse>,
        datadog::Error<GenerateCostTagDescriptionByKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.generate_cost_tag_description_by_key";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_descriptions/{tag_key}/generate",
            local_configuration.get_operation_host(operation_id),
            tag_key = datadog::urlencode(tag_key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::GenerateCostTagDescriptionResponse>(
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
            let local_entity: Option<GenerateCostTagDescriptionByKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a budget by ID. Pass `actual=true` or `forecast=true` to include cost data in the response. Use `start` and `end` (millisecond epochs, both required) to set the cost window. When `forecast=true`, each entry also includes `ootb_forecast` (the ML forecast before overrides) and `custom_forecast` (`null` if no override is set, a number if one is).
    pub async fn get_budget(
        &self,
        budget_id: String,
        params: GetBudgetOptionalParams,
    ) -> Result<crate::datadogV2::model::BudgetWithEntries, datadog::Error<GetBudgetError>> {
        match self.get_budget_with_http_info(budget_id, params).await {
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

    /// Get a budget by ID. Pass `actual=true` or `forecast=true` to include cost data in the response. Use `start` and `end` (millisecond epochs, both required) to set the cost window. When `forecast=true`, each entry also includes `ootb_forecast` (the ML forecast before overrides) and `custom_forecast` (`null` if no override is set, a number if one is).
    pub async fn get_budget_with_http_info(
        &self,
        budget_id: String,
        params: GetBudgetOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BudgetWithEntries>,
        datadog::Error<GetBudgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_budget";

        // unbox and build optional parameters
        let actual = params.actual;
        let forecast = params.forecast;
        let start = params.start;
        let end = params.end;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/{budget_id}",
            local_configuration.get_operation_host(operation_id),
            budget_id = datadog::urlencode(budget_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = actual {
            local_req_builder =
                local_req_builder.query(&[("actual", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = forecast {
            local_req_builder =
                local_req_builder.query(&[("forecast", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end {
            local_req_builder = local_req_builder.query(&[("end", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::BudgetWithEntries>(&local_content)
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
            let local_entity: Option<GetBudgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of individual cloud commitments (Reserved Instances or Savings Plans) with their utilization details. The response schema varies based on the provider, product, and commitment type.
    pub async fn get_commitments_commitment_list(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCommitmentListOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsListResponse,
        datadog::Error<GetCommitmentsCommitmentListError>,
    > {
        match self
            .get_commitments_commitment_list_with_http_info(provider, product, start, end, params)
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

    /// Get a list of individual cloud commitments (Reserved Instances or Savings Plans) with their utilization details. The response schema varies based on the provider, product, and commitment type.
    pub async fn get_commitments_commitment_list_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCommitmentListOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsListResponse>,
        datadog::Error<GetCommitmentsCommitmentListError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_commitment_list";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_commitment_list' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;
        let commitment_type = params.commitment_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/commitment-list",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = commitment_type {
            local_req_builder =
                local_req_builder.query(&[("commitmentType", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CommitmentsListResponse>(
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
            let local_entity: Option<GetCommitmentsCommitmentListError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get scalar coverage metrics for cloud commitment programs, including hours and cost coverage percentages.
    pub async fn get_commitments_coverage_scalar(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCoverageScalarOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsCoverageScalarResponse,
        datadog::Error<GetCommitmentsCoverageScalarError>,
    > {
        match self
            .get_commitments_coverage_scalar_with_http_info(provider, product, start, end, params)
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

    /// Get scalar coverage metrics for cloud commitment programs, including hours and cost coverage percentages.
    pub async fn get_commitments_coverage_scalar_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCoverageScalarOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsCoverageScalarResponse>,
        datadog::Error<GetCommitmentsCoverageScalarError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_coverage_scalar";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_coverage_scalar' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/coverage/scalar",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CommitmentsCoverageScalarResponse>(
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
            let local_entity: Option<GetCommitmentsCoverageScalarError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get timeseries coverage metrics for cloud commitment programs, broken down by coverage type (Reserved Instances, Savings Plans, On-Demand, and Spot) for both hours and cost.
    pub async fn get_commitments_coverage_timeseries(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCoverageTimeseriesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsCoverageTimeseriesResponse,
        datadog::Error<GetCommitmentsCoverageTimeseriesError>,
    > {
        match self
            .get_commitments_coverage_timeseries_with_http_info(
                provider, product, start, end, params,
            )
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

    /// Get timeseries coverage metrics for cloud commitment programs, broken down by coverage type (Reserved Instances, Savings Plans, On-Demand, and Spot) for both hours and cost.
    pub async fn get_commitments_coverage_timeseries_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsCoverageTimeseriesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsCoverageTimeseriesResponse>,
        datadog::Error<GetCommitmentsCoverageTimeseriesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_coverage_timeseries";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_coverage_timeseries' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/coverage/timeseries",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
                crate::datadogV2::model::CommitmentsCoverageTimeseriesResponse,
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
            let local_entity: Option<GetCommitmentsCoverageTimeseriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get scalar on-demand hot-spots data for cloud commitment programs, showing per-dimension breakdowns of on-demand spending with coverage metrics and potential savings.
    pub async fn get_commitments_on_demand_hotspots_scalar(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsOnDemandHotspotsScalarOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarResponse,
        datadog::Error<GetCommitmentsOnDemandHotspotsScalarError>,
    > {
        match self
            .get_commitments_on_demand_hotspots_scalar_with_http_info(
                provider, product, start, end, params,
            )
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

    /// Get scalar on-demand hot-spots data for cloud commitment programs, showing per-dimension breakdowns of on-demand spending with coverage metrics and potential savings.
    pub async fn get_commitments_on_demand_hotspots_scalar_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsOnDemandHotspotsScalarOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarResponse,
        >,
        datadog::Error<GetCommitmentsOnDemandHotspotsScalarError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_on_demand_hotspots_scalar";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_on_demand_hotspots_scalar' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/on-demand-hot-spots/scalar",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
                crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarResponse,
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
            let local_entity: Option<GetCommitmentsOnDemandHotspotsScalarError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get scalar savings metrics for cloud commitment programs, including realized savings and effective savings rate.
    pub async fn get_commitments_savings_scalar(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsSavingsScalarOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsSavingsScalarResponse,
        datadog::Error<GetCommitmentsSavingsScalarError>,
    > {
        match self
            .get_commitments_savings_scalar_with_http_info(provider, product, start, end, params)
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

    /// Get scalar savings metrics for cloud commitment programs, including realized savings and effective savings rate.
    pub async fn get_commitments_savings_scalar_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsSavingsScalarOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsSavingsScalarResponse>,
        datadog::Error<GetCommitmentsSavingsScalarError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_savings_scalar";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_savings_scalar' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/savings/scalar",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CommitmentsSavingsScalarResponse>(
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
            let local_entity: Option<GetCommitmentsSavingsScalarError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get timeseries savings metrics for cloud commitment programs, including actual cost, on-demand equivalent cost, realized savings, and effective savings rate over time.
    pub async fn get_commitments_savings_timeseries(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsSavingsTimeseriesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsSavingsTimeseriesResponse,
        datadog::Error<GetCommitmentsSavingsTimeseriesError>,
    > {
        match self
            .get_commitments_savings_timeseries_with_http_info(
                provider, product, start, end, params,
            )
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

    /// Get timeseries savings metrics for cloud commitment programs, including actual cost, on-demand equivalent cost, realized savings, and effective savings rate over time.
    pub async fn get_commitments_savings_timeseries_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsSavingsTimeseriesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsSavingsTimeseriesResponse>,
        datadog::Error<GetCommitmentsSavingsTimeseriesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_savings_timeseries";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_savings_timeseries' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/savings/timeseries",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
                crate::datadogV2::model::CommitmentsSavingsTimeseriesResponse,
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
            let local_entity: Option<GetCommitmentsSavingsTimeseriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get scalar utilization metrics for cloud commitment programs, including utilization percentage and unused cost.
    pub async fn get_commitments_utilization_scalar(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsUtilizationScalarOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsUtilizationScalarResponse,
        datadog::Error<GetCommitmentsUtilizationScalarError>,
    > {
        match self
            .get_commitments_utilization_scalar_with_http_info(
                provider, product, start, end, params,
            )
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

    /// Get scalar utilization metrics for cloud commitment programs, including utilization percentage and unused cost.
    pub async fn get_commitments_utilization_scalar_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsUtilizationScalarOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsUtilizationScalarResponse>,
        datadog::Error<GetCommitmentsUtilizationScalarError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_utilization_scalar";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_utilization_scalar' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;
        let commitment_type = params.commitment_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/utilization/scalar",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = commitment_type {
            local_req_builder =
                local_req_builder.query(&[("commitmentType", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
                crate::datadogV2::model::CommitmentsUtilizationScalarResponse,
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
            let local_entity: Option<GetCommitmentsUtilizationScalarError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get timeseries utilization metrics for cloud commitment programs, including used and unused cost series over time.
    pub async fn get_commitments_utilization_timeseries(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsUtilizationTimeseriesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CommitmentsUtilizationTimeseriesResponse,
        datadog::Error<GetCommitmentsUtilizationTimeseriesError>,
    > {
        match self
            .get_commitments_utilization_timeseries_with_http_info(
                provider, product, start, end, params,
            )
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

    /// Get timeseries utilization metrics for cloud commitment programs, including used and unused cost series over time.
    pub async fn get_commitments_utilization_timeseries_with_http_info(
        &self,
        provider: crate::datadogV2::model::CommitmentsProvider,
        product: String,
        start: i64,
        end: i64,
        params: GetCommitmentsUtilizationTimeseriesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CommitmentsUtilizationTimeseriesResponse>,
        datadog::Error<GetCommitmentsUtilizationTimeseriesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_commitments_utilization_timeseries";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_commitments_utilization_timeseries' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_by = params.filter_by;
        let commitment_type = params.commitment_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/commitments/utilization/timeseries",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("provider", &provider.to_string())]);
        local_req_builder = local_req_builder.query(&[("product", &product.to_string())]);
        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = filter_by {
            local_req_builder =
                local_req_builder.query(&[("filterBy", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = commitment_type {
            local_req_builder =
                local_req_builder.query(&[("commitmentType", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
                crate::datadogV2::model::CommitmentsUtilizationTimeseriesResponse,
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
            let local_entity: Option<GetCommitmentsUtilizationTimeseriesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific AWS CUR config.
    pub async fn get_cost_awscur_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<
        crate::datadogV2::model::AwsCurConfigResponse,
        datadog::Error<GetCostAWSCURConfigError>,
    > {
        match self
            .get_cost_awscur_config_with_http_info(cloud_account_id)
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

    /// Get a specific AWS CUR config.
    pub async fn get_cost_awscur_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AwsCurConfigResponse>,
        datadog::Error<GetCostAWSCURConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_awscur_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/aws_cur_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::AwsCurConfigResponse>(
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
            let local_entity: Option<GetCostAWSCURConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a detected Cloud Cost Management anomaly by UUID.
    pub async fn get_cost_anomaly(
        &self,
        anomaly_id: String,
    ) -> Result<crate::datadogV2::model::CostAnomalyResponse, datadog::Error<GetCostAnomalyError>>
    {
        match self.get_cost_anomaly_with_http_info(anomaly_id).await {
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

    /// Get a detected Cloud Cost Management anomaly by UUID.
    pub async fn get_cost_anomaly_with_http_info(
        &self,
        anomaly_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostAnomalyResponse>,
        datadog::Error<GetCostAnomalyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_anomaly";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_cost_anomaly' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/anomalies/{anomaly_id}",
            local_configuration.get_operation_host(operation_id),
            anomaly_id = datadog::urlencode(anomaly_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostAnomalyResponse>(
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
            let local_entity: Option<GetCostAnomalyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific Azure config.
    pub async fn get_cost_azure_uc_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<crate::datadogV2::model::UCConfigPair, datadog::Error<GetCostAzureUCConfigError>>
    {
        match self
            .get_cost_azure_uc_config_with_http_info(cloud_account_id)
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

    /// Get a specific Azure config.
    pub async fn get_cost_azure_uc_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UCConfigPair>,
        datadog::Error<GetCostAzureUCConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_azure_uc_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/azure_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::UCConfigPair>(&local_content) {
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
            let local_entity: Option<GetCostAzureUCConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific Google Cloud Usage Cost config.
    pub async fn get_cost_gcp_usage_cost_config(
        &self,
        cloud_account_id: i64,
    ) -> Result<
        crate::datadogV2::model::GcpUcConfigResponse,
        datadog::Error<GetCostGCPUsageCostConfigError>,
    > {
        match self
            .get_cost_gcp_usage_cost_config_with_http_info(cloud_account_id)
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

    /// Get a specific Google Cloud Usage Cost config.
    pub async fn get_cost_gcp_usage_cost_config_with_http_info(
        &self,
        cloud_account_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GcpUcConfigResponse>,
        datadog::Error<GetCostGCPUsageCostConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_gcp_usage_cost_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/gcp_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::GcpUcConfigResponse>(
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
            let local_entity: Option<GetCostGCPUsageCostConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the Cloud Cost Management description for a single tag key. Use `filter[cloud]` to scope the lookup to a specific cloud provider; when omitted, the response resolves the description in fallback order (cloud-specific organization override, then cloudless organization default, then Datadog's global default).
    pub async fn get_cost_tag_description_by_key(
        &self,
        tag_key: String,
        params: GetCostTagDescriptionByKeyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostTagDescriptionResponse,
        datadog::Error<GetCostTagDescriptionByKeyError>,
    > {
        match self
            .get_cost_tag_description_by_key_with_http_info(tag_key, params)
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

    /// Get the Cloud Cost Management description for a single tag key. Use `filter[cloud]` to scope the lookup to a specific cloud provider; when omitted, the response resolves the description in fallback order (cloud-specific organization override, then cloudless organization default, then Datadog's global default).
    pub async fn get_cost_tag_description_by_key_with_http_info(
        &self,
        tag_key: String,
        params: GetCostTagDescriptionByKeyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagDescriptionResponse>,
        datadog::Error<GetCostTagDescriptionByKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_tag_description_by_key";

        // unbox and build optional parameters
        let filter_cloud = params.filter_cloud;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_descriptions/{tag_key}",
            local_configuration.get_operation_host(operation_id),
            tag_key = datadog::urlencode(tag_key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_cloud {
            local_req_builder =
                local_req_builder.query(&[("filter[cloud]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagDescriptionResponse>(
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
            let local_entity: Option<GetCostTagDescriptionByKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get details for a specific Cloud Cost Management tag key, including example tag values and description.
    pub async fn get_cost_tag_key(
        &self,
        tag_key: String,
        params: GetCostTagKeyOptionalParams,
    ) -> Result<crate::datadogV2::model::CostTagKeyResponse, datadog::Error<GetCostTagKeyError>>
    {
        match self.get_cost_tag_key_with_http_info(tag_key, params).await {
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

    /// Get details for a specific Cloud Cost Management tag key, including example tag values and description.
    pub async fn get_cost_tag_key_with_http_info(
        &self,
        tag_key: String,
        params: GetCostTagKeyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagKeyResponse>,
        datadog::Error<GetCostTagKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_tag_key";

        // unbox and build optional parameters
        let filter_metric = params.filter_metric;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_keys/{tag_key}",
            local_configuration.get_operation_host(operation_id),
            tag_key = datadog::urlencode(tag_key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_metric {
            local_req_builder =
                local_req_builder.query(&[("filter[metric]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagKeyResponse>(
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
            let local_entity: Option<GetCostTagKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the dominant billing currency observed in Cloud Cost Management data for the requested period. The response wraps the currency in a JSON:API `data` array containing at most one entry; the array is empty when no currency data is available.
    pub async fn get_cost_tag_metadata_currency(
        &self,
        filter_month: String,
        params: GetCostTagMetadataCurrencyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostCurrencyResponse,
        datadog::Error<GetCostTagMetadataCurrencyError>,
    > {
        match self
            .get_cost_tag_metadata_currency_with_http_info(filter_month, params)
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

    /// Get the dominant billing currency observed in Cloud Cost Management data for the requested period. The response wraps the currency in a JSON:API `data` array containing at most one entry; the array is empty when no currency data is available.
    pub async fn get_cost_tag_metadata_currency_with_http_info(
        &self,
        filter_month: String,
        params: GetCostTagMetadataCurrencyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostCurrencyResponse>,
        datadog::Error<GetCostTagMetadataCurrencyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_tag_metadata_currency";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_cost_tag_metadata_currency' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_provider = params.filter_provider;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata/currency",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[month]", &filter_month.to_string())]);
        if let Some(ref local_query_param) = filter_provider {
            local_req_builder =
                local_req_builder.query(&[("filter[provider]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostCurrencyResponse>(
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
            let local_entity: Option<GetCostTagMetadataCurrencyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific custom allocation rule - Retrieve a specific custom allocation rule by its ID
    pub async fn get_custom_allocation_rule(
        &self,
        rule_id: i64,
    ) -> Result<
        crate::datadogV2::model::ArbitraryRuleResponse,
        datadog::Error<GetCustomAllocationRuleError>,
    > {
        match self
            .get_custom_allocation_rule_with_http_info(rule_id)
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

    /// Get a specific custom allocation rule - Retrieve a specific custom allocation rule by its ID
    pub async fn get_custom_allocation_rule_with_http_info(
        &self,
        rule_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ArbitraryRuleResponse>,
        datadog::Error<GetCustomAllocationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_custom_allocation_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = rule_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::ArbitraryRuleResponse>(
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
            let local_entity: Option<GetCustomAllocationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Fetch the specified Custom Costs file.
    pub async fn get_custom_costs_file(
        &self,
        file_id: String,
    ) -> Result<
        crate::datadogV2::model::CustomCostsFileGetResponse,
        datadog::Error<GetCustomCostsFileError>,
    > {
        match self.get_custom_costs_file_with_http_info(file_id).await {
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

    /// Fetch the specified Custom Costs file.
    pub async fn get_custom_costs_file_with_http_info(
        &self,
        file_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CustomCostsFileGetResponse>,
        datadog::Error<GetCustomCostsFileError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_custom_costs_file";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/custom_costs/{file_id}",
            local_configuration.get_operation_host(operation_id),
            file_id = datadog::urlencode(file_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CustomCostsFileGetResponse>(
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
            let local_entity: Option<GetCustomCostsFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a specific tag pipeline ruleset - Retrieve a specific tag pipeline ruleset by its ID
    pub async fn get_tag_pipelines_ruleset(
        &self,
        ruleset_id: String,
    ) -> Result<crate::datadogV2::model::RulesetResp, datadog::Error<GetTagPipelinesRulesetError>>
    {
        match self
            .get_tag_pipelines_ruleset_with_http_info(ruleset_id)
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

    /// Get a specific tag pipeline ruleset - Retrieve a specific tag pipeline ruleset by its ID
    pub async fn get_tag_pipelines_ruleset_with_http_info(
        &self,
        ruleset_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesetResp>,
        datadog::Error<GetTagPipelinesRulesetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_tag_pipelines_ruleset";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/{ruleset_id}",
            local_configuration.get_operation_host(operation_id),
            ruleset_id = datadog::urlencode(ruleset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::RulesetResp>(&local_content) {
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
            let local_entity: Option<GetTagPipelinesRulesetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List budgets.
    pub async fn list_budgets(
        &self,
    ) -> Result<crate::datadogV2::model::BudgetArray, datadog::Error<ListBudgetsError>> {
        match self.list_budgets_with_http_info().await {
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

    /// List budgets.
    pub async fn list_budgets_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BudgetArray>,
        datadog::Error<ListBudgetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_budgets";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budgets",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::BudgetArray>(&local_content) {
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
            let local_entity: Option<ListBudgetsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the AWS CUR configs.
    pub async fn list_cost_awscur_configs(
        &self,
    ) -> Result<
        crate::datadogV2::model::AwsCURConfigsResponse,
        datadog::Error<ListCostAWSCURConfigsError>,
    > {
        match self.list_cost_awscur_configs_with_http_info().await {
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

    /// List the AWS CUR configs.
    pub async fn list_cost_awscur_configs_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AwsCURConfigsResponse>,
        datadog::Error<ListCostAWSCURConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_awscur_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/aws_cur_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::AwsCURConfigsResponse>(
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
            let local_entity: Option<ListCostAWSCURConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List detected Cloud Cost Management anomalies for the organization.
    pub async fn list_cost_anomalies(
        &self,
        params: ListCostAnomaliesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostAnomaliesResponse,
        datadog::Error<ListCostAnomaliesError>,
    > {
        match self.list_cost_anomalies_with_http_info(params).await {
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

    /// List detected Cloud Cost Management anomalies for the organization.
    pub async fn list_cost_anomalies_with_http_info(
        &self,
        params: ListCostAnomaliesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostAnomaliesResponse>,
        datadog::Error<ListCostAnomaliesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_anomalies";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_anomalies' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let start = params.start;
        let end = params.end;
        let filter = params.filter;
        let min_anomalous_threshold = params.min_anomalous_threshold;
        let min_cost_threshold = params.min_cost_threshold;
        let dismissal_cause = params.dismissal_cause;
        let order_by = params.order_by;
        let order = params.order;
        let limit = params.limit;
        let offset = params.offset;
        let provider_ids = params.provider_ids;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/anomalies",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end {
            local_req_builder = local_req_builder.query(&[("end", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = min_anomalous_threshold {
            local_req_builder = local_req_builder
                .query(&[("min_anomalous_threshold", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = min_cost_threshold {
            local_req_builder =
                local_req_builder.query(&[("min_cost_threshold", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = dismissal_cause {
            local_req_builder =
                local_req_builder.query(&[("dismissal_cause", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = order_by {
            local_req_builder =
                local_req_builder.query(&[("order_by", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = order {
            local_req_builder =
                local_req_builder.query(&[("order", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };
        if let Some(ref local) = provider_ids {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("provider_ids", &param.to_string())]);
            }
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostAnomaliesResponse>(
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
            let local_entity: Option<ListCostAnomaliesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the Azure configs.
    pub async fn list_cost_azure_uc_configs(
        &self,
    ) -> Result<
        crate::datadogV2::model::AzureUCConfigsResponse,
        datadog::Error<ListCostAzureUCConfigsError>,
    > {
        match self.list_cost_azure_uc_configs_with_http_info().await {
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

    /// List the Azure configs.
    pub async fn list_cost_azure_uc_configs_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AzureUCConfigsResponse>,
        datadog::Error<ListCostAzureUCConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_azure_uc_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/azure_uc_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::AzureUCConfigsResponse>(
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
            let local_entity: Option<ListCostAzureUCConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the Google Cloud Usage Cost configs.
    pub async fn list_cost_gcp_usage_cost_configs(
        &self,
    ) -> Result<
        crate::datadogV2::model::GCPUsageCostConfigsResponse,
        datadog::Error<ListCostGCPUsageCostConfigsError>,
    > {
        match self.list_cost_gcp_usage_cost_configs_with_http_info().await {
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

    /// List the Google Cloud Usage Cost configs.
    pub async fn list_cost_gcp_usage_cost_configs_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GCPUsageCostConfigsResponse>,
        datadog::Error<ListCostGCPUsageCostConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_gcp_usage_cost_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/gcp_uc_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::GCPUsageCostConfigsResponse>(
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
            let local_entity: Option<ListCostGCPUsageCostConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the OCI configs.
    pub async fn list_cost_oci_configs(
        &self,
    ) -> Result<crate::datadogV2::model::OCIConfigsResponse, datadog::Error<ListCostOCIConfigsError>>
    {
        match self.list_cost_oci_configs_with_http_info().await {
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

    /// List the OCI configs.
    pub async fn list_cost_oci_configs_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OCIConfigsResponse>,
        datadog::Error<ListCostOCIConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_oci_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/oci_config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::OCIConfigsResponse>(
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
            let local_entity: Option<ListCostOCIConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management tag key descriptions for the organization. Use `filter[cloud]` to scope the result to a single cloud provider; when omitted, both cross-cloud defaults and cloud-specific descriptions are returned.
    pub async fn list_cost_tag_descriptions(
        &self,
        params: ListCostTagDescriptionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostTagDescriptionsResponse,
        datadog::Error<ListCostTagDescriptionsError>,
    > {
        match self.list_cost_tag_descriptions_with_http_info(params).await {
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

    /// List Cloud Cost Management tag key descriptions for the organization. Use `filter[cloud]` to scope the result to a single cloud provider; when omitted, both cross-cloud defaults and cloud-specific descriptions are returned.
    pub async fn list_cost_tag_descriptions_with_http_info(
        &self,
        params: ListCostTagDescriptionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagDescriptionsResponse>,
        datadog::Error<ListCostTagDescriptionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_descriptions";

        // unbox and build optional parameters
        let filter_cloud = params.filter_cloud;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_descriptions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_cloud {
            local_req_builder =
                local_req_builder.query(&[("filter[cloud]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagDescriptionsResponse>(
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
            let local_entity: Option<ListCostTagDescriptionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management tag keys observed for the requested period, along with the origin sources that produced them (for example, `aws-user-defined`, `custom`).
    pub async fn list_cost_tag_key_sources(
        &self,
        filter_month: String,
        params: ListCostTagKeySourcesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostTagKeySourcesResponse,
        datadog::Error<ListCostTagKeySourcesError>,
    > {
        match self
            .list_cost_tag_key_sources_with_http_info(filter_month, params)
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

    /// List Cloud Cost Management tag keys observed for the requested period, along with the origin sources that produced them (for example, `aws-user-defined`, `custom`).
    pub async fn list_cost_tag_key_sources_with_http_info(
        &self,
        filter_month: String,
        params: ListCostTagKeySourcesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagKeySourcesResponse>,
        datadog::Error<ListCostTagKeySourcesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_key_sources";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_tag_key_sources' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_provider = params.filter_provider;
        let filter_metric = params.filter_metric;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata/tag_sources",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[month]", &filter_month.to_string())]);
        if let Some(ref local_query_param) = filter_provider {
            local_req_builder =
                local_req_builder.query(&[("filter[provider]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_metric {
            local_req_builder =
                local_req_builder.query(&[("filter[metric]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagKeySourcesResponse>(
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
            let local_entity: Option<ListCostTagKeySourcesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management tag keys.
    pub async fn list_cost_tag_keys(
        &self,
        params: ListCostTagKeysOptionalParams,
    ) -> Result<crate::datadogV2::model::CostTagKeysResponse, datadog::Error<ListCostTagKeysError>>
    {
        match self.list_cost_tag_keys_with_http_info(params).await {
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

    /// List Cloud Cost Management tag keys.
    pub async fn list_cost_tag_keys_with_http_info(
        &self,
        params: ListCostTagKeysOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagKeysResponse>,
        datadog::Error<ListCostTagKeysError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_keys";

        // unbox and build optional parameters
        let filter_metric = params.filter_metric;
        let filter_tags = params.filter_tags;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_metric {
            local_req_builder =
                local_req_builder.query(&[("filter[metric]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_tags {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[tags]", &param.to_string())]);
            }
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagKeysResponse>(
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
            let local_entity: Option<ListCostTagKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management tag key metadata, including row counts, cost covered, cardinality, and a sample of top tag values per cloud account. Use `filter[daily]=true` to return daily rows instead of the default monthly roll-up.
    pub async fn list_cost_tag_metadata(
        &self,
        filter_month: String,
        params: ListCostTagMetadataOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostTagKeyMetadataResponse,
        datadog::Error<ListCostTagMetadataError>,
    > {
        match self
            .list_cost_tag_metadata_with_http_info(filter_month, params)
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

    /// List Cloud Cost Management tag key metadata, including row counts, cost covered, cardinality, and a sample of top tag values per cloud account. Use `filter[daily]=true` to return daily rows instead of the default monthly roll-up.
    pub async fn list_cost_tag_metadata_with_http_info(
        &self,
        filter_month: String,
        params: ListCostTagMetadataOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagKeyMetadataResponse>,
        datadog::Error<ListCostTagMetadataError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_metadata";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_tag_metadata' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_provider = params.filter_provider;
        let filter_metric = params.filter_metric;
        let filter_tag_key = params.filter_tag_key;
        let filter_daily = params.filter_daily;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[month]", &filter_month.to_string())]);
        if let Some(ref local_query_param) = filter_provider {
            local_req_builder =
                local_req_builder.query(&[("filter[provider]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_metric {
            local_req_builder =
                local_req_builder.query(&[("filter[metric]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tag_key {
            local_req_builder =
                local_req_builder.query(&[("filter[tag_key]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_daily {
            local_req_builder =
                local_req_builder.query(&[("filter[daily]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagKeyMetadataResponse>(
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
            let local_entity: Option<ListCostTagMetadataError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management metrics that have data for the requested period.
    pub async fn list_cost_tag_metadata_metrics(
        &self,
        filter_month: String,
        params: ListCostTagMetadataMetricsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostMetricsResponse,
        datadog::Error<ListCostTagMetadataMetricsError>,
    > {
        match self
            .list_cost_tag_metadata_metrics_with_http_info(filter_month, params)
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

    /// List Cloud Cost Management metrics that have data for the requested period.
    pub async fn list_cost_tag_metadata_metrics_with_http_info(
        &self,
        filter_month: String,
        params: ListCostTagMetadataMetricsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostMetricsResponse>,
        datadog::Error<ListCostTagMetadataMetricsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_metadata_metrics";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_tag_metadata_metrics' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_provider = params.filter_provider;
        let filter_enabled_metrics_only = params.filter_enabled_metrics_only;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata/metrics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[month]", &filter_month.to_string())]);
        if let Some(ref local_query_param) = filter_provider {
            local_req_builder =
                local_req_builder.query(&[("filter[provider]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_enabled_metrics_only {
            local_req_builder = local_req_builder.query(&[(
                "filter[enabled_metrics_only]",
                &local_query_param.to_string(),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostMetricsResponse>(
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
            let local_entity: Option<ListCostTagMetadataMetricsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List months that have Cloud Cost Management tag metadata for a given provider,
    /// ordered most-recent first. The response is capped at 36 months.
    pub async fn list_cost_tag_metadata_months(
        &self,
        filter_provider: String,
    ) -> Result<
        crate::datadogV2::model::CostTagMetadataMonthsResponse,
        datadog::Error<ListCostTagMetadataMonthsError>,
    > {
        match self
            .list_cost_tag_metadata_months_with_http_info(filter_provider)
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

    /// List months that have Cloud Cost Management tag metadata for a given provider,
    /// ordered most-recent first. The response is capped at 36 months.
    pub async fn list_cost_tag_metadata_months_with_http_info(
        &self,
        filter_provider: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagMetadataMonthsResponse>,
        datadog::Error<ListCostTagMetadataMonthsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_metadata_months";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_tag_metadata_months' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata/months",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[provider]", &filter_provider.to_string())]);

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagMetadataMonthsResponse>(
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
            let local_entity: Option<ListCostTagMetadataMonthsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List container orchestrators (for example, `kubernetes`, `ecs`) detected in Cloud Cost Management data for the requested period.
    pub async fn list_cost_tag_metadata_orchestrators(
        &self,
        filter_month: String,
        params: ListCostTagMetadataOrchestratorsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostOrchestratorsResponse,
        datadog::Error<ListCostTagMetadataOrchestratorsError>,
    > {
        match self
            .list_cost_tag_metadata_orchestrators_with_http_info(filter_month, params)
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

    /// List container orchestrators (for example, `kubernetes`, `ecs`) detected in Cloud Cost Management data for the requested period.
    pub async fn list_cost_tag_metadata_orchestrators_with_http_info(
        &self,
        filter_month: String,
        params: ListCostTagMetadataOrchestratorsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostOrchestratorsResponse>,
        datadog::Error<ListCostTagMetadataOrchestratorsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tag_metadata_orchestrators";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_cost_tag_metadata_orchestrators' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_provider = params.filter_provider;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_metadata/orchestrators",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[month]", &filter_month.to_string())]);
        if let Some(ref local_query_param) = filter_provider {
            local_req_builder =
                local_req_builder.query(&[("filter[provider]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostOrchestratorsResponse>(
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
            let local_entity: Option<ListCostTagMetadataOrchestratorsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Cloud Cost Management tags for a given metric.
    pub async fn list_cost_tags(
        &self,
        params: ListCostTagsOptionalParams,
    ) -> Result<crate::datadogV2::model::CostTagsResponse, datadog::Error<ListCostTagsError>> {
        match self.list_cost_tags_with_http_info(params).await {
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

    /// List Cloud Cost Management tags for a given metric.
    pub async fn list_cost_tags_with_http_info(
        &self,
        params: ListCostTagsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostTagsResponse>,
        datadog::Error<ListCostTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cost_tags";

        // unbox and build optional parameters
        let filter_metric = params.filter_metric;
        let filter_match = params.filter_match;
        let filter_tags = params.filter_tags;
        let filter_tag_keys = params.filter_tag_keys;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tags",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_metric {
            local_req_builder =
                local_req_builder.query(&[("filter[metric]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_match {
            local_req_builder =
                local_req_builder.query(&[("filter[match]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_tags {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[tags]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_tag_keys {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[tag_keys]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CostTagsResponse>(&local_content)
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
            let local_entity: Option<ListCostTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all custom allocation rules - Retrieve a list of all custom allocation rules for the organization
    pub async fn list_custom_allocation_rules(
        &self,
    ) -> Result<
        crate::datadogV2::model::ArbitraryRuleResponseArray,
        datadog::Error<ListCustomAllocationRulesError>,
    > {
        match self.list_custom_allocation_rules_with_http_info().await {
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

    /// List all custom allocation rules - Retrieve a list of all custom allocation rules for the organization
    pub async fn list_custom_allocation_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ArbitraryRuleResponseArray>,
        datadog::Error<ListCustomAllocationRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_custom_allocation_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::ArbitraryRuleResponseArray>(
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
            let local_entity: Option<ListCustomAllocationRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the processing status of all custom allocation rules. Returns only the ID and processing status for each rule.
    pub async fn list_custom_allocation_rules_status(
        &self,
    ) -> Result<
        crate::datadogV2::model::ArbitraryRuleStatusResponseArray,
        datadog::Error<ListCustomAllocationRulesStatusError>,
    > {
        match self
            .list_custom_allocation_rules_status_with_http_info()
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

    /// List the processing status of all custom allocation rules. Returns only the ID and processing status for each rule.
    pub async fn list_custom_allocation_rules_status_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ArbitraryRuleStatusResponseArray>,
        datadog::Error<ListCustomAllocationRulesStatusError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_custom_allocation_rules_status";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule/status",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::ArbitraryRuleStatusResponseArray>(
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
            let local_entity: Option<ListCustomAllocationRulesStatusError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the Custom Costs files.
    pub async fn list_custom_costs_files(
        &self,
        params: ListCustomCostsFilesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CustomCostsFileListResponse,
        datadog::Error<ListCustomCostsFilesError>,
    > {
        match self.list_custom_costs_files_with_http_info(params).await {
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

    /// List the Custom Costs files.
    pub async fn list_custom_costs_files_with_http_info(
        &self,
        params: ListCustomCostsFilesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CustomCostsFileListResponse>,
        datadog::Error<ListCustomCostsFilesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_custom_costs_files";

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let filter_status = params.filter_status;
        let filter_name = params.filter_name;
        let filter_provider = params.filter_provider;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/custom_costs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_provider {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[provider]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::CustomCostsFileListResponse>(
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
            let local_entity: Option<ListCustomCostsFilesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all tag pipeline rulesets - Retrieve a list of all tag pipeline rulesets for the organization
    pub async fn list_tag_pipelines_rulesets(
        &self,
    ) -> Result<
        crate::datadogV2::model::RulesetRespArray,
        datadog::Error<ListTagPipelinesRulesetsError>,
    > {
        match self.list_tag_pipelines_rulesets_with_http_info().await {
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

    /// List all tag pipeline rulesets - Retrieve a list of all tag pipeline rulesets for the organization
    pub async fn list_tag_pipelines_rulesets_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesetRespArray>,
        datadog::Error<ListTagPipelinesRulesetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_tag_pipelines_rulesets";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::RulesetRespArray>(&local_content)
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
            let local_entity: Option<ListTagPipelinesRulesetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the processing status of all tag pipeline rulesets. Returns only the ID and processing status for each ruleset.
    pub async fn list_tag_pipelines_rulesets_status(
        &self,
    ) -> Result<
        crate::datadogV2::model::RulesetStatusRespArray,
        datadog::Error<ListTagPipelinesRulesetsStatusError>,
    > {
        match self
            .list_tag_pipelines_rulesets_status_with_http_info()
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

    /// List the processing status of all tag pipeline rulesets. Returns only the ID and processing status for each ruleset.
    pub async fn list_tag_pipelines_rulesets_status_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesetStatusRespArray>,
        datadog::Error<ListTagPipelinesRulesetsStatusError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_tag_pipelines_rulesets_status";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/status",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            match serde_json::from_str::<crate::datadogV2::model::RulesetStatusRespArray>(
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
            let local_entity: Option<ListTagPipelinesRulesetsStatusError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Reorder custom allocation rules - Change the execution order of custom allocation rules.
    ///
    /// **Important**: You must provide the **complete list** of all rule IDs in the desired execution order. The API will reorder ALL rules according to the provided sequence.
    ///
    /// Rules are executed in the order specified, with lower indices (earlier in the array) having higher priority.
    ///
    /// **Example**: If you have rules with IDs [123, 456, 789] and want to change order from 123→456→789 to 456→123→789, send: [{"id": "456"}, {"id": "123"}, {"id": "789"}]
    pub async fn reorder_custom_allocation_rules(
        &self,
        body: crate::datadogV2::model::ReorderRuleResourceArray,
    ) -> Result<(), datadog::Error<ReorderCustomAllocationRulesError>> {
        match self
            .reorder_custom_allocation_rules_with_http_info(body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Reorder custom allocation rules - Change the execution order of custom allocation rules.
    ///
    /// **Important**: You must provide the **complete list** of all rule IDs in the desired execution order. The API will reorder ALL rules according to the provided sequence.
    ///
    /// Rules are executed in the order specified, with lower indices (earlier in the array) having higher priority.
    ///
    /// **Example**: If you have rules with IDs [123, 456, 789] and want to change order from 123→456→789 to 456→123→789, send: [{"id": "456"}, {"id": "123"}, {"id": "789"}]
    pub async fn reorder_custom_allocation_rules_with_http_info(
        &self,
        body: crate::datadogV2::model::ReorderRuleResourceArray,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ReorderCustomAllocationRulesError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_custom_allocation_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule/reorder",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<ReorderCustomAllocationRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Reorder tag pipeline rulesets - Change the execution order of tag pipeline rulesets
    pub async fn reorder_tag_pipelines_rulesets(
        &self,
        body: crate::datadogV2::model::ReorderRulesetResourceArray,
    ) -> Result<(), datadog::Error<ReorderTagPipelinesRulesetsError>> {
        match self
            .reorder_tag_pipelines_rulesets_with_http_info(body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Reorder tag pipeline rulesets - Change the execution order of tag pipeline rulesets
    pub async fn reorder_tag_pipelines_rulesets_with_http_info(
        &self,
        body: crate::datadogV2::model::ReorderRulesetResourceArray,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ReorderTagPipelinesRulesetsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_tag_pipelines_rulesets";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/reorder",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<ReorderTagPipelinesRulesetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List cost recommendations matching a filter, with pagination and sorting.
    pub async fn search_cost_recommendations(
        &self,
        body: crate::datadogV2::model::RecommendationsFilterRequest,
        params: SearchCostRecommendationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostRecommendationArray,
        datadog::Error<SearchCostRecommendationsError>,
    > {
        match self
            .search_cost_recommendations_with_http_info(body, params)
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

    /// List cost recommendations matching a filter, with pagination and sorting.
    pub async fn search_cost_recommendations_with_http_info(
        &self,
        body: crate::datadogV2::model::RecommendationsFilterRequest,
        params: SearchCostRecommendationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostRecommendationArray>,
        datadog::Error<SearchCostRecommendationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_cost_recommendations";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.search_cost_recommendations' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_token = params.page_token;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/recommendations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_token {
            local_req_builder =
                local_req_builder.query(&[("page[token]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CostRecommendationArray>(
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
            let local_entity: Option<SearchCostRecommendationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the status (active/archived) and/or account filtering configuration of an AWS CUR config.
    pub async fn update_cost_awscur_config(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::AwsCURConfigPatchRequest,
    ) -> Result<
        crate::datadogV2::model::AwsCURConfigsResponse,
        datadog::Error<UpdateCostAWSCURConfigError>,
    > {
        match self
            .update_cost_awscur_config_with_http_info(cloud_account_id, body)
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

    /// Update the status (active/archived) and/or account filtering configuration of an AWS CUR config.
    pub async fn update_cost_awscur_config_with_http_info(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::AwsCURConfigPatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AwsCURConfigsResponse>,
        datadog::Error<UpdateCostAWSCURConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_cost_awscur_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/aws_cur_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AwsCURConfigsResponse>(
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
            let local_entity: Option<UpdateCostAWSCURConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the status of an  Azure config (active/archived).
    pub async fn update_cost_azure_uc_configs(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::AzureUCConfigPatchRequest,
    ) -> Result<
        crate::datadogV2::model::AzureUCConfigPairsResponse,
        datadog::Error<UpdateCostAzureUCConfigsError>,
    > {
        match self
            .update_cost_azure_uc_configs_with_http_info(cloud_account_id, body)
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

    /// Update the status of an  Azure config (active/archived).
    pub async fn update_cost_azure_uc_configs_with_http_info(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::AzureUCConfigPatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AzureUCConfigPairsResponse>,
        datadog::Error<UpdateCostAzureUCConfigsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_cost_azure_uc_configs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/azure_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AzureUCConfigPairsResponse>(
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
            let local_entity: Option<UpdateCostAzureUCConfigsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the status of an Google Cloud Usage Cost config (active/archived).
    pub async fn update_cost_gcp_usage_cost_config(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::GCPUsageCostConfigPatchRequest,
    ) -> Result<
        crate::datadogV2::model::GCPUsageCostConfigResponse,
        datadog::Error<UpdateCostGCPUsageCostConfigError>,
    > {
        match self
            .update_cost_gcp_usage_cost_config_with_http_info(cloud_account_id, body)
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

    /// Update the status of an Google Cloud Usage Cost config (active/archived).
    pub async fn update_cost_gcp_usage_cost_config_with_http_info(
        &self,
        cloud_account_id: i64,
        body: crate::datadogV2::model::GCPUsageCostConfigPatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GCPUsageCostConfigResponse>,
        datadog::Error<UpdateCostGCPUsageCostConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_cost_gcp_usage_cost_config";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/gcp_uc_config/{cloud_account_id}",
            local_configuration.get_operation_host(operation_id),
            cloud_account_id = cloud_account_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GCPUsageCostConfigResponse>(
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
            let local_entity: Option<UpdateCostGCPUsageCostConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an existing custom allocation rule with new filters and allocation strategy.
    ///
    /// **Strategy Methods:**
    /// - **PROPORTIONAL/EVEN**: Allocates costs proportionally/evenly based on existing costs. Requires: granularity, allocated_by_tag_keys. Optional: based_on_costs, allocated_by_filters, evaluate_grouped_by_tag_keys, evaluate_grouped_by_filters.
    /// - **PROPORTIONAL_TIMESERIES/EVEN_TIMESERIES**: Allocates based on timeseries data. Requires: granularity, based_on_timeseries. Optional: evaluate_grouped_by_tag_keys.
    /// - **PERCENT**: Allocates fixed percentages to specific tags. Requires: allocated_by (array of percentage allocations).
    /// - **USAGE_METRIC**: Allocates based on usage metrics (implementation varies).
    ///
    /// **Filter Conditions:**
    /// - Use **value** for single-value conditions: "is", "is not", "contains", "=", "!=", "like", "not like"
    /// - Use **values** for multi-value conditions: "in", "not in"
    /// - Cannot use both value and values simultaneously.
    ///
    /// **Supported operators**: is, is not, contains, in, not in, =, !=, like, not like
    pub async fn update_custom_allocation_rule(
        &self,
        rule_id: i64,
        body: crate::datadogV2::model::ArbitraryCostUpsertRequest,
    ) -> Result<
        crate::datadogV2::model::ArbitraryRuleResponse,
        datadog::Error<UpdateCustomAllocationRuleError>,
    > {
        match self
            .update_custom_allocation_rule_with_http_info(rule_id, body)
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

    /// Update an existing custom allocation rule with new filters and allocation strategy.
    ///
    /// **Strategy Methods:**
    /// - **PROPORTIONAL/EVEN**: Allocates costs proportionally/evenly based on existing costs. Requires: granularity, allocated_by_tag_keys. Optional: based_on_costs, allocated_by_filters, evaluate_grouped_by_tag_keys, evaluate_grouped_by_filters.
    /// - **PROPORTIONAL_TIMESERIES/EVEN_TIMESERIES**: Allocates based on timeseries data. Requires: granularity, based_on_timeseries. Optional: evaluate_grouped_by_tag_keys.
    /// - **PERCENT**: Allocates fixed percentages to specific tags. Requires: allocated_by (array of percentage allocations).
    /// - **USAGE_METRIC**: Allocates based on usage metrics (implementation varies).
    ///
    /// **Filter Conditions:**
    /// - Use **value** for single-value conditions: "is", "is not", "contains", "=", "!=", "like", "not like"
    /// - Use **values** for multi-value conditions: "in", "not in"
    /// - Cannot use both value and values simultaneously.
    ///
    /// **Supported operators**: is, is not, contains, in, not in, =, !=, like, not like
    pub async fn update_custom_allocation_rule_with_http_info(
        &self,
        rule_id: i64,
        body: crate::datadogV2::model::ArbitraryCostUpsertRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ArbitraryRuleResponse>,
        datadog::Error<UpdateCustomAllocationRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_custom_allocation_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/arbitrary_rule/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = rule_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ArbitraryRuleResponse>(
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
            let local_entity: Option<UpdateCustomAllocationRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a tag pipeline ruleset - Update an existing tag pipeline ruleset with new rules and configuration
    pub async fn update_tag_pipelines_ruleset(
        &self,
        ruleset_id: String,
        body: crate::datadogV2::model::UpdateRulesetRequest,
    ) -> Result<crate::datadogV2::model::RulesetResp, datadog::Error<UpdateTagPipelinesRulesetError>>
    {
        match self
            .update_tag_pipelines_ruleset_with_http_info(ruleset_id, body)
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

    /// Update a tag pipeline ruleset - Update an existing tag pipeline ruleset with new rules and configuration
    pub async fn update_tag_pipelines_ruleset_with_http_info(
        &self,
        ruleset_id: String,
        body: crate::datadogV2::model::UpdateRulesetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesetResp>,
        datadog::Error<UpdateTagPipelinesRulesetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_tag_pipelines_ruleset";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/{ruleset_id}",
            local_configuration.get_operation_host(operation_id),
            ruleset_id = datadog::urlencode(ruleset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::RulesetResp>(&local_content) {
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
            let local_entity: Option<UpdateTagPipelinesRulesetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Upload a Custom Costs file.
    pub async fn upload_custom_costs_file(
        &self,
        body: Vec<crate::datadogV2::model::CustomCostsFileLineItem>,
    ) -> Result<
        crate::datadogV2::model::CustomCostsFileUploadResponse,
        datadog::Error<UploadCustomCostsFileError>,
    > {
        match self.upload_custom_costs_file_with_http_info(body).await {
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

    /// Upload a Custom Costs file.
    pub async fn upload_custom_costs_file_with_http_info(
        &self,
        body: Vec<crate::datadogV2::model::CustomCostsFileLineItem>,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CustomCostsFileUploadResponse>,
        datadog::Error<UploadCustomCostsFileError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.upload_custom_costs_file";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/custom_costs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CustomCostsFileUploadResponse>(
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
            let local_entity: Option<UploadCustomCostsFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new budget or update an existing one.
    pub async fn upsert_budget(
        &self,
        body: crate::datadogV2::model::BudgetWithEntries,
    ) -> Result<crate::datadogV2::model::BudgetWithEntries, datadog::Error<UpsertBudgetError>> {
        match self.upsert_budget_with_http_info(body).await {
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

    /// Create a new budget or update an existing one.
    pub async fn upsert_budget_with_http_info(
        &self,
        body: crate::datadogV2::model::BudgetWithEntries,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BudgetWithEntries>,
        datadog::Error<UpsertBudgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.upsert_budget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::BudgetWithEntries>(&local_content)
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
            let local_entity: Option<UpsertBudgetError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or update a Cloud Cost Management tag key description. The new description and optional cloud scoping are supplied in the request body. Omit `cloud` to set a cross-cloud default for the tag key.
    pub async fn upsert_cost_tag_description_by_key(
        &self,
        tag_key: String,
        body: crate::datadogV2::model::CostTagDescriptionUpsertRequest,
    ) -> Result<(), datadog::Error<UpsertCostTagDescriptionByKeyError>> {
        match self
            .upsert_cost_tag_description_by_key_with_http_info(tag_key, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Create or update a Cloud Cost Management tag key description. The new description and optional cloud scoping are supplied in the request body. Omit `cloud` to set a cross-cloud default for the tag key.
    pub async fn upsert_cost_tag_description_by_key_with_http_info(
        &self,
        tag_key: String,
        body: crate::datadogV2::model::CostTagDescriptionUpsertRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpsertCostTagDescriptionByKeyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.upsert_cost_tag_description_by_key";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/tag_descriptions/{tag_key}",
            local_configuration.get_operation_host(operation_id),
            tag_key = datadog::urlencode(tag_key)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpsertCostTagDescriptionByKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or replace the custom forecast for an existing budget.
    /// Pass an empty `entries` list to delete the custom forecast for the budget.
    pub async fn upsert_custom_forecast(
        &self,
        body: crate::datadogV2::model::CustomForecastUpsertRequest,
    ) -> Result<
        crate::datadogV2::model::CustomForecastResponse,
        datadog::Error<UpsertCustomForecastError>,
    > {
        match self.upsert_custom_forecast_with_http_info(body).await {
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

    /// Create or replace the custom forecast for an existing budget.
    /// Pass an empty `entries` list to delete the custom forecast for the budget.
    pub async fn upsert_custom_forecast_with_http_info(
        &self,
        body: crate::datadogV2::model::CustomForecastUpsertRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CustomForecastResponse>,
        datadog::Error<UpsertCustomForecastError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.upsert_custom_forecast";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.upsert_custom_forecast' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/custom-forecast",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CustomForecastResponse>(
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
            let local_entity: Option<UpsertCustomForecastError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Validate a budget configuration without creating or modifying it
    pub async fn validate_budget(
        &self,
        body: crate::datadogV2::model::BudgetValidationRequest,
    ) -> Result<
        crate::datadogV2::model::BudgetValidationResponse,
        datadog::Error<ValidateBudgetError>,
    > {
        match self.validate_budget_with_http_info(body).await {
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

    /// Validate a budget configuration without creating or modifying it
    pub async fn validate_budget_with_http_info(
        &self,
        body: crate::datadogV2::model::BudgetValidationRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BudgetValidationResponse>,
        datadog::Error<ValidateBudgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.validate_budget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/validate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::BudgetValidationResponse>(
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
            let local_entity: Option<ValidateBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    pub async fn validate_csv_budget(
        &self,
    ) -> Result<crate::datadogV2::model::ValidationResponse, datadog::Error<ValidateCsvBudgetError>>
    {
        match self.validate_csv_budget_with_http_info().await {
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

    pub async fn validate_csv_budget_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ValidationResponse>,
        datadog::Error<ValidateCsvBudgetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.validate_csv_budget";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost/budget/csv/validate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ValidationResponse>(
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
            let local_entity: Option<ValidateCsvBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Validate a tag pipeline query - Validate the syntax and structure of a tag pipeline query
    pub async fn validate_query(
        &self,
        body: crate::datadogV2::model::RulesValidateQueryRequest,
    ) -> Result<
        crate::datadogV2::model::RulesValidateQueryResponse,
        datadog::Error<ValidateQueryError>,
    > {
        match self.validate_query_with_http_info(body).await {
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

    /// Validate a tag pipeline query - Validate the syntax and structure of a tag pipeline query
    pub async fn validate_query_with_http_info(
        &self,
        body: crate::datadogV2::model::RulesValidateQueryRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RulesValidateQueryResponse>,
        datadog::Error<ValidateQueryError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.validate_query";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/tags/enrichment/validate-query",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::RulesValidateQueryResponse>(
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
            let local_entity: Option<ValidateQueryError> =
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
