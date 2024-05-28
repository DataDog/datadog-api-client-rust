// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of the test of the rule queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleTestResponse {
    /// Assert results are returned in the same order as the rule query payloads.
    /// For each payload, it returns True if the result matched the expected result,
    /// False otherwise.
    #[serde(rename = "results")]
    pub results: Option<Vec<bool>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleTestResponse {
    pub fn new() -> SecurityMonitoringRuleTestResponse {
        SecurityMonitoringRuleTestResponse {
            results: None,
            _unparsed: false,
        }
    }

    pub fn results(mut self, value: Vec<bool>) -> Self {
        self.results = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleTestResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleTestResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleTestResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleTestResponseVisitor {
            type Value = SecurityMonitoringRuleTestResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut results: Option<Vec<bool>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringRuleTestResponse { results, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleTestResponseVisitor)
    }
}
