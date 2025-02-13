// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A change in a rule version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleVersionUpdate {
    /// The new value of the field.
    #[serde(rename = "change")]
    pub change: Option<String>,
    /// The field that was changed.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The type of change.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleVersionUpdateType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleVersionUpdate {
    pub fn new() -> RuleVersionUpdate {
        RuleVersionUpdate {
            change: None,
            field: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change(mut self, value: String) -> Self {
        self.change = Some(value);
        self
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::RuleVersionUpdateType) -> Self {
        self.type_ = Some(value);
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

impl Default for RuleVersionUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RuleVersionUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleVersionUpdateVisitor;
        impl<'a> Visitor<'a> for RuleVersionUpdateVisitor {
            type Value = RuleVersionUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change: Option<String> = None;
                let mut field: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::RuleVersionUpdateType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change" => {
                            if v.is_null() {
                                continue;
                            }
                            change = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RuleVersionUpdateType::UnparsedObject(_type_) => {
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

                let content = RuleVersionUpdate {
                    change,
                    field,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleVersionUpdateVisitor)
    }
}
