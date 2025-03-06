// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One condition of the WAF Custom Rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleCondition {
    /// Operator to use for the WAF Condition.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator,
    /// The scope of the WAF custom rule.
    #[serde(rename = "parameters")]
    pub parameters: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionParameters,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleCondition {
    pub fn new(
        operator: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator,
        parameters: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionParameters,
    ) -> ApplicationSecurityWafCustomRuleCondition {
        ApplicationSecurityWafCustomRuleCondition {
            operator,
            parameters,
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleConditionVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleConditionVisitor {
            type Value = ApplicationSecurityWafCustomRuleCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut operator: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator,
                > = None;
                let mut parameters: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionParameters,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "parameters" => {
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let parameters = parameters.ok_or_else(|| M::Error::missing_field("parameters"))?;

                let content = ApplicationSecurityWafCustomRuleCondition {
                    operator,
                    parameters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleConditionVisitor)
    }
}
