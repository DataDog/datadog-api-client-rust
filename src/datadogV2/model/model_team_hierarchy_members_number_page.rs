// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata using number and size.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyMembersNumberPage {
    /// First page number.
    #[serde(rename = "first_number")]
    pub first_number: i64,
    /// Last page number. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "last_number")]
    pub last_number: Option<i64>,
    /// Next page number. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "next_number")]
    pub next_number: Option<i64>,
    /// Current zero-based page number.
    #[serde(rename = "number")]
    pub number: i64,
    /// Previous page number. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "prev_number")]
    pub prev_number: Option<i64>,
    /// Maximum number of members per page.
    #[serde(rename = "size")]
    pub size: i64,
    /// Total number of members matching the filters.
    #[serde(rename = "total")]
    pub total: i64,
    /// Pagination mode.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamHierarchyMembersNumberPageType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyMembersNumberPage {
    pub fn new(
        first_number: i64,
        last_number: Option<i64>,
        next_number: Option<i64>,
        number: i64,
        prev_number: Option<i64>,
        size: i64,
        total: i64,
        type_: crate::datadogV2::model::TeamHierarchyMembersNumberPageType,
    ) -> TeamHierarchyMembersNumberPage {
        TeamHierarchyMembersNumberPage {
            first_number,
            last_number,
            next_number,
            number,
            prev_number,
            size,
            total,
            type_,
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

impl<'de> Deserialize<'de> for TeamHierarchyMembersNumberPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyMembersNumberPageVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyMembersNumberPageVisitor {
            type Value = TeamHierarchyMembersNumberPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first_number: Option<i64> = None;
                let mut last_number: Option<Option<i64>> = None;
                let mut next_number: Option<Option<i64>> = None;
                let mut number: Option<i64> = None;
                let mut prev_number: Option<Option<i64>> = None;
                let mut size: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::TeamHierarchyMembersNumberPageType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_number" => {
                            first_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_number" => {
                            last_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_number" => {
                            next_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number" => {
                            number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_number" => {
                            prev_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TeamHierarchyMembersNumberPageType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let first_number =
                    first_number.ok_or_else(|| M::Error::missing_field("first_number"))?;
                let last_number =
                    last_number.ok_or_else(|| M::Error::missing_field("last_number"))?;
                let next_number =
                    next_number.ok_or_else(|| M::Error::missing_field("next_number"))?;
                let number = number.ok_or_else(|| M::Error::missing_field("number"))?;
                let prev_number =
                    prev_number.ok_or_else(|| M::Error::missing_field("prev_number"))?;
                let size = size.ok_or_else(|| M::Error::missing_field("size"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TeamHierarchyMembersNumberPage {
                    first_number,
                    last_number,
                    next_number,
                    number,
                    prev_number,
                    size,
                    total,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyMembersNumberPageVisitor)
    }
}
