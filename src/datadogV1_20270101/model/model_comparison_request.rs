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
pub struct ComparisonRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "directionality")]
    pub directionality: Option<crate::datadogV1_20270101::model::ChangeDirectionality>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "duration")]
    pub duration: crate::datadogV1_20270101::model::ChangeDuration,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV1_20270101::model::ChangeLayout>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1_20270101::model::ChangeType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ComparisonRequest {
    pub fn new(duration: crate::datadogV1_20270101::model::ChangeDuration) -> ComparisonRequest {
        ComparisonRequest {
            directionality: None,
            duration,
            layout: None,
            type_: None,
            _unparsed: false,
        }
    }
    
    pub fn directionality(mut self, value: crate::datadogV1_20270101::model::ChangeDirectionality) -> Self {
        self.directionality = Some(value);
        self
    }
    
    pub fn layout(mut self, value: crate::datadogV1_20270101::model::ChangeLayout) -> Self {
        self.layout = Some(value);
        self
    }
    
    pub fn type_(mut self, value: crate::datadogV1_20270101::model::ChangeType) -> Self {
        self.type_ = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ComparisonRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComparisonRequestVisitor;
        impl<'a> Visitor<'a> for ComparisonRequestVisitor {
            type Value = ComparisonRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut directionality: Option<crate::datadogV1_20270101::model::ChangeDirectionality> = None;
                let mut duration: Option<crate::datadogV1_20270101::model::ChangeDuration> = None;
                let mut layout: Option<crate::datadogV1_20270101::model::ChangeLayout> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::ChangeType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "directionality" => {
                            if v.is_null() {
                                continue;
                            }
                            directionality = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _directionality) = directionality {
                                match _directionality {
                                    crate::datadogV1_20270101::model::ChangeDirectionality::UnparsedObject(_directionality) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _duration) = duration {
                                match _duration {
                                    crate::datadogV1_20270101::model::ChangeDuration::UnparsedObject(_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "layout" => {
                            if v.is_null() {
                                continue;
                            }
                            layout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout) = layout {
                                match _layout {
                                    crate::datadogV1_20270101::model::ChangeLayout::UnparsedObject(_layout) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::ChangeType::UnparsedObject(_type_) => {
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
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;

                let content = ComparisonRequest {
                    directionality,
                    duration,
                    layout,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComparisonRequestVisitor)
    }
}
