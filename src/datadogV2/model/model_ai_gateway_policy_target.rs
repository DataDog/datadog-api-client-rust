// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resolved routing target in a policy response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayPolicyTarget {
    /// Saved target row ID. On replacement, retain this ID only within its original route and at most once. A stale or foreign ID is rejected.
    #[serde(rename = "id")]
    pub id: i64,
    /// Model name or pattern. Omitted when retaining a saved target by ID or when its provider is hidden.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// Routing priority.
    #[serde(rename = "priority")]
    pub priority: i64,
    /// Provider slug. Omitted for dynamic resolution or when its provider is hidden.
    #[serde(
        rename = "provider",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub provider: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayPolicyTarget {
    pub fn new(id: i64, priority: i64) -> AiGatewayPolicyTarget {
        AiGatewayPolicyTarget {
            id,
            model: None,
            priority,
            provider: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    pub fn provider(mut self, value: Option<String>) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AiGatewayPolicyTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayPolicyTargetVisitor;
        impl<'a> Visitor<'a> for AiGatewayPolicyTargetVisitor {
            type Value = AiGatewayPolicyTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<i64> = None;
                let mut model: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut provider: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model" => {
                            if v.is_null() {
                                continue;
                            }
                            model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;

                let content = AiGatewayPolicyTarget {
                    id,
                    model,
                    priority,
                    provider,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayPolicyTargetVisitor)
    }
}
