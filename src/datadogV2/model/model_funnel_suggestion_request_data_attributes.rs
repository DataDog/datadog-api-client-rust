// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelSuggestionRequestDataAttributes {
    #[serde(rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::FunnelSuggestionRequestDataAttributesSearch>,
    #[serde(rename = "term_search")]
    pub term_search:
        Option<crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTermSearch>,
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTime>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelSuggestionRequestDataAttributes {
    pub fn new() -> FunnelSuggestionRequestDataAttributes {
        FunnelSuggestionRequestDataAttributes {
            data_source: None,
            search: None,
            term_search: None,
            time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_source(mut self, value: String) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn search(
        mut self,
        value: crate::datadogV2::model::FunnelSuggestionRequestDataAttributesSearch,
    ) -> Self {
        self.search = Some(value);
        self
    }

    pub fn term_search(
        mut self,
        value: crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTermSearch,
    ) -> Self {
        self.term_search = Some(value);
        self
    }

    pub fn time(
        mut self,
        value: crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTime,
    ) -> Self {
        self.time = Some(value);
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

impl Default for FunnelSuggestionRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FunnelSuggestionRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelSuggestionRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for FunnelSuggestionRequestDataAttributesVisitor {
            type Value = FunnelSuggestionRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut search: Option<
                    crate::datadogV2::model::FunnelSuggestionRequestDataAttributesSearch,
                > = None;
                let mut term_search: Option<
                    crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTermSearch,
                > = None;
                let mut time: Option<
                    crate::datadogV2::model::FunnelSuggestionRequestDataAttributesTime,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "term_search" => {
                            if v.is_null() {
                                continue;
                            }
                            term_search =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FunnelSuggestionRequestDataAttributes {
                    data_source,
                    search,
                    term_search,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelSuggestionRequestDataAttributesVisitor)
    }
}
