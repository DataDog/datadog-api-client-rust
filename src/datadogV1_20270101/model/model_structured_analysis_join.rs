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
pub struct StructuredAnalysisJoin {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_conditions")]
    pub join_conditions: Vec<crate::datadogV1_20270101::model::StructuredAnalysisJoinCondition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_type")]
    pub join_type: crate::datadogV1_20270101::model::StructuredAnalysisJoinJoinType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "target_columns")]
    pub target_columns: Vec<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "target_dataset")]
    pub target_dataset: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::StructuredAnalysisJoinType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl StructuredAnalysisJoin {
    pub fn new(join_conditions: Vec<crate::datadogV1_20270101::model::StructuredAnalysisJoinCondition>, join_type: crate::datadogV1_20270101::model::StructuredAnalysisJoinJoinType, target_columns: Vec<String>, target_dataset: String, type_: crate::datadogV1_20270101::model::StructuredAnalysisJoinType) -> StructuredAnalysisJoin {
        StructuredAnalysisJoin {
            join_conditions,
            join_type,
            target_columns,
            target_dataset,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for StructuredAnalysisJoin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StructuredAnalysisJoinVisitor;
        impl<'a> Visitor<'a> for StructuredAnalysisJoinVisitor {
            type Value = StructuredAnalysisJoin;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut join_conditions: Option<Vec<crate::datadogV1_20270101::model::StructuredAnalysisJoinCondition>> = None;
                let mut join_type: Option<crate::datadogV1_20270101::model::StructuredAnalysisJoinJoinType> = None;
                let mut target_columns: Option<Vec<String>> = None;
                let mut target_dataset: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::StructuredAnalysisJoinType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "join_conditions" => {
                            join_conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "join_type" => {
                            join_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _join_type) = join_type {
                                match _join_type {
                                    crate::datadogV1_20270101::model::StructuredAnalysisJoinJoinType::UnparsedObject(_join_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "target_columns" => {
                            target_columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "target_dataset" => {
                            target_dataset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::StructuredAnalysisJoinType::UnparsedObject(_type_) => {
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
                let join_conditions = join_conditions.ok_or_else(|| M::Error::missing_field("join_conditions"))?;
                let join_type = join_type.ok_or_else(|| M::Error::missing_field("join_type"))?;
                let target_columns = target_columns.ok_or_else(|| M::Error::missing_field("target_columns"))?;
                let target_dataset = target_dataset.ok_or_else(|| M::Error::missing_field("target_dataset"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = StructuredAnalysisJoin {
                    join_conditions,
                    join_type,
                    target_columns,
                    target_dataset,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StructuredAnalysisJoinVisitor)
    }
}
