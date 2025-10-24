// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Monitor options for deployment rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsMonitor {
    /// Seconds the monitor needs to stay in OK status for the rule to pass.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Monitors that match this query are evaluated.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsMonitor {
    pub fn new(query: String) -> DeploymentRuleOptionsMonitor {
        DeploymentRuleOptionsMonitor {
            duration: None,
            query,
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DeploymentRuleOptionsMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsMonitorVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsMonitorVisitor {
            type Value = DeploymentRuleOptionsMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = DeploymentRuleOptionsMonitor {
                    duration,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsMonitorVisitor)
    }
}
