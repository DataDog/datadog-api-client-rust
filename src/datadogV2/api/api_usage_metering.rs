// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetCostByOrgParams is a struct for passing parameters to the method [`GetCostByOrg`]
#[derive(Clone, Debug, Default)]
pub struct GetCostByOrgParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month.
    pub start_month: String,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<String>,
}

/// GetEstimatedCostByOrgParams is a struct for passing parameters to the method [`GetEstimatedCostByOrg`]
#[derive(Clone, Debug, Default)]
pub struct GetEstimatedCostByOrgParams {
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

/// GetHistoricalCostByOrgParams is a struct for passing parameters to the method [`GetHistoricalCostByOrg`]
#[derive(Clone, Debug, Default)]
pub struct GetHistoricalCostByOrgParams {
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost beginning this month.
    pub start_month: String,
    /// String to specify whether cost is broken down at a parent-org level or at the sub-org level. Available views are `summary` and `sub-org`.  Defaults to `summary`.
    pub view: Option<String>,
    /// Datetime in ISO-8601 format, UTC, precise to month: `[YYYY-MM]` for cost ending this month.
    pub end_month: Option<String>,
}

/// GetHourlyUsageParams is a struct for passing parameters to the method [`GetHourlyUsage`]
#[derive(Clone, Debug, Default)]
pub struct GetHourlyUsageParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: [YYYY-MM-DDThh] for usage beginning at this hour.
    pub filter_timestamp_start: String,
    /// Comma separated list of product families to retrieve. Available families are `all`, `analyzed_logs`,
    /// `application_security`, `audit_trail`, `serverless`, `ci_app`, `cloud_cost_management`,
    /// `csm_container_enterprise`, `csm_host_enterprise`, `cspm`, `custom_events`, `cws`, `dbm`, `fargate`,
    /// `infra_hosts`, `incident_management`, `indexed_logs`, `indexed_spans`, `ingested_spans`, `iot`,
    /// `lambda_traced_invocations`, `logs`, `network_flows`, `network_hosts`, `netflow_monitoring`, `observability_pipelines`,
    /// `online_archive`, `profiling`, `rum`, `rum_browser_sessions`, `rum_mobile_sessions`, `sds`, `snmp`,
    /// `synthetics_api`, `synthetics_browser`, `synthetics_mobile`, `synthetics_parallel_testing`, and `timeseries`.
    /// The following product family has been **deprecated**: `audit_logs`.
    pub filter_product_families: String,
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

/// GetUsageApplicationSecurityMonitoringParams is a struct for passing parameters to the method [`GetUsageApplicationSecurityMonitoring`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageApplicationSecurityMonitoringParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageLambdaTracedInvocationsParams is a struct for passing parameters to the method [`GetUsageLambdaTracedInvocations`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageLambdaTracedInvocationsParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetUsageObservabilityPipelinesParams is a struct for passing parameters to the method [`GetUsageObservabilityPipelines`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageObservabilityPipelinesParams {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage beginning at this hour.
    pub start_hr: String,
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]` for usage ending
    /// **before** this hour.
    pub end_hr: Option<String>,
}

/// GetCostByOrgError is a struct for typed errors of method [`GetCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetEstimatedCostByOrgError is a struct for typed errors of method [`GetEstimatedCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEstimatedCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHistoricalCostByOrgError is a struct for typed errors of method [`GetHistoricalCostByOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoricalCostByOrgError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetHourlyUsageError is a struct for typed errors of method [`GetHourlyUsage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHourlyUsageError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageApplicationSecurityMonitoringError is a struct for typed errors of method [`GetUsageApplicationSecurityMonitoring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageApplicationSecurityMonitoringError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageLambdaTracedInvocationsError is a struct for typed errors of method [`GetUsageLambdaTracedInvocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageLambdaTracedInvocationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUsageObservabilityPipelinesError is a struct for typed errors of method [`GetUsageObservabilityPipelines`]
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

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account)
    /// instead.
    pub async fn get_cost_by_org(
        &self,
        params: GetCostByOrgParams,
    ) -> Result<Option<crate::datadogV2::model::CostByOrgResponse>, Error<GetCostByOrgError>> {
        match self.get_cost_by_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get cost across multi-org account.
    /// Cost by org data for a given month becomes available no later than the 16th of the following month.
    /// **Note:** This endpoint has been deprecated. Please use the new endpoint
    /// [`/historical_cost`](https://docs.datadoghq.com/api/latest/usage-metering/#get-historical-cost-across-your-account)
    /// instead.
    pub async fn get_cost_by_org_with_http_info(
        &self,
        params: GetCostByOrgParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::CostByOrgResponse>, Error<GetCostByOrgError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_month = params.start_month;
        let end_month = params.end_month;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/usage/cost_by_org", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::CostByOrgResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetEstimatedCostByOrgParams,
    ) -> Result<Option<crate::datadogV2::model::CostByOrgResponse>, Error<GetEstimatedCostByOrgError>>
    {
        match self.get_estimated_cost_by_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get estimated cost across multi-org and single root-org accounts.
    /// Estimated cost data is only available for the current month and previous month
    /// and is delayed by up to 72 hours from when it was incurred.
    /// To access historical costs prior to this, use the `/historical_cost` endpoint.
    pub async fn get_estimated_cost_by_org_with_http_info(
        &self,
        params: GetEstimatedCostByOrgParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        Error<GetEstimatedCostByOrgError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let view = params.view;
        let start_month = params.start_month;
        let end_month = params.end_month;
        let start_date = params.start_date;
        let end_date = params.end_date;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/estimated_cost",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = view {
            local_req_builder = local_req_builder.query(&[("view", &local_str.to_string())]);
        };
        if let Some(ref local_str) = start_month {
            local_req_builder = local_req_builder.query(&[("start_month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
        };
        if let Some(ref local_str) = start_date {
            local_req_builder = local_req_builder.query(&[("start_date", &local_str.to_string())]);
        };
        if let Some(ref local_str) = end_date {
            local_req_builder = local_req_builder.query(&[("end_date", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::CostByOrgResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetHistoricalCostByOrgParams,
    ) -> Result<
        Option<crate::datadogV2::model::CostByOrgResponse>,
        Error<GetHistoricalCostByOrgError>,
    > {
        match self.get_historical_cost_by_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get historical cost across multi-org and single root-org accounts.
    /// Cost data for a given month becomes available no later than the 16th of the following month.
    pub async fn get_historical_cost_by_org_with_http_info(
        &self,
        params: GetHistoricalCostByOrgParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CostByOrgResponse>,
        Error<GetHistoricalCostByOrgError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_month = params.start_month;
        let view = params.view;
        let end_month = params.end_month;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/historical_cost",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start_month", &start_month.to_string())]);
        if let Some(ref local_str) = view {
            local_req_builder = local_req_builder.query(&[("view", &local_str.to_string())]);
        };
        if let Some(ref local_str) = end_month {
            local_req_builder = local_req_builder.query(&[("end_month", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::CostByOrgResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetHourlyUsageParams,
    ) -> Result<Option<crate::datadogV2::model::HourlyUsageResponse>, Error<GetHourlyUsageError>>
    {
        match self.get_hourly_usage_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage by product family.
    pub async fn get_hourly_usage_with_http_info(
        &self,
        params: GetHourlyUsageParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::HourlyUsageResponse>,
        Error<GetHourlyUsageError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_timestamp_start = params.filter_timestamp_start;
        let filter_product_families = params.filter_product_families;
        let filter_timestamp_end = params.filter_timestamp_end;
        let filter_include_descendants = params.filter_include_descendants;
        let filter_include_breakdown = params.filter_include_breakdown;
        let filter_versions = params.filter_versions;
        let page_limit = params.page_limit;
        let page_next_record_id = params.page_next_record_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/hourly_usage",
            local_configuration.base_path
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
        if let Some(ref local_str) = filter_timestamp_end {
            local_req_builder =
                local_req_builder.query(&[("filter[timestamp][end]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_include_descendants {
            local_req_builder =
                local_req_builder.query(&[("filter[include_descendants]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_include_breakdown {
            local_req_builder =
                local_req_builder.query(&[("filter[include_breakdown]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_versions {
            local_req_builder =
                local_req_builder.query(&[("filter[versions]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_limit {
            local_req_builder = local_req_builder.query(&[("page[limit]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_next_record_id {
            local_req_builder =
                local_req_builder.query(&[("page[next_record_id]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::HourlyUsageResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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

    /// Get hourly usage for application security .
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_application_security_monitoring(
        &self,
        params: GetUsageApplicationSecurityMonitoringParams,
    ) -> Result<
        Option<crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse>,
        Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        match self
            .get_usage_application_security_monitoring_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for application security .
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_application_security_monitoring_with_http_info(
        &self,
        params: GetUsageApplicationSecurityMonitoringParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse>,
        Error<GetUsageApplicationSecurityMonitoringError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/application_security",
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
            let local_entity: Option<
                crate::datadogV2::model::UsageApplicationSecurityMonitoringResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_lambda_traced_invocations(
        &self,
        params: GetUsageLambdaTracedInvocationsParams,
    ) -> Result<
        Option<crate::datadogV2::model::UsageLambdaTracedInvocationsResponse>,
        Error<GetUsageLambdaTracedInvocationsError>,
    > {
        match self
            .get_usage_lambda_traced_invocations_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for lambda traced invocations.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_lambda_traced_invocations_with_http_info(
        &self,
        params: GetUsageLambdaTracedInvocationsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageLambdaTracedInvocationsResponse>,
        Error<GetUsageLambdaTracedInvocationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/lambda_traced_invocations",
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
            let local_entity: Option<
                crate::datadogV2::model::UsageLambdaTracedInvocationsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_observability_pipelines(
        &self,
        params: GetUsageObservabilityPipelinesParams,
    ) -> Result<
        Option<crate::datadogV2::model::UsageObservabilityPipelinesResponse>,
        Error<GetUsageObservabilityPipelinesError>,
    > {
        match self
            .get_usage_observability_pipelines_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get hourly usage for observability pipelines.
    /// **Note:** hourly usage data for all products is now available in the [Get hourly usage by product family API](https://docs.datadoghq.com/api/latest/usage-metering/#get-hourly-usage-by-product-family)
    pub async fn get_usage_observability_pipelines_with_http_info(
        &self,
        params: GetUsageObservabilityPipelinesParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsageObservabilityPipelinesResponse>,
        Error<GetUsageObservabilityPipelinesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let start_hr = params.start_hr;
        let end_hr = params.end_hr;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/usage/observability_pipelines",
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
            let local_entity: Option<crate::datadogV2::model::UsageObservabilityPipelinesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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