// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AI-generated analysis details for a replay issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisLLMDetails {
    /// Interpreted user intent derived from session analysis.
    #[serde(rename = "intent")]
    pub intent: String,
    /// List of user behavior steps observed in the session.
    #[serde(rename = "user_pattern")]
    pub user_pattern: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisLLMDetails {
    pub fn new(intent: String, user_pattern: Vec<String>) -> ReplayAnalysisLLMDetails {
        ReplayAnalysisLLMDetails {
            intent,
            user_pattern,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ReplayAnalysisLLMDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisLLMDetailsVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisLLMDetailsVisitor {
            type Value = ReplayAnalysisLLMDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut intent: Option<String> = None;
                let mut user_pattern: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "intent" => {
                            intent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_pattern" => {
                            user_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let intent = intent.ok_or_else(|| M::Error::missing_field("intent"))?;
                let user_pattern =
                    user_pattern.ok_or_else(|| M::Error::missing_field("user_pattern"))?;

                let content = ReplayAnalysisLLMDetails {
                    intent,
                    user_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisLLMDetailsVisitor)
    }
}
