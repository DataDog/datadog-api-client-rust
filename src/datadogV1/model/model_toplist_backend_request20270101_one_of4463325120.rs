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
pub struct ToplistBackendRequest20270101OneOf4463325120 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "log_query")]
    pub log_query: crate::datadogV1::model::LogQueryDefinition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ToplistBackendRequest20270101OneOf4463325120 {
    pub fn new(log_query: crate::datadogV1::model::LogQueryDefinition) -> ToplistBackendRequest20270101OneOf4463325120 {
        ToplistBackendRequest20270101OneOf4463325120 {
            conditional_formats: None,
            log_query,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ToplistBackendRequest20270101OneOf4463325120 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistBackendRequest20270101OneOf4463325120Visitor;
        impl<'a> Visitor<'a> for ToplistBackendRequest20270101OneOf4463325120Visitor {
            type Value = ToplistBackendRequest20270101OneOf4463325120;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "log_query" => {
                            log_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let log_query = log_query.ok_or_else(|| M::Error::missing_field("log_query"))?;

                let content = ToplistBackendRequest20270101OneOf4463325120 {
                    conditional_formats,
                    log_query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistBackendRequest20270101OneOf4463325120Visitor)
    }
}
