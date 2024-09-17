// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Text format rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableWidgetTextFormatRule {
    /// Hex representation of the custom background color. Used with custom background palette option.
    #[serde(rename = "custom_bg_color")]
    pub custom_bg_color: Option<String>,
    /// Hex representation of the custom text color. Used with custom text palette option.
    #[serde(rename = "custom_fg_color")]
    pub custom_fg_color: Option<String>,
    /// Match rule for the table widget text format.
    #[serde(rename = "match")]
    pub match_: crate::datadogV1::model::TableWidgetTextFormatMatch,
    /// Color-on-color palette to highlight replaced text.
    #[serde(rename = "palette")]
    pub palette: Option<crate::datadogV1::model::TableWidgetTextFormatPalette>,
    /// Replace rule for the table widget text format.
    #[serde(rename = "replace")]
    pub replace: Option<crate::datadogV1::model::TableWidgetTextFormatReplace>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableWidgetTextFormatRule {
    pub fn new(
        match_: crate::datadogV1::model::TableWidgetTextFormatMatch,
    ) -> TableWidgetTextFormatRule {
        TableWidgetTextFormatRule {
            custom_bg_color: None,
            custom_fg_color: None,
            match_,
            palette: None,
            replace: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn palette(mut self, value: crate::datadogV1::model::TableWidgetTextFormatPalette) -> Self {
        self.palette = Some(value);
        self
    }

    pub fn replace(mut self, value: crate::datadogV1::model::TableWidgetTextFormatReplace) -> Self {
        self.replace = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TableWidgetTextFormatRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableWidgetTextFormatRuleVisitor;
        impl<'a> Visitor<'a> for TableWidgetTextFormatRuleVisitor {
            type Value = TableWidgetTextFormatRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_bg_color: Option<String> = None;
                let mut custom_fg_color: Option<String> = None;
                let mut match_: Option<crate::datadogV1::model::TableWidgetTextFormatMatch> = None;
                let mut palette: Option<crate::datadogV1::model::TableWidgetTextFormatPalette> =
                    None;
                let mut replace: Option<crate::datadogV1::model::TableWidgetTextFormatReplace> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_bg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_bg_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_fg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_fg_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match" => {
                            match_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1::model::TableWidgetTextFormatPalette::UnparsedObject(_palette) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "replace" => {
                            if v.is_null() {
                                continue;
                            }
                            replace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _replace) = replace {
                                match _replace {
                                    crate::datadogV1::model::TableWidgetTextFormatReplace::UnparsedObject(_replace) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let match_ = match_.ok_or_else(|| M::Error::missing_field("match_"))?;

                let content = TableWidgetTextFormatRule {
                    custom_bg_color,
                    custom_fg_color,
                    match_,
                    palette,
                    replace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableWidgetTextFormatRuleVisitor)
    }
}
