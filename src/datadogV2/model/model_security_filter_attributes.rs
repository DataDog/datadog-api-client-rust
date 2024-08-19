// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing a security filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityFilterAttributes {
    /// The list of exclusion filters applied in this security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters:
        Option<Vec<crate::datadogV2::model::SecurityFilterExclusionFilterResponse>>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: Option<crate::datadogV2::model::SecurityFilterFilteredDataType>,
    /// Whether the security filter is the built-in filter.
    #[serde(rename = "is_builtin")]
    pub is_builtin: Option<bool>,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The security filter name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The security filter query. Logs accepted by this query will be accepted by this filter.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The version of the security filter.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityFilterAttributes {
    pub fn new() -> SecurityFilterAttributes {
        SecurityFilterAttributes {
            exclusion_filters: None,
            filtered_data_type: None,
            is_builtin: None,
            is_enabled: None,
            name: None,
            query: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclusion_filters(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityFilterExclusionFilterResponse>,
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

    pub fn is_builtin(mut self, value: bool) -> Self {
        self.is_builtin = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SecurityFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityFilterAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityFilterAttributesVisitor {
            type Value = SecurityFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclusion_filters: Option<
                    Vec<crate::datadogV2::model::SecurityFilterExclusionFilterResponse>,
                > = None;
                let mut filtered_data_type: Option<
                    crate::datadogV2::model::SecurityFilterFilteredDataType,
                > = None;
                let mut is_builtin: Option<bool> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut version: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "is_builtin" => {
                            if v.is_null() {
                                continue;
                            }
                            is_builtin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityFilterAttributes {
                    exclusion_filters,
                    filtered_data_type,
                    is_builtin,
                    is_enabled,
                    name,
                    query,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityFilterAttributesVisitor)
    }
}
