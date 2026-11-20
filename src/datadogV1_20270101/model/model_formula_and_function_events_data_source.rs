// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionEventsDataSource {
	EVENTS,
	LOGS,
	RUM,
	SYNTHETICS_TEST_RUNS,
	PRODUCT_ANALYTICS,
	NETWORK,
	PROFILES,
	NETWORK_DEVICE_FLOWS,
	RECOMMENDATIONS,
	SECURITY_FINDINGS,
	SECURITY_SIGNALS,
	CI_PIPELINES,
	DATABASE_QUERIES,
	LLM_OBSERVABILITY,
	AI_USAGE,
	AUDIT,
	CI_TESTS,
	ERRORS,
	SPANS,
	SECURITY_RUNTIME,
	APP_SEC_SPANS,
	APP_SEC_ENTITIES,
	BITS_AI_INVESTIGATIONS,
	CI_CODE_SCAN,
	CI_DEPENDENCIES,
	CI_GATES,
	CI_COVERAGE_REPORTS,
	DATA_OBSERVABILITY_EVENTS,
	DORA_METRICS,
	COMPLIANCE_FINDINGS,
	DEPLOYMENTS,
	DEPLOYMENT_GATES,
	EXPOSURES,
	FLAG_EVALUATIONS,
	NETWORK_PATH,
	OBSERVABILITY_PIPELINES,
	ON_CALL_EVENTS,
	ORCHESTRATION_TROUBLESHOOTING,
	SECURITY_HISTORICAL_RESULTS,
	SECURITY_INFO,
	SOFTWARE_INVENTORY,
	SYNTHETICS_BATCHES,
	SYNTHETICS_MAP,
	REPLAY,
	BEHAVIOR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FormulaAndFunctionEventsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::EVENTS => String::from("events"),
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::SYNTHETICS_TEST_RUNS => String::from("synthetics_test_runs"),
            Self::PRODUCT_ANALYTICS => String::from("product_analytics"),
            Self::NETWORK => String::from("network"),
            Self::PROFILES => String::from("profiles"),
            Self::NETWORK_DEVICE_FLOWS => String::from("network_device_flows"),
            Self::RECOMMENDATIONS => String::from("recommendations"),
            Self::SECURITY_FINDINGS => String::from("security_findings"),
            Self::SECURITY_SIGNALS => String::from("security_signals"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::DATABASE_QUERIES => String::from("database_queries"),
            Self::LLM_OBSERVABILITY => String::from("llm_observability"),
            Self::AI_USAGE => String::from("ai_usage"),
            Self::AUDIT => String::from("audit"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::ERRORS => String::from("errors"),
            Self::SPANS => String::from("spans"),
            Self::SECURITY_RUNTIME => String::from("security_runtime"),
            Self::APP_SEC_SPANS => String::from("app_sec_spans"),
            Self::APP_SEC_ENTITIES => String::from("app_sec_entities"),
            Self::BITS_AI_INVESTIGATIONS => String::from("bits_ai_investigations"),
            Self::CI_CODE_SCAN => String::from("ci_code_scan"),
            Self::CI_DEPENDENCIES => String::from("ci_dependencies"),
            Self::CI_GATES => String::from("ci_gates"),
            Self::CI_COVERAGE_REPORTS => String::from("ci_coverage_reports"),
            Self::DATA_OBSERVABILITY_EVENTS => String::from("data_observability_events"),
            Self::DORA_METRICS => String::from("dora_metrics"),
            Self::COMPLIANCE_FINDINGS => String::from("compliance_findings"),
            Self::DEPLOYMENTS => String::from("deployments"),
            Self::DEPLOYMENT_GATES => String::from("deployment_gates"),
            Self::EXPOSURES => String::from("exposures"),
            Self::FLAG_EVALUATIONS => String::from("flag_evaluations"),
            Self::NETWORK_PATH => String::from("network_path"),
            Self::OBSERVABILITY_PIPELINES => String::from("observability_pipelines"),
            Self::ON_CALL_EVENTS => String::from("on_call_events"),
            Self::ORCHESTRATION_TROUBLESHOOTING => String::from("orchestration_troubleshooting"),
            Self::SECURITY_HISTORICAL_RESULTS => String::from("security_historical_results"),
            Self::SECURITY_INFO => String::from("security_info"),
            Self::SOFTWARE_INVENTORY => String::from("software_inventory"),
            Self::SYNTHETICS_BATCHES => String::from("synthetics_batches"),
            Self::SYNTHETICS_MAP => String::from("synthetics_map"),
            Self::REPLAY => String::from("replay"),
            Self::BEHAVIOR => String::from("behavior"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormulaAndFunctionEventsDataSource {
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

impl<'de> Deserialize<'de> for FormulaAndFunctionEventsDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"events" => Self::EVENTS,"logs" => Self::LOGS,"rum" => Self::RUM,"synthetics_test_runs" => Self::SYNTHETICS_TEST_RUNS,"product_analytics" => Self::PRODUCT_ANALYTICS,"network" => Self::NETWORK,"profiles" => Self::PROFILES,"network_device_flows" => Self::NETWORK_DEVICE_FLOWS,"recommendations" => Self::RECOMMENDATIONS,"security_findings" => Self::SECURITY_FINDINGS,"security_signals" => Self::SECURITY_SIGNALS,"ci_pipelines" => Self::CI_PIPELINES,"database_queries" => Self::DATABASE_QUERIES,"llm_observability" => Self::LLM_OBSERVABILITY,"ai_usage" => Self::AI_USAGE,"audit" => Self::AUDIT,"ci_tests" => Self::CI_TESTS,"errors" => Self::ERRORS,"spans" => Self::SPANS,"security_runtime" => Self::SECURITY_RUNTIME,"app_sec_spans" => Self::APP_SEC_SPANS,"app_sec_entities" => Self::APP_SEC_ENTITIES,"bits_ai_investigations" => Self::BITS_AI_INVESTIGATIONS,"ci_code_scan" => Self::CI_CODE_SCAN,"ci_dependencies" => Self::CI_DEPENDENCIES,"ci_gates" => Self::CI_GATES,"ci_coverage_reports" => Self::CI_COVERAGE_REPORTS,"data_observability_events" => Self::DATA_OBSERVABILITY_EVENTS,"dora_metrics" => Self::DORA_METRICS,"compliance_findings" => Self::COMPLIANCE_FINDINGS,"deployments" => Self::DEPLOYMENTS,"deployment_gates" => Self::DEPLOYMENT_GATES,"exposures" => Self::EXPOSURES,"flag_evaluations" => Self::FLAG_EVALUATIONS,"network_path" => Self::NETWORK_PATH,"observability_pipelines" => Self::OBSERVABILITY_PIPELINES,"on_call_events" => Self::ON_CALL_EVENTS,"orchestration_troubleshooting" => Self::ORCHESTRATION_TROUBLESHOOTING,"security_historical_results" => Self::SECURITY_HISTORICAL_RESULTS,"security_info" => Self::SECURITY_INFO,"software_inventory" => Self::SOFTWARE_INVENTORY,"synthetics_batches" => Self::SYNTHETICS_BATCHES,"synthetics_map" => Self::SYNTHETICS_MAP,"replay" => Self::REPLAY,"behavior" => Self::BEHAVIOR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
