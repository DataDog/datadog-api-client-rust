// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule for matching and optionally replacing a group label.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridTextFormatRule {
    /// Match a group label against a string.
    #[serde(rename = "match")]
    pub match_: crate::datadogV1::model::HeatgridTextFormatMatch,
    /// Replace all or part of a matching group label.
    #[serde(rename = "replace")]
    pub replace: Option<crate::datadogV1::model::HeatgridTextFormatReplace>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridTextFormatRule {
    pub fn new(match_: crate::datadogV1::model::HeatgridTextFormatMatch) -> HeatgridTextFormatRule {
        HeatgridTextFormatRule {
            match_,
            replace: None,
            _unparsed: false,
        }
    }

    pub fn replace(mut self, value: crate::datadogV1::model::HeatgridTextFormatReplace) -> Self {
        self.replace = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for HeatgridTextFormatRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridTextFormatRuleVisitor;
        impl<'a> Visitor<'a> for HeatgridTextFormatRuleVisitor {
            type Value = HeatgridTextFormatRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_: Option<crate::datadogV1::model::HeatgridTextFormatMatch> = None;
                let mut replace: Option<crate::datadogV1::model::HeatgridTextFormatReplace> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match" => {
                            match_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "replace" => {
                            if v.is_null() {
                                continue;
                            }
                            replace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _replace) = replace {
                                match _replace {
                                    crate::datadogV1::model::HeatgridTextFormatReplace::UnparsedObject(_replace) => {
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
                let match_ = match_.ok_or_else(|| M::Error::missing_field("match_"))?;

                let content = HeatgridTextFormatRule {
                    match_,
                    replace,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridTextFormatRuleVisitor)
    }
}
