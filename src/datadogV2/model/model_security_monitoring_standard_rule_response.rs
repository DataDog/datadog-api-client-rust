// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringStandardRuleResponse {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>>,
    /// How to generate compliance signals. Useful for cloud_configuration rules only.
    #[serde(rename = "complianceSignalOptions")]
    pub compliance_signal_options:
        Option<crate::datadogV2::model::CloudConfigurationRuleComplianceSignalOptions>,
    /// When the rule was created, timestamp in milliseconds.
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    /// User ID of the user who created the rule.
    #[serde(rename = "creationAuthorId")]
    pub creation_author_id: Option<i64>,
    /// Custom/Overridden message for generated signals (used in case of Default rule update).
    #[serde(rename = "customMessage")]
    pub custom_message: Option<String>,
    /// Custom/Overridden name of the rule (used in case of Default rule update).
    #[serde(rename = "customName")]
    pub custom_name: Option<String>,
    /// Default Tags for default rules (included in tags)
    #[serde(rename = "defaultTags")]
    pub default_tags: Option<Vec<String>>,
    /// When the rule will be deprecated, timestamp in milliseconds.
    #[serde(rename = "deprecationDate")]
    pub deprecation_date: Option<i64>,
    /// Additional queries to filter matched events before they are processed. This field is deprecated for log detection, signal correlation, and workload security rules.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Additional grouping to perform on top of the existing groups in the query section. Must be a subset of the existing groups.
    #[serde(rename = "groupSignalsBy")]
    pub group_signals_by: Option<Vec<String>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// The ID of the rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether the rule is included by default.
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
    /// Whether the rule has been deleted.
    #[serde(rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Options.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>>,
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
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeRead>,
    /// User ID of the user who updated the rule.
    #[serde(rename = "updateAuthorId")]
    pub update_author_id: Option<i64>,
    /// The date the rule was last updated, in milliseconds.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    /// The version of the rule.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringStandardRuleResponse {
    pub fn new() -> SecurityMonitoringStandardRuleResponse {
        SecurityMonitoringStandardRuleResponse {
            cases: None,
            compliance_signal_options: None,
            created_at: None,
            creation_author_id: None,
            custom_message: None,
            custom_name: None,
            default_tags: None,
            deprecation_date: None,
            filters: None,
            group_signals_by: None,
            has_extended_title: None,
            id: None,
            is_default: None,
            is_deleted: None,
            is_enabled: None,
            message: None,
            name: None,
            options: None,
            queries: None,
            reference_tables: None,
            tags: None,
            third_party_cases: None,
            type_: None,
            update_author_id: None,
            updated_at: None,
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

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creation_author_id(mut self, value: i64) -> Self {
        self.creation_author_id = Some(value);
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

    pub fn default_tags(mut self, value: Vec<String>) -> Self {
        self.default_tags = Some(value);
        self
    }

    pub fn deprecation_date(mut self, value: i64) -> Self {
        self.deprecation_date = Some(value);
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

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
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
        value: Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
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

    pub fn type_(mut self, value: crate::datadogV2::model::SecurityMonitoringRuleTypeRead) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn update_author_id(mut self, value: i64) -> Self {
        self.update_author_id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
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

impl Default for SecurityMonitoringStandardRuleResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringStandardRuleResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringStandardRuleResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringStandardRuleResponseVisitor {
            type Value = SecurityMonitoringStandardRuleResponse;

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
                let mut created_at: Option<i64> = None;
                let mut creation_author_id: Option<i64> = None;
                let mut custom_message: Option<String> = None;
                let mut custom_name: Option<String> = None;
                let mut default_tags: Option<Vec<String>> = None;
                let mut deprecation_date: Option<i64> = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut group_signals_by: Option<Vec<String>> = None;
                let mut has_extended_title: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut is_deleted: Option<bool> = None;
                let mut is_enabled: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions> =
                    None;
                let mut queries: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
                > = None;
                let mut reference_tables: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut third_party_cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>,
                > = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeRead> =
                    None;
                let mut update_author_id: Option<i64> = None;
                let mut updated_at: Option<i64> = None;
                let mut version: Option<i64> = None;
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
                        "createdAt" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creationAuthorId" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_author_id =
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
                        "defaultTags" => {
                            if v.is_null() {
                                continue;
                            }
                            default_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deprecationDate" => {
                            if v.is_null() {
                                continue;
                            }
                            deprecation_date =
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
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDefault" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDeleted" => {
                            if v.is_null() {
                                continue;
                            }
                            is_deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringRuleTypeRead::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updateAuthorId" => {
                            if v.is_null() {
                                continue;
                            }
                            update_author_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SecurityMonitoringStandardRuleResponse {
                    cases,
                    compliance_signal_options,
                    created_at,
                    creation_author_id,
                    custom_message,
                    custom_name,
                    default_tags,
                    deprecation_date,
                    filters,
                    group_signals_by,
                    has_extended_title,
                    id,
                    is_default,
                    is_deleted,
                    is_enabled,
                    message,
                    name,
                    options,
                    queries,
                    reference_tables,
                    tags,
                    third_party_cases,
                    type_,
                    update_author_id,
                    updated_at,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringStandardRuleResponseVisitor)
    }
}
