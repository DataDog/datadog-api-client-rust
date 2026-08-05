// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Shifts matched findings up or down by one severity rank.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeverityModifierRuleShiftAction {
    /// An optional free-form explanation for the severity change.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The direction in which to shift the severity of matched findings by one rank.
    #[serde(rename = "severity_delta")]
    pub severity_delta: crate::datadogV2::model::SeverityModifierSeverityDelta,
    /// The type of a severity modifier rule action that shifts the severity by one rank.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SeverityModifierRuleShiftActionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SeverityModifierRuleShiftAction {
    pub fn new(
        severity_delta: crate::datadogV2::model::SeverityModifierSeverityDelta,
        type_: crate::datadogV2::model::SeverityModifierRuleShiftActionType,
    ) -> SeverityModifierRuleShiftAction {
        SeverityModifierRuleShiftAction {
            description: None,
            severity_delta,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for SeverityModifierRuleShiftAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeverityModifierRuleShiftActionVisitor;
        impl<'a> Visitor<'a> for SeverityModifierRuleShiftActionVisitor {
            type Value = SeverityModifierRuleShiftAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut severity_delta: Option<
                    crate::datadogV2::model::SeverityModifierSeverityDelta,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::SeverityModifierRuleShiftActionType,
                > = None;
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
                        "severity_delta" => {
                            severity_delta =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity_delta) = severity_delta {
                                match _severity_delta {
                                    crate::datadogV2::model::SeverityModifierSeverityDelta::UnparsedObject(_severity_delta) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SeverityModifierRuleShiftActionType::UnparsedObject(_type_) => {
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
                let severity_delta =
                    severity_delta.ok_or_else(|| M::Error::missing_field("severity_delta"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SeverityModifierRuleShiftAction {
                    description,
                    severity_delta,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SeverityModifierRuleShiftActionVisitor)
    }
}
