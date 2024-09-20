// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Match Sub-string definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableWidgetTextFormatReplaceSubstring {
    /// Text that will be replaced.
    #[serde(rename = "substring")]
    pub substring: String,
    /// Table widget text format replace sub-string type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TableWidgetTextFormatReplaceSubstringType,
    /// Text that will replace original sub-string.
    #[serde(rename = "with")]
    pub with: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableWidgetTextFormatReplaceSubstring {
    pub fn new(
        substring: String,
        type_: crate::datadogV1::model::TableWidgetTextFormatReplaceSubstringType,
        with: String,
    ) -> TableWidgetTextFormatReplaceSubstring {
        TableWidgetTextFormatReplaceSubstring {
            substring,
            type_,
            with,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TableWidgetTextFormatReplaceSubstring {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableWidgetTextFormatReplaceSubstringVisitor;
        impl<'a> Visitor<'a> for TableWidgetTextFormatReplaceSubstringVisitor {
            type Value = TableWidgetTextFormatReplaceSubstring;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut substring: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV1::model::TableWidgetTextFormatReplaceSubstringType,
                > = None;
                let mut with: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV1::model::TableWidgetTextFormatReplaceSubstringType::UnparsedObject(_type_) => {
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let substring = substring.ok_or_else(|| M::Error::missing_field("substring"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let with = with.ok_or_else(|| M::Error::missing_field("with"))?;

                let content = TableWidgetTextFormatReplaceSubstring {
                    substring,
                    type_,
                    with,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableWidgetTextFormatReplaceSubstringVisitor)
    }
}