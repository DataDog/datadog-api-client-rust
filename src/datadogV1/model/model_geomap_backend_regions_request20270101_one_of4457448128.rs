// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapBackendRegionsRequest20270101OneOf4457448128 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "_query_options")]
    pub _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1::model::DashboardInlineSchema6>>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "q")]
    pub q: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::GeomapRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapBackendRegionsRequest20270101OneOf4457448128 {
    pub fn new(q: String) -> GeomapBackendRegionsRequest20270101OneOf4457448128 {
        GeomapBackendRegionsRequest20270101OneOf4457448128 {
            _query_options: None,
            conditional_formats: None,
            q,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn _query_options(mut self, value: std::collections::BTreeMap<String, Option<crate::datadogV1::model::DashboardInlineSchema6>>) -> Self {
        self._query_options = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1::model::GeomapRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GeomapBackendRegionsRequest20270101OneOf4457448128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapBackendRegionsRequest20270101OneOf4457448128Visitor;
        impl<'a> Visitor<'a> for GeomapBackendRegionsRequest20270101OneOf4457448128Visitor {
            type Value = GeomapBackendRegionsRequest20270101OneOf4457448128;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut _query_options: Option<std::collections::BTreeMap<String, Option<crate::datadogV1::model::DashboardInlineSchema6>>> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut q: Option<String> = None;
                let mut style: Option<crate::datadogV1::model::GeomapRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "_query_options" => {
                            if v.is_null() {
                                continue;
                            }
                            _query_options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = GeomapBackendRegionsRequest20270101OneOf4457448128 {
                    _query_options,
                    conditional_formats,
                    q,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapBackendRegionsRequest20270101OneOf4457448128Visitor)
    }
}
