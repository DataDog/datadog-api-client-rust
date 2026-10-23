// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a scheduled action.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageScheduledActionAttributes {
    /// The time when the action is scheduled to run.
    #[serde(rename = "at")]
    pub at: Option<chrono::DateTime<chrono::Utc>>,
    /// The kind of action that is scheduled.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OnCallPageScheduledActionAttributesType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageScheduledActionAttributes {
    pub fn new() -> OnCallPageScheduledActionAttributes {
        OnCallPageScheduledActionAttributes {
            at: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.at = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::OnCallPageScheduledActionAttributesType,
    ) -> Self {
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

impl Default for OnCallPageScheduledActionAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OnCallPageScheduledActionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageScheduledActionAttributesVisitor;
        impl<'a> Visitor<'a> for OnCallPageScheduledActionAttributesVisitor {
            type Value = OnCallPageScheduledActionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::OnCallPageScheduledActionAttributesType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "at" => {
                            if v.is_null() {
                                continue;
                            }
                            at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::OnCallPageScheduledActionAttributesType::UnparsedObject(_type_) => {
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

                let content = OnCallPageScheduledActionAttributes {
                    at,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageScheduledActionAttributesVisitor)
    }
}
