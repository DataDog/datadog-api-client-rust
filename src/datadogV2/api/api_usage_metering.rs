// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetBillingDimensionMappingOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_billing_dimension_mapping`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetBillingDimensionMappingOptionalParams {
    /// Datetime in ISO-8601 format, UTC, and for mappings beginning this month. Defaults to the current month.
    pub filter_month: Option<chrono::DateTime<chrono::Utc>>,
    /// String to specify whether to retrieve active billing dimension mappings for the contract or for all available mappings. Allowed views have the string `active` or `all`. Defaults to `active`.
    pub filter_view: Option<String>,
}

impl GetBillingDimensionMappingOptionalParams {
    /// Datetime in ISO-8601 format, UTC, and for mappings beginning this month. Defaults to the current month.
    pub fn filter_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_month = Some(value);
        self
    }
    /// String to specify whether to retrieve active billing dimension mappings for the contract or for all available mappings. Allowed views have the string `active` or `all`. Defaults to `active`.
    pub fn filter_view(mut self, value: String) -> Self {
        self.filter_view = Some(value);
        self
    }
}

/// GetCostByOrgOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_cost_by_org`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCostByOrgOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetCostByOrgOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
}

/// GetEstimatedCostByOrgOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_estimated_cost_by_org`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetEstimatedCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub view: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. **Either start_month or start_date should be specified, but not both.** (start_month cannot go beyond two months in the past). Provide an `end_month` to view month-over-month cost.
    pub start_month: Option<chrono::DateTime<chrono::Utc>>,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost beginning this day. **Either start_month or start_date should be specified, but not both.** (start_date cannot go beyond two months in the past). Provide an `end_date` to view day-over-day cumulative cost.
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost ending this day.
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub include_connected_accounts: Option<bool>,
}

impl GetEstimatedCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. **Either start_month or start_date should be specified, but not both.** (start_month cannot go beyond two months in the past). Provide an `end_month` to view month-over-month cost.
    pub fn start_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost beginning this day. **Either start_month or start_date should be specified, but not both.** (start_date cannot go beyond two months in the past). Provide an `end_date` to view day-over-day cumulative cost.
    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost ending this day.
    pub fn end_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub fn include_connected_accounts(mut self, value: bool) -> Self {
        self.include_connected_accounts = Some(value);
        self
    }
}

/// GetHistoricalCostByOrgOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_historical_cost_by_org`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHistoricalCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`.  Defaults to `summary`.
    pub view: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub include_connected_accounts: Option<bool>,
}

impl GetHistoricalCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`.  Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub fn include_connected_accounts(mut self, value: bool) -> Self {
        self.include_connected_accounts = Some(value);
        self
    }
}

/// GetHourlyUsageOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_hourly_usage`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHourlyUsageOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub filter_timestamp_end: Option<chrono::DateTime<chrono::Utc>>,
    /// Include child org usage in the response. Defaults to false.
    pub filter_include_descendants: Option<bool>,
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to false.
    pub filter_include_connected_accounts: Option<bool>,
    /// Include breakdown of usage by subcategories where applicable (for product family logs only). Defaults to false.
    pub filter_include_breakdown: Option<bool>,
    /// Comma separated list of product family versions to use in the format `product_family:version`. For example,
    /// `infra_hosts:1.0.0`. If this parameter is not used, the API will use the latest version of each requested
    /// product family. Currently all families have one version `1.0.0`.
    pub filter_versions: Option<String>,
    /// Maximum number of results to return (between 1 and 500) - defaults to 500 if limit not specified.
    pub page_limit: Option<i32>,
    /// List following results with a next_record_id provided in the previous query.
    pub page_next_record_id: Option<String>,
}

impl GetHourlyUsageOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub fn filter_timestamp_end(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.filter_timestamp_end = Some(value);
        self
    }
    /// Include child org usage in the response. Defaults to false.
    pub fn filter_include_descendants(mut self, value: bool) -> Self {
        self.filter_include_descendants = Some(value);
        self
    }
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to false.
    pub fn filter_include_connected_accounts(mut self, value: bool) -> Self {
        self.filter_include_connected_accounts = Some(value);
        self
    }
    /// Include breakdown of usage by subcategories where applicable (for product family logs only). Defaults to false.
    pub fn filter_include_breakdown(mut self, value: bool) -> Self {
        self.filter_include_breakdown = Some(value);
        self
    }
    /// Comma separated list of product family versions to use in the format `product_family:version`. For example,
    /// `infra_hosts:1.0.0`. If this parameter is not used, the API will use the latest version of each requested
    /// product family. Currently all families have one version `1.0.0`.
    pub fn filter_versions(mut self, value: String) -> Self {
        self.filter_versions = Some(value);
        self
    }
    /// Maximum number of results to return (between 1 and 500) - defaults to 500 if limit not specified.
    pub fn page_limit(mut self, value: i32) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// List following results with a next_record_id provided in the previous query.
    pub fn page_next_record_id(mut self, value: String) -> Self {
        self.page_next_record_id = Some(value);
        self
    }
}

/// GetMonthlyCostAttributionOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_monthly_cost_attribution`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMonthlyCostAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<chrono::DateTime<chrono::Utc>>,
    /// The direction to sort by: `[desc, asc]`.
    pub sort_direction: Option<crate::datadogV2::model::SortDirection>,
    /// The billing dimension to sort by. Always sorted by total cost. Example: `infra_host`.
    pub sort_name: Option<String>,
    /// Comma separated list of tag keys used to group cost. If no value is provided the cost will not be broken down by tags.
    /// To see which tags are available, look for the value of `tag_config_source` in the API response.
    pub tag_breakdown_keys: Option<String>,
    /// List following results with a next_record_id provided in the previous query.
    pub next_record_id: Option<String>,
    /// Include child org cost in the response. Defaults to `true`.
    pub include_descendants: Option<bool>,
}

impl GetMonthlyCostAttributionOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_month = Some(value);
        self
    }
    /// The direction to sort by: `[desc, asc]`.
    pub fn sort_direction(mut self, value: crate::datadogV2::model::SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }
    /// The billing dimension to sort by. Always sorted by total cost. Example: `infra_host`.
    pub fn sort_name(mut self, value: String) -> Self {
        self.sort_name = Some(value);
        self
    }
    /// Comma separated list of tag keys used to group cost. If no value is provided the cost will not be broken down by tags.
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
    /// Include child org cost in the response. Defaults to `true`.
    pub fn include_descendants(mut self, value: bool) -> Self {
        self.include_descendants = Some(value);
        self
    }
}

/// GetProjectedCostOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_projected_cost`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetProjectedCostOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub view: Option<String>,
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub include_connected_accounts: Option<bool>,
}

impl GetProjectedCostOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
    /// Boolean to specify whether to include accounts connected to the current account as partner customers in the Datadog partner network program. Defaults to `false`.
    pub fn include_connected_accounts(mut self, value: bool) -> Self {
        self.include_connected_accounts = Some(value);
        self
    }
}

/// GetUsageApplicationSecurityMonitoringOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_application_security_monitoring`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageApplicationSecurityMonitoringOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageApplicationSecurityMonitoringOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageLambdaTracedInvocationsOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_lambda_traced_invocations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageLambdaTracedInvocationsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageLambdaTracedInvocationsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetUsageObservabilityPipelinesOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_observability_pipelines`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageObservabilityPipelinesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<chrono::DateTime<chrono::Utc>>,
}

impl GetUsageObservabilityPipelinesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetActiveBillingDimensionsError is a struct for typed errors of method [`UsageMeteringAPI::get_active_billing_dimensions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveBillingDimensionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBillingDimensionMappingError is a struct for typed errors of method [`UsageMeteringAPI::get_billing_dimension_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBillingDimensionMappingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostByOrgError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetEstimatedCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_estimated_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEstimatedCostByOrgError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHistoricalCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_historical_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoricalCostByOrgError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageError is a struct for typed errors of method [`UsageMeteringAPI::get_hourly_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCostAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_cost_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCostAttributionError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetProjectedCostError is a struct for typed errors of method [`UsageMeteringAPI::get_projected_cost`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectedCostError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageApplicationSecurityMonitoringError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_application_security_monitoring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageApplicationSecurityMonitoringError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaTracedInvocationsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_lambda_traced_invocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaTracedInvocationsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUsageObservabilityPipelinesError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_observability_pipelines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageObservabilityPipelinesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
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

    /// Get active billing dimensions for cost attribution. Cost data for a given month becomes available no later than the 19th of the following month.
    pub async fn get_active_billing_dimensions(
        &self,
    ) -> Result<
        crate::datadogV2::model::ActiveBillingDimensionsResponse,
        datadog::Error<GetActiveBillingDimensionsError>,
    > {
        match self.get_active_billing_dimensions_with_http_info().await {
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

    /// Get active billing dimensions for cost attribution. Cost data for a given month becomes available no later than the 19th of the following month.
    pub async fn get_active_billing_dimensions_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ActiveBillingDimensionsResponse>,
        datadog::Error<GetActiveBillingDimensionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_active_billing_dimensions";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost_by_tag/active_billing_dimensions",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ActiveBillingDimensionsResponse>(
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
            let local_entity: Option<GetActiveBillingDimensionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a mapping of billing dimensions to the corresponding keys for the supported usage metering public API endpoints.
    /// Mapping data is updated on a monthly cadence.
    ///
    /// This endpoint is only accessible to [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_billing_dimension_mapping(
        &self,
        params: GetBillingDimensionMappingOptionalParams,
    ) -> Result<
        crate::datadogV2::model::BillingDimensionsMappingResponse,
        datadog::Error<GetBillingDimensionMappingError>,
    > {
        match self
            .get_billing_dimension_mapping_with_http_info(params)
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

    /// Get a mapping of billing dimensions to the corresponding keys for the supported usage metering public API endpoints.
    /// Mapping data is updated on a monthly cadence.
    ///
    /// This endpoint is only accessible to [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_billing_dimension_mapping_with_http_info(
        &self,
        params: GetBillingDimensionMappingOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::BillingDimensionsMappingResponse>,
        datadog::Error<GetBillingDimensionMappingError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_billing_dimension_mapping";

        // unbox and build optional parameters
        let filter_month = params.filter_month;
        let filter_view = params.filter_view;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/billing_dimension_mapping",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_month {
            local_req_builder = local_req_builder.query(&[(
                "filter[month]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = filter_view {
            local_req_builder =
                local_req_builder.query(&[("filter[view]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::BillingDimensionsMappingResponse>(
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
            let local_entity: Option<GetBillingDimensionMappingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](<https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account>)
    /// instead.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_cost_by_org(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetCostByOrgOptionalParams,
    ) -> Result<crate::datadogV2::model::CostByOrgResponse, datadog::Error<GetCostByOrgError>> {
        match self
            .get_cost_by_org_with_http_info(start_month, params)
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

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](<https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account>)
    /// instead.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_cost_by_org_with_http_info(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetCostByOrgOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        datadog::Error<GetCostByOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_by_org";

        // unbox and build optional parameters
        let end_month = params.end_month;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/cost_by_org",
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetCostByOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get estimated cost across multi-org and single root-org accounts.
    /// Estimated cost data is only available for the current month and previous month
    /// and is delayed by up to 72 hours from when it was incurred.
    /// To access historical costs prior to this, use the `/historical_cost` endpoint.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_estimated_cost_by_org(
        &self,
        params: GetEstimatedCostByOrgOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostByOrgResponse,
        datadog::Error<GetEstimatedCostByOrgError>,
    > {
        match self.get_estimated_cost_by_org_with_http_info(params).await {
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

    /// Get estimated cost across multi-org and single root-org accounts.
    /// Estimated cost data is only available for the current month and previous month
    /// and is delayed by up to 72 hours from when it was incurred.
    /// To access historical costs prior to this, use the `/historical_cost` endpoint.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_estimated_cost_by_org_with_http_info(
        &self,
        params: GetEstimatedCostByOrgOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        datadog::Error<GetEstimatedCostByOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_estimated_cost_by_org";

        // unbox and build optional parameters
        let view = params.view;
        let start_month = params.start_month;
        let end_month = params.end_month;
        let start_date = params.start_date;
        let end_date = params.end_date;
        let include_connected_accounts = params.include_connected_accounts;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/estimated_cost",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = view {
            local_req_builder =
                local_req_builder.query(&[("view", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start_month {
            local_req_builder = local_req_builder.query(&[(
                "start_month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = end_month {
            local_req_builder = local_req_builder.query(&[(
                "end_month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = start_date {
            local_req_builder = local_req_builder.query(&[(
                "start_date",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = end_date {
            local_req_builder = local_req_builder.query(&[(
                "end_date",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = include_connected_accounts {
            local_req_builder = local_req_builder
                .query(&[("include_connected_accounts", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetEstimatedCostByOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get historical cost across multi-org and single root-org accounts.
    /// Cost data for a given month becomes available no later than the 16th of the following month.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_historical_cost_by_org(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetHistoricalCostByOrgOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CostByOrgResponse,
        datadog::Error<GetHistoricalCostByOrgError>,
    > {
        match self
            .get_historical_cost_by_org_with_http_info(start_month, params)
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

    /// Get historical cost across multi-org and single root-org accounts.
    /// Cost data for a given month becomes available no later than the 16th of the following month.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_historical_cost_by_org_with_http_info(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        params: GetHistoricalCostByOrgOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        datadog::Error<GetHistoricalCostByOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_historical_cost_by_org";

        // unbox and build optional parameters
        let view = params.view;
        let end_month = params.end_month;
        let include_connected_accounts = params.include_connected_accounts;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/historical_cost",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "start_month",
            &start_month.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        if let Some(ref local_query_param) = view {
            local_req_builder =
                local_req_builder.query(&[("view", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end_month {
            local_req_builder = local_req_builder.query(&[(
                "end_month",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = include_connected_accounts {
            local_req_builder = local_req_builder
                .query(&[("include_connected_accounts", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetHistoricalCostByOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage by product family.
    pub async fn get_hourly_usage(
        &self,
        filter_timestamp_start: chrono::DateTime<chrono::Utc>,
        filter_product_families: String,
        params: GetHourlyUsageOptionalParams,
    ) -> Result<crate::datadogV2::model::HourlyUsageResponse, datadog::Error<GetHourlyUsageError>>
    {
        match self
            .get_hourly_usage_with_http_info(
                filter_timestamp_start,
                filter_product_families,
                params,
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

    /// Get hourly usage by product family.
    pub async fn get_hourly_usage_with_http_info(
        &self,
        filter_timestamp_start: chrono::DateTime<chrono::Utc>,
        filter_product_families: String,
        params: GetHourlyUsageOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::HourlyUsageResponse>,
        datadog::Error<GetHourlyUsageError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_hourly_usage";

        // unbox and build optional parameters
        let filter_timestamp_end = params.filter_timestamp_end;
        let filter_include_descendants = params.filter_include_descendants;
        let filter_include_connected_accounts = params.filter_include_connected_accounts;
        let filter_include_breakdown = params.filter_include_breakdown;
        let filter_versions = params.filter_versions;
        let page_limit = params.page_limit;
        let page_next_record_id = params.page_next_record_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/hourly_usage",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "filter[timestamp][start]",
            &filter_timestamp_start.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        )]);
        local_req_builder = local_req_builder.query(&[(
            "filter[product_families]",
            &filter_product_families.to_string(),
        )]);
        if let Some(ref local_query_param) = filter_timestamp_end {
            local_req_builder = local_req_builder.query(&[(
                "filter[timestamp][end]",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = filter_include_descendants {
            local_req_builder = local_req_builder.query(&[(
                "filter[include_descendants]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_include_connected_accounts {
            local_req_builder = local_req_builder.query(&[(
                "filter[include_connected_accounts]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_include_breakdown {
            local_req_builder = local_req_builder
                .query(&[("filter[include_breakdown]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_versions {
            local_req_builder =
                local_req_builder.query(&[("filter[versions]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_next_record_id {
            local_req_builder = local_req_builder
                .query(&[("page[next_record_id]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::HourlyUsageResponse>(
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
            let local_entity: Option<GetHourlyUsageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get monthly cost attribution by tag across multi-org and single root-org accounts.
    /// Cost Attribution data for a given month becomes available no later than the 19th of the following month.
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    /// ```
    /// response := GetMonthlyCostAttribution(start_month, end_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetMonthlyCostAttribution(start_month, end_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>). This endpoint is not available in the Government (US1-FED) site.
    pub async fn get_monthly_cost_attribution(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        fields: String,
        params: GetMonthlyCostAttributionOptionalParams,
    ) -> Result<
        crate::datadogV2::model::MonthlyCostAttributionResponse,
        datadog::Error<GetMonthlyCostAttributionError>,
    > {
        match self
            .get_monthly_cost_attribution_with_http_info(start_month, fields, params)
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

    /// Get monthly cost attribution by tag across multi-org and single root-org accounts.
    /// Cost Attribution data for a given month becomes available no later than the 19th of the following month.
    /// This API endpoint is paginated. To make sure you receive all records, check if the value of `next_record_id` is
    /// set in the response. If it is, make another request and pass `next_record_id` as a parameter.
    /// Pseudo code example:
    /// ```
    /// response := GetMonthlyCostAttribution(start_month, end_month)
    /// cursor := response.metadata.pagination.next_record_id
    /// WHILE cursor != null BEGIN
    ///   sleep(5 seconds)  # Avoid running into rate limit
    ///   response := GetMonthlyCostAttribution(start_month, end_month, next_record_id=cursor)
    ///   cursor := response.metadata.pagination.next_record_id
    /// END
    /// ```
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>). This endpoint is not available in the Government (US1-FED) site.
    pub async fn get_monthly_cost_attribution_with_http_info(
        &self,
        start_month: chrono::DateTime<chrono::Utc>,
        fields: String,
        params: GetMonthlyCostAttributionOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::MonthlyCostAttributionResponse>,
        datadog::Error<GetMonthlyCostAttributionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_monthly_cost_attribution";

        // unbox and build optional parameters
        let end_month = params.end_month;
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let next_record_id = params.next_record_id;
        let include_descendants = params.include_descendants;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/cost_by_tag/monthly_cost_attribution",
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
            match serde_json::from_str::<crate::datadogV2::model::MonthlyCostAttributionResponse>(
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
            let local_entity: Option<GetMonthlyCostAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get projected cost across multi-org and single root-org accounts.
    /// Projected cost data is only available for the current month and becomes available around the 12th of the month.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_projected_cost(
        &self,
        params: GetProjectedCostOptionalParams,
    ) -> Result<crate::datadogV2::model::ProjectedCostResponse, datadog::Error<GetProjectedCostError>>
    {
        match self.get_projected_cost_with_http_info(params).await {
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

    /// Get projected cost across multi-org and single root-org accounts.
    /// Projected cost data is only available for the current month and becomes available around the 12th of the month.
    ///
    /// This endpoint is only accessible for [parent-level organizations](<https://docs.datadoghq.com/account_management/multi_organization/>).
    pub async fn get_projected_cost_with_http_info(
        &self,
        params: GetProjectedCostOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ProjectedCostResponse>,
        datadog::Error<GetProjectedCostError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_projected_cost";

        // unbox and build optional parameters
        let view = params.view;
        let include_connected_accounts = params.include_connected_accounts;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/projected_cost",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = view {
            local_req_builder =
                local_req_builder.query(&[("view", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_connected_accounts {
            local_req_builder = local_req_builder
                .query(&[("include_connected_accounts", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ProjectedCostResponse>(
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
            let local_entity: Option<GetProjectedCostError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for application security .
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_application_security_monitoring(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageApplicationSecurityMonitoringOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
        datadog::Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        match self
            .get_usage_application_security_monitoring_with_http_info(start_hr, params)
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

    /// Get hourly usage for application security .
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_application_security_monitoring_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageApplicationSecurityMonitoringOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
        >,
        datadog::Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_application_security_monitoring";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/application_security",
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
                crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
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
            let local_entity: Option<GetUsageApplicationSecurityMonitoringError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for Lambda traced invocations.
    /// **Note:** This endpoint has been deprecated.. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_lambda_traced_invocations(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLambdaTracedInvocationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageLambdaTracedInvocationsResponse,
        datadog::Error<GetUsageLambdaTracedInvocationsError>,
    > {
        match self
            .get_usage_lambda_traced_invocations_with_http_info(start_hr, params)
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

    /// Get hourly usage for Lambda traced invocations.
    /// **Note:** This endpoint has been deprecated.. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_lambda_traced_invocations_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageLambdaTracedInvocationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UsageLambdaTracedInvocationsResponse>,
        datadog::Error<GetUsageLambdaTracedInvocationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_lambda_traced_invocations";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/lambda_traced_invocations",
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
                crate::datadogV2::model::UsageLambdaTracedInvocationsResponse,
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
            let local_entity: Option<GetUsageLambdaTracedInvocationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for observability pipelines.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_observability_pipelines(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageObservabilityPipelinesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageObservabilityPipelinesResponse,
        datadog::Error<GetUsageObservabilityPipelinesError>,
    > {
        match self
            .get_usage_observability_pipelines_with_http_info(start_hr, params)
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

    /// Get hourly usage for observability pipelines.
    /// **Note:** This endpoint has been deprecated. Hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_observability_pipelines_with_http_info(
        &self,
        start_hr: chrono::DateTime<chrono::Utc>,
        params: GetUsageObservabilityPipelinesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UsageObservabilityPipelinesResponse>,
        datadog::Error<GetUsageObservabilityPipelinesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_observability_pipelines";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/observability_pipelines",
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
            match serde_json::from_str::<crate::datadogV2::model::UsageObservabilityPipelinesResponse>(
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
            let local_entity: Option<GetUsageObservabilityPipelinesError> =
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
