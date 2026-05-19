// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for grouping aggregated results by one or more case fields.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseAggregateGroupBy {
    /// Fields to group by.
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
    /// Maximum number of groups to return.
    #[serde(rename = "limit")]
    pub limit: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseAggregateGroupBy {
    pub fn new(groups: Vec<String>, limit: i32) -> CaseAggregateGroupBy {
        CaseAggregateGroupBy {
            groups,
            limit,
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

impl<'de> Deserialize<'de> for CaseAggregateGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseAggregateGroupByVisitor;
        impl<'a> Visitor<'a> for CaseAggregateGroupByVisitor {
            type Value = CaseAggregateGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut groups: Option<Vec<String>> = None;
                let mut limit: Option<i32> = None;
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
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let groups = groups.ok_or_else(|| M::Error::missing_field("groups"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;

                let content = CaseAggregateGroupBy {
                    groups,
                    limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseAggregateGroupByVisitor)
    }
}
