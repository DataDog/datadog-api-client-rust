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
pub struct EventsDataExtractionExtraction {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "pattern")]
    pub pattern: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::EventsDataExtractionExtractionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl EventsDataExtractionExtraction {
    pub fn new(pattern: String, type_: crate::datadogV1_20270101::model::EventsDataExtractionExtractionType) -> EventsDataExtractionExtraction {
        EventsDataExtractionExtraction {
            pattern,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for EventsDataExtractionExtraction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsDataExtractionExtractionVisitor;
        impl<'a> Visitor<'a> for EventsDataExtractionExtractionVisitor {
            type Value = EventsDataExtractionExtraction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pattern: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::EventsDataExtractionExtractionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pattern" => {
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::EventsDataExtractionExtractionType::UnparsedObject(_type_) => {
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
                let pattern = pattern.ok_or_else(|| M::Error::missing_field("pattern"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = EventsDataExtractionExtraction {
                    pattern,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsDataExtractionExtractionVisitor)
    }
}
