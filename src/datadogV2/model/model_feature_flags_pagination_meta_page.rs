// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata for feature flags list responses.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagsPaginationMetaPage {
    /// Offset of the first page.
    #[serde(rename = "first_offset")]
    pub first_offset: Option<i64>,
    /// Offset of the last page.
    #[serde(
        rename = "last_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_offset: Option<Option<i64>>,
    /// Maximum number of flags returned on this page.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Offset of the next page, or null when there is no next page.
    #[serde(
        rename = "next_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_offset: Option<Option<i64>>,
    /// Number of matching flags skipped before this page.
    #[serde(rename = "offset")]
    pub offset: Option<i64>,
    /// Offset of the previous page, or null when there is no previous page.
    #[serde(
        rename = "prev_offset",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub prev_offset: Option<Option<i64>>,
    /// Total number of flags matching the filters, independent of the page size.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Legacy count field. Use `total` for the number of matching flags.
    #[deprecated]
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    /// Legacy filtered count field. Use `total` for the number of matching flags.
    #[deprecated]
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
    /// The pagination method. Always `offset_limit`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagsPaginationMetaPage {
    pub fn new() -> FeatureFlagsPaginationMetaPage {
        #[allow(deprecated)]
        FeatureFlagsPaginationMetaPage {
            first_offset: None,
            last_offset: None,
            limit: None,
            next_offset: None,
            offset: None,
            prev_offset: None,
            total: None,
            total_count: None,
            total_filtered_count: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn first_offset(mut self, value: i64) -> Self {
        self.first_offset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn last_offset(mut self, value: Option<i64>) -> Self {
        self.last_offset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn next_offset(mut self, value: Option<i64>) -> Self {
        self.next_offset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn prev_offset(mut self, value: Option<i64>) -> Self {
        self.prev_offset = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
        self
    }

    #[allow(deprecated)]
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

impl Default for FeatureFlagsPaginationMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FeatureFlagsPaginationMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagsPaginationMetaPageVisitor;
        impl<'a> Visitor<'a> for FeatureFlagsPaginationMetaPageVisitor {
            type Value = FeatureFlagsPaginationMetaPage;

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
                let mut total_count: Option<i64> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "first_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            first_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_offset" => {
                            last_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_offset" => {
                            next_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            if v.is_null() {
                                continue;
                            }
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_offset" => {
                            prev_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                #[allow(deprecated)]
                let content = FeatureFlagsPaginationMetaPage {
                    first_offset,
                    last_offset,
                    limit,
                    next_offset,
                    offset,
                    prev_offset,
                    total,
                    total_count,
                    total_filtered_count,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagsPaginationMetaPageVisitor)
    }
}
