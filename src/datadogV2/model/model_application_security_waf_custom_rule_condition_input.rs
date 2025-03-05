// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Input from the request on which the condition should apply.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleConditionInput {
    /// Input from the request on which the condition should apply.
    #[serde(rename = "address")]
    pub address: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress,
    /// Specific path for the input.
    #[serde(rename = "key_path")]
    pub key_path: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleConditionInput {
    pub fn new(
        address: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress,
    ) -> ApplicationSecurityWafCustomRuleConditionInput {
        ApplicationSecurityWafCustomRuleConditionInput {
            address,
            key_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn key_path(mut self, value: Vec<String>) -> Self {
        self.key_path = Some(value);
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleConditionInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleConditionInputVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleConditionInputVisitor {
            type Value = ApplicationSecurityWafCustomRuleConditionInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut address: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress,
                > = None;
                let mut key_path: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "address" => {
                            address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _address) = address {
                                match _address {
                                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInputAddress::UnparsedObject(_address) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "key_path" => {
                            if v.is_null() {
                                continue;
                            }
                            key_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let address = address.ok_or_else(|| M::Error::missing_field("address"))?;

                let content = ApplicationSecurityWafCustomRuleConditionInput {
                    address,
                    key_path,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleConditionInputVisitor)
    }
}
