// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A transform applied to the values of one tag key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleTagValueTransform {
    /// The value used when no `map` entry matches.
    #[serde(rename = "default")]
    pub default: Option<String>,
    /// Lookup map from input value to output value, for `lookup_map`.
    #[serde(rename = "map")]
    pub map_: Option<std::collections::BTreeMap<String, String>>,
    /// The regular expression, for `regex_extract` and `regex_replace`.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// The replacement string, for `string_replace` and `regex_replace`.
    #[serde(rename = "replacement")]
    pub replacement: Option<String>,
    /// The substring to replace, for `string_replace`.
    #[serde(rename = "target")]
    pub target: Option<String>,
    /// The kind of transform.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ArbitraryRuleTagValueTransformType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryRuleTagValueTransform {
    pub fn new(
        type_: crate::datadogV2::model::ArbitraryRuleTagValueTransformType,
    ) -> ArbitraryRuleTagValueTransform {
        ArbitraryRuleTagValueTransform {
            default: None,
            map_: None,
            pattern: None,
            replacement: None,
            target: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default(mut self, value: String) -> Self {
        self.default = Some(value);
        self
    }

    pub fn map_(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.map_ = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn replacement(mut self, value: String) -> Self {
        self.replacement = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for ArbitraryRuleTagValueTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleTagValueTransformVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleTagValueTransformVisitor {
            type Value = ArbitraryRuleTagValueTransform;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default: Option<String> = None;
                let mut map_: Option<std::collections::BTreeMap<String, String>> = None;
                let mut pattern: Option<String> = None;
                let mut replacement: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ArbitraryRuleTagValueTransformType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default" => {
                            if v.is_null() {
                                continue;
                            }
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "map" => {
                            if v.is_null() {
                                continue;
                            }
                            map_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "replacement" => {
                            if v.is_null() {
                                continue;
                            }
                            replacement =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ArbitraryRuleTagValueTransformType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ArbitraryRuleTagValueTransform {
                    default,
                    map_,
                    pattern,
                    replacement,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryRuleTagValueTransformVisitor)
    }
}
