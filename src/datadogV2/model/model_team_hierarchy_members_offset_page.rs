// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata using offset and limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyMembersOffsetPage {
    /// First offset.
    #[serde(rename = "first_offset")]
    pub first_offset: i64,
    /// Last offset. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "last_offset")]
    pub last_offset: Option<i64>,
    /// Maximum number of members per page.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Next offset. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// Current offset.
    #[serde(rename = "offset")]
    pub offset: i64,
    /// Previous offset. Null when unavailable.
    #[serialize_always]
    #[serde(rename = "prev_offset")]
    pub prev_offset: Option<i64>,
    /// Total number of members matching the filters.
    #[serde(rename = "total")]
    pub total: i64,
    /// Pagination mode.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamHierarchyMembersOffsetPageType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyMembersOffsetPage {
    pub fn new(
        first_offset: i64,
        last_offset: Option<i64>,
        limit: i64,
        next_offset: Option<i64>,
        offset: i64,
        prev_offset: Option<i64>,
        total: i64,
        type_: crate::datadogV2::model::TeamHierarchyMembersOffsetPageType,
    ) -> TeamHierarchyMembersOffsetPage {
        TeamHierarchyMembersOffsetPage {
            first_offset,
            last_offset,
            limit,
            next_offset,
            offset,
            prev_offset,
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

impl<'de> Deserialize<'de> for TeamHierarchyMembersOffsetPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyMembersOffsetPageVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyMembersOffsetPageVisitor {
            type Value = TeamHierarchyMembersOffsetPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first_offset: Option<i64> = None;
                let mut last_offset: Option<Option<i64>> = None;
                let mut limit: Option<i64> = None;
                let mut next_offset: Option<Option<i64>> = None;
                let mut offset: Option<i64> = None;
                let mut prev_offset: Option<Option<i64>> = None;
                let mut total: Option<i64> = None;
                let mut type_: Option<crate::datadogV2::model::TeamHierarchyMembersOffsetPageType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_offset" => {
                            first_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_offset" => {
                            last_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_offset" => {
                            next_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_offset" => {
                            prev_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TeamHierarchyMembersOffsetPageType::UnparsedObject(_type_) => {
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
                let first_offset =
                    first_offset.ok_or_else(|| M::Error::missing_field("first_offset"))?;
                let last_offset =
                    last_offset.ok_or_else(|| M::Error::missing_field("last_offset"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let next_offset =
                    next_offset.ok_or_else(|| M::Error::missing_field("next_offset"))?;
                let offset = offset.ok_or_else(|| M::Error::missing_field("offset"))?;
                let prev_offset =
                    prev_offset.ok_or_else(|| M::Error::missing_field("prev_offset"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TeamHierarchyMembersOffsetPage {
                    first_offset,
                    last_offset,
                    limit,
                    next_offset,
                    offset,
                    prev_offset,
                    total,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyMembersOffsetPageVisitor)
    }
}
