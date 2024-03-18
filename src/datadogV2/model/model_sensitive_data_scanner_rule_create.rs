// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data related to the creation of a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerRuleCreate {
    /// Attributes of the Sensitive Data Scanner rule.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
    /// Relationships of a scanning rule.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
    /// Sensitive Data Scanner rule type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SensitiveDataScannerRuleType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerRuleCreate {
    pub fn new(
        attributes: crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
        relationships: crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
        type_: crate::datadogV2::model::SensitiveDataScannerRuleType,
    ) -> SensitiveDataScannerRuleCreate {
        SensitiveDataScannerRuleCreate {
            attributes,
            relationships,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerRuleCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerRuleCreateVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerRuleCreateVisitor {
            type Value = SensitiveDataScannerRuleCreate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
                > = None;
                let mut relationships: Option<
                    crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::SensitiveDataScannerRuleType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SensitiveDataScannerRuleType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let relationships =
                    relationships.ok_or_else(|| M::Error::missing_field("relationships"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SensitiveDataScannerRuleCreate {
                    attributes,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerRuleCreateVisitor)
    }
}
