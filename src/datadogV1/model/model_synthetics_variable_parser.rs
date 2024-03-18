// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the parser to use for the global variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsVariableParser {
    /// Type of parser for a Synthetic global variable from a synthetics test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsGlobalVariableParserType,
    /// Regex or JSON path used for the parser. Not used with type `raw`.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsVariableParser {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsGlobalVariableParserType,
    ) -> SyntheticsVariableParser {
        SyntheticsVariableParser {
            type_,
            value: None,
            _unparsed: false,
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsVariableParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsVariableParserVisitor;
        impl<'a> Visitor<'a> for SyntheticsVariableParserVisitor {
            type Value = SyntheticsVariableParser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1::model::SyntheticsGlobalVariableParserType> =
                    None;
                let mut value: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsGlobalVariableParserType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsVariableParser {
                    type_,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsVariableParserVisitor)
    }
}
