// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for autocomplete-type user-defined fields, describing how to populate autocomplete options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUserDefinedFieldMetadata {
    /// The category of the autocomplete source.
    #[serde(rename = "category")]
    pub category: String,
    /// The query parameter used to limit the number of autocomplete results.
    #[serde(rename = "search_limit_param")]
    pub search_limit_param: String,
    /// Additional query parameters to include in the search URL.
    #[serde(rename = "search_params")]
    pub search_params: std::collections::BTreeMap<String, serde_json::Value>,
    /// The query parameter used to pass typed input to the search URL.
    #[serde(rename = "search_query_param")]
    pub search_query_param: String,
    /// The JSON path to the results in the response body.
    #[serde(rename = "search_result_path")]
    pub search_result_path: String,
    /// The URL used to populate autocomplete options.
    #[serde(rename = "search_url")]
    pub search_url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUserDefinedFieldMetadata {
    pub fn new(
        category: String,
        search_limit_param: String,
        search_params: std::collections::BTreeMap<String, serde_json::Value>,
        search_query_param: String,
        search_result_path: String,
        search_url: String,
    ) -> IncidentUserDefinedFieldMetadata {
        IncidentUserDefinedFieldMetadata {
            category,
            search_limit_param,
            search_params,
            search_query_param,
            search_result_path,
            search_url,
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

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUserDefinedFieldMetadataVisitor;
        impl<'a> Visitor<'a> for IncidentUserDefinedFieldMetadataVisitor {
            type Value = IncidentUserDefinedFieldMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut search_limit_param: Option<String> = None;
                let mut search_params: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut search_query_param: Option<String> = None;
                let mut search_result_path: Option<String> = None;
                let mut search_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_limit_param" => {
                            search_limit_param =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_params" => {
                            search_params =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_query_param" => {
                            search_query_param =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_result_path" => {
                            search_result_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_url" => {
                            search_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let search_limit_param = search_limit_param
                    .ok_or_else(|| M::Error::missing_field("search_limit_param"))?;
                let search_params =
                    search_params.ok_or_else(|| M::Error::missing_field("search_params"))?;
                let search_query_param = search_query_param
                    .ok_or_else(|| M::Error::missing_field("search_query_param"))?;
                let search_result_path = search_result_path
                    .ok_or_else(|| M::Error::missing_field("search_result_path"))?;
                let search_url = search_url.ok_or_else(|| M::Error::missing_field("search_url"))?;

                let content = IncidentUserDefinedFieldMetadata {
                    category,
                    search_limit_param,
                    search_params,
                    search_query_param,
                    search_result_path,
                    search_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUserDefinedFieldMetadataVisitor)
    }
}
