// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata related to paging information that is included in the response when querying the team hierarchy links
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsHierarchyLinksResponseMetaPage {
    /// First page number.
    #[serde(rename = "first_number")]
    pub first_number: Option<i64>,
    /// Last page number.
    #[serde(rename = "last_number")]
    pub last_number: Option<i64>,
    /// Next page number.
    #[serde(
        rename = "next_number",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_number: Option<Option<i64>>,
    /// Page number.
    #[serde(rename = "number")]
    pub number: Option<i64>,
    /// Previous page number.
    #[serde(
        rename = "prev_number",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub prev_number: Option<Option<i64>>,
    /// Page size.
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// Total number of results.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Pagination type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsHierarchyLinksResponseMetaPage {
    pub fn new() -> TeamsHierarchyLinksResponseMetaPage {
        TeamsHierarchyLinksResponseMetaPage {
            first_number: None,
            last_number: None,
            next_number: None,
            number: None,
            prev_number: None,
            size: None,
            total: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn first_number(mut self, value: i64) -> Self {
        self.first_number = Some(value);
        self
    }

    pub fn last_number(mut self, value: i64) -> Self {
        self.last_number = Some(value);
        self
    }

    pub fn next_number(mut self, value: Option<i64>) -> Self {
        self.next_number = Some(value);
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn prev_number(mut self, value: Option<i64>) -> Self {
        self.prev_number = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for TeamsHierarchyLinksResponseMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamsHierarchyLinksResponseMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsHierarchyLinksResponseMetaPageVisitor;
        impl<'a> Visitor<'a> for TeamsHierarchyLinksResponseMetaPageVisitor {
            type Value = TeamsHierarchyLinksResponseMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut first_number: Option<i64> = None;
                let mut last_number: Option<i64> = None;
                let mut next_number: Option<Option<i64>> = None;
                let mut number: Option<i64> = None;
                let mut prev_number: Option<Option<i64>> = None;
                let mut size: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_number" => {
                            if v.is_null() {
                                continue;
                            }
                            first_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_number" => {
                            if v.is_null() {
                                continue;
                            }
                            last_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_number" => {
                            next_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number" => {
                            if v.is_null() {
                                continue;
                            }
                            number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_number" => {
                            prev_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamsHierarchyLinksResponseMetaPage {
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

        deserializer.deserialize_any(TeamsHierarchyLinksResponseMetaPageVisitor)
    }
}
