// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MonthlyUsageAttributionSupportedMetrics {
    #[serde(rename = "api_usage")]
    API_USAGE,
    #[serde(rename = "api_percentage")]
    API_PERCENTAGE,
    #[serde(rename = "apm_fargate_usage")]
    APM_FARGATE_USAGE,
    #[serde(rename = "apm_fargate_percentage")]
    APM_FARGATE_PERCENTAGE,
    #[serde(rename = "appsec_fargate_usage")]
    APPSEC_FARGATE_USAGE,
    #[serde(rename = "appsec_fargate_percentage")]
    APPSEC_FARGATE_PERCENTAGE,
    #[serde(rename = "apm_host_usage")]
    APM_HOST_USAGE,
    #[serde(rename = "apm_host_percentage")]
    APM_HOST_PERCENTAGE,
    #[serde(rename = "apm_usm_usage")]
    APM_USM_USAGE,
    #[serde(rename = "apm_usm_percentage")]
    APM_USM_PERCENTAGE,
    #[serde(rename = "appsec_usage")]
    APPSEC_USAGE,
    #[serde(rename = "appsec_percentage")]
    APPSEC_PERCENTAGE,
    #[serde(rename = "browser_usage")]
    BROWSER_USAGE,
    #[serde(rename = "browser_percentage")]
    BROWSER_PERCENTAGE,
    #[serde(rename = "ci_visibility_itr_usage")]
    CI_VISIBILITY_ITR_USAGE,
    #[serde(rename = "ci_visibility_itr_percentage")]
    CI_VISIBILITY_ITR_PERCENTAGE,
    #[serde(rename = "cloud_siem_usage")]
    CLOUD_SIEM_USAGE,
    #[serde(rename = "cloud_siem_percentage")]
    CLOUD_SIEM_PERCENTAGE,
    #[serde(rename = "container_excl_agent_usage")]
    CONTAINER_EXCL_AGENT_USAGE,
    #[serde(rename = "container_excl_agent_percentage")]
    CONTAINER_EXCL_AGENT_PERCENTAGE,
    #[serde(rename = "container_usage")]
    CONTAINER_USAGE,
    #[serde(rename = "container_percentage")]
    CONTAINER_PERCENTAGE,
    #[serde(rename = "cspm_containers_percentage")]
    CSPM_CONTAINERS_PERCENTAGE,
    #[serde(rename = "cspm_containers_usage")]
    CSPM_CONTAINERS_USAGE,
    #[serde(rename = "cspm_hosts_percentage")]
    CSPM_HOSTS_PERCENTAGE,
    #[serde(rename = "cspm_hosts_usage")]
    CSPM_HOSTS_USAGE,
    #[serde(rename = "custom_timeseries_usage")]
    CUSTOM_TIMESERIES_USAGE,
    #[serde(rename = "custom_timeseries_percentage")]
    CUSTOM_TIMESERIES_PERCENTAGE,
    #[serde(rename = "custom_ingested_timeseries_usage")]
    CUSTOM_INGESTED_TIMESERIES_USAGE,
    #[serde(rename = "custom_ingested_timeseries_percentage")]
    CUSTOM_INGESTED_TIMESERIES_PERCENTAGE,
    #[serde(rename = "cws_containers_percentage")]
    CWS_CONTAINERS_PERCENTAGE,
    #[serde(rename = "cws_containers_usage")]
    CWS_CONTAINERS_USAGE,
    #[serde(rename = "cws_hosts_percentage")]
    CWS_HOSTS_PERCENTAGE,
    #[serde(rename = "cws_hosts_usage")]
    CWS_HOSTS_USAGE,
    #[serde(rename = "dbm_hosts_percentage")]
    DBM_HOSTS_PERCENTAGE,
    #[serde(rename = "dbm_hosts_usage")]
    DBM_HOSTS_USAGE,
    #[serde(rename = "dbm_queries_percentage")]
    DBM_QUERIES_PERCENTAGE,
    #[serde(rename = "dbm_queries_usage")]
    DBM_QUERIES_USAGE,
    #[serde(rename = "estimated_indexed_logs_usage")]
    ESTIMATED_INDEXED_LOGS_USAGE,
    #[serde(rename = "estimated_indexed_logs_percentage")]
    ESTIMATED_INDEXED_LOGS_PERCENTAGE,
    #[serde(rename = "estimated_ingested_logs_usage")]
    ESTIMATED_INGESTED_LOGS_USAGE,
    #[serde(rename = "estimated_ingested_logs_percentage")]
    ESTIMATED_INGESTED_LOGS_PERCENTAGE,
    #[serde(rename = "estimated_indexed_spans_usage")]
    ESTIMATED_INDEXED_SPANS_USAGE,
    #[serde(rename = "estimated_indexed_spans_percentage")]
    ESTIMATED_INDEXED_SPANS_PERCENTAGE,
    #[serde(rename = "estimated_ingested_spans_usage")]
    ESTIMATED_INGESTED_SPANS_USAGE,
    #[serde(rename = "estimated_ingested_spans_percentage")]
    ESTIMATED_INGESTED_SPANS_PERCENTAGE,
    #[serde(rename = "fargate_usage")]
    FARGATE_USAGE,
    #[serde(rename = "fargate_percentage")]
    FARGATE_PERCENTAGE,
    #[serde(rename = "functions_usage")]
    FUNCTIONS_USAGE,
    #[serde(rename = "functions_percentage")]
    FUNCTIONS_PERCENTAGE,
    #[serde(rename = "infra_host_usage")]
    INFRA_HOST_USAGE,
    #[serde(rename = "infra_host_percentage")]
    INFRA_HOST_PERCENTAGE,
    #[serde(rename = "invocations_usage")]
    INVOCATIONS_USAGE,
    #[serde(rename = "invocations_percentage")]
    INVOCATIONS_PERCENTAGE,
    #[serde(rename = "lambda_traced_invocations_usage")]
    LAMBDA_TRACED_INVOCATIONS_USAGE,
    #[serde(rename = "lambda_traced_invocations_percentage")]
    LAMBDA_TRACED_INVOCATIONS_PERCENTAGE,
    #[serde(rename = "mobile_app_testing_percentage")]
    MOBILE_APP_TESTING_USAGE,
    #[serde(rename = "mobile_app_testing_usage")]
    MOBILE_APP_TESTING_PERCENTAGE,
    #[serde(rename = "ndm_netflow_usage")]
    NDM_NETFLOW_USAGE,
    #[serde(rename = "ndm_netflow_percentage")]
    NDM_NETFLOW_PERCENTAGE,
    #[serde(rename = "npm_host_usage")]
    NPM_HOST_USAGE,
    #[serde(rename = "npm_host_percentage")]
    NPM_HOST_PERCENTAGE,
    #[serde(rename = "obs_pipeline_bytes_usage")]
    OBS_PIPELINE_BYTES_USAGE,
    #[serde(rename = "obs_pipeline_bytes_percentage")]
    OBS_PIPELINE_BYTES_PERCENTAGE,
    #[serde(rename = "profiled_container_usage")]
    PROFILED_CONTAINER_USAGE,
    #[serde(rename = "profiled_container_percentage")]
    PROFILED_CONTAINER_PERCENTAGE,
    #[serde(rename = "profiled_fargate_usage")]
    PROFILED_FARGATE_USAGE,
    #[serde(rename = "profiled_fargate_percentage")]
    PROFILED_FARGATE_PERCENTAGE,
    #[serde(rename = "profiled_host_usage")]
    PROFILED_HOST_USAGE,
    #[serde(rename = "profiled_host_percentage")]
    PROFILED_HOST_PERCENTAGE,
    #[serde(rename = "serverless_apps_usage")]
    SERVERLESS_APPS_USAGE,
    #[serde(rename = "serverless_apps_percentage")]
    SERVERLESS_APPS_PERCENTAGE,
    #[serde(rename = "snmp_usage")]
    SNMP_USAGE,
    #[serde(rename = "snmp_percentage")]
    SNMP_PERCENTAGE,
    #[serde(rename = "estimated_rum_sessions_usage")]
    ESTIMATED_RUM_SESSIONS_USAGE,
    #[serde(rename = "estimated_rum_sessions_percentage")]
    ESTIMATED_RUM_SESSIONS_PERCENTAGE,
    #[serde(rename = "universal_service_monitoring_usage")]
    UNIVERSAL_SERVICE_MONITORING_USAGE,
    #[serde(rename = "universal_service_monitoring_percentage")]
    UNIVERSAL_SERVICE_MONITORING_PERCENTAGE,
    #[serde(rename = "vuln_management_hosts_usage")]
    VULN_MANAGEMENT_HOSTS_USAGE,
    #[serde(rename = "vuln_management_hosts_percentage")]
    VULN_MANAGEMENT_HOSTS_PERCENTAGE,
    #[serde(rename = "sds_scanned_bytes_usage")]
    SDS_SCANNED_BYTES_USAGE,
    #[serde(rename = "sds_scanned_bytes_percentage")]
    SDS_SCANNED_BYTES_PERCENTAGE,
    #[serde(rename = "ci_test_indexed_spans_usage")]
    CI_TEST_INDEXED_SPANS_USAGE,
    #[serde(rename = "ci_test_indexed_spans_percentage")]
    CI_TEST_INDEXED_SPANS_PERCENTAGE,
    #[serde(rename = "ingested_logs_bytes_usage")]
    INGESTED_LOGS_BYTES_USAGE,
    #[serde(rename = "ingested_logs_bytes_percentage")]
    INGESTED_LOGS_BYTES_PERCENTAGE,
    #[serde(rename = "ci_pipeline_indexed_spans_usage")]
    CI_PIPELINE_INDEXED_SPANS_USAGE,
    #[serde(rename = "ci_pipeline_indexed_spans_percentage")]
    CI_PIPELINE_INDEXED_SPANS_PERCENTAGE,
    #[serde(rename = "indexed_spans_usage")]
    INDEXED_SPANS_USAGE,
    #[serde(rename = "indexed_spans_percentage")]
    INDEXED_SPANS_PERCENTAGE,
    #[serde(rename = "custom_event_usage")]
    CUSTOM_EVENT_USAGE,
    #[serde(rename = "custom_event_percentage")]
    CUSTOM_EVENT_PERCENTAGE,
    #[serde(rename = "logs_indexed_custom_retention_usage")]
    LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
    #[serde(rename = "logs_indexed_custom_retention_percentage")]
    LOGS_INDEXED_CUSTOM_RETENTION_PERCENTAGE,
    #[serde(rename = "logs_indexed_360day_usage")]
    LOGS_INDEXED_360DAY_USAGE,
    #[serde(rename = "logs_indexed_360day_percentage")]
    LOGS_INDEXED_360DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_180day_usage")]
    LOGS_INDEXED_180DAY_USAGE,
    #[serde(rename = "logs_indexed_180day_percentage")]
    LOGS_INDEXED_180DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_90day_usage")]
    LOGS_INDEXED_90DAY_USAGE,
    #[serde(rename = "logs_indexed_90day_percentage")]
    LOGS_INDEXED_90DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_60day_usage")]
    LOGS_INDEXED_60DAY_USAGE,
    #[serde(rename = "logs_indexed_60day_percentage")]
    LOGS_INDEXED_60DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_45day_usage")]
    LOGS_INDEXED_45DAY_USAGE,
    #[serde(rename = "logs_indexed_45day_percentage")]
    LOGS_INDEXED_45DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_30day_usage")]
    LOGS_INDEXED_30DAY_USAGE,
    #[serde(rename = "logs_indexed_30day_percentage")]
    LOGS_INDEXED_30DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_15day_usage")]
    LOGS_INDEXED_15DAY_USAGE,
    #[serde(rename = "logs_indexed_15day_percentage")]
    LOGS_INDEXED_15DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_7day_usage")]
    LOGS_INDEXED_7DAY_USAGE,
    #[serde(rename = "logs_indexed_7day_percentage")]
    LOGS_INDEXED_7DAY_PERCENTAGE,
    #[serde(rename = "logs_indexed_3day_usage")]
    LOGS_INDEXED_3DAY_USAGE,
    #[serde(rename = "logs_indexed_3day_percentage")]
    LOGS_INDEXED_3DAY_PERCENTAGE,
    #[serde(rename = "rum_replay_sessions_usage")]
    RUM_REPLAY_SESSIONS_USAGE,
    #[serde(rename = "rum_replay_sessions_percentage")]
    RUM_REPLAY_SESSIONS_PERCENTAGE,
    #[serde(rename = "rum_browser_mobile_sessions_usage")]
    RUM_BROWSER_MOBILE_SESSIONS_USAGE,
    #[serde(rename = "rum_browser_mobile_sessions_percentage")]
    RUM_BROWSER_MOBILE_SESSIONS_PERCENTAGE,
    #[serde(rename = "ingested_spans_bytes_usage")]
    INGESTED_SPANS_BYTES_USAGE,
    #[serde(rename = "ingested_spans_bytes_percentage")]
    INGESTED_SPANS_BYTES_PERCENTAGE,
    #[serde(rename = "siem_ingested_bytes_usage")]
    SIEM_INGESTED_BYTES_USAGE,
    #[serde(rename = "siem_ingested_bytes_percentage")]
    SIEM_INGESTED_BYTES_PERCENTAGE,
    #[serde(rename = "*")]
    ALL,
}
impl ToString for MonthlyUsageAttributionSupportedMetrics {
    fn to_string(&self) -> String {
        match self {
            Self::API_USAGE => String::from("api_usage"),
            Self::API_PERCENTAGE => String::from("api_percentage"),
            Self::APM_FARGATE_USAGE => String::from("apm_fargate_usage"),
            Self::APM_FARGATE_PERCENTAGE => String::from("apm_fargate_percentage"),
            Self::APPSEC_FARGATE_USAGE => String::from("appsec_fargate_usage"),
            Self::APPSEC_FARGATE_PERCENTAGE => String::from("appsec_fargate_percentage"),
            Self::APM_HOST_USAGE => String::from("apm_host_usage"),
            Self::APM_HOST_PERCENTAGE => String::from("apm_host_percentage"),
            Self::APM_USM_USAGE => String::from("apm_usm_usage"),
            Self::APM_USM_PERCENTAGE => String::from("apm_usm_percentage"),
            Self::APPSEC_USAGE => String::from("appsec_usage"),
            Self::APPSEC_PERCENTAGE => String::from("appsec_percentage"),
            Self::BROWSER_USAGE => String::from("browser_usage"),
            Self::BROWSER_PERCENTAGE => String::from("browser_percentage"),
            Self::CI_VISIBILITY_ITR_USAGE => String::from("ci_visibility_itr_usage"),
            Self::CI_VISIBILITY_ITR_PERCENTAGE => String::from("ci_visibility_itr_percentage"),
            Self::CLOUD_SIEM_USAGE => String::from("cloud_siem_usage"),
            Self::CLOUD_SIEM_PERCENTAGE => String::from("cloud_siem_percentage"),
            Self::CONTAINER_EXCL_AGENT_USAGE => String::from("container_excl_agent_usage"),
            Self::CONTAINER_EXCL_AGENT_PERCENTAGE => {
                String::from("container_excl_agent_percentage")
            }
            Self::CONTAINER_USAGE => String::from("container_usage"),
            Self::CONTAINER_PERCENTAGE => String::from("container_percentage"),
            Self::CSPM_CONTAINERS_PERCENTAGE => String::from("cspm_containers_percentage"),
            Self::CSPM_CONTAINERS_USAGE => String::from("cspm_containers_usage"),
            Self::CSPM_HOSTS_PERCENTAGE => String::from("cspm_hosts_percentage"),
            Self::CSPM_HOSTS_USAGE => String::from("cspm_hosts_usage"),
            Self::CUSTOM_TIMESERIES_USAGE => String::from("custom_timeseries_usage"),
            Self::CUSTOM_TIMESERIES_PERCENTAGE => String::from("custom_timeseries_percentage"),
            Self::CUSTOM_INGESTED_TIMESERIES_USAGE => {
                String::from("custom_ingested_timeseries_usage")
            }
            Self::CUSTOM_INGESTED_TIMESERIES_PERCENTAGE => {
                String::from("custom_ingested_timeseries_percentage")
            }
            Self::CWS_CONTAINERS_PERCENTAGE => String::from("cws_containers_percentage"),
            Self::CWS_CONTAINERS_USAGE => String::from("cws_containers_usage"),
            Self::CWS_HOSTS_PERCENTAGE => String::from("cws_hosts_percentage"),
            Self::CWS_HOSTS_USAGE => String::from("cws_hosts_usage"),
            Self::DBM_HOSTS_PERCENTAGE => String::from("dbm_hosts_percentage"),
            Self::DBM_HOSTS_USAGE => String::from("dbm_hosts_usage"),
            Self::DBM_QUERIES_PERCENTAGE => String::from("dbm_queries_percentage"),
            Self::DBM_QUERIES_USAGE => String::from("dbm_queries_usage"),
            Self::ESTIMATED_INDEXED_LOGS_USAGE => String::from("estimated_indexed_logs_usage"),
            Self::ESTIMATED_INDEXED_LOGS_PERCENTAGE => {
                String::from("estimated_indexed_logs_percentage")
            }
            Self::ESTIMATED_INGESTED_LOGS_USAGE => String::from("estimated_ingested_logs_usage"),
            Self::ESTIMATED_INGESTED_LOGS_PERCENTAGE => {
                String::from("estimated_ingested_logs_percentage")
            }
            Self::ESTIMATED_INDEXED_SPANS_USAGE => String::from("estimated_indexed_spans_usage"),
            Self::ESTIMATED_INDEXED_SPANS_PERCENTAGE => {
                String::from("estimated_indexed_spans_percentage")
            }
            Self::ESTIMATED_INGESTED_SPANS_USAGE => String::from("estimated_ingested_spans_usage"),
            Self::ESTIMATED_INGESTED_SPANS_PERCENTAGE => {
                String::from("estimated_ingested_spans_percentage")
            }
            Self::FARGATE_USAGE => String::from("fargate_usage"),
            Self::FARGATE_PERCENTAGE => String::from("fargate_percentage"),
            Self::FUNCTIONS_USAGE => String::from("functions_usage"),
            Self::FUNCTIONS_PERCENTAGE => String::from("functions_percentage"),
            Self::INFRA_HOST_USAGE => String::from("infra_host_usage"),
            Self::INFRA_HOST_PERCENTAGE => String::from("infra_host_percentage"),
            Self::INVOCATIONS_USAGE => String::from("invocations_usage"),
            Self::INVOCATIONS_PERCENTAGE => String::from("invocations_percentage"),
            Self::LAMBDA_TRACED_INVOCATIONS_USAGE => {
                String::from("lambda_traced_invocations_usage")
            }
            Self::LAMBDA_TRACED_INVOCATIONS_PERCENTAGE => {
                String::from("lambda_traced_invocations_percentage")
            }
            Self::MOBILE_APP_TESTING_USAGE => String::from("mobile_app_testing_percentage"),
            Self::MOBILE_APP_TESTING_PERCENTAGE => String::from("mobile_app_testing_usage"),
            Self::NDM_NETFLOW_USAGE => String::from("ndm_netflow_usage"),
            Self::NDM_NETFLOW_PERCENTAGE => String::from("ndm_netflow_percentage"),
            Self::NPM_HOST_USAGE => String::from("npm_host_usage"),
            Self::NPM_HOST_PERCENTAGE => String::from("npm_host_percentage"),
            Self::OBS_PIPELINE_BYTES_USAGE => String::from("obs_pipeline_bytes_usage"),
            Self::OBS_PIPELINE_BYTES_PERCENTAGE => String::from("obs_pipeline_bytes_percentage"),
            Self::PROFILED_CONTAINER_USAGE => String::from("profiled_container_usage"),
            Self::PROFILED_CONTAINER_PERCENTAGE => String::from("profiled_container_percentage"),
            Self::PROFILED_FARGATE_USAGE => String::from("profiled_fargate_usage"),
            Self::PROFILED_FARGATE_PERCENTAGE => String::from("profiled_fargate_percentage"),
            Self::PROFILED_HOST_USAGE => String::from("profiled_host_usage"),
            Self::PROFILED_HOST_PERCENTAGE => String::from("profiled_host_percentage"),
            Self::SERVERLESS_APPS_USAGE => String::from("serverless_apps_usage"),
            Self::SERVERLESS_APPS_PERCENTAGE => String::from("serverless_apps_percentage"),
            Self::SNMP_USAGE => String::from("snmp_usage"),
            Self::SNMP_PERCENTAGE => String::from("snmp_percentage"),
            Self::ESTIMATED_RUM_SESSIONS_USAGE => String::from("estimated_rum_sessions_usage"),
            Self::ESTIMATED_RUM_SESSIONS_PERCENTAGE => {
                String::from("estimated_rum_sessions_percentage")
            }
            Self::UNIVERSAL_SERVICE_MONITORING_USAGE => {
                String::from("universal_service_monitoring_usage")
            }
            Self::UNIVERSAL_SERVICE_MONITORING_PERCENTAGE => {
                String::from("universal_service_monitoring_percentage")
            }
            Self::VULN_MANAGEMENT_HOSTS_USAGE => String::from("vuln_management_hosts_usage"),
            Self::VULN_MANAGEMENT_HOSTS_PERCENTAGE => {
                String::from("vuln_management_hosts_percentage")
            }
            Self::SDS_SCANNED_BYTES_USAGE => String::from("sds_scanned_bytes_usage"),
            Self::SDS_SCANNED_BYTES_PERCENTAGE => String::from("sds_scanned_bytes_percentage"),
            Self::CI_TEST_INDEXED_SPANS_USAGE => String::from("ci_test_indexed_spans_usage"),
            Self::CI_TEST_INDEXED_SPANS_PERCENTAGE => {
                String::from("ci_test_indexed_spans_percentage")
            }
            Self::INGESTED_LOGS_BYTES_USAGE => String::from("ingested_logs_bytes_usage"),
            Self::INGESTED_LOGS_BYTES_PERCENTAGE => String::from("ingested_logs_bytes_percentage"),
            Self::CI_PIPELINE_INDEXED_SPANS_USAGE => {
                String::from("ci_pipeline_indexed_spans_usage")
            }
            Self::CI_PIPELINE_INDEXED_SPANS_PERCENTAGE => {
                String::from("ci_pipeline_indexed_spans_percentage")
            }
            Self::INDEXED_SPANS_USAGE => String::from("indexed_spans_usage"),
            Self::INDEXED_SPANS_PERCENTAGE => String::from("indexed_spans_percentage"),
            Self::CUSTOM_EVENT_USAGE => String::from("custom_event_usage"),
            Self::CUSTOM_EVENT_PERCENTAGE => String::from("custom_event_percentage"),
            Self::LOGS_INDEXED_CUSTOM_RETENTION_USAGE => {
                String::from("logs_indexed_custom_retention_usage")
            }
            Self::LOGS_INDEXED_CUSTOM_RETENTION_PERCENTAGE => {
                String::from("logs_indexed_custom_retention_percentage")
            }
            Self::LOGS_INDEXED_360DAY_USAGE => String::from("logs_indexed_360day_usage"),
            Self::LOGS_INDEXED_360DAY_PERCENTAGE => String::from("logs_indexed_360day_percentage"),
            Self::LOGS_INDEXED_180DAY_USAGE => String::from("logs_indexed_180day_usage"),
            Self::LOGS_INDEXED_180DAY_PERCENTAGE => String::from("logs_indexed_180day_percentage"),
            Self::LOGS_INDEXED_90DAY_USAGE => String::from("logs_indexed_90day_usage"),
            Self::LOGS_INDEXED_90DAY_PERCENTAGE => String::from("logs_indexed_90day_percentage"),
            Self::LOGS_INDEXED_60DAY_USAGE => String::from("logs_indexed_60day_usage"),
            Self::LOGS_INDEXED_60DAY_PERCENTAGE => String::from("logs_indexed_60day_percentage"),
            Self::LOGS_INDEXED_45DAY_USAGE => String::from("logs_indexed_45day_usage"),
            Self::LOGS_INDEXED_45DAY_PERCENTAGE => String::from("logs_indexed_45day_percentage"),
            Self::LOGS_INDEXED_30DAY_USAGE => String::from("logs_indexed_30day_usage"),
            Self::LOGS_INDEXED_30DAY_PERCENTAGE => String::from("logs_indexed_30day_percentage"),
            Self::LOGS_INDEXED_15DAY_USAGE => String::from("logs_indexed_15day_usage"),
            Self::LOGS_INDEXED_15DAY_PERCENTAGE => String::from("logs_indexed_15day_percentage"),
            Self::LOGS_INDEXED_7DAY_USAGE => String::from("logs_indexed_7day_usage"),
            Self::LOGS_INDEXED_7DAY_PERCENTAGE => String::from("logs_indexed_7day_percentage"),
            Self::LOGS_INDEXED_3DAY_USAGE => String::from("logs_indexed_3day_usage"),
            Self::LOGS_INDEXED_3DAY_PERCENTAGE => String::from("logs_indexed_3day_percentage"),
            Self::RUM_REPLAY_SESSIONS_USAGE => String::from("rum_replay_sessions_usage"),
            Self::RUM_REPLAY_SESSIONS_PERCENTAGE => String::from("rum_replay_sessions_percentage"),
            Self::RUM_BROWSER_MOBILE_SESSIONS_USAGE => {
                String::from("rum_browser_mobile_sessions_usage")
            }
            Self::RUM_BROWSER_MOBILE_SESSIONS_PERCENTAGE => {
                String::from("rum_browser_mobile_sessions_percentage")
            }
            Self::INGESTED_SPANS_BYTES_USAGE => String::from("ingested_spans_bytes_usage"),
            Self::INGESTED_SPANS_BYTES_PERCENTAGE => {
                String::from("ingested_spans_bytes_percentage")
            }
            Self::SIEM_INGESTED_BYTES_USAGE => String::from("siem_ingested_bytes_usage"),
            Self::SIEM_INGESTED_BYTES_PERCENTAGE => String::from("siem_ingested_bytes_percentage"),
            Self::ALL => String::from("*"),
        }
    }
}
