// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the rule proposal to start.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleProposalStartRequestAttributes {
    /// The natural-language description or rule to generate the proposed rule from, in the
    /// format given by `inputType`.
    #[serde(rename = "input")]
    pub input: String,
    /// The type of input to generate the proposed rule from: `natural_language` for a
    /// plain-English description of the behavior to detect, `spl` for a Splunk SPL search or
    /// saved search definition, or `kql` for a Microsoft KQL query. SPL inputs must be
    /// self-contained, with search macros expanded.
    #[serde(rename = "inputType")]
    pub input_type: crate::datadogV2::model::SecurityMonitoringRuleProposalInputType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleProposalStartRequestAttributes {
    pub fn new(
        input: String,
        input_type: crate::datadogV2::model::SecurityMonitoringRuleProposalInputType,
    ) -> SecurityMonitoringRuleProposalStartRequestAttributes {
        SecurityMonitoringRuleProposalStartRequestAttributes {
            input,
            input_type,
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleProposalStartRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleProposalStartRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleProposalStartRequestAttributesVisitor {
            type Value = SecurityMonitoringRuleProposalStartRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut input: Option<String> = None;
                let mut input_type: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleProposalInputType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "input" => {
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputType" => {
                            input_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input_type) = input_type {
                                match _input_type {
                                    crate::datadogV2::model::SecurityMonitoringRuleProposalInputType::UnparsedObject(_input_type) => {
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
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;
                let input_type = input_type.ok_or_else(|| M::Error::missing_field("input_type"))?;

                let content = SecurityMonitoringRuleProposalStartRequestAttributes {
                    input,
                    input_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleProposalStartRequestAttributesVisitor)
    }
}
