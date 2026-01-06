// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case creation attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseCreateAttributes {
    /// Description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Case priority
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::CasePriority>,
    /// Title
    #[serde(rename = "title")]
    pub title: String,
    /// Case type UUID
    #[serde(rename = "type_id")]
    pub type_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseCreateAttributes {
    pub fn new(title: String, type_id: String) -> CaseCreateAttributes {
        CaseCreateAttributes {
            description: None,
            priority: None,
            title,
            type_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn priority(mut self, value: crate::datadogV2::model::CasePriority) -> Self {
        self.priority = Some(value);
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

impl<'de> Deserialize<'de> for CaseCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseCreateAttributesVisitor;
        impl<'a> Visitor<'a> for CaseCreateAttributesVisitor {
            type Value = CaseCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut priority: Option<crate::datadogV2::model::CasePriority> = None;
                let mut title: Option<String> = None;
                let mut type_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::CasePriority::UnparsedObject(
                                        _priority,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type_id" => {
                            type_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let type_id = type_id.ok_or_else(|| M::Error::missing_field("type_id"))?;

                let content = CaseCreateAttributes {
                    description,
                    priority,
                    title,
                    type_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseCreateAttributesVisitor)
    }
}
