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
pub struct TextReplacementRule {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "match")]
    pub match_: crate::datadogV1_20270101::model::TextMatch,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "replace")]
    pub replace: Option<crate::datadogV1_20270101::model::TextReplacementRule20270101Replace>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TextReplacementRule {
    pub fn new(match_: crate::datadogV1_20270101::model::TextMatch) -> TextReplacementRule {
        TextReplacementRule {
            match_,
            replace: None,
            _unparsed: false,
        }
    }
    
    pub fn replace(mut self, value: crate::datadogV1_20270101::model::TextReplacementRule20270101Replace) -> Self {
        self.replace = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TextReplacementRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TextReplacementRuleVisitor;
        impl<'a> Visitor<'a> for TextReplacementRuleVisitor {
            type Value = TextReplacementRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_: Option<crate::datadogV1_20270101::model::TextMatch> = None;
                let mut replace: Option<crate::datadogV1_20270101::model::TextReplacementRule20270101Replace> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match" => {
                            match_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _match_) = match_ {
                                match _match_ {
                                    crate::datadogV1_20270101::model::TextMatch::UnparsedObject(_match_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "replace" => {
                            if v.is_null() {
                                continue;
                            }
                            replace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _replace) = replace {
                                match _replace {
                                    crate::datadogV1_20270101::model::TextReplacementRule20270101Replace::UnparsedObject(_replace) => {
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
                let match_ = match_.ok_or_else(|| M::Error::missing_field("match_"))?;

                let content = TextReplacementRule {
                    match_,
                    replace,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TextReplacementRuleVisitor)
    }
}
