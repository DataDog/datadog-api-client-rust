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
pub struct NumberFormatUnitCustom {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::NumberFormatUnitCustomType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl NumberFormatUnitCustom {
    pub fn new(label: String, type_: crate::datadogV1_20270101::model::NumberFormatUnitCustomType) -> NumberFormatUnitCustom {
        NumberFormatUnitCustom {
            label,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for NumberFormatUnitCustom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberFormatUnitCustomVisitor;
        impl<'a> Visitor<'a> for NumberFormatUnitCustomVisitor {
            type Value = NumberFormatUnitCustom;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut label: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::NumberFormatUnitCustomType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::NumberFormatUnitCustomType::UnparsedObject(_type_) => {
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
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NumberFormatUnitCustom {
                    label,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NumberFormatUnitCustomVisitor)
    }
}
