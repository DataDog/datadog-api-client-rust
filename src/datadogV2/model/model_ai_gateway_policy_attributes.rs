// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Routing policy configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayPolicyAttributes {
    /// Routing priority.
    #[serde(rename = "priority")]
    pub priority: i64,
    #[serde(rename = "tiers")]
    pub tiers: Vec<crate::datadogV2::model::AiGatewayPolicyTier>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayPolicyAttributes {
    pub fn new(
        priority: i64,
        tiers: Vec<crate::datadogV2::model::AiGatewayPolicyTier>,
    ) -> AiGatewayPolicyAttributes {
        AiGatewayPolicyAttributes {
            priority,
            tiers,
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

impl<'de> Deserialize<'de> for AiGatewayPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayPolicyAttributesVisitor {
            type Value = AiGatewayPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut priority: Option<i64> = None;
                let mut tiers: Option<Vec<crate::datadogV2::model::AiGatewayPolicyTier>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tiers" => {
                            tiers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let tiers = tiers.ok_or_else(|| M::Error::missing_field("tiers"))?;

                let content = AiGatewayPolicyAttributes {
                    priority,
                    tiers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayPolicyAttributesVisitor)
    }
}
