// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Owners of the group's open findings. `items` lists at most 5 owners, the owners of the most findings first,
/// then by name. `total` counts every distinct owner, so the list is cut when `total` is larger than the number
/// of items.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupOwnerList {
    /// Owners of the most findings in the group, at most 5.
    #[serde(rename = "items")]
    pub items: Vec<String>,
    /// Number of distinct owners across the group's findings.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupOwnerList {
    pub fn new(items: Vec<String>, total: i64) -> RemediationGroupOwnerList {
        RemediationGroupOwnerList {
            items,
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

impl<'de> Deserialize<'de> for RemediationGroupOwnerList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupOwnerListVisitor;
        impl<'a> Visitor<'a> for RemediationGroupOwnerListVisitor {
            type Value = RemediationGroupOwnerList;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut items: Option<Vec<String>> = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "items" => {
                            items = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let items = items.ok_or_else(|| M::Error::missing_field("items"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = RemediationGroupOwnerList {
                    items,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupOwnerListVisitor)
    }
}
