// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsageAttributionSupportedMetrics {
    CUSTOM_TIMESERIES_USAGE,
    CONTAINER_USAGE,
    SNMP_PERCENTAGE,
    APM_HOST_USAGE,
    BROWSER_USAGE,
    NPM_HOST_PERCENTAGE,
    INFRA_HOST_USAGE,
    CUSTOM_TIMESERIES_PERCENTAGE,
    CONTAINER_PERCENTAGE,
    API_USAGE,
    APM_HOST_PERCENTAGE,
    INFRA_HOST_PERCENTAGE,
    SNMP_USAGE,
    BROWSER_PERCENTAGE,
    API_PERCENTAGE,
    NPM_HOST_USAGE,
    LAMBDA_FUNCTIONS_USAGE,
    LAMBDA_FUNCTIONS_PERCENTAGE,
    LAMBDA_INVOCATIONS_USAGE,
    LAMBDA_INVOCATIONS_PERCENTAGE,
    FARGATE_USAGE,
    FARGATE_PERCENTAGE,
    PROFILED_HOST_USAGE,
    PROFILED_HOST_PERCENTAGE,
    PROFILED_CONTAINER_USAGE,
    PROFILED_CONTAINER_PERCENTAGE,
    DBM_HOSTS_USAGE,
    DBM_HOSTS_PERCENTAGE,
    DBM_QUERIES_USAGE,
    DBM_QUERIES_PERCENTAGE,
    ESTIMATED_INDEXED_LOGS_USAGE,
    ESTIMATED_INDEXED_LOGS_PERCENTAGE,
    ESTIMATED_INGESTED_LOGS_USAGE,
    ESTIMATED_INGESTED_LOGS_PERCENTAGE,
    APPSEC_USAGE,
    APPSEC_PERCENTAGE,
    ESTIMATED_INDEXED_SPANS_USAGE,
    ESTIMATED_INDEXED_SPANS_PERCENTAGE,
    ESTIMATED_INGESTED_SPANS_USAGE,
    ESTIMATED_INGESTED_SPANS_PERCENTAGE,
    APM_FARGATE_USAGE,
    APM_FARGATE_PERCENTAGE,
    APPSEC_FARGATE_USAGE,
    APPSEC_FARGATE_PERCENTAGE,
    ESTIMATED_RUM_USAGE_ATTRIBUTION_USAGE,
    ESTIMATED_RUM_USAGE_ATTRIBUTION_PERCENTAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE,
    ALL,
    UnparsedObject(crate::datadog::UnparsedObject),
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
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE => {
                String::from("asm_serverless_traced_invocations_usage")
            }
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE => {
                String::from("asm_serverless_traced_invocations_percentage")
            }
            Self::ALL => String::from("*"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for UsageAttributionSupportedMetrics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for UsageAttributionSupportedMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom_timeseries_usage" => Self::CUSTOM_TIMESERIES_USAGE,
            "container_usage" => Self::CONTAINER_USAGE,
            "snmp_percentage" => Self::SNMP_PERCENTAGE,
            "apm_host_usage" => Self::APM_HOST_USAGE,
            "browser_usage" => Self::BROWSER_USAGE,
            "npm_host_percentage" => Self::NPM_HOST_PERCENTAGE,
            "infra_host_usage" => Self::INFRA_HOST_USAGE,
            "custom_timeseries_percentage" => Self::CUSTOM_TIMESERIES_PERCENTAGE,
            "container_percentage" => Self::CONTAINER_PERCENTAGE,
            "api_usage" => Self::API_USAGE,
            "apm_host_percentage" => Self::APM_HOST_PERCENTAGE,
            "infra_host_percentage" => Self::INFRA_HOST_PERCENTAGE,
            "snmp_usage" => Self::SNMP_USAGE,
            "browser_percentage" => Self::BROWSER_PERCENTAGE,
            "api_percentage" => Self::API_PERCENTAGE,
            "npm_host_usage" => Self::NPM_HOST_USAGE,
            "lambda_functions_usage" => Self::LAMBDA_FUNCTIONS_USAGE,
            "lambda_functions_percentage" => Self::LAMBDA_FUNCTIONS_PERCENTAGE,
            "lambda_invocations_usage" => Self::LAMBDA_INVOCATIONS_USAGE,
            "lambda_invocations_percentage" => Self::LAMBDA_INVOCATIONS_PERCENTAGE,
            "fargate_usage" => Self::FARGATE_USAGE,
            "fargate_percentage" => Self::FARGATE_PERCENTAGE,
            "profiled_host_usage" => Self::PROFILED_HOST_USAGE,
            "profiled_host_percentage" => Self::PROFILED_HOST_PERCENTAGE,
            "profiled_container_usage" => Self::PROFILED_CONTAINER_USAGE,
            "profiled_container_percentage" => Self::PROFILED_CONTAINER_PERCENTAGE,
            "dbm_hosts_usage" => Self::DBM_HOSTS_USAGE,
            "dbm_hosts_percentage" => Self::DBM_HOSTS_PERCENTAGE,
            "dbm_queries_usage" => Self::DBM_QUERIES_USAGE,
            "dbm_queries_percentage" => Self::DBM_QUERIES_PERCENTAGE,
            "estimated_indexed_logs_usage" => Self::ESTIMATED_INDEXED_LOGS_USAGE,
            "estimated_indexed_logs_percentage" => Self::ESTIMATED_INDEXED_LOGS_PERCENTAGE,
            "estimated_ingested_logs_usage" => Self::ESTIMATED_INGESTED_LOGS_USAGE,
            "estimated_ingested_logs_percentage" => Self::ESTIMATED_INGESTED_LOGS_PERCENTAGE,
            "appsec_usage" => Self::APPSEC_USAGE,
            "appsec_percentage" => Self::APPSEC_PERCENTAGE,
            "estimated_indexed_spans_usage" => Self::ESTIMATED_INDEXED_SPANS_USAGE,
            "estimated_indexed_spans_percentage" => Self::ESTIMATED_INDEXED_SPANS_PERCENTAGE,
            "estimated_ingested_spans_usage" => Self::ESTIMATED_INGESTED_SPANS_USAGE,
            "estimated_ingested_spans_percentage" => Self::ESTIMATED_INGESTED_SPANS_PERCENTAGE,
            "apm_fargate_usage" => Self::APM_FARGATE_USAGE,
            "apm_fargate_percentage" => Self::APM_FARGATE_PERCENTAGE,
            "appsec_fargate_usage" => Self::APPSEC_FARGATE_USAGE,
            "appsec_fargate_percentage" => Self::APPSEC_FARGATE_PERCENTAGE,
            "estimated_rum_usage_attribution_usage" => Self::ESTIMATED_RUM_USAGE_ATTRIBUTION_USAGE,
            "estimated_rum_usage_attribution_percentage" => {
                Self::ESTIMATED_RUM_USAGE_ATTRIBUTION_PERCENTAGE
            }
            "asm_serverless_traced_invocations_usage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE
            }
            "asm_serverless_traced_invocations_percentage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE
            }
            "*" => Self::ALL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
