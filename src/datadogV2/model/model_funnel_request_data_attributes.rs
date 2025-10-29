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
pub struct FunnelRequestDataAttributes {
    #[serde(rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(rename = "enforced_execution_type")]
    pub enforced_execution_type: Option<String>,
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::FunnelRequestDataAttributesSearch>,
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV2::model::FunnelRequestDataAttributesTime>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelRequestDataAttributes {
    pub fn new() -> FunnelRequestDataAttributes {
        FunnelRequestDataAttributes {
            data_source: None,
            enforced_execution_type: None,
            request_id: None,
            search: None,
            time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_source(mut self, value: String) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn enforced_execution_type(mut self, value: String) -> Self {
        self.enforced_execution_type = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn search(
        mut self,
        value: crate::datadogV2::model::FunnelRequestDataAttributesSearch,
    ) -> Self {
        self.search = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV2::model::FunnelRequestDataAttributesTime) -> Self {
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

impl Default for FunnelRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FunnelRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for FunnelRequestDataAttributesVisitor {
            type Value = FunnelRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut enforced_execution_type: Option<String> = None;
                let mut request_id: Option<String> = None;
                let mut search: Option<crate::datadogV2::model::FunnelRequestDataAttributesSearch> =
                    None;
                let mut time: Option<crate::datadogV2::model::FunnelRequestDataAttributesTime> =
                    None;
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
                        "enforced_execution_type" => {
                            if v.is_null() {
                                continue;
                            }
                            enforced_execution_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FunnelRequestDataAttributes {
                    data_source,
                    enforced_execution_type,
                    request_id,
                    search,
                    time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelRequestDataAttributesVisitor)
    }
}
