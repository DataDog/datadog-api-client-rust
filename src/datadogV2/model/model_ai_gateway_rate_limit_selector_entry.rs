// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One dimension rule in a template rate limit selector.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitSelectorEntry {
    /// Dimension to match: user_id, team_id, model, provider, or tag:<key>. Reserved metadata tag keys are rejected.
    #[serde(rename = "dim")]
    pub dim: String,
    /// Give each matched value its own allowance when true. Otherwise, matched values share the allowance within the other separated dimensions.
    #[serde(rename = "per_value")]
    pub per_value: bool,
    /// Allowed dimension values. Omit to match any value, including an absent dimension. An explicit list must contain at least one value. A per-value tag dimension must list its values.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitSelectorEntry {
    pub fn new(dim: String, per_value: bool) -> AiGatewayRateLimitSelectorEntry {
        AiGatewayRateLimitSelectorEntry {
            dim,
            per_value,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitSelectorEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitSelectorEntryVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitSelectorEntryVisitor {
            type Value = AiGatewayRateLimitSelectorEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dim: Option<String> = None;
                let mut per_value: Option<bool> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dim" => {
                            dim = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "per_value" => {
                            per_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dim = dim.ok_or_else(|| M::Error::missing_field("dim"))?;
                let per_value = per_value.ok_or_else(|| M::Error::missing_field("per_value"))?;

                let content = AiGatewayRateLimitSelectorEntry {
                    dim,
                    per_value,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitSelectorEntryVisitor)
    }
}
