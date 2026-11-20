// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextMenuLinkLabel {
	EVENTS,
	LOGS,
	RUM,
	PRODUCT_ANALYTICS,
	NETWORK,
	CLOUD_COST,
	FUNCTIONS,
	HOSTS,
	TRACES,
	PROFILES,
	PROCESSES,
	CONTAINERS,
	CHANGE_TRACKING,
	AUDIT_TRAIL,
	NETWORK_DEVICE_FLOWS,
	DEVICES,
	RECOMMENDATIONS,
	SECURITY_FINDINGS,
	SECURITY_SIGNALS,
	SECURITY_TRACES,
	INCIDENTS,
	CASES,
	METRICS_SUMMARY,
	PODS,
	CI_PIPELINES,
	DATABASE_QUERIES,
	WATCH_SESSIONS,
	GPU_MONITORING,
	LLM_OBSERVABILITY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ContextMenuLinkLabel {
    fn to_string(&self) -> String {
        match self {
            Self::EVENTS => String::from("events"),
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::PRODUCT_ANALYTICS => String::from("product_analytics"),
            Self::NETWORK => String::from("network"),
            Self::CLOUD_COST => String::from("cloud_cost"),
            Self::FUNCTIONS => String::from("functions"),
            Self::HOSTS => String::from("hosts"),
            Self::TRACES => String::from("traces"),
            Self::PROFILES => String::from("profiles"),
            Self::PROCESSES => String::from("processes"),
            Self::CONTAINERS => String::from("containers"),
            Self::CHANGE_TRACKING => String::from("change_tracking"),
            Self::AUDIT_TRAIL => String::from("audit_trail"),
            Self::NETWORK_DEVICE_FLOWS => String::from("network_device_flows"),
            Self::DEVICES => String::from("devices"),
            Self::RECOMMENDATIONS => String::from("recommendations"),
            Self::SECURITY_FINDINGS => String::from("security_findings"),
            Self::SECURITY_SIGNALS => String::from("security_signals"),
            Self::SECURITY_TRACES => String::from("security_traces"),
            Self::INCIDENTS => String::from("incidents"),
            Self::CASES => String::from("cases"),
            Self::METRICS_SUMMARY => String::from("metrics_summary"),
            Self::PODS => String::from("pods"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::DATABASE_QUERIES => String::from("database_queries"),
            Self::WATCH_SESSIONS => String::from("watch_sessions"),
            Self::GPU_MONITORING => String::from("gpu_monitoring"),
            Self::LLM_OBSERVABILITY => String::from("llm_observability"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ContextMenuLinkLabel {
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

impl<'de> Deserialize<'de> for ContextMenuLinkLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"events" => Self::EVENTS,"logs" => Self::LOGS,"rum" => Self::RUM,"product_analytics" => Self::PRODUCT_ANALYTICS,"network" => Self::NETWORK,"cloud_cost" => Self::CLOUD_COST,"functions" => Self::FUNCTIONS,"hosts" => Self::HOSTS,"traces" => Self::TRACES,"profiles" => Self::PROFILES,"processes" => Self::PROCESSES,"containers" => Self::CONTAINERS,"change_tracking" => Self::CHANGE_TRACKING,"audit_trail" => Self::AUDIT_TRAIL,"network_device_flows" => Self::NETWORK_DEVICE_FLOWS,"devices" => Self::DEVICES,"recommendations" => Self::RECOMMENDATIONS,"security_findings" => Self::SECURITY_FINDINGS,"security_signals" => Self::SECURITY_SIGNALS,"security_traces" => Self::SECURITY_TRACES,"incidents" => Self::INCIDENTS,"cases" => Self::CASES,"metrics_summary" => Self::METRICS_SUMMARY,"pods" => Self::PODS,"ci_pipelines" => Self::CI_PIPELINES,"database_queries" => Self::DATABASE_QUERIES,"watch_sessions" => Self::WATCH_SESSIONS,"gpu_monitoring" => Self::GPU_MONITORING,"llm_observability" => Self::LLM_OBSERVABILITY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
