// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventPlatformStreamDataSource {
	LOGS_STREAM,
	AUDIT_STREAM,
	TRACE_STREAM,
	ERRORS_STREAM,
	EVENT_STREAM,
	RUM_STREAM,
	CI_PIPELINE_STREAM,
	CI_TEST_STREAM,
	CI_GATE_STREAM,
	CI_CODE_SCAN_STREAM,
	CI_DEPENDENCY_STREAM,
	DEPLOYMENT_STREAM,
	RECOMMENDATIONS_STREAM,
	SYNTHETICS_BATCH_STREAM,
	SYNTHETICS_TEST_RUNS_STREAM,
	OBSERVABILITY_PIPELINES_STREAM,
	SECURITY_RUNTIME_STREAM,
	SECURITY_SIGNALS_STREAM,
	PRODUCT_ANALYTICS_STREAM,
	APP_SEC_SPAN_STREAM,
	APP_SEC_ENTITIES_STREAM,
	COMPLIANCE_EVENT_STREAM,
	COMPLIANCE_FINDINGS_STREAM,
	SECURITY_FINDINGS_STREAM,
	SUPPLY_CHAIN_FIREWALL_REPORT_STREAM,
	DATA_OBSERVABILITY_STREAM,
	DATABASE_QUERY_STREAM,
	DATABASE_RAW_QUERY_STREAM,
	DEPLOYMENT_GATES_STREAM,
	LLM_OBSERVABILITY_STREAM,
	NETWORK_DEVICE_FLOW_STREAM,
	NETWORK_FLOW_STREAM,
	NETWORK_PATH_STREAM,
	ON_CALL_EVENTS_STREAM,
	PROFILE_STREAM,
	SECURITY_DUMP_STREAM,
	SECURITY_HISTORICAL_RESULTS_STREAM,
	SESSION_REPLAY_STREAM,
	SOFTWARE_INVENTORY_STREAM,
	CRITICAL_ASSET_STREAM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for EventPlatformStreamDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS_STREAM => String::from("logs_stream"),
            Self::AUDIT_STREAM => String::from("audit_stream"),
            Self::TRACE_STREAM => String::from("trace_stream"),
            Self::ERRORS_STREAM => String::from("errors_stream"),
            Self::EVENT_STREAM => String::from("event_stream"),
            Self::RUM_STREAM => String::from("rum_stream"),
            Self::CI_PIPELINE_STREAM => String::from("ci_pipeline_stream"),
            Self::CI_TEST_STREAM => String::from("ci_test_stream"),
            Self::CI_GATE_STREAM => String::from("ci_gate_stream"),
            Self::CI_CODE_SCAN_STREAM => String::from("ci_code_scan_stream"),
            Self::CI_DEPENDENCY_STREAM => String::from("ci_dependency_stream"),
            Self::DEPLOYMENT_STREAM => String::from("deployment_stream"),
            Self::RECOMMENDATIONS_STREAM => String::from("recommendations_stream"),
            Self::SYNTHETICS_BATCH_STREAM => String::from("synthetics_batch_stream"),
            Self::SYNTHETICS_TEST_RUNS_STREAM => String::from("synthetics_test_runs_stream"),
            Self::OBSERVABILITY_PIPELINES_STREAM => String::from("observability_pipelines_stream"),
            Self::SECURITY_RUNTIME_STREAM => String::from("security_runtime_stream"),
            Self::SECURITY_SIGNALS_STREAM => String::from("security_signals_stream"),
            Self::PRODUCT_ANALYTICS_STREAM => String::from("product_analytics_stream"),
            Self::APP_SEC_SPAN_STREAM => String::from("app_sec_span_stream"),
            Self::APP_SEC_ENTITIES_STREAM => String::from("app_sec_entities_stream"),
            Self::COMPLIANCE_EVENT_STREAM => String::from("compliance_event_stream"),
            Self::COMPLIANCE_FINDINGS_STREAM => String::from("compliance_findings_stream"),
            Self::SECURITY_FINDINGS_STREAM => String::from("security_findings_stream"),
            Self::SUPPLY_CHAIN_FIREWALL_REPORT_STREAM => String::from("supply_chain_firewall_report_stream"),
            Self::DATA_OBSERVABILITY_STREAM => String::from("data_observability_stream"),
            Self::DATABASE_QUERY_STREAM => String::from("database_query_stream"),
            Self::DATABASE_RAW_QUERY_STREAM => String::from("database_raw_query_stream"),
            Self::DEPLOYMENT_GATES_STREAM => String::from("deployment_gates_stream"),
            Self::LLM_OBSERVABILITY_STREAM => String::from("llm_observability_stream"),
            Self::NETWORK_DEVICE_FLOW_STREAM => String::from("network_device_flow_stream"),
            Self::NETWORK_FLOW_STREAM => String::from("network_flow_stream"),
            Self::NETWORK_PATH_STREAM => String::from("network_path_stream"),
            Self::ON_CALL_EVENTS_STREAM => String::from("on_call_events_stream"),
            Self::PROFILE_STREAM => String::from("profile_stream"),
            Self::SECURITY_DUMP_STREAM => String::from("security_dump_stream"),
            Self::SECURITY_HISTORICAL_RESULTS_STREAM => String::from("security_historical_results_stream"),
            Self::SESSION_REPLAY_STREAM => String::from("session_replay_stream"),
            Self::SOFTWARE_INVENTORY_STREAM => String::from("software_inventory_stream"),
            Self::CRITICAL_ASSET_STREAM => String::from("critical_asset_stream"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for EventPlatformStreamDataSource {
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

impl<'de> Deserialize<'de> for EventPlatformStreamDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"logs_stream" => Self::LOGS_STREAM,"audit_stream" => Self::AUDIT_STREAM,"trace_stream" => Self::TRACE_STREAM,"errors_stream" => Self::ERRORS_STREAM,"event_stream" => Self::EVENT_STREAM,"rum_stream" => Self::RUM_STREAM,"ci_pipeline_stream" => Self::CI_PIPELINE_STREAM,"ci_test_stream" => Self::CI_TEST_STREAM,"ci_gate_stream" => Self::CI_GATE_STREAM,"ci_code_scan_stream" => Self::CI_CODE_SCAN_STREAM,"ci_dependency_stream" => Self::CI_DEPENDENCY_STREAM,"deployment_stream" => Self::DEPLOYMENT_STREAM,"recommendations_stream" => Self::RECOMMENDATIONS_STREAM,"synthetics_batch_stream" => Self::SYNTHETICS_BATCH_STREAM,"synthetics_test_runs_stream" => Self::SYNTHETICS_TEST_RUNS_STREAM,"observability_pipelines_stream" => Self::OBSERVABILITY_PIPELINES_STREAM,"security_runtime_stream" => Self::SECURITY_RUNTIME_STREAM,"security_signals_stream" => Self::SECURITY_SIGNALS_STREAM,"product_analytics_stream" => Self::PRODUCT_ANALYTICS_STREAM,"app_sec_span_stream" => Self::APP_SEC_SPAN_STREAM,"app_sec_entities_stream" => Self::APP_SEC_ENTITIES_STREAM,"compliance_event_stream" => Self::COMPLIANCE_EVENT_STREAM,"compliance_findings_stream" => Self::COMPLIANCE_FINDINGS_STREAM,"security_findings_stream" => Self::SECURITY_FINDINGS_STREAM,"supply_chain_firewall_report_stream" => Self::SUPPLY_CHAIN_FIREWALL_REPORT_STREAM,"data_observability_stream" => Self::DATA_OBSERVABILITY_STREAM,"database_query_stream" => Self::DATABASE_QUERY_STREAM,"database_raw_query_stream" => Self::DATABASE_RAW_QUERY_STREAM,"deployment_gates_stream" => Self::DEPLOYMENT_GATES_STREAM,"llm_observability_stream" => Self::LLM_OBSERVABILITY_STREAM,"network_device_flow_stream" => Self::NETWORK_DEVICE_FLOW_STREAM,"network_flow_stream" => Self::NETWORK_FLOW_STREAM,"network_path_stream" => Self::NETWORK_PATH_STREAM,"on_call_events_stream" => Self::ON_CALL_EVENTS_STREAM,"profile_stream" => Self::PROFILE_STREAM,"security_dump_stream" => Self::SECURITY_DUMP_STREAM,"security_historical_results_stream" => Self::SECURITY_HISTORICAL_RESULTS_STREAM,"session_replay_stream" => Self::SESSION_REPLAY_STREAM,"software_inventory_stream" => Self::SOFTWARE_INVENTORY_STREAM,"critical_asset_stream" => Self::CRITICAL_ASSET_STREAM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
