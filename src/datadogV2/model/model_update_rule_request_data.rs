// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Data for the request to update a scorecard rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRuleRequestData {
    /// Attributes for creating or updating a rule. Server-managed fields (created_at, modified_at, custom) are excluded.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RuleAttributesRequest>,
    /// The JSON:API type for scorecard rules.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl UpdateRuleRequestData {
    pub fn new() -> UpdateRuleRequestData {
        UpdateRuleRequestData {
            attributes: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn attributes(mut self, value: crate::datadogV2::model::RuleAttributesRequest) -> Self {
        self.attributes = Some(value);
        self
    }
    
    pub fn type_(mut self, value: crate::datadogV2::model::RuleType) -> Self {
        self.type_ = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for UpdateRuleRequestData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateRuleRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateRuleRequestDataVisitor;
        impl<'a> Visitor<'a> for UpdateRuleRequestDataVisitor {
            type Value = UpdateRuleRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::RuleAttributesRequest> = None;
                let mut type_: Option<crate::datadogV2::model::RuleType> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RuleType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = UpdateRuleRequestData {
                    attributes,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateRuleRequestDataVisitor)
    }
}
