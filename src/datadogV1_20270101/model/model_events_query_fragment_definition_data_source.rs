// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventsQueryFragmentDefinitionDataSource {
	EVENTS,
	LOGS,
	RUM,
	SYNTHETICS_TEST_RUNS,
	PRODUCT_ANALYTICS,
	NETWORK_DEVICE_FLOWS,
	CI_PIPELINES,
	DATABASE_QUERIES,
	LLM_OBSERVABILITY,
	AUDIT,
	CI_TESTS,
	SPANS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for EventsQueryFragmentDefinitionDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::EVENTS => String::from("events"),
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::SYNTHETICS_TEST_RUNS => String::from("synthetics_test_runs"),
            Self::PRODUCT_ANALYTICS => String::from("product_analytics"),
            Self::NETWORK_DEVICE_FLOWS => String::from("network_device_flows"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::DATABASE_QUERIES => String::from("database_queries"),
            Self::LLM_OBSERVABILITY => String::from("llm_observability"),
            Self::AUDIT => String::from("audit"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::SPANS => String::from("spans"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for EventsQueryFragmentDefinitionDataSource {
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

impl<'de> Deserialize<'de> for EventsQueryFragmentDefinitionDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"events" => Self::EVENTS,"logs" => Self::LOGS,"rum" => Self::RUM,"synthetics_test_runs" => Self::SYNTHETICS_TEST_RUNS,"product_analytics" => Self::PRODUCT_ANALYTICS,"network_device_flows" => Self::NETWORK_DEVICE_FLOWS,"ci_pipelines" => Self::CI_PIPELINES,"database_queries" => Self::DATABASE_QUERIES,"llm_observability" => Self::LLM_OBSERVABILITY,"audit" => Self::AUDIT,"ci_tests" => Self::CI_TESTS,"spans" => Self::SPANS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
