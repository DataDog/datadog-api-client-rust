// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HourlyUsageAttributionUsageType {
    #[serde(rename = "api_usage")]
    API_USAGE,
    #[serde(rename = "apm_fargate_usage")]
    APM_FARGATE_USAGE,
    #[serde(rename = "apm_host_usage")]
    APM_HOST_USAGE,
    #[serde(rename = "apm_usm_usage")]
    APM_USM_USAGE,
    #[serde(rename = "appsec_fargate_usage")]
    APPSEC_FARGATE_USAGE,
    #[serde(rename = "appsec_usage")]
    APPSEC_USAGE,
    #[serde(rename = "browser_usage")]
    BROWSER_USAGE,
    #[serde(rename = "ci_pipeline_indexed_spans_usage")]
    CI_PIPELINE_INDEXED_SPANS_USAGE,
    #[serde(rename = "ci_test_indexed_spans_usage")]
    CI_TEST_INDEXED_SPANS_USAGE,
    #[serde(rename = "ci_visibility_itr_usage")]
    CI_VISIBILITY_ITR_USAGE,
    #[serde(rename = "cloud_siem_usage")]
    CLOUD_SIEM_USAGE,
    #[serde(rename = "container_excl_agent_usage")]
    CONTAINER_EXCL_AGENT_USAGE,
    #[serde(rename = "container_usage")]
    CONTAINER_USAGE,
    #[serde(rename = "cspm_containers_usage")]
    CSPM_CONTAINERS_USAGE,
    #[serde(rename = "cspm_hosts_usage")]
    CSPM_HOSTS_USAGE,
    #[serde(rename = "custom_event_usage")]
    CUSTOM_EVENT_USAGE,
    #[serde(rename = "custom_ingested_timeseries_usage")]
    CUSTOM_INGESTED_TIMESERIES_USAGE,
    #[serde(rename = "custom_timeseries_usage")]
    CUSTOM_TIMESERIES_USAGE,
    #[serde(rename = "cws_containers_usage")]
    CWS_CONTAINERS_USAGE,
    #[serde(rename = "cws_hosts_usage")]
    CWS_HOSTS_USAGE,
    #[serde(rename = "dbm_hosts_usage")]
    DBM_HOSTS_USAGE,
    #[serde(rename = "dbm_queries_usage")]
    DBM_QUERIES_USAGE,
    #[serde(rename = "estimated_indexed_logs_usage")]
    ESTIMATED_INDEXED_LOGS_USAGE,
    #[serde(rename = "estimated_indexed_spans_usage")]
    ESTIMATED_INDEXED_SPANS_USAGE,
    #[serde(rename = "estimated_ingested_logs_usage")]
    ESTIMATED_INGESTED_LOGS_USAGE,
    #[serde(rename = "estimated_ingested_spans_usage")]
    ESTIMATED_INGESTED_SPANS_USAGE,
    #[serde(rename = "estimated_rum_sessions_usage")]
    ESTIMATED_RUM_SESSIONS_USAGE,
    #[serde(rename = "fargate_usage")]
    FARGATE_USAGE,
    #[serde(rename = "functions_usage")]
    FUNCTIONS_USAGE,
    #[serde(rename = "indexed_spans_usage")]
    INDEXED_SPANS_USAGE,
    #[serde(rename = "infra_host_usage")]
    INFRA_HOST_USAGE,
    #[serde(rename = "ingested_logs_bytes_usage")]
    INGESTED_LOGS_BYTES_USAGE,
    #[serde(rename = "ingested_spans_bytes_usage")]
    INGESTED_SPANS_BYTES_USAGE,
    #[serde(rename = "invocations_usage")]
    INVOCATIONS_USAGE,
    #[serde(rename = "lambda_traced_invocations_usage")]
    LAMBDA_TRACED_INVOCATIONS_USAGE,
    #[serde(rename = "logs_indexed_15day_usage")]
    LOGS_INDEXED_15DAY_USAGE,
    #[serde(rename = "logs_indexed_180day_usage")]
    LOGS_INDEXED_180DAY_USAGE,
    #[serde(rename = "logs_indexed_30day_usage")]
    LOGS_INDEXED_30DAY_USAGE,
    #[serde(rename = "logs_indexed_360day_usage")]
    LOGS_INDEXED_360DAY_USAGE,
    #[serde(rename = "logs_indexed_3day_usage")]
    LOGS_INDEXED_3DAY_USAGE,
    #[serde(rename = "logs_indexed_45day_usage")]
    LOGS_INDEXED_45DAY_USAGE,
    #[serde(rename = "logs_indexed_60day_usage")]
    LOGS_INDEXED_60DAY_USAGE,
    #[serde(rename = "logs_indexed_7day_usage")]
    LOGS_INDEXED_7DAY_USAGE,
    #[serde(rename = "logs_indexed_90day_usage")]
    LOGS_INDEXED_90DAY_USAGE,
    #[serde(rename = "logs_indexed_custom_retention_usage")]
    LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
    #[serde(rename = "mobile_app_testing_usage")]
    MOBILE_APP_TESTING_USAGE,
    #[serde(rename = "ndm_netflow_usage")]
    NDM_NETFLOW_USAGE,
    #[serde(rename = "npm_host_usage")]
    NPM_HOST_USAGE,
    #[serde(rename = "obs_pipeline_bytes_usage")]
    OBS_PIPELINE_BYTES_USAGE,
    #[serde(rename = "profiled_container_usage")]
    PROFILED_CONTAINER_USAGE,
    #[serde(rename = "profiled_fargate_usage")]
    PROFILED_FARGATE_USAGE,
    #[serde(rename = "profiled_host_usage")]
    PROFILED_HOST_USAGE,
    #[serde(rename = "rum_browser_mobile_sessions_usage")]
    RUM_BROWSER_MOBILE_SESSIONS_USAGE,
    #[serde(rename = "rum_replay_sessions_usage")]
    RUM_REPLAY_SESSIONS_USAGE,
    #[serde(rename = "sds_scanned_bytes_usage")]
    SDS_SCANNED_BYTES_USAGE,
    #[serde(rename = "serverless_apps_usage")]
    SERVERLESS_APPS_USAGE,
    #[serde(rename = "siem_ingested_bytes_usage")]
    SIEM_INGESTED_BYTES_USAGE,
    #[serde(rename = "snmp_usage")]
    SNMP_USAGE,
    #[serde(rename = "universal_service_monitoring_usage")]
    UNIVERSAL_SERVICE_MONITORING_USAGE,
    #[serde(rename = "vuln_management_hosts_usage")]
    VULN_MANAGEMENT_HOSTS_USAGE,
}

