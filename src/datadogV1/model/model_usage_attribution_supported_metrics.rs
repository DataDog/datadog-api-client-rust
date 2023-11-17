// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageAttributionSupportedMetrics {
    #[serde(rename = "custom_timeseries_usage")]
    CUSTOM_TIMESERIES_USAGE,
    #[serde(rename = "container_usage")]
    CONTAINER_USAGE,
    #[serde(rename = "snmp_percentage")]
    SNMP_PERCENTAGE,
    #[serde(rename = "apm_host_usage")]
    APM_HOST_USAGE,
    #[serde(rename = "browser_usage")]
    BROWSER_USAGE,
    #[serde(rename = "npm_host_percentage")]
    NPM_HOST_PERCENTAGE,
    #[serde(rename = "infra_host_usage")]
    INFRA_HOST_USAGE,
    #[serde(rename = "custom_timeseries_percentage")]
    CUSTOM_TIMESERIES_PERCENTAGE,
    #[serde(rename = "container_percentage")]
    CONTAINER_PERCENTAGE,
    #[serde(rename = "api_usage")]
    API_USAGE,
    #[serde(rename = "apm_host_percentage")]
    APM_HOST_PERCENTAGE,
    #[serde(rename = "infra_host_percentage")]
    INFRA_HOST_PERCENTAGE,
    #[serde(rename = "snmp_usage")]
    SNMP_USAGE,
    #[serde(rename = "browser_percentage")]
    BROWSER_PERCENTAGE,
    #[serde(rename = "api_percentage")]
    API_PERCENTAGE,
    #[serde(rename = "npm_host_usage")]
    NPM_HOST_USAGE,
    #[serde(rename = "lambda_functions_usage")]
    LAMBDA_FUNCTIONS_USAGE,
    #[serde(rename = "lambda_functions_percentage")]
    LAMBDA_FUNCTIONS_PERCENTAGE,
    #[serde(rename = "lambda_invocations_usage")]
    LAMBDA_INVOCATIONS_USAGE,
    #[serde(rename = "lambda_invocations_percentage")]
    LAMBDA_INVOCATIONS_PERCENTAGE,
    #[serde(rename = "fargate_usage")]
    FARGATE_USAGE,
    #[serde(rename = "fargate_percentage")]
    FARGATE_PERCENTAGE,
    #[serde(rename = "profiled_host_usage")]
    PROFILED_HOST_USAGE,
    #[serde(rename = "profiled_host_percentage")]
    PROFILED_HOST_PERCENTAGE,
    #[serde(rename = "profiled_container_usage")]
    PROFILED_CONTAINER_USAGE,
    #[serde(rename = "profiled_container_percentage")]
    PROFILED_CONTAINER_PERCENTAGE,
    #[serde(rename = "dbm_hosts_usage")]
    DBM_HOSTS_USAGE,
    #[serde(rename = "dbm_hosts_percentage")]
    DBM_HOSTS_PERCENTAGE,
    #[serde(rename = "dbm_queries_usage")]
    DBM_QUERIES_USAGE,
    #[serde(rename = "dbm_queries_percentage")]
    DBM_QUERIES_PERCENTAGE,
    #[serde(rename = "estimated_indexed_logs_usage")]
    ESTIMATED_INDEXED_LOGS_USAGE,
    #[serde(rename = "estimated_indexed_logs_percentage")]
    ESTIMATED_INDEXED_LOGS_PERCENTAGE,
    #[serde(rename = "estimated_ingested_logs_usage")]
    ESTIMATED_INGESTED_LOGS_USAGE,
    #[serde(rename = "estimated_ingested_logs_percentage")]
    ESTIMATED_INGESTED_LOGS_PERCENTAGE,
    #[serde(rename = "appsec_usage")]
    APPSEC_USAGE,
    #[serde(rename = "appsec_percentage")]
    APPSEC_PERCENTAGE,
    #[serde(rename = "estimated_indexed_spans_usage")]
    ESTIMATED_INDEXED_SPANS_USAGE,
    #[serde(rename = "estimated_indexed_spans_percentage")]
    ESTIMATED_INDEXED_SPANS_PERCENTAGE,
    #[serde(rename = "estimated_ingested_spans_usage")]
    ESTIMATED_INGESTED_SPANS_USAGE,
    #[serde(rename = "estimated_ingested_spans_percentage")]
    ESTIMATED_INGESTED_SPANS_PERCENTAGE,
    #[serde(rename = "apm_fargate_usage")]
    APM_FARGATE_USAGE,
    #[serde(rename = "apm_fargate_percentage")]
    APM_FARGATE_PERCENTAGE,
    #[serde(rename = "appsec_fargate_usage")]
    APPSEC_FARGATE_USAGE,
    #[serde(rename = "appsec_fargate_percentage")]
    APPSEC_FARGATE_PERCENTAGE,
    #[serde(rename = "estimated_rum_usage_attribution_usage")]
    ESTIMATED_RUM_USAGE_ATTRIBUTION_USAGE,
    #[serde(rename = "estimated_rum_usage_attribution_percentage")]
    ESTIMATED_RUM_USAGE_ATTRIBUTION_PERCENTAGE,
    #[serde(rename = "*")]
    ALL,
}

