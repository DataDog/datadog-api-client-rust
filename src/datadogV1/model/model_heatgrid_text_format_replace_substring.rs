// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Replace a substring of the matching group label.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridTextFormatReplaceSubstring {
    /// Substring to replace.
    #[serde(rename = "substring")]
    pub substring: String,
    /// Replace a substring of a matching group label.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::HeatgridTextFormatReplaceSubstringType,
    /// Replacement string.
    #[serde(rename = "with")]
    pub with: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridTextFormatReplaceSubstring {
    pub fn new(
        substring: String,
        type_: crate::datadogV1::model::HeatgridTextFormatReplaceSubstringType,
        with: String,
    ) -> HeatgridTextFormatReplaceSubstring {
        HeatgridTextFormatReplaceSubstring {
            substring,
            type_,
            with,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridTextFormatReplaceSubstring {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridTextFormatReplaceSubstringVisitor;
        impl<'a> Visitor<'a> for HeatgridTextFormatReplaceSubstringVisitor {
            type Value = HeatgridTextFormatReplaceSubstring;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut substring: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV1::model::HeatgridTextFormatReplaceSubstringType,
                > = None;
                let mut with: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "substring" => {
                            substring = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::HeatgridTextFormatReplaceSubstringType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "with" => {
                            with = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let substring = substring.ok_or_else(|| M::Error::missing_field("substring"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let with = with.ok_or_else(|| M::Error::missing_field("with"))?;

                let content = HeatgridTextFormatReplaceSubstring {
                    substring,
                    type_,
                    with,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridTextFormatReplaceSubstringVisitor)
    }
}