impl ToString for HourlyUsageAttributionUsageType {
    fn to_string(&self) -> String {
        match self {
            Self::API_USAGE => String::from("api_usage"),
            Self::APM_FARGATE_USAGE => String::from("apm_fargate_usage"),
            Self::APM_HOST_USAGE => String::from("apm_host_usage"),
            Self::APM_USM_USAGE => String::from("apm_usm_usage"),
            Self::APPSEC_FARGATE_USAGE => String::from("appsec_fargate_usage"),
            Self::APPSEC_USAGE => String::from("appsec_usage"),
            Self::BROWSER_USAGE => String::from("browser_usage"),
            Self::CI_PIPELINE_INDEXED_SPANS_USAGE => {
                String::from("ci_pipeline_indexed_spans_usage")
            }
            Self::CI_TEST_INDEXED_SPANS_USAGE => String::from("ci_test_indexed_spans_usage"),
            Self::CI_VISIBILITY_ITR_USAGE => String::from("ci_visibility_itr_usage"),
            Self::CLOUD_SIEM_USAGE => String::from("cloud_siem_usage"),
            Self::CONTAINER_EXCL_AGENT_USAGE => String::from("container_excl_agent_usage"),
            Self::CONTAINER_USAGE => String::from("container_usage"),
            Self::CSPM_CONTAINERS_USAGE => String::from("cspm_containers_usage"),
            Self::CSPM_HOSTS_USAGE => String::from("cspm_hosts_usage"),
            Self::CUSTOM_EVENT_USAGE => String::from("custom_event_usage"),
            Self::CUSTOM_INGESTED_TIMESERIES_USAGE => {
                String::from("custom_ingested_timeseries_usage")
            }
            Self::CUSTOM_TIMESERIES_USAGE => String::from("custom_timeseries_usage"),
            Self::CWS_CONTAINERS_USAGE => String::from("cws_containers_usage"),
            Self::CWS_HOSTS_USAGE => String::from("cws_hosts_usage"),
            Self::DBM_HOSTS_USAGE => String::from("dbm_hosts_usage"),
            Self::DBM_QUERIES_USAGE => String::from("dbm_queries_usage"),
            Self::ESTIMATED_INDEXED_LOGS_USAGE => String::from("estimated_indexed_logs_usage"),
            Self::ESTIMATED_INDEXED_SPANS_USAGE => String::from("estimated_indexed_spans_usage"),
            Self::ESTIMATED_INGESTED_LOGS_USAGE => String::from("estimated_ingested_logs_usage"),
            Self::ESTIMATED_INGESTED_SPANS_USAGE => String::from("estimated_ingested_spans_usage"),
            Self::ESTIMATED_RUM_SESSIONS_USAGE => String::from("estimated_rum_sessions_usage"),
            Self::FARGATE_USAGE => String::from("fargate_usage"),
            Self::FUNCTIONS_USAGE => String::from("functions_usage"),
            Self::INDEXED_SPANS_USAGE => String::from("indexed_spans_usage"),
            Self::INFRA_HOST_USAGE => String::from("infra_host_usage"),
            Self::INGESTED_LOGS_BYTES_USAGE => String::from("ingested_logs_bytes_usage"),
            Self::INGESTED_SPANS_BYTES_USAGE => String::from("ingested_spans_bytes_usage"),
            Self::INVOCATIONS_USAGE => String::from("invocations_usage"),
            Self::LAMBDA_TRACED_INVOCATIONS_USAGE => {
                String::from("lambda_traced_invocations_usage")
            }
            Self::LOGS_INDEXED_15DAY_USAGE => String::from("logs_indexed_15day_usage"),
            Self::LOGS_INDEXED_180DAY_USAGE => String::from("logs_indexed_180day_usage"),
            Self::LOGS_INDEXED_30DAY_USAGE => String::from("logs_indexed_30day_usage"),
            Self::LOGS_INDEXED_360DAY_USAGE => String::from("logs_indexed_360day_usage"),
            Self::LOGS_INDEXED_3DAY_USAGE => String::from("logs_indexed_3day_usage"),
            Self::LOGS_INDEXED_45DAY_USAGE => String::from("logs_indexed_45day_usage"),
            Self::LOGS_INDEXED_60DAY_USAGE => String::from("logs_indexed_60day_usage"),
            Self::LOGS_INDEXED_7DAY_USAGE => String::from("logs_indexed_7day_usage"),
            Self::LOGS_INDEXED_90DAY_USAGE => String::from("logs_indexed_90day_usage"),
            Self::LOGS_INDEXED_CUSTOM_RETENTION_USAGE => {
                String::from("logs_indexed_custom_retention_usage")
            }
            Self::MOBILE_APP_TESTING_USAGE => String::from("mobile_app_testing_usage"),
            Self::NDM_NETFLOW_USAGE => String::from("ndm_netflow_usage"),
            Self::NPM_HOST_USAGE => String::from("npm_host_usage"),
            Self::OBS_PIPELINE_BYTES_USAGE => String::from("obs_pipeline_bytes_usage"),
            Self::PROFILED_CONTAINER_USAGE => String::from("profiled_container_usage"),
            Self::PROFILED_FARGATE_USAGE => String::from("profiled_fargate_usage"),
            Self::PROFILED_HOST_USAGE => String::from("profiled_host_usage"),
            Self::RUM_BROWSER_MOBILE_SESSIONS_USAGE => {
                String::from("rum_browser_mobile_sessions_usage")
            }
            Self::RUM_REPLAY_SESSIONS_USAGE => String::from("rum_replay_sessions_usage"),
            Self::SDS_SCANNED_BYTES_USAGE => String::from("sds_scanned_bytes_usage"),
            Self::SERVERLESS_APPS_USAGE => String::from("serverless_apps_usage"),
            Self::SIEM_INGESTED_BYTES_USAGE => String::from("siem_ingested_bytes_usage"),
            Self::SNMP_USAGE => String::from("snmp_usage"),
            Self::UNIVERSAL_SERVICE_MONITORING_USAGE => {
                String::from("universal_service_monitoring_usage")
            }
            Self::VULN_MANAGEMENT_HOSTS_USAGE => String::from("vuln_management_hosts_usage"),
        }
    }
}
