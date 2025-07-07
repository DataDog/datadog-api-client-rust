// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonthlyUsageAttributionSupportedMetrics {
    API_USAGE,
    API_PERCENTAGE,
    APM_FARGATE_USAGE,
    APM_FARGATE_PERCENTAGE,
    APPSEC_FARGATE_USAGE,
    APPSEC_FARGATE_PERCENTAGE,
    APM_HOST_USAGE,
    APM_HOST_PERCENTAGE,
    APM_USM_USAGE,
    APM_USM_PERCENTAGE,
    APPSEC_USAGE,
    APPSEC_PERCENTAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE,
    BROWSER_USAGE,
    BROWSER_PERCENTAGE,
    CI_VISIBILITY_ITR_USAGE,
    CI_VISIBILITY_ITR_PERCENTAGE,
    CLOUD_SIEM_USAGE,
    CLOUD_SIEM_PERCENTAGE,
    CODE_SECURITY_HOST_USAGE,
    CODE_SECURITY_HOST_PERCENTAGE,
    CONTAINER_EXCL_AGENT_USAGE,
    CONTAINER_EXCL_AGENT_PERCENTAGE,
    CONTAINER_USAGE,
    CONTAINER_PERCENTAGE,
    CSPM_CONTAINERS_PERCENTAGE,
    CSPM_CONTAINERS_USAGE,
    CSPM_HOSTS_PERCENTAGE,
    CSPM_HOSTS_USAGE,
    CUSTOM_TIMESERIES_USAGE,
    CUSTOM_TIMESERIES_PERCENTAGE,
    CUSTOM_INGESTED_TIMESERIES_USAGE,
    CUSTOM_INGESTED_TIMESERIES_PERCENTAGE,
    CWS_CONTAINERS_PERCENTAGE,
    CWS_CONTAINERS_USAGE,
    CWS_FARGATE_TASK_PERCENTAGE,
    CWS_FARGATE_TASK_USAGE,
    CWS_HOSTS_PERCENTAGE,
    CWS_HOSTS_USAGE,
    DATA_JOBS_MONITORING_USAGE,
    DATA_JOBS_MONITORING_PERCENTAGE,
    DATA_STREAM_MONITORING_USAGE,
    DATA_STREAM_MONITORING_PERCENTAGE,
    DBM_HOSTS_PERCENTAGE,
    DBM_HOSTS_USAGE,
    DBM_QUERIES_PERCENTAGE,
    DBM_QUERIES_USAGE,
    ERROR_TRACKING_USAGE,
    ERROR_TRACKING_PERCENTAGE,
    ESTIMATED_INDEXED_SPANS_USAGE,
    ESTIMATED_INDEXED_SPANS_PERCENTAGE,
    ESTIMATED_INGESTED_SPANS_USAGE,
    ESTIMATED_INGESTED_SPANS_PERCENTAGE,
    FARGATE_USAGE,
    FARGATE_PERCENTAGE,
    FUNCTIONS_USAGE,
    FUNCTIONS_PERCENTAGE,
    INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE,
    INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_PERCENTAGE,
    INFRA_HOST_USAGE,
    INFRA_HOST_PERCENTAGE,
    INVOCATIONS_USAGE,
    INVOCATIONS_PERCENTAGE,
    LAMBDA_TRACED_INVOCATIONS_USAGE,
    LAMBDA_TRACED_INVOCATIONS_PERCENTAGE,
    LLM_OBSERVABILITY_USAGE,
    LLM_OBSERVABILITY_PERCENTAGE,
    MOBILE_APP_TESTING_USAGE,
    MOBILE_APP_TESTING_PERCENTAGE,
    NDM_NETFLOW_USAGE,
    NDM_NETFLOW_PERCENTAGE,
    NETWORK_DEVICE_WIRELESS_USAGE,
    NETWORK_DEVICE_WIRELESS_PERCENTAGE,
    NPM_HOST_USAGE,
    NPM_HOST_PERCENTAGE,
    OBS_PIPELINE_BYTES_USAGE,
    OBS_PIPELINE_BYTES_PERCENTAGE,
    OBS_PIPELINES_VCPU_USAGE,
    OBS_PIPELINES_VCPU_PERCENTAGE,
    ONLINE_ARCHIVE_USAGE,
    ONLINE_ARCHIVE_PERCENTAGE,
    PRODUCT_ANALYTICS_SESSION_USAGE,
    PRODUCT_ANALYTICS_SESSION_PERCENTAGE,
    PROFILED_CONTAINER_USAGE,
    PROFILED_CONTAINER_PERCENTAGE,
    PROFILED_FARGATE_USAGE,
    PROFILED_FARGATE_PERCENTAGE,
    PROFILED_HOST_USAGE,
    PROFILED_HOST_PERCENTAGE,
    PUBLISHED_APP_USAGE,
    PUBLISHED_APP_PERCENTAGE,
    SERVERLESS_APPS_USAGE,
    SERVERLESS_APPS_PERCENTAGE,
    SNMP_USAGE,
    SNMP_PERCENTAGE,
    UNIVERSAL_SERVICE_MONITORING_USAGE,
    UNIVERSAL_SERVICE_MONITORING_PERCENTAGE,
    VULN_MANAGEMENT_HOSTS_USAGE,
    VULN_MANAGEMENT_HOSTS_PERCENTAGE,
    SDS_SCANNED_BYTES_USAGE,
    SDS_SCANNED_BYTES_PERCENTAGE,
    CI_TEST_INDEXED_SPANS_USAGE,
    CI_TEST_INDEXED_SPANS_PERCENTAGE,
    INGESTED_LOGS_BYTES_USAGE,
    INGESTED_LOGS_BYTES_PERCENTAGE,
    CI_PIPELINE_INDEXED_SPANS_USAGE,
    CI_PIPELINE_INDEXED_SPANS_PERCENTAGE,
    INDEXED_SPANS_USAGE,
    INDEXED_SPANS_PERCENTAGE,
    CUSTOM_EVENT_USAGE,
    CUSTOM_EVENT_PERCENTAGE,
    LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
    LOGS_INDEXED_CUSTOM_RETENTION_PERCENTAGE,
    LOGS_INDEXED_360DAY_USAGE,
    LOGS_INDEXED_360DAY_PERCENTAGE,
    LOGS_INDEXED_180DAY_USAGE,
    LOGS_INDEXED_180DAY_PERCENTAGE,
    LOGS_INDEXED_90DAY_USAGE,
    LOGS_INDEXED_90DAY_PERCENTAGE,
    LOGS_INDEXED_60DAY_USAGE,
    LOGS_INDEXED_60DAY_PERCENTAGE,
    LOGS_INDEXED_45DAY_USAGE,
    LOGS_INDEXED_45DAY_PERCENTAGE,
    LOGS_INDEXED_30DAY_USAGE,
    LOGS_INDEXED_30DAY_PERCENTAGE,
    LOGS_INDEXED_15DAY_USAGE,
    LOGS_INDEXED_15DAY_PERCENTAGE,
    LOGS_INDEXED_7DAY_USAGE,
    LOGS_INDEXED_7DAY_PERCENTAGE,
    LOGS_INDEXED_3DAY_USAGE,
    LOGS_INDEXED_3DAY_PERCENTAGE,
    LOGS_INDEXED_1DAY_USAGE,
    LOGS_INDEXED_1DAY_PERCENTAGE,
    RUM_INGESTED_USAGE,
    RUM_INGESTED_PERCENTAGE,
    RUM_INVESTIGATE_USAGE,
    RUM_INVESTIGATE_PERCENTAGE,
    RUM_REPLAY_SESSIONS_USAGE,
    RUM_REPLAY_SESSIONS_PERCENTAGE,
    RUM_SESSION_REPLAY_ADD_ON_USAGE,
    RUM_SESSION_REPLAY_ADD_ON_PERCENTAGE,
    RUM_BROWSER_MOBILE_SESSIONS_USAGE,
    RUM_BROWSER_MOBILE_SESSIONS_PERCENTAGE,
    INGESTED_SPANS_BYTES_USAGE,
    INGESTED_SPANS_BYTES_PERCENTAGE,
    SIEM_ANALYZED_LOGS_ADD_ON_USAGE,
    SIEM_ANALYZED_LOGS_ADD_ON_PERCENTAGE,
    SIEM_INGESTED_BYTES_USAGE,
    SIEM_INGESTED_BYTES_PERCENTAGE,
    WORKFLOW_EXECUTIONS_USAGE,
    WORKFLOW_EXECUTIONS_PERCENTAGE,
    SCA_FARGATE_USAGE,
    SCA_FARGATE_PERCENTAGE,
    ALL,
    UnparsedObject(crate::datadog::UnparsedObject),
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
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE => {
                String::from("asm_serverless_traced_invocations_usage")
            }
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE => {
                String::from("asm_serverless_traced_invocations_percentage")
            }
            Self::BROWSER_USAGE => String::from("browser_usage"),
            Self::BROWSER_PERCENTAGE => String::from("browser_percentage"),
            Self::CI_VISIBILITY_ITR_USAGE => String::from("ci_visibility_itr_usage"),
            Self::CI_VISIBILITY_ITR_PERCENTAGE => String::from("ci_visibility_itr_percentage"),
            Self::CLOUD_SIEM_USAGE => String::from("cloud_siem_usage"),
            Self::CLOUD_SIEM_PERCENTAGE => String::from("cloud_siem_percentage"),
            Self::CODE_SECURITY_HOST_USAGE => String::from("code_security_host_usage"),
            Self::CODE_SECURITY_HOST_PERCENTAGE => String::from("code_security_host_percentage"),
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
            Self::CWS_FARGATE_TASK_PERCENTAGE => String::from("cws_fargate_task_percentage"),
            Self::CWS_FARGATE_TASK_USAGE => String::from("cws_fargate_task_usage"),
            Self::CWS_HOSTS_PERCENTAGE => String::from("cws_hosts_percentage"),
            Self::CWS_HOSTS_USAGE => String::from("cws_hosts_usage"),
            Self::DATA_JOBS_MONITORING_USAGE => String::from("data_jobs_monitoring_usage"),
            Self::DATA_JOBS_MONITORING_PERCENTAGE => {
                String::from("data_jobs_monitoring_percentage")
            }
            Self::DATA_STREAM_MONITORING_USAGE => String::from("data_stream_monitoring_usage"),
            Self::DATA_STREAM_MONITORING_PERCENTAGE => {
                String::from("data_stream_monitoring_percentage")
            }
            Self::DBM_HOSTS_PERCENTAGE => String::from("dbm_hosts_percentage"),
            Self::DBM_HOSTS_USAGE => String::from("dbm_hosts_usage"),
            Self::DBM_QUERIES_PERCENTAGE => String::from("dbm_queries_percentage"),
            Self::DBM_QUERIES_USAGE => String::from("dbm_queries_usage"),
            Self::ERROR_TRACKING_USAGE => String::from("error_tracking_usage"),
            Self::ERROR_TRACKING_PERCENTAGE => String::from("error_tracking_percentage"),
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
            Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE => {
                String::from("incident_management_monthly_active_users_usage")
            }
            Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_PERCENTAGE => {
                String::from("incident_management_monthly_active_users_percentage")
            }
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
            Self::LLM_OBSERVABILITY_USAGE => String::from("llm_observability_usage"),
            Self::LLM_OBSERVABILITY_PERCENTAGE => String::from("llm_observability_percentage"),
            Self::MOBILE_APP_TESTING_USAGE => String::from("mobile_app_testing_percentage"),
            Self::MOBILE_APP_TESTING_PERCENTAGE => String::from("mobile_app_testing_usage"),
            Self::NDM_NETFLOW_USAGE => String::from("ndm_netflow_usage"),
            Self::NDM_NETFLOW_PERCENTAGE => String::from("ndm_netflow_percentage"),
            Self::NETWORK_DEVICE_WIRELESS_USAGE => String::from("network_device_wireless_usage"),
            Self::NETWORK_DEVICE_WIRELESS_PERCENTAGE => {
                String::from("network_device_wireless_percentage")
            }
            Self::NPM_HOST_USAGE => String::from("npm_host_usage"),
            Self::NPM_HOST_PERCENTAGE => String::from("npm_host_percentage"),
            Self::OBS_PIPELINE_BYTES_USAGE => String::from("obs_pipeline_bytes_usage"),
            Self::OBS_PIPELINE_BYTES_PERCENTAGE => String::from("obs_pipeline_bytes_percentage"),
            Self::OBS_PIPELINES_VCPU_USAGE => String::from("obs_pipelines_vcpu_usage"),
            Self::OBS_PIPELINES_VCPU_PERCENTAGE => String::from("obs_pipelines_vcpu_percentage"),
            Self::ONLINE_ARCHIVE_USAGE => String::from("online_archive_usage"),
            Self::ONLINE_ARCHIVE_PERCENTAGE => String::from("online_archive_percentage"),
            Self::PRODUCT_ANALYTICS_SESSION_USAGE => {
                String::from("product_analytics_session_usage")
            }
            Self::PRODUCT_ANALYTICS_SESSION_PERCENTAGE => {
                String::from("product_analytics_session_percentage")
            }
            Self::PROFILED_CONTAINER_USAGE => String::from("profiled_container_usage"),
            Self::PROFILED_CONTAINER_PERCENTAGE => String::from("profiled_container_percentage"),
            Self::PROFILED_FARGATE_USAGE => String::from("profiled_fargate_usage"),
            Self::PROFILED_FARGATE_PERCENTAGE => String::from("profiled_fargate_percentage"),
            Self::PROFILED_HOST_USAGE => String::from("profiled_host_usage"),
            Self::PROFILED_HOST_PERCENTAGE => String::from("profiled_host_percentage"),
            Self::PUBLISHED_APP_USAGE => String::from("published_app_usage"),
            Self::PUBLISHED_APP_PERCENTAGE => String::from("published_app_percentage"),
            Self::SERVERLESS_APPS_USAGE => String::from("serverless_apps_usage"),
            Self::SERVERLESS_APPS_PERCENTAGE => String::from("serverless_apps_percentage"),
            Self::SNMP_USAGE => String::from("snmp_usage"),
            Self::SNMP_PERCENTAGE => String::from("snmp_percentage"),
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
            Self::LOGS_INDEXED_1DAY_USAGE => String::from("logs_indexed_1day_usage"),
            Self::LOGS_INDEXED_1DAY_PERCENTAGE => String::from("logs_indexed_1day_percentage"),
            Self::RUM_INGESTED_USAGE => String::from("rum_ingested_usage"),
            Self::RUM_INGESTED_PERCENTAGE => String::from("rum_ingested_percentage"),
            Self::RUM_INVESTIGATE_USAGE => String::from("rum_investigate_usage"),
            Self::RUM_INVESTIGATE_PERCENTAGE => String::from("rum_investigate_percentage"),
            Self::RUM_REPLAY_SESSIONS_USAGE => String::from("rum_replay_sessions_usage"),
            Self::RUM_REPLAY_SESSIONS_PERCENTAGE => String::from("rum_replay_sessions_percentage"),
            Self::RUM_SESSION_REPLAY_ADD_ON_USAGE => {
                String::from("rum_session_replay_add_on_usage")
            }
            Self::RUM_SESSION_REPLAY_ADD_ON_PERCENTAGE => {
                String::from("rum_session_replay_add_on_percentage")
            }
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
            Self::SIEM_ANALYZED_LOGS_ADD_ON_USAGE => {
                String::from("siem_analyzed_logs_add_on_usage")
            }
            Self::SIEM_ANALYZED_LOGS_ADD_ON_PERCENTAGE => {
                String::from("siem_analyzed_logs_add_on_percentage")
            }
            Self::SIEM_INGESTED_BYTES_USAGE => String::from("siem_ingested_bytes_usage"),
            Self::SIEM_INGESTED_BYTES_PERCENTAGE => String::from("siem_ingested_bytes_percentage"),
            Self::WORKFLOW_EXECUTIONS_USAGE => String::from("workflow_executions_usage"),
            Self::WORKFLOW_EXECUTIONS_PERCENTAGE => String::from("workflow_executions_percentage"),
            Self::SCA_FARGATE_USAGE => String::from("sca_fargate_usage"),
            Self::SCA_FARGATE_PERCENTAGE => String::from("sca_fargate_percentage"),
            Self::ALL => String::from("*"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonthlyUsageAttributionSupportedMetrics {
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

impl<'de> Deserialize<'de> for MonthlyUsageAttributionSupportedMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "api_usage" => Self::API_USAGE,
            "api_percentage" => Self::API_PERCENTAGE,
            "apm_fargate_usage" => Self::APM_FARGATE_USAGE,
            "apm_fargate_percentage" => Self::APM_FARGATE_PERCENTAGE,
            "appsec_fargate_usage" => Self::APPSEC_FARGATE_USAGE,
            "appsec_fargate_percentage" => Self::APPSEC_FARGATE_PERCENTAGE,
            "apm_host_usage" => Self::APM_HOST_USAGE,
            "apm_host_percentage" => Self::APM_HOST_PERCENTAGE,
            "apm_usm_usage" => Self::APM_USM_USAGE,
            "apm_usm_percentage" => Self::APM_USM_PERCENTAGE,
            "appsec_usage" => Self::APPSEC_USAGE,
            "appsec_percentage" => Self::APPSEC_PERCENTAGE,
            "asm_serverless_traced_invocations_usage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE
            }
            "asm_serverless_traced_invocations_percentage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE
            }
            "browser_usage" => Self::BROWSER_USAGE,
            "browser_percentage" => Self::BROWSER_PERCENTAGE,
            "ci_visibility_itr_usage" => Self::CI_VISIBILITY_ITR_USAGE,
            "ci_visibility_itr_percentage" => Self::CI_VISIBILITY_ITR_PERCENTAGE,
            "cloud_siem_usage" => Self::CLOUD_SIEM_USAGE,
            "cloud_siem_percentage" => Self::CLOUD_SIEM_PERCENTAGE,
            "code_security_host_usage" => Self::CODE_SECURITY_HOST_USAGE,
            "code_security_host_percentage" => Self::CODE_SECURITY_HOST_PERCENTAGE,
            "container_excl_agent_usage" => Self::CONTAINER_EXCL_AGENT_USAGE,
            "container_excl_agent_percentage" => Self::CONTAINER_EXCL_AGENT_PERCENTAGE,
            "container_usage" => Self::CONTAINER_USAGE,
            "container_percentage" => Self::CONTAINER_PERCENTAGE,
            "cspm_containers_percentage" => Self::CSPM_CONTAINERS_PERCENTAGE,
            "cspm_containers_usage" => Self::CSPM_CONTAINERS_USAGE,
            "cspm_hosts_percentage" => Self::CSPM_HOSTS_PERCENTAGE,
            "cspm_hosts_usage" => Self::CSPM_HOSTS_USAGE,
            "custom_timeseries_usage" => Self::CUSTOM_TIMESERIES_USAGE,
            "custom_timeseries_percentage" => Self::CUSTOM_TIMESERIES_PERCENTAGE,
            "custom_ingested_timeseries_usage" => Self::CUSTOM_INGESTED_TIMESERIES_USAGE,
            "custom_ingested_timeseries_percentage" => Self::CUSTOM_INGESTED_TIMESERIES_PERCENTAGE,
            "cws_containers_percentage" => Self::CWS_CONTAINERS_PERCENTAGE,
            "cws_containers_usage" => Self::CWS_CONTAINERS_USAGE,
            "cws_fargate_task_percentage" => Self::CWS_FARGATE_TASK_PERCENTAGE,
            "cws_fargate_task_usage" => Self::CWS_FARGATE_TASK_USAGE,
            "cws_hosts_percentage" => Self::CWS_HOSTS_PERCENTAGE,
            "cws_hosts_usage" => Self::CWS_HOSTS_USAGE,
            "data_jobs_monitoring_usage" => Self::DATA_JOBS_MONITORING_USAGE,
            "data_jobs_monitoring_percentage" => Self::DATA_JOBS_MONITORING_PERCENTAGE,
            "data_stream_monitoring_usage" => Self::DATA_STREAM_MONITORING_USAGE,
            "data_stream_monitoring_percentage" => Self::DATA_STREAM_MONITORING_PERCENTAGE,
            "dbm_hosts_percentage" => Self::DBM_HOSTS_PERCENTAGE,
            "dbm_hosts_usage" => Self::DBM_HOSTS_USAGE,
            "dbm_queries_percentage" => Self::DBM_QUERIES_PERCENTAGE,
            "dbm_queries_usage" => Self::DBM_QUERIES_USAGE,
            "error_tracking_usage" => Self::ERROR_TRACKING_USAGE,
            "error_tracking_percentage" => Self::ERROR_TRACKING_PERCENTAGE,
            "estimated_indexed_spans_usage" => Self::ESTIMATED_INDEXED_SPANS_USAGE,
            "estimated_indexed_spans_percentage" => Self::ESTIMATED_INDEXED_SPANS_PERCENTAGE,
            "estimated_ingested_spans_usage" => Self::ESTIMATED_INGESTED_SPANS_USAGE,
            "estimated_ingested_spans_percentage" => Self::ESTIMATED_INGESTED_SPANS_PERCENTAGE,
            "fargate_usage" => Self::FARGATE_USAGE,
            "fargate_percentage" => Self::FARGATE_PERCENTAGE,
            "functions_usage" => Self::FUNCTIONS_USAGE,
            "functions_percentage" => Self::FUNCTIONS_PERCENTAGE,
            "incident_management_monthly_active_users_usage" => {
                Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE
            }
            "incident_management_monthly_active_users_percentage" => {
                Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_PERCENTAGE
            }
            "infra_host_usage" => Self::INFRA_HOST_USAGE,
            "infra_host_percentage" => Self::INFRA_HOST_PERCENTAGE,
            "invocations_usage" => Self::INVOCATIONS_USAGE,
            "invocations_percentage" => Self::INVOCATIONS_PERCENTAGE,
            "lambda_traced_invocations_usage" => Self::LAMBDA_TRACED_INVOCATIONS_USAGE,
            "lambda_traced_invocations_percentage" => Self::LAMBDA_TRACED_INVOCATIONS_PERCENTAGE,
            "llm_observability_usage" => Self::LLM_OBSERVABILITY_USAGE,
            "llm_observability_percentage" => Self::LLM_OBSERVABILITY_PERCENTAGE,
            "mobile_app_testing_percentage" => Self::MOBILE_APP_TESTING_USAGE,
            "mobile_app_testing_usage" => Self::MOBILE_APP_TESTING_PERCENTAGE,
            "ndm_netflow_usage" => Self::NDM_NETFLOW_USAGE,
            "ndm_netflow_percentage" => Self::NDM_NETFLOW_PERCENTAGE,
            "network_device_wireless_usage" => Self::NETWORK_DEVICE_WIRELESS_USAGE,
            "network_device_wireless_percentage" => Self::NETWORK_DEVICE_WIRELESS_PERCENTAGE,
            "npm_host_usage" => Self::NPM_HOST_USAGE,
            "npm_host_percentage" => Self::NPM_HOST_PERCENTAGE,
            "obs_pipeline_bytes_usage" => Self::OBS_PIPELINE_BYTES_USAGE,
            "obs_pipeline_bytes_percentage" => Self::OBS_PIPELINE_BYTES_PERCENTAGE,
            "obs_pipelines_vcpu_usage" => Self::OBS_PIPELINES_VCPU_USAGE,
            "obs_pipelines_vcpu_percentage" => Self::OBS_PIPELINES_VCPU_PERCENTAGE,
            "online_archive_usage" => Self::ONLINE_ARCHIVE_USAGE,
            "online_archive_percentage" => Self::ONLINE_ARCHIVE_PERCENTAGE,
            "product_analytics_session_usage" => Self::PRODUCT_ANALYTICS_SESSION_USAGE,
            "product_analytics_session_percentage" => Self::PRODUCT_ANALYTICS_SESSION_PERCENTAGE,
            "profiled_container_usage" => Self::PROFILED_CONTAINER_USAGE,
            "profiled_container_percentage" => Self::PROFILED_CONTAINER_PERCENTAGE,
            "profiled_fargate_usage" => Self::PROFILED_FARGATE_USAGE,
            "profiled_fargate_percentage" => Self::PROFILED_FARGATE_PERCENTAGE,
            "profiled_host_usage" => Self::PROFILED_HOST_USAGE,
            "profiled_host_percentage" => Self::PROFILED_HOST_PERCENTAGE,
            "published_app_usage" => Self::PUBLISHED_APP_USAGE,
            "published_app_percentage" => Self::PUBLISHED_APP_PERCENTAGE,
            "serverless_apps_usage" => Self::SERVERLESS_APPS_USAGE,
            "serverless_apps_percentage" => Self::SERVERLESS_APPS_PERCENTAGE,
            "snmp_usage" => Self::SNMP_USAGE,
            "snmp_percentage" => Self::SNMP_PERCENTAGE,
            "universal_service_monitoring_usage" => Self::UNIVERSAL_SERVICE_MONITORING_USAGE,
            "universal_service_monitoring_percentage" => {
                Self::UNIVERSAL_SERVICE_MONITORING_PERCENTAGE
            }
            "vuln_management_hosts_usage" => Self::VULN_MANAGEMENT_HOSTS_USAGE,
            "vuln_management_hosts_percentage" => Self::VULN_MANAGEMENT_HOSTS_PERCENTAGE,
            "sds_scanned_bytes_usage" => Self::SDS_SCANNED_BYTES_USAGE,
            "sds_scanned_bytes_percentage" => Self::SDS_SCANNED_BYTES_PERCENTAGE,
            "ci_test_indexed_spans_usage" => Self::CI_TEST_INDEXED_SPANS_USAGE,
            "ci_test_indexed_spans_percentage" => Self::CI_TEST_INDEXED_SPANS_PERCENTAGE,
            "ingested_logs_bytes_usage" => Self::INGESTED_LOGS_BYTES_USAGE,
            "ingested_logs_bytes_percentage" => Self::INGESTED_LOGS_BYTES_PERCENTAGE,
            "ci_pipeline_indexed_spans_usage" => Self::CI_PIPELINE_INDEXED_SPANS_USAGE,
            "ci_pipeline_indexed_spans_percentage" => Self::CI_PIPELINE_INDEXED_SPANS_PERCENTAGE,
            "indexed_spans_usage" => Self::INDEXED_SPANS_USAGE,
            "indexed_spans_percentage" => Self::INDEXED_SPANS_PERCENTAGE,
            "custom_event_usage" => Self::CUSTOM_EVENT_USAGE,
            "custom_event_percentage" => Self::CUSTOM_EVENT_PERCENTAGE,
            "logs_indexed_custom_retention_usage" => Self::LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
            "logs_indexed_custom_retention_percentage" => {
                Self::LOGS_INDEXED_CUSTOM_RETENTION_PERCENTAGE
            }
            "logs_indexed_360day_usage" => Self::LOGS_INDEXED_360DAY_USAGE,
            "logs_indexed_360day_percentage" => Self::LOGS_INDEXED_360DAY_PERCENTAGE,
            "logs_indexed_180day_usage" => Self::LOGS_INDEXED_180DAY_USAGE,
            "logs_indexed_180day_percentage" => Self::LOGS_INDEXED_180DAY_PERCENTAGE,
            "logs_indexed_90day_usage" => Self::LOGS_INDEXED_90DAY_USAGE,
            "logs_indexed_90day_percentage" => Self::LOGS_INDEXED_90DAY_PERCENTAGE,
            "logs_indexed_60day_usage" => Self::LOGS_INDEXED_60DAY_USAGE,
            "logs_indexed_60day_percentage" => Self::LOGS_INDEXED_60DAY_PERCENTAGE,
            "logs_indexed_45day_usage" => Self::LOGS_INDEXED_45DAY_USAGE,
            "logs_indexed_45day_percentage" => Self::LOGS_INDEXED_45DAY_PERCENTAGE,
            "logs_indexed_30day_usage" => Self::LOGS_INDEXED_30DAY_USAGE,
            "logs_indexed_30day_percentage" => Self::LOGS_INDEXED_30DAY_PERCENTAGE,
            "logs_indexed_15day_usage" => Self::LOGS_INDEXED_15DAY_USAGE,
            "logs_indexed_15day_percentage" => Self::LOGS_INDEXED_15DAY_PERCENTAGE,
            "logs_indexed_7day_usage" => Self::LOGS_INDEXED_7DAY_USAGE,
            "logs_indexed_7day_percentage" => Self::LOGS_INDEXED_7DAY_PERCENTAGE,
            "logs_indexed_3day_usage" => Self::LOGS_INDEXED_3DAY_USAGE,
            "logs_indexed_3day_percentage" => Self::LOGS_INDEXED_3DAY_PERCENTAGE,
            "logs_indexed_1day_usage" => Self::LOGS_INDEXED_1DAY_USAGE,
            "logs_indexed_1day_percentage" => Self::LOGS_INDEXED_1DAY_PERCENTAGE,
            "rum_ingested_usage" => Self::RUM_INGESTED_USAGE,
            "rum_ingested_percentage" => Self::RUM_INGESTED_PERCENTAGE,
            "rum_investigate_usage" => Self::RUM_INVESTIGATE_USAGE,
            "rum_investigate_percentage" => Self::RUM_INVESTIGATE_PERCENTAGE,
            "rum_replay_sessions_usage" => Self::RUM_REPLAY_SESSIONS_USAGE,
            "rum_replay_sessions_percentage" => Self::RUM_REPLAY_SESSIONS_PERCENTAGE,
            "rum_session_replay_add_on_usage" => Self::RUM_SESSION_REPLAY_ADD_ON_USAGE,
            "rum_session_replay_add_on_percentage" => Self::RUM_SESSION_REPLAY_ADD_ON_PERCENTAGE,
            "rum_browser_mobile_sessions_usage" => Self::RUM_BROWSER_MOBILE_SESSIONS_USAGE,
            "rum_browser_mobile_sessions_percentage" => {
                Self::RUM_BROWSER_MOBILE_SESSIONS_PERCENTAGE
            }
            "ingested_spans_bytes_usage" => Self::INGESTED_SPANS_BYTES_USAGE,
            "ingested_spans_bytes_percentage" => Self::INGESTED_SPANS_BYTES_PERCENTAGE,
            "siem_analyzed_logs_add_on_usage" => Self::SIEM_ANALYZED_LOGS_ADD_ON_USAGE,
            "siem_analyzed_logs_add_on_percentage" => Self::SIEM_ANALYZED_LOGS_ADD_ON_PERCENTAGE,
            "siem_ingested_bytes_usage" => Self::SIEM_INGESTED_BYTES_USAGE,
            "siem_ingested_bytes_percentage" => Self::SIEM_INGESTED_BYTES_PERCENTAGE,
            "workflow_executions_usage" => Self::WORKFLOW_EXECUTIONS_USAGE,
            "workflow_executions_percentage" => Self::WORKFLOW_EXECUTIONS_PERCENTAGE,
            "sca_fargate_usage" => Self::SCA_FARGATE_USAGE,
            "sca_fargate_percentage" => Self::SCA_FARGATE_PERCENTAGE,
            "*" => Self::ALL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
