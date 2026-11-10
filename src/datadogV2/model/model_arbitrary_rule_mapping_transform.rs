// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Only supported for `proportional_timeseries` and `even_timeseries` rules. Creates a synthetic tag from timeseries input rows before `tag_transforms` are applied.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleMappingTransform {
    /// Recursive mapping node. Each `match_on_values` branch contains a value and a nested node.
    #[serde(rename = "mapping")]
    pub mapping: std::collections::BTreeMap<String, serde_json::Value>,
    /// The name of the synthetic tag created by the mapping.
    #[serde(rename = "synthetic_tag")]
    pub synthetic_tag: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryRuleMappingTransform {
    pub fn new(
        mapping: std::collections::BTreeMap<String, serde_json::Value>,
        synthetic_tag: String,
    ) -> ArbitraryRuleMappingTransform {
        ArbitraryRuleMappingTransform {
            mapping,
            synthetic_tag,
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

impl<'de> Deserialize<'de> for ArbitraryRuleMappingTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleMappingTransformVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleMappingTransformVisitor {
            type Value = ArbitraryRuleMappingTransform;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mapping: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut synthetic_tag: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mapping" => {
                            mapping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synthetic_tag" => {
                            synthetic_tag =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mapping = mapping.ok_or_else(|| M::Error::missing_field("mapping"))?;
                let synthetic_tag =
                    synthetic_tag.ok_or_else(|| M::Error::missing_field("synthetic_tag"))?;

                let content = ArbitraryRuleMappingTransform {
                    mapping,
                    synthetic_tag,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryRuleMappingTransformVisitor)
    }
}
