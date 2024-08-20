// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the attributes of the security filter to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityFilterCreateAttributes {
    /// Exclusion filters to exclude some logs from the security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: crate::datadogV2::model::SecurityFilterFilteredDataType,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    /// The name of the security filter.
    #[serde(rename = "name")]
    pub name: String,
    /// The query of the security filter.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityFilterCreateAttributes {
    pub fn new(
        exclusion_filters: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
        filtered_data_type: crate::datadogV2::model::SecurityFilterFilteredDataType,
        is_enabled: bool,
        name: String,
        query: String,
    ) -> SecurityFilterCreateAttributes {
        SecurityFilterCreateAttributes {
            exclusion_filters,
            filtered_data_type,
            is_enabled,
            name,
            query,
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

impl<'de> Deserialize<'de> for SecurityFilterCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityFilterCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityFilterCreateAttributesVisitor {
            type Value = SecurityFilterCreateAttributes;

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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclusion_filters" => {
                            exclusion_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filtered_data_type" => {
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
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let exclusion_filters = exclusion_filters
                    .ok_or_else(|| M::Error::missing_field("exclusion_filters"))?;
                let filtered_data_type = filtered_data_type
                    .ok_or_else(|| M::Error::missing_field("filtered_data_type"))?;
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = SecurityFilterCreateAttributes {
                    exclusion_filters,
                    filtered_data_type,
                    is_enabled,
                    name,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityFilterCreateAttributesVisitor)
    }
}
