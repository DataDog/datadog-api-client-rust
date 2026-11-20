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
pub struct GeomapBackendRegionsRequest20270101OneOf4457454688 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compliance_findings_query")]
    pub compliance_findings_query: crate::datadogV1::model::LogQueryDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::GeomapRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapBackendRegionsRequest20270101OneOf4457454688 {
    pub fn new(compliance_findings_query: crate::datadogV1::model::LogQueryDefinition) -> GeomapBackendRegionsRequest20270101OneOf4457454688 {
        GeomapBackendRegionsRequest20270101OneOf4457454688 {
            compliance_findings_query,
            conditional_formats: None,
            style: None,
            _unparsed: false,
        }
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



impl<'de> Deserialize<'de> for GeomapBackendRegionsRequest20270101OneOf4457454688 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapBackendRegionsRequest20270101OneOf4457454688Visitor;
        impl<'a> Visitor<'a> for GeomapBackendRegionsRequest20270101OneOf4457454688Visitor {
            type Value = GeomapBackendRegionsRequest20270101OneOf4457454688;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compliance_findings_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut style: Option<crate::datadogV1::model::GeomapRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compliance_findings_query" => {
                            compliance_findings_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let compliance_findings_query = compliance_findings_query.ok_or_else(|| M::Error::missing_field("compliance_findings_query"))?;

                let content = GeomapBackendRegionsRequest20270101OneOf4457454688 {
                    compliance_findings_query,
                    conditional_formats,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapBackendRegionsRequest20270101OneOf4457454688Visitor)
    }
}
