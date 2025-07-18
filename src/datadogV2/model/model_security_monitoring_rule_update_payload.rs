// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update an existing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleUpdatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options:
        Option<crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions>,
    /// Custom/Overridden Message for generated signals (used in case of Default rule update).
    #[serde(rename = "customMessage")]
    pub custom_message: Option<String>,
    /// Custom/Overridden name (used in case of Default rule update).
    #[serde(rename = "customName")]
    pub custom_name: Option<String>,
    /// Additional queries to filter matched events before they are processed. This field is deprecated for log detection, signal correlation, and workload security rules.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Additional grouping to perform on top of the existing groups in the query section. Must be a subset of the existing groups.
    #[serde(rename = "groupSignalsBy")]
    pub group_signals_by: Option<Vec<String>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Options.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleQuery>>,
    /// Reference tables for the rule.
    #[serde(rename = "referenceTables")]
    pub reference_tables: Option<Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating signals from third-party rules. Only available for third-party rules.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>>,
    /// The version of the rule being updated.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleUpdatePayload {
    pub fn new() -> SecurityMonitoringRuleUpdatePayload {
        SecurityMonitoringRuleUpdatePayload {
            cases: None,
            compliance_signal_options: None,
            custom_message: None,
            custom_name: None,
            filters: None,
            group_signals_by: None,
            has_extended_title: None,
            is_enabled: None,
            message: None,
            name: None,
            options: None,
            queries: None,
            reference_tables: None,
            tags: None,
            third_party_cases: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cases(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>,
    ) -> Self {
        self.cases = Some(value);
        self
    }

    pub fn compliance_signal_options(
        mut self,
        value: crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
    ) -> Self {
        self.compliance_signal_options = Some(value);
        self
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

    pub fn group_signals_by(mut self, value: Vec<String>) -> Self {
        self.group_signals_by = Some(value);
        self
    }

    pub fn has_extended_title(mut self, value: bool) -> Self {
        self.has_extended_title = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleOptions,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn queries(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleQuery>,
    ) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn reference_tables(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
    ) -> Self {
        self.reference_tables = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn third_party_cases(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>,
    ) -> Self {
        self.third_party_cases = Some(value);
        self
    }

    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
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

impl Default for SecurityMonitoringRuleUpdatePayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleUpdatePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleUpdatePayloadVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleUpdatePayloadVisitor {
            type Value = SecurityMonitoringRuleUpdatePayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cases: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>> =
                    None;
                let mut compliance_signal_options: Option<
                    crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions,
                > = None;
                let mut custom_message: Option<String> = None;
                let mut custom_name: Option<String> = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut group_signals_by: Option<Vec<String>> = None;
                let mut has_extended_title: Option<bool> = None;
                let mut is_enabled: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions> =
                    None;
                let mut queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleQuery>> =
                    None;
                let mut reference_tables: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut third_party_cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>,
                > = None;
                let mut version: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cases" => {
                            if v.is_null() {
                                continue;
                            }
                            cases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "complianceSignalOptions" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "groupSignalsBy" => {
                            if v.is_null() {
                                continue;
                            }
                            group_signals_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasExtendedTitle" => {
                            if v.is_null() {
                                continue;
                            }
                            has_extended_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isEnabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            if v.is_null() {
                                continue;
                            }
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "referenceTables" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_tables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thirdPartyCases" => {
                            if v.is_null() {
                                continue;
                            }
                            third_party_cases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleUpdatePayload {
                    cases,
                    compliance_signal_options,
                    custom_message,
                    custom_name,
                    filters,
                    group_signals_by,
                    has_extended_title,
                    is_enabled,
                    message,
                    name,
                    options,
                    queries,
                    reference_tables,
                    tags,
                    third_party_cases,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleUpdatePayloadVisitor)
    }
}
