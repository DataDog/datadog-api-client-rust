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
pub struct LegacyEventOverlayQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "q")]
    pub q: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "tags_execution")]
    pub tags_execution: Option<crate::datadogV1_20270101::model::TagsExecution>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl LegacyEventOverlayQuery {
    pub fn new(q: String) -> LegacyEventOverlayQuery {
        LegacyEventOverlayQuery {
            q,
            tags_execution: None,
            _unparsed: false,
        }
    }
    
    pub fn tags_execution(mut self, value: crate::datadogV1_20270101::model::TagsExecution) -> Self {
        self.tags_execution = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for LegacyEventOverlayQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LegacyEventOverlayQueryVisitor;
        impl<'a> Visitor<'a> for LegacyEventOverlayQueryVisitor {
            type Value = LegacyEventOverlayQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut q: Option<String> = None;
                let mut tags_execution: Option<crate::datadogV1_20270101::model::TagsExecution> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "q" => {
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "tags_execution" => {
                            if v.is_null() {
                                continue;
                            }
                            tags_execution = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _tags_execution) = tags_execution {
                                match _tags_execution {
                                    crate::datadogV1_20270101::model::TagsExecution::UnparsedObject(_tags_execution) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let q = q.ok_or_else(|| M::Error::missing_field("q"))?;

                let content = LegacyEventOverlayQuery {
                    q,
                    tags_execution,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LegacyEventOverlayQueryVisitor)
    }
}
