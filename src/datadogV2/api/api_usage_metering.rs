// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetCostByOrgOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_cost_by_org`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCostByOrgOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<String>,
}

impl GetCostByOrgOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: String) -> Self {
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
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. Either start_month or start_date should be specified, but not both. (start_month cannot go beyond two months in the past). Provide an `end_month` to view month-over-month cost.
    pub start_month: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost beginning this day. Either start_month or start_date should be specified, but not both. (start_date cannot go beyond two months in the past). Provide an `end_date` to view day-over-day cumulative cost.
    pub start_date: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost ending this day.
    pub end_date: Option<String>,
}

impl GetEstimatedCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month. Either start_month or start_date should be specified, but not both. (start_month cannot go beyond two months in the past). Provide an `end_month` to view month-over-month cost.
    pub fn start_month(mut self, value: String) -> Self {
        self.start_month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: String) -> Self {
        self.end_month = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost beginning this day. Either start_month or start_date should be specified, but not both. (start_date cannot go beyond two months in the past). Provide an `end_date` to view day-over-day cumulative cost.
    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to day: `[YYYY-MM-DD]` for cost ending this day.
    pub fn end_date(mut self, value: String) -> Self {
        self.end_date = Some(value);
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
    pub end_month: Option<String>,
}

impl GetHistoricalCostByOrgOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`.  Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub fn end_month(mut self, value: String) -> Self {
        self.end_month = Some(value);
        self
    }
}

/// GetHourlyUsageOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_hourly_usage`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetHourlyUsageOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage ending **before** this hour.
    pub filter_timestamp_end: Option<String>,
    /// Include child org usage in the response. Defaults to false.
    pub filter_include_descendants: Option<bool>,
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
    pub fn filter_timestamp_end(mut self, value: String) -> Self {
        self.filter_timestamp_end = Some(value);
        self
    }
    /// Include child org usage in the response. Defaults to false.
    pub fn filter_include_descendants(mut self, value: bool) -> Self {
        self.filter_include_descendants = Some(value);
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
}

impl GetProjectedCostOptionalParams {
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`. Defaults to `summary`.
    pub fn view(mut self, value: String) -> Self {
        self.view = Some(value);
        self
    }
}

/// GetUsageApplicationSecurityMonitoringOptionalParams is a struct for passing parameters to the method [`UsageMeteringAPI::get_usage_application_security_monitoring`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetUsageApplicationSecurityMonitoringOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

impl GetUsageApplicationSecurityMonitoringOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageLambdaTracedInvocationsOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
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
    pub end_hr: Option<String>,
}

impl GetUsageObservabilityPipelinesOptionalParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub fn end_hr(mut self, value: String) -> Self {
        self.end_hr = Some(value);
        self
    }
}

/// GetActiveBillingDimensionsError is a struct for typed errors of method [`UsageMeteringAPI::get_active_billing_dimensions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveBillingDimensionsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetEstimatedCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_estimated_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEstimatedCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHistoricalCostByOrgError is a struct for typed errors of method [`UsageMeteringAPI::get_historical_cost_by_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoricalCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageError is a struct for typed errors of method [`UsageMeteringAPI::get_hourly_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonthlyCostAttributionError is a struct for typed errors of method [`UsageMeteringAPI::get_monthly_cost_attribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonthlyCostAttributionError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetProjectedCostError is a struct for typed errors of method [`UsageMeteringAPI::get_projected_cost`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectedCostError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageApplicationSecurityMonitoringError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_application_security_monitoring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageApplicationSecurityMonitoringError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaTracedInvocationsError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_lambda_traced_invocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaTracedInvocationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageObservabilityPipelinesError is a struct for typed errors of method [`UsageMeteringAPI::get_usage_observability_pipelines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageObservabilityPipelinesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
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

    /// Get active billing dimensions for cost attribution. Cost data for a given month becomes available no later than the 17th of the following month.
    pub async fn get_active_billing_dimensions(
        &self,
    ) -> Result<
        crate::datadogV2::model::ActiveBillingDimensionsResponse,
        Error<GetActiveBillingDimensionsError>,
    > {
        match self.get_active_billing_dimensions_with_http_info().await {
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

    /// Get active billing dimensions for cost attribution. Cost data for a given month becomes available no later than the 17th of the following month.
    pub async fn get_active_billing_dimensions_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ActiveBillingDimensionsResponse>,
        Error<GetActiveBillingDimensionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_active_billing_dimensions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_active_billing_dimensions' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/cost_by_tag/active_billing_dimensions",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ActiveBillingDimensionsResponse>(
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
            let local_entity: Option<GetActiveBillingDimensionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](<https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account>)
    /// instead.
    pub async fn get_cost_by_org(
        &self,
        start_month: String,
        params: GetCostByOrgOptionalParams,
    ) -> Result<crate::datadogV2::model::CostByOrgResponse, Error<GetCostByOrgError>> {
        match self
            .get_cost_by_org_with_http_info(start_month, params)
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

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](<https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account>)
    /// instead.
    pub async fn get_cost_by_org_with_http_info(
        &self,
        start_month: String,
        params: GetCostByOrgOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::CostByOrgResponse>, Error<GetCostByOrgError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cost_by_org";

        // unbox and build optional parameters
        let end_month = params.end_month;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/cost_by_org",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetCostByOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get estimated cost across multi-org and single root-org accounts.
    /// Estimated cost data is only available for the current month and previous month
    /// and is delayed by up to 72 hours from when it was incurred.
    /// To access historical costs prior to this, use the `/historical_cost` endpoint.
    pub async fn get_estimated_cost_by_org(
        &self,
        params: GetEstimatedCostByOrgOptionalParams,
    ) -> Result<crate::datadogV2::model::CostByOrgResponse, Error<GetEstimatedCostByOrgError>> {
        match self.get_estimated_cost_by_org_with_http_info(params).await {
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

    /// Get estimated cost across multi-org and single root-org accounts.
    /// Estimated cost data is only available for the current month and previous month
    /// and is delayed by up to 72 hours from when it was incurred.
    /// To access historical costs prior to this, use the `/historical_cost` endpoint.
    pub async fn get_estimated_cost_by_org_with_http_info(
        &self,
        params: GetEstimatedCostByOrgOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        Error<GetEstimatedCostByOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_estimated_cost_by_org";

        // unbox and build optional parameters
        let view = params.view;
        let start_month = params.start_month;
        let end_month = params.end_month;
        let start_date = params.start_date;
        let end_date = params.end_date;

        let local_client = &local_configuration.client;

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
            local_req_builder =
                local_req_builder.query(&[("start_month", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start_date {
            local_req_builder =
                local_req_builder.query(&[("start_date", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end_date {
            local_req_builder =
                local_req_builder.query(&[("end_date", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetEstimatedCostByOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get historical cost across multi-org and single root-org accounts.
    /// Cost data for a given month becomes available no later than the 16th of the following month.
    pub async fn get_historical_cost_by_org(
        &self,
        start_month: String,
        params: GetHistoricalCostByOrgOptionalParams,
    ) -> Result<crate::datadogV2::model::CostByOrgResponse, Error<GetHistoricalCostByOrgError>>
    {
        match self
            .get_historical_cost_by_org_with_http_info(start_month, params)
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

    /// Get historical cost across multi-org and single root-org accounts.
    /// Cost data for a given month becomes available no later than the 16th of the following month.
    pub async fn get_historical_cost_by_org_with_http_info(
        &self,
        start_month: String,
        params: GetHistoricalCostByOrgOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        Error<GetHistoricalCostByOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_historical_cost_by_org";

        // unbox and build optional parameters
        let view = params.view;
        let end_month = params.end_month;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/historical_cost",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_query_param) = view {
            local_req_builder =
                local_req_builder.query(&[("view", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = end_month {
            local_req_builder =
                local_req_builder.query(&[("end_month", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CostByOrgResponse>(&local_content)
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
            let local_entity: Option<GetHistoricalCostByOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage by product family.
    pub async fn get_hourly_usage(
        &self,
        filter_timestamp_start: String,
        filter_product_families: String,
        params: GetHourlyUsageOptionalParams,
    ) -> Result<crate::datadogV2::model::HourlyUsageResponse, Error<GetHourlyUsageError>> {
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
                    Err(Error::Serde(serde::de::Error::custom(
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
        filter_timestamp_start: String,
        filter_product_families: String,
        params: GetHourlyUsageOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::HourlyUsageResponse>,
        Error<GetHourlyUsageError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_hourly_usage";

        // unbox and build optional parameters
        let filter_timestamp_end = params.filter_timestamp_end;
        let filter_include_descendants = params.filter_include_descendants;
        let filter_include_breakdown = params.filter_include_breakdown;
        let filter_versions = params.filter_versions;
        let page_limit = params.page_limit;
        let page_next_record_id = params.page_next_record_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/hourly_usage",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "filter[timestamp][start]",
            &filter_timestamp_start.to_string(),
        )]);
        local_req_builder = local_req_builder.query(&[(
            "filter[product_families]",
            &filter_product_families.to_string(),
        )]);
        if let Some(ref local_query_param) = filter_timestamp_end {
            local_req_builder = local_req_builder
                .query(&[("filter[timestamp][end]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_include_descendants {
            local_req_builder = local_req_builder.query(&[(
                "filter[include_descendants]",
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
            match serde_json::from_str::<crate::datadogV2::model::HourlyUsageResponse>(
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
            let local_entity: Option<GetHourlyUsageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get monthly cost attribution by tag across multi-org and single root-org accounts.
    /// Cost Attribution data for a given month becomes available no later than the 17th of the following month.
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
    pub async fn get_monthly_cost_attribution(
        &self,
        start_month: String,
        end_month: String,
        fields: String,
        params: GetMonthlyCostAttributionOptionalParams,
    ) -> Result<
        crate::datadogV2::model::MonthlyCostAttributionResponse,
        Error<GetMonthlyCostAttributionError>,
    > {
        match self
            .get_monthly_cost_attribution_with_http_info(start_month, end_month, fields, params)
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

    /// Get monthly cost attribution by tag across multi-org and single root-org accounts.
    /// Cost Attribution data for a given month becomes available no later than the 17th of the following month.
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
    pub async fn get_monthly_cost_attribution_with_http_info(
        &self,
        start_month: String,
        end_month: String,
        fields: String,
        params: GetMonthlyCostAttributionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::MonthlyCostAttributionResponse>,
        Error<GetMonthlyCostAttributionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_monthly_cost_attribution";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_monthly_cost_attribution' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let sort_direction = params.sort_direction;
        let sort_name = params.sort_name;
        let tag_breakdown_keys = params.tag_breakdown_keys;
        let next_record_id = params.next_record_id;
        let include_descendants = params.include_descendants;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/cost_by_tag/monthly_cost_attribution",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("end_month", &end_month.to_string())]);
        local_req_builder = local_req_builder.query(&[("fields", &fields.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::MonthlyCostAttributionResponse>(
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
            let local_entity: Option<GetMonthlyCostAttributionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get projected cost across multi-org and single root-org accounts.
    /// Projected cost data is only available for the current month and becomes available around the 12th of the month.
    /// This endpoint requires the usage_read authorization scope.
    pub async fn get_projected_cost(
        &self,
        params: GetProjectedCostOptionalParams,
    ) -> Result<crate::datadogV2::model::ProjectedCostResponse, Error<GetProjectedCostError>> {
        match self.get_projected_cost_with_http_info(params).await {
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

    /// Get projected cost across multi-org and single root-org accounts.
    /// Projected cost data is only available for the current month and becomes available around the 12th of the month.
    /// This endpoint requires the usage_read authorization scope.
    pub async fn get_projected_cost_with_http_info(
        &self,
        params: GetProjectedCostOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ProjectedCostResponse>,
        Error<GetProjectedCostError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_projected_cost";

        // unbox and build optional parameters
        let view = params.view;

        let local_client = &local_configuration.client;

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
            match serde_json::from_str::<crate::datadogV2::model::ProjectedCostResponse>(
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
            let local_entity: Option<GetProjectedCostError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for application security .
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_application_security_monitoring(
        &self,
        start_hr: String,
        params: GetUsageApplicationSecurityMonitoringOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
        Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        match self
            .get_usage_application_security_monitoring_with_http_info(start_hr, params)
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

    /// Get hourly usage for application security .
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_application_security_monitoring_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageApplicationSecurityMonitoringOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse>,
        Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_application_security_monitoring";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/application_security",
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
                crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
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
            let local_entity: Option<GetUsageApplicationSecurityMonitoringError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for lambda traced invocations.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_lambda_traced_invocations(
        &self,
        start_hr: String,
        params: GetUsageLambdaTracedInvocationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageLambdaTracedInvocationsResponse,
        Error<GetUsageLambdaTracedInvocationsError>,
    > {
        match self
            .get_usage_lambda_traced_invocations_with_http_info(start_hr, params)
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

    /// Get hourly usage for lambda traced invocations.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_lambda_traced_invocations_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageLambdaTracedInvocationsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageLambdaTracedInvocationsResponse>,
        Error<GetUsageLambdaTracedInvocationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_lambda_traced_invocations";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/lambda_traced_invocations",
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
                crate::datadogV2::model::UsageLambdaTracedInvocationsResponse,
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
            let local_entity: Option<GetUsageLambdaTracedInvocationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get hourly usage for observability pipelines.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_observability_pipelines(
        &self,
        start_hr: String,
        params: GetUsageObservabilityPipelinesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UsageObservabilityPipelinesResponse,
        Error<GetUsageObservabilityPipelinesError>,
    > {
        match self
            .get_usage_observability_pipelines_with_http_info(start_hr, params)
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

    /// Get hourly usage for observability pipelines.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](<https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family>)
    pub async fn get_usage_observability_pipelines_with_http_info(
        &self,
        start_hr: String,
        params: GetUsageObservabilityPipelinesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageObservabilityPipelinesResponse>,
        Error<GetUsageObservabilityPipelinesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_usage_observability_pipelines";

        // unbox and build optional parameters
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/observability_pipelines",
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
            match serde_json::from_str::<crate::datadogV2::model::UsageObservabilityPipelinesResponse>(
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
            let local_entity: Option<GetUsageObservabilityPipelinesError> =
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
