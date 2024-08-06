// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A JavaScript assertion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionJavascript {
    /// The JavaScript code that performs the assertions.
    #[serde(rename = "code")]
    pub code: String,
    /// Type of the assertion.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAssertionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionJavascript {
    pub fn new(
        code: String,
        type_: crate::datadogV1::model::SyntheticsAssertionType,
    ) -> SyntheticsAssertionJavascript {
        SyntheticsAssertionJavascript {
            code,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionJavascript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionJavascriptVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionJavascriptVisitor {
            type Value = SyntheticsAssertionJavascript;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsAssertionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsAssertionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsAssertionJavascript {
                    code,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionJavascriptVisitor)
    }
}
