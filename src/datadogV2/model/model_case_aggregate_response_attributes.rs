// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the aggregation result, including the total count across all groups and the per-group breakdowns.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseAggregateResponseAttributes {
    /// Aggregated groups.
    #[serde(rename = "groups")]
    pub groups: Vec<crate::datadogV2::model::CaseAggregateGroup>,
    /// Total count of aggregated cases.
    #[serde(rename = "total")]
    pub total: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseAggregateResponseAttributes {
    pub fn new(
        groups: Vec<crate::datadogV2::model::CaseAggregateGroup>,
        total: f64,
    ) -> CaseAggregateResponseAttributes {
        CaseAggregateResponseAttributes {
            groups,
            total,
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

impl<'de> Deserialize<'de> for CaseAggregateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseAggregateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CaseAggregateResponseAttributesVisitor {
            type Value = CaseAggregateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut groups: Option<Vec<crate::datadogV2::model::CaseAggregateGroup>> = None;
                let mut total: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "groups" => {
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let groups = groups.ok_or_else(|| M::Error::missing_field("groups"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = CaseAggregateResponseAttributes {
                    groups,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseAggregateResponseAttributesVisitor)
    }
}
