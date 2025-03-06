// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ApplicationSecurityWafCustomRuleAction` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleAction {
    /// Override the default action to take when the WAF custom rule would block.
    #[serde(rename = "action")]
    pub action: Option<crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionAction>,
    /// The definition of `ApplicationSecurityWafCustomRuleActionParameters` object.
    #[serde(rename = "parameters")]
    pub parameters:
        Option<crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionParameters>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleAction {
    pub fn new() -> ApplicationSecurityWafCustomRuleAction {
        ApplicationSecurityWafCustomRuleAction {
            action: None,
            parameters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionAction,
    ) -> Self {
        self.action = Some(value);
        self
    }

    pub fn parameters(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionParameters,
    ) -> Self {
        self.parameters = Some(value);
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

impl Default for ApplicationSecurityWafCustomRuleAction {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleActionVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleActionVisitor {
            type Value = ApplicationSecurityWafCustomRuleAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionAction,
                > = None;
                let mut parameters: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionParameters,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleActionAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityWafCustomRuleAction {
                    action,
                    parameters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleActionVisitor)
    }
}
