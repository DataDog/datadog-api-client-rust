// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The payload of a cloud configuration rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationRulePayload {
    /// Description of generated findings and signals (severity and channels to be notified in case of a signal). Must contain exactly one item.
    ///
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::CloudConfigurationRuleCaseCreate>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options:
        crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
    /// Custom/Overridden message for generated signals (used in case of Default rule update).
    #[serde(rename = "customMessage")]
    pub custom_message: Option<String>,
    /// Custom/Overridden name of the rule (used in case of Default rule update).
    #[serde(rename = "customName")]
    pub custom_name: Option<String>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message in markdown format for generated findings and signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options on cloud configuration rules.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::CloudConfigurationRuleOptions,
    /// Tags for generated findings and signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CloudConfigurationRuleType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationRulePayload {
    pub fn new(
        cases: Vec<crate::datadogV2::model::CloudConfigurationRuleCaseCreate>,
        compliance_signal_options: crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
        is_enabled: bool,
        message: String,
        name: String,
        options: crate::datadogV2::model::CloudConfigurationRuleOptions,
    ) -> CloudConfigurationRulePayload {
        CloudConfigurationRulePayload {
            cases,
            compliance_signal_options,
            custom_message: None,
            custom_name: None,
            filters: None,
            is_enabled,
            message,
            name,
            options,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_message(mut self, value: String) -> Self {
        self.custom_message = Some(value);
        self
    }

    pub fn custom_name(mut self, value: String) -> Self {
        self.custom_name = Some(value);
        self
    }

    pub fn filters(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringFilter>,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::CloudConfigurationRuleType) -> Self {
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

impl<'de> Deserialize<'de> for CloudConfigurationRulePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationRulePayloadVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationRulePayloadVisitor {
            type Value = CloudConfigurationRulePayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cases: Option<
                    Vec<crate::datadogV2::model::CloudConfigurationRuleCaseCreate>,
                > = None;
                let mut compliance_signal_options: Option<
                    crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
                > = None;
                let mut custom_message: Option<String> = None;
                let mut custom_name: Option<String> = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut is_enabled: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::CloudConfigurationRuleOptions> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::CloudConfigurationRuleType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cases" => {
                            cases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "complianceSignalOptions" => {
                            compliance_signal_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customMessage" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customName" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isEnabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CloudConfigurationRuleType::UnparsedObject(_type_) => {
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
                let cases = cases.ok_or_else(|| M::Error::missing_field("cases"))?;
                let compliance_signal_options = compliance_signal_options
                    .ok_or_else(|| M::Error::missing_field("compliance_signal_options"))?;
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;

                let content = CloudConfigurationRulePayload {
                    cases,
                    compliance_signal_options,
                    custom_message,
                    custom_name,
                    filters,
                    is_enabled,
                    message,
                    name,
                    options,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationRulePayloadVisitor)
    }
}
