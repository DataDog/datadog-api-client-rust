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
pub struct ListAugmentedQueryJoinCondition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "augment_attribute")]
    pub augment_attribute: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "augment_attribute_normalization")]
    pub augment_attribute_normalization: Option<crate::datadogV1_20270101::model::ListAugmentedQueryJoinConditionAugmentAttributeNormalization>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "base_attribute")]
    pub base_attribute: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_negated")]
    pub is_negated: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_type")]
    pub join_type: crate::datadogV1_20270101::model::JoinType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ListAugmentedQueryJoinCondition {
    pub fn new(augment_attribute: String, base_attribute: String, join_type: crate::datadogV1_20270101::model::JoinType) -> ListAugmentedQueryJoinCondition {
        ListAugmentedQueryJoinCondition {
            augment_attribute,
            augment_attribute_normalization: None,
            base_attribute,
            is_negated: None,
            join_type,
            _unparsed: false,
        }
    }
    
    pub fn augment_attribute_normalization(mut self, value: crate::datadogV1_20270101::model::ListAugmentedQueryJoinConditionAugmentAttributeNormalization) -> Self {
        self.augment_attribute_normalization = Some(value);
        self
    }
    
    pub fn is_negated(mut self, value: bool) -> Self {
        self.is_negated = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ListAugmentedQueryJoinCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListAugmentedQueryJoinConditionVisitor;
        impl<'a> Visitor<'a> for ListAugmentedQueryJoinConditionVisitor {
            type Value = ListAugmentedQueryJoinCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut augment_attribute: Option<String> = None;
                let mut augment_attribute_normalization: Option<crate::datadogV1_20270101::model::ListAugmentedQueryJoinConditionAugmentAttributeNormalization> = None;
                let mut base_attribute: Option<String> = None;
                let mut is_negated: Option<bool> = None;
                let mut join_type: Option<crate::datadogV1_20270101::model::JoinType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "augment_attribute" => {
                            augment_attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "augment_attribute_normalization" => {
                            if v.is_null() {
                                continue;
                            }
                            augment_attribute_normalization = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _augment_attribute_normalization) = augment_attribute_normalization {
                                match _augment_attribute_normalization {
                                    crate::datadogV1_20270101::model::ListAugmentedQueryJoinConditionAugmentAttributeNormalization::UnparsedObject(_augment_attribute_normalization) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "base_attribute" => {
                            base_attribute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_negated" => {
                            if v.is_null() {
                                continue;
                            }
                            is_negated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "join_type" => {
                            join_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _join_type) = join_type {
                                match _join_type {
                                    crate::datadogV1_20270101::model::JoinType::UnparsedObject(_join_type) => {
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
                let augment_attribute = augment_attribute.ok_or_else(|| M::Error::missing_field("augment_attribute"))?;
                let base_attribute = base_attribute.ok_or_else(|| M::Error::missing_field("base_attribute"))?;
                let join_type = join_type.ok_or_else(|| M::Error::missing_field("join_type"))?;

                let content = ListAugmentedQueryJoinCondition {
                    augment_attribute,
                    augment_attribute_normalization,
                    base_attribute,
                    is_negated,
                    join_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListAugmentedQueryJoinConditionVisitor)
    }
}
