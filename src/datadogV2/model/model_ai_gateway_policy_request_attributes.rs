// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create or replace a routing policy document.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayPolicyRequestAttributes {
    /// Inbound request pattern.
    #[serde(rename = "inbound")]
    pub inbound: String,
    /// Routing priority.
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    #[serde(rename = "tiers")]
    pub tiers: Vec<crate::datadogV2::model::AiGatewayPolicyTierRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayPolicyRequestAttributes {
    pub fn new(
        inbound: String,
        tiers: Vec<crate::datadogV2::model::AiGatewayPolicyTierRequest>,
    ) -> AiGatewayPolicyRequestAttributes {
        AiGatewayPolicyRequestAttributes {
            inbound,
            priority: None,
            tiers,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayPolicyRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayPolicyRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayPolicyRequestAttributesVisitor {
            type Value = AiGatewayPolicyRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut inbound: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut tiers: Option<Vec<crate::datadogV2::model::AiGatewayPolicyTierRequest>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "inbound" => {
                            inbound = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
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
                let inbound = inbound.ok_or_else(|| M::Error::missing_field("inbound"))?;
                let tiers = tiers.ok_or_else(|| M::Error::missing_field("tiers"))?;

                let content = AiGatewayPolicyRequestAttributes {
                    inbound,
                    priority,
                    tiers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayPolicyRequestAttributesVisitor)
    }
}
