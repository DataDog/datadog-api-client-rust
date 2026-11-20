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
pub struct StructuredAnalysisCast {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "source_column")]
    pub source_column: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "target_column")]
    pub target_column: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "target_type")]
    pub target_type: crate::datadogV1_20270101::model::StructuredAnalysisCastTargetType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::StructuredAnalysisCastType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl StructuredAnalysisCast {
    pub fn new(source_column: String, target_column: String, target_type: crate::datadogV1_20270101::model::StructuredAnalysisCastTargetType, type_: crate::datadogV1_20270101::model::StructuredAnalysisCastType) -> StructuredAnalysisCast {
        StructuredAnalysisCast {
            source_column,
            target_column,
            target_type,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for StructuredAnalysisCast {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StructuredAnalysisCastVisitor;
        impl<'a> Visitor<'a> for StructuredAnalysisCastVisitor {
            type Value = StructuredAnalysisCast;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut source_column: Option<String> = None;
                let mut target_column: Option<String> = None;
                let mut target_type: Option<crate::datadogV1_20270101::model::StructuredAnalysisCastTargetType> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::StructuredAnalysisCastType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "source_column" => {
                            source_column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "target_column" => {
                            target_column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "target_type" => {
                            target_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target_type) = target_type {
                                match _target_type {
                                    crate::datadogV1_20270101::model::StructuredAnalysisCastTargetType::UnparsedObject(_target_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::StructuredAnalysisCastType::UnparsedObject(_type_) => {
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
                let source_column = source_column.ok_or_else(|| M::Error::missing_field("source_column"))?;
                let target_column = target_column.ok_or_else(|| M::Error::missing_field("target_column"))?;
                let target_type = target_type.ok_or_else(|| M::Error::missing_field("target_type"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = StructuredAnalysisCast {
                    source_column,
                    target_column,
                    target_type,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StructuredAnalysisCastVisitor)
    }
}
