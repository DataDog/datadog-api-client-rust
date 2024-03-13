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
    /// When the rule will be deprecated, timestamp in milliseconds.
    #[serde(rename = "deprecationDate")]
    pub deprecation_date: Option<i64>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
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
    /// Options on rules.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating signals from third party rules. Only available for third party rules.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeRead>,
    /// User ID of the user who updated the rule.
    #[serde(rename = "updateAuthorId")]
    pub update_author_id: Option<i64>,
    /// The version of the rule.
    #[serde(rename = "version")]
    pub version: Option<i64>,
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
            deprecation_date: None,
            filters: None,
            has_extended_title: None,
            id: None,
            is_default: None,
            is_deleted: None,
            is_enabled: None,
            message: None,
            name: None,
            options: None,
            queries: None,
            tags: None,
            third_party_cases: None,
            type_: None,
            update_author_id: None,
            version: None,
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

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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
                let mut deprecation_date: Option<i64> = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
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
                let mut tags: Option<Vec<String>> = None;
                let mut third_party_cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCase>,
                > = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeRead> =
                    None;
                let mut update_author_id: Option<i64> = None;
                let mut version: Option<i64> = None;
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
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringStandardRuleResponse {
                    cases,
                    compliance_signal_options,
                    created_at,
                    creation_author_id,
                    deprecation_date,
                    filters,
                    has_extended_title,
                    id,
                    is_default,
                    is_deleted,
                    is_enabled,
                    message,
                    name,
                    options,
                    queries,
                    tags,
                    third_party_cases,
                    type_,
                    update_author_id,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringStandardRuleResponseVisitor)
    }
}
