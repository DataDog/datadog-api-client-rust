// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A facet group containing counts broken down by the distinct values of a case field (for example, status or priority).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseCountGroup {
    /// The name of the field being grouped on (for example, `status` or `priority`).
    #[serde(rename = "group")]
    pub group: String,
    /// Values within this group.
    #[serde(rename = "group_values")]
    pub group_values: Vec<crate::datadogV2::model::CaseCountGroupValue>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseCountGroup {
    pub fn new(
        group: String,
        group_values: Vec<crate::datadogV2::model::CaseCountGroupValue>,
    ) -> CaseCountGroup {
        CaseCountGroup {
            group,
            group_values,
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

impl<'de> Deserialize<'de> for CaseCountGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseCountGroupVisitor;
        impl<'a> Visitor<'a> for CaseCountGroupVisitor {
            type Value = CaseCountGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group: Option<String> = None;
                let mut group_values: Option<Vec<crate::datadogV2::model::CaseCountGroupValue>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group" => {
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_values" => {
                            group_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let group = group.ok_or_else(|| M::Error::missing_field("group"))?;
                let group_values =
                    group_values.ok_or_else(|| M::Error::missing_field("group_values"))?;

                let content = CaseCountGroup {
                    group,
                    group_values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseCountGroupVisitor)
    }
}
