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
pub struct ToplistBackendRequest20270101OneOf4463528992 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "ci_test_query")]
    pub ci_test_query: crate::datadogV1_20270101::model::LogQueryDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ToplistBackendRequest20270101OneOf4463528992 {
    pub fn new(ci_test_query: crate::datadogV1_20270101::model::LogQueryDefinition) -> ToplistBackendRequest20270101OneOf4463528992 {
        ToplistBackendRequest20270101OneOf4463528992 {
            ci_test_query,
            conditional_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ToplistBackendRequest20270101OneOf4463528992 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistBackendRequest20270101OneOf4463528992Visitor;
        impl<'a> Visitor<'a> for ToplistBackendRequest20270101OneOf4463528992Visitor {
            type Value = ToplistBackendRequest20270101OneOf4463528992;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ci_test_query: Option<crate::datadogV1_20270101::model::LogQueryDefinition> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ci_test_query" => {
                            ci_test_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let ci_test_query = ci_test_query.ok_or_else(|| M::Error::missing_field("ci_test_query"))?;

                let content = ToplistBackendRequest20270101OneOf4463528992 {
                    ci_test_query,
                    conditional_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistBackendRequest20270101OneOf4463528992Visitor)
    }
}
