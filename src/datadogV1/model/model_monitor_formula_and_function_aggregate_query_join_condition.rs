// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Join condition for aggregate augmented queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionAggregateQueryJoinCondition {
    /// Attribute from the augment query to join on.
    #[serde(rename = "augment_attribute")]
    pub augment_attribute: String,
    /// Attribute from the base query to join on.
    #[serde(rename = "base_attribute")]
    pub base_attribute: String,
    /// Join type for aggregate query join conditions.
    #[serde(rename = "join_type")]
    pub join_type: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionAggregateQueryJoinCondition {
    pub fn new(
        augment_attribute: String,
        base_attribute: String,
        join_type: crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinType,
    ) -> MonitorFormulaAndFunctionAggregateQueryJoinCondition {
        MonitorFormulaAndFunctionAggregateQueryJoinCondition {
            augment_attribute,
            base_attribute,
            join_type,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionAggregateQueryJoinCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionAggregateQueryJoinConditionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionAggregateQueryJoinConditionVisitor {
            type Value = MonitorFormulaAndFunctionAggregateQueryJoinCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut augment_attribute: Option<String> = None;
                let mut base_attribute: Option<String> = None;
                let mut join_type: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "augment_attribute" => {
                            augment_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "base_attribute" => {
                            base_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_type" => {
                            join_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _join_type) = join_type {
                                match _join_type {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionAggregateQueryJoinType::UnparsedObject(_join_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let augment_attribute = augment_attribute
                    .ok_or_else(|| M::Error::missing_field("augment_attribute"))?;
                let base_attribute =
                    base_attribute.ok_or_else(|| M::Error::missing_field("base_attribute"))?;
                let join_type = join_type.ok_or_else(|| M::Error::missing_field("join_type"))?;

                let content = MonitorFormulaAndFunctionAggregateQueryJoinCondition {
                    augment_attribute,
                    base_attribute,
                    join_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionAggregateQueryJoinConditionVisitor)
    }
}
