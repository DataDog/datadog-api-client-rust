// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata reporting that the cost data lacks the unused costs of some commitments, as with Azure exports
/// scoped below the billing profile. Utilization is then overstated, and realized savings and effective
/// savings rate are upper bounds. Omitted when the unused costs are available.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsUnusedCostMeta {
    /// Whether the cost data lacks the unused costs of some commitments.
    #[serde(rename = "unused_cost_unavailable")]
    pub unused_cost_unavailable: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsUnusedCostMeta {
    pub fn new() -> CommitmentsUnusedCostMeta {
        CommitmentsUnusedCostMeta {
            unused_cost_unavailable: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn unused_cost_unavailable(mut self, value: bool) -> Self {
        self.unused_cost_unavailable = Some(value);
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

impl Default for CommitmentsUnusedCostMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CommitmentsUnusedCostMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsUnusedCostMetaVisitor;
        impl<'a> Visitor<'a> for CommitmentsUnusedCostMetaVisitor {
            type Value = CommitmentsUnusedCostMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut unused_cost_unavailable: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "unused_cost_unavailable" => {
                            if v.is_null() {
                                continue;
                            }
                            unused_cost_unavailable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CommitmentsUnusedCostMeta {
                    unused_cost_unavailable,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsUnusedCostMetaVisitor)
    }
}
