// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The security filters properties to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityFilterUpdateAttributes {
    /// Exclusion filters to exclude some logs from the security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: Option<crate::datadogV2::model::SecurityFilterFilteredDataType>,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The name of the security filter.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query of the security filter.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The version of the security filter to update.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityFilterUpdateAttributes {
    pub fn new() -> SecurityFilterUpdateAttributes {
        SecurityFilterUpdateAttributes {
            exclusion_filters: None,
            filtered_data_type: None,
            is_enabled: None,
            name: None,
            query: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn exclusion_filters(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
    ) -> Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn filtered_data_type(
        mut self,
        value: crate::datadogV2::model::SecurityFilterFilteredDataType,
    ) -> Self {
        self.filtered_data_type = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for SecurityFilterUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityFilterUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityFilterUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityFilterUpdateAttributesVisitor {
            type Value = SecurityFilterUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclusion_filters: Option<
                    Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
                > = None;
                let mut filtered_data_type: Option<
                    crate::datadogV2::model::SecurityFilterFilteredDataType,
                > = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut version: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclusion_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            exclusion_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filtered_data_type" => {
                            if v.is_null() {
                                continue;
                            }
                            filtered_data_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filtered_data_type) = filtered_data_type {
                                match _filtered_data_type {
                                    crate::datadogV2::model::SecurityFilterFilteredDataType::UnparsedObject(_filtered_data_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityFilterUpdateAttributes {
                    exclusion_filters,
                    filtered_data_type,
                    is_enabled,
                    name,
                    query,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityFilterUpdateAttributesVisitor)
    }
}
