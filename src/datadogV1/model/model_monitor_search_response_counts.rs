// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The counts of monitors per different criteria.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorSearchResponseCounts {
    /// Search facets.
    #[serde(rename = "muted")]
    pub muted: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "status")]
    pub status: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "tag")]
    pub tag: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "type")]
    pub type_: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorSearchResponseCounts {
    pub fn new() -> MonitorSearchResponseCounts {
        MonitorSearchResponseCounts {
            muted: None,
            status: None,
            tag: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn muted(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
        self.muted = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tag(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
        self.tag = Some(value);
        self
    }

    pub fn type_(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
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

impl Default for MonitorSearchResponseCounts {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorSearchResponseCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSearchResponseCountsVisitor;
        impl<'a> Visitor<'a> for MonitorSearchResponseCountsVisitor {
            type Value = MonitorSearchResponseCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut muted: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut status: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut tag: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut type_: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "muted" => {
                            if v.is_null() {
                                continue;
                            }
                            muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag" => {
                            if v.is_null() {
                                continue;
                            }
                            tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = MonitorSearchResponseCounts {
                    muted,
                    status,
                    tag,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSearchResponseCountsVisitor)
    }
}
