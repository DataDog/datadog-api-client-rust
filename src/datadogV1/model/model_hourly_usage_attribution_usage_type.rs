// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HourlyUsageAttributionUsageType {
    API_USAGE,
    APM_FARGATE_USAGE,
    APM_HOST_USAGE,
    APM_USM_USAGE,
    APPSEC_FARGATE_USAGE,
    APPSEC_USAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE,
    ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE,
    BROWSER_USAGE,
    CI_PIPELINE_INDEXED_SPANS_USAGE,
    CI_TEST_INDEXED_SPANS_USAGE,
    CI_VISIBILITY_ITR_USAGE,
    CLOUD_SIEM_USAGE,
    CONTAINER_EXCL_AGENT_USAGE,
    CONTAINER_USAGE,
    CSPM_CONTAINERS_USAGE,
    CSPM_HOSTS_USAGE,
    CUSTOM_EVENT_USAGE,
    CUSTOM_INGESTED_TIMESERIES_USAGE,
    CUSTOM_TIMESERIES_USAGE,
    CWS_CONTAINERS_USAGE,
    CWS_HOSTS_USAGE,
    DBM_HOSTS_USAGE,
    DBM_QUERIES_USAGE,
    ERROR_TRACKING_USAGE,
    ERROR_TRACKING_PERCENTAGE,
    ESTIMATED_INDEXED_LOGS_USAGE,
    ESTIMATED_INDEXED_SPANS_USAGE,
    ESTIMATED_INGESTED_LOGS_USAGE,
    ESTIMATED_INGESTED_SPANS_USAGE,
    ESTIMATED_RUM_SESSIONS_USAGE,
    FARGATE_USAGE,
    FUNCTIONS_USAGE,
    INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE,
    INDEXED_SPANS_USAGE,
    INFRA_HOST_USAGE,
    INGESTED_LOGS_BYTES_USAGE,
    INGESTED_SPANS_BYTES_USAGE,
    INVOCATIONS_USAGE,
    LAMBDA_TRACED_INVOCATIONS_USAGE,
    LOGS_INDEXED_15DAY_USAGE,
    LOGS_INDEXED_180DAY_USAGE,
    LOGS_INDEXED_1DAY_USAGE,
    LOGS_INDEXED_30DAY_USAGE,
    LOGS_INDEXED_360DAY_USAGE,
    LOGS_INDEXED_3DAY_USAGE,
    LOGS_INDEXED_45DAY_USAGE,
    LOGS_INDEXED_60DAY_USAGE,
    LOGS_INDEXED_7DAY_USAGE,
    LOGS_INDEXED_90DAY_USAGE,
    LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
    MOBILE_APP_TESTING_USAGE,
    NDM_NETFLOW_USAGE,
    NPM_HOST_USAGE,
    OBS_PIPELINE_BYTES_USAGE,
    OBS_PIPELINE_VCPU_USAGE,
    ONLINE_ARCHIVE_USAGE,
    PROFILED_CONTAINER_USAGE,
    PROFILED_FARGATE_USAGE,
    PROFILED_HOST_USAGE,
    RUM_BROWSER_MOBILE_SESSIONS_USAGE,
    RUM_REPLAY_SESSIONS_USAGE,
    SDS_SCANNED_BYTES_USAGE,
    SERVERLESS_APPS_USAGE,
    SIEM_INGESTED_BYTES_USAGE,
    SNMP_USAGE,
    UNIVERSAL_SERVICE_MONITORING_USAGE,
    VULN_MANAGEMENT_HOSTS_USAGE,
    WORKFLOW_EXECUTIONS_USAGE,
    UnparsedObject(crate::datadog::UnparsedObject),
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
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE => {
                String::from("asm_serverless_traced_invocations_usage")
            }
            Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE => {
                String::from("asm_serverless_traced_invocations_percentage")
            }
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
            Self::ERROR_TRACKING_USAGE => String::from("error_tracking_usage"),
            Self::ERROR_TRACKING_PERCENTAGE => String::from("error_tracking_percentage"),
            Self::ESTIMATED_INDEXED_LOGS_USAGE => String::from("estimated_indexed_logs_usage"),
            Self::ESTIMATED_INDEXED_SPANS_USAGE => String::from("estimated_indexed_spans_usage"),
            Self::ESTIMATED_INGESTED_LOGS_USAGE => String::from("estimated_ingested_logs_usage"),
            Self::ESTIMATED_INGESTED_SPANS_USAGE => String::from("estimated_ingested_spans_usage"),
            Self::ESTIMATED_RUM_SESSIONS_USAGE => String::from("estimated_rum_sessions_usage"),
            Self::FARGATE_USAGE => String::from("fargate_usage"),
            Self::FUNCTIONS_USAGE => String::from("functions_usage"),
            Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE => {
                String::from("incident_management_monthly_active_users_usage")
            }
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
            Self::LOGS_INDEXED_1DAY_USAGE => String::from("logs_indexed_1day_usage"),
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
            Self::OBS_PIPELINE_VCPU_USAGE => String::from("obs_pipelines_vcpu_usage"),
            Self::ONLINE_ARCHIVE_USAGE => String::from("online_archive_usage"),
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
            Self::WORKFLOW_EXECUTIONS_USAGE => String::from("workflow_executions_usage"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for HourlyUsageAttributionUsageType {
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

impl<'de> Deserialize<'de> for HourlyUsageAttributionUsageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "api_usage" => Self::API_USAGE,
            "apm_fargate_usage" => Self::APM_FARGATE_USAGE,
            "apm_host_usage" => Self::APM_HOST_USAGE,
            "apm_usm_usage" => Self::APM_USM_USAGE,
            "appsec_fargate_usage" => Self::APPSEC_FARGATE_USAGE,
            "appsec_usage" => Self::APPSEC_USAGE,
            "asm_serverless_traced_invocations_usage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_USAGE
            }
            "asm_serverless_traced_invocations_percentage" => {
                Self::ASM_SERVERLESS_TRACED_INVOCATIONS_PERCENTAGE
            }
            "browser_usage" => Self::BROWSER_USAGE,
            "ci_pipeline_indexed_spans_usage" => Self::CI_PIPELINE_INDEXED_SPANS_USAGE,
            "ci_test_indexed_spans_usage" => Self::CI_TEST_INDEXED_SPANS_USAGE,
            "ci_visibility_itr_usage" => Self::CI_VISIBILITY_ITR_USAGE,
            "cloud_siem_usage" => Self::CLOUD_SIEM_USAGE,
            "container_excl_agent_usage" => Self::CONTAINER_EXCL_AGENT_USAGE,
            "container_usage" => Self::CONTAINER_USAGE,
            "cspm_containers_usage" => Self::CSPM_CONTAINERS_USAGE,
            "cspm_hosts_usage" => Self::CSPM_HOSTS_USAGE,
            "custom_event_usage" => Self::CUSTOM_EVENT_USAGE,
            "custom_ingested_timeseries_usage" => Self::CUSTOM_INGESTED_TIMESERIES_USAGE,
            "custom_timeseries_usage" => Self::CUSTOM_TIMESERIES_USAGE,
            "cws_containers_usage" => Self::CWS_CONTAINERS_USAGE,
            "cws_hosts_usage" => Self::CWS_HOSTS_USAGE,
            "dbm_hosts_usage" => Self::DBM_HOSTS_USAGE,
            "dbm_queries_usage" => Self::DBM_QUERIES_USAGE,
            "error_tracking_usage" => Self::ERROR_TRACKING_USAGE,
            "error_tracking_percentage" => Self::ERROR_TRACKING_PERCENTAGE,
            "estimated_indexed_logs_usage" => Self::ESTIMATED_INDEXED_LOGS_USAGE,
            "estimated_indexed_spans_usage" => Self::ESTIMATED_INDEXED_SPANS_USAGE,
            "estimated_ingested_logs_usage" => Self::ESTIMATED_INGESTED_LOGS_USAGE,
            "estimated_ingested_spans_usage" => Self::ESTIMATED_INGESTED_SPANS_USAGE,
            "estimated_rum_sessions_usage" => Self::ESTIMATED_RUM_SESSIONS_USAGE,
            "fargate_usage" => Self::FARGATE_USAGE,
            "functions_usage" => Self::FUNCTIONS_USAGE,
            "incident_management_monthly_active_users_usage" => {
                Self::INCIDENT_MANAGEMENT_MONTHLY_ACTIVE_USERS_USAGE
            }
            "indexed_spans_usage" => Self::INDEXED_SPANS_USAGE,
            "infra_host_usage" => Self::INFRA_HOST_USAGE,
            "ingested_logs_bytes_usage" => Self::INGESTED_LOGS_BYTES_USAGE,
            "ingested_spans_bytes_usage" => Self::INGESTED_SPANS_BYTES_USAGE,
            "invocations_usage" => Self::INVOCATIONS_USAGE,
            "lambda_traced_invocations_usage" => Self::LAMBDA_TRACED_INVOCATIONS_USAGE,
            "logs_indexed_15day_usage" => Self::LOGS_INDEXED_15DAY_USAGE,
            "logs_indexed_180day_usage" => Self::LOGS_INDEXED_180DAY_USAGE,
            "logs_indexed_1day_usage" => Self::LOGS_INDEXED_1DAY_USAGE,
            "logs_indexed_30day_usage" => Self::LOGS_INDEXED_30DAY_USAGE,
            "logs_indexed_360day_usage" => Self::LOGS_INDEXED_360DAY_USAGE,
            "logs_indexed_3day_usage" => Self::LOGS_INDEXED_3DAY_USAGE,
            "logs_indexed_45day_usage" => Self::LOGS_INDEXED_45DAY_USAGE,
            "logs_indexed_60day_usage" => Self::LOGS_INDEXED_60DAY_USAGE,
            "logs_indexed_7day_usage" => Self::LOGS_INDEXED_7DAY_USAGE,
            "logs_indexed_90day_usage" => Self::LOGS_INDEXED_90DAY_USAGE,
            "logs_indexed_custom_retention_usage" => Self::LOGS_INDEXED_CUSTOM_RETENTION_USAGE,
            "mobile_app_testing_usage" => Self::MOBILE_APP_TESTING_USAGE,
            "ndm_netflow_usage" => Self::NDM_NETFLOW_USAGE,
            "npm_host_usage" => Self::NPM_HOST_USAGE,
            "obs_pipeline_bytes_usage" => Self::OBS_PIPELINE_BYTES_USAGE,
            "obs_pipelines_vcpu_usage" => Self::OBS_PIPELINE_VCPU_USAGE,
            "online_archive_usage" => Self::ONLINE_ARCHIVE_USAGE,
            "profiled_container_usage" => Self::PROFILED_CONTAINER_USAGE,
            "profiled_fargate_usage" => Self::PROFILED_FARGATE_USAGE,
            "profiled_host_usage" => Self::PROFILED_HOST_USAGE,
            "rum_browser_mobile_sessions_usage" => Self::RUM_BROWSER_MOBILE_SESSIONS_USAGE,
            "rum_replay_sessions_usage" => Self::RUM_REPLAY_SESSIONS_USAGE,
            "sds_scanned_bytes_usage" => Self::SDS_SCANNED_BYTES_USAGE,
            "serverless_apps_usage" => Self::SERVERLESS_APPS_USAGE,
            "siem_ingested_bytes_usage" => Self::SIEM_INGESTED_BYTES_USAGE,
            "snmp_usage" => Self::SNMP_USAGE,
            "universal_service_monitoring_usage" => Self::UNIVERSAL_SERVICE_MONITORING_USAGE,
            "vuln_management_hosts_usage" => Self::VULN_MANAGEMENT_HOSTS_USAGE,
            "workflow_executions_usage" => Self::WORKFLOW_EXECUTIONS_USAGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