impl ToString for UsageAttributionSupportedMetrics {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM_TIMESERIES_USAGE => String::from("custom_timeseries_usage"),
            Self::CONTAINER_USAGE => String::from("container_usage"),
            Self::SNMP_PERCENTAGE => String::from("snmp_percentage"),
            Self::APM_HOST_USAGE => String::from("apm_host_usage"),
            Self::BROWSER_USAGE => String::from("browser_usage"),
            Self::NPM_HOST_PERCENTAGE => String::from("npm_host_percentage"),
            Self::INFRA_HOST_USAGE => String::from("infra_host_usage"),
            Self::CUSTOM_TIMESERIES_PERCENTAGE => String::from("custom_timeseries_percentage"),
            Self::CONTAINER_PERCENTAGE => String::from("container_percentage"),
            Self::API_USAGE => String::from("api_usage"),
            Self::APM_HOST_PERCENTAGE => String::from("apm_host_percentage"),
            Self::INFRA_HOST_PERCENTAGE => String::from("infra_host_percentage"),
            Self::SNMP_USAGE => String::from("snmp_usage"),
            Self::BROWSER_PERCENTAGE => String::from("browser_percentage"),
            Self::API_PERCENTAGE => String::from("api_percentage"),
            Self::NPM_HOST_USAGE => String::from("npm_host_usage"),
            Self::LAMBDA_FUNCTIONS_USAGE => String::from("lambda_functions_usage"),
            Self::LAMBDA_FUNCTIONS_PERCENTAGE => String::from("lambda_functions_percentage"),
            Self::LAMBDA_INVOCATIONS_USAGE => String::from("lambda_invocations_usage"),
            Self::LAMBDA_INVOCATIONS_PERCENTAGE => String::from("lambda_invocations_percentage"),
            Self::FARGATE_USAGE => String::from("fargate_usage"),
            Self::FARGATE_PERCENTAGE => String::from("fargate_percentage"),
            Self::PROFILED_HOST_USAGE => String::from("profiled_host_usage"),
            Self::PROFILED_HOST_PERCENTAGE => String::from("profiled_host_percentage"),
            Self::PROFILED_CONTAINER_USAGE => String::from("profiled_container_usage"),
            Self::PROFILED_CONTAINER_PERCENTAGE => String::from("profiled_container_percentage"),
            Self::DBM_HOSTS_USAGE => String::from("dbm_hosts_usage"),
            Self::DBM_HOSTS_PERCENTAGE => String::from("dbm_hosts_percentage"),
            Self::DBM_QUERIES_USAGE => String::from("dbm_queries_usage"),
            Self::DBM_QUERIES_PERCENTAGE => String::from("dbm_queries_percentage"),
            Self::ESTIMATED_INDEXED_LOGS_USAGE => String::from("estimated_indexed_logs_usage"),
            Self::ESTIMATED_INDEXED_LOGS_PERCENTAGE => {
                String::from("estimated_indexed_logs_percentage")
            }
            Self::ESTIMATED_INGESTED_LOGS_USAGE => String::from("estimated_ingested_logs_usage"),
            Self::ESTIMATED_INGESTED_LOGS_PERCENTAGE => {
                String::from("estimated_ingested_logs_percentage")
            }
            Self::APPSEC_USAGE => String::from("appsec_usage"),
            Self::APPSEC_PERCENTAGE => String::from("appsec_percentage"),
            Self::ESTIMATED_INDEXED_SPANS_USAGE => String::from("estimated_indexed_spans_usage"),
            Self::ESTIMATED_INDEXED_SPANS_PERCENTAGE => {
                String::from("estimated_indexed_spans_percentage")
            }
            Self::ESTIMATED_INGESTED_SPANS_USAGE => String::from("estimated_ingested_spans_usage"),
            Self::ESTIMATED_INGESTED_SPANS_PERCENTAGE => {
                String::from("estimated_ingested_spans_percentage")
            }
            Self::APM_FARGATE_USAGE => String::from("apm_fargate_usage"),
            Self::APM_FARGATE_PERCENTAGE => String::from("apm_fargate_percentage"),
            Self::APPSEC_FARGATE_USAGE => String::from("appsec_fargate_usage"),
            Self::APPSEC_FARGATE_PERCENTAGE => String::from("appsec_fargate_percentage"),
            Self::ESTIMATED_RUM_USAGE_ATTRIBUTION_USAGE => {
                String::from("estimated_rum_usage_attribution_usage")
            }
            Self::ESTIMATED_RUM_USAGE_ATTRIBUTION_PERCENTAGE => {
                String::from("estimated_rum_usage_attribution_percentage")
            }
            Self::ALL => String::from("*"),
        }
    }
}

impl Default for UsageAttributionSupportedMetrics {
    fn default() -> UsageAttributionSupportedMetrics {
        Self::CUSTOM_TIMESERIES_USAGE
    }
}
