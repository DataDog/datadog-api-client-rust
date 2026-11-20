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
pub struct TextFormattingRule20270101ReplaceOneOf4462863328 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::TextFormattingRuleReplaceAnyOfAnyOfItemType4452730016,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "with")]
    pub with: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TextFormattingRule20270101ReplaceOneOf4462863328 {
    pub fn new(type_: crate::datadogV1_20270101::model::TextFormattingRuleReplaceAnyOfAnyOfItemType4452730016, with: String) -> TextFormattingRule20270101ReplaceOneOf4462863328 {
        TextFormattingRule20270101ReplaceOneOf4462863328 {
            type_,
            with,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for TextFormattingRule20270101ReplaceOneOf4462863328 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TextFormattingRule20270101ReplaceOneOf4462863328Visitor;
        impl<'a> Visitor<'a> for TextFormattingRule20270101ReplaceOneOf4462863328Visitor {
            type Value = TextFormattingRule20270101ReplaceOneOf4462863328;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1_20270101::model::TextFormattingRuleReplaceAnyOfAnyOfItemType4452730016> = None;
                let mut with: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::TextFormattingRuleReplaceAnyOfAnyOfItemType4452730016::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "with" => {
                            with = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let with = with.ok_or_else(|| M::Error::missing_field("with"))?;

                let content = TextFormattingRule20270101ReplaceOneOf4462863328 {
                    type_,
                    with,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TextFormattingRule20270101ReplaceOneOf4462863328Visitor)
    }
}
