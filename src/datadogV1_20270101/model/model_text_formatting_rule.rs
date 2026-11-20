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
pub struct TextFormattingRule {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_bg_color")]
    pub custom_bg_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_fg_color")]
    pub custom_fg_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "highlight_row")]
    pub highlight_row: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "match")]
    pub match_: crate::datadogV1_20270101::model::TextMatch,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<crate::datadogV1_20270101::model::ThresholdPaletteWithCustom>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "replace")]
    pub replace: Option<crate::datadogV1_20270101::model::TextFormattingRule20270101Replace>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TextFormattingRule {
    pub fn new(match_: crate::datadogV1_20270101::model::TextMatch) -> TextFormattingRule {
        TextFormattingRule {
            custom_bg_color: None,
            custom_fg_color: None,
            highlight_row: None,
            match_,
            palette: None,
            replace: None,
            _unparsed: false,
        }
    }
    
    pub fn custom_bg_color(mut self, value: String) -> Self {
        self.custom_bg_color = Some(value);
        self
    }
    
    pub fn custom_fg_color(mut self, value: String) -> Self {
        self.custom_fg_color = Some(value);
        self
    }
    
    pub fn highlight_row(mut self, value: bool) -> Self {
        self.highlight_row = Some(value);
        self
    }
    
    pub fn palette(mut self, value: crate::datadogV1_20270101::model::ThresholdPaletteWithCustom) -> Self {
        self.palette = Some(value);
        self
    }
    
    pub fn replace(mut self, value: crate::datadogV1_20270101::model::TextFormattingRule20270101Replace) -> Self {
        self.replace = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TextFormattingRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TextFormattingRuleVisitor;
        impl<'a> Visitor<'a> for TextFormattingRuleVisitor {
            type Value = TextFormattingRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_bg_color: Option<String> = None;
                let mut custom_fg_color: Option<String> = None;
                let mut highlight_row: Option<bool> = None;
                let mut match_: Option<crate::datadogV1_20270101::model::TextMatch> = None;
                let mut palette: Option<crate::datadogV1_20270101::model::ThresholdPaletteWithCustom> = None;
                let mut replace: Option<crate::datadogV1_20270101::model::TextFormattingRule20270101Replace> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_bg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_bg_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "custom_fg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_fg_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "highlight_row" => {
                            if v.is_null() {
                                continue;
                            }
                            highlight_row = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
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
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1_20270101::model::ThresholdPaletteWithCustom::UnparsedObject(_palette) => {
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
                                    crate::datadogV1_20270101::model::TextFormattingRule20270101Replace::UnparsedObject(_replace) => {
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

                let content = TextFormattingRule {
                    custom_bg_color,
                    custom_fg_color,
                    highlight_row,
                    match_,
                    palette,
                    replace,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TextFormattingRuleVisitor)
    }
}
