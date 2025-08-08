// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The payload of a rule to test
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringStandardRuleTestPayload {
    /// Calculated fields. Only allowed for scheduled rules - in other words, when schedulingOptions is also defined.
    #[serde(rename = "calculatedFields")]
    pub calculated_fields: Option<Vec<crate::datadogV2::model::CalculatedField>>,
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
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
    pub is_enabled: bool,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::SecurityMonitoringRuleOptions,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
    /// Reference tables for the rule.
    #[serde(rename = "referenceTables")]
    pub reference_tables: Option<Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>>,
    /// Options for scheduled rules. When this field is present, the rule runs based on the schedule. When absent, it runs real-time on ingested logs.
    #[serde(
        rename = "schedulingOptions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub scheduling_options:
        Option<Option<crate::datadogV2::model::SecurityMonitoringSchedulingOptions>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating signals from third-party rules. Only available for third-party rules.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeTest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringStandardRuleTestPayload {
    pub fn new(
        cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
        is_enabled: bool,
        message: String,
        name: String,
        options: crate::datadogV2::model::SecurityMonitoringRuleOptions,
        queries: Vec<crate::datadogV2::model::SecurityMonitoringStandardRuleQuery>,
    ) -> SecurityMonitoringStandardRuleTestPayload {
        SecurityMonitoringStandardRuleTestPayload {
            calculated_fields: None,
            cases,
            filters: None,
            group_signals_by: None,
            has_extended_title: None,
            is_enabled,
            message,
            name,
            options,
            queries,
            reference_tables: None,
            scheduling_options: None,
            tags: None,
            third_party_cases: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn calculated_fields(
        mut self,
        value: Vec<crate::datadogV2::model::CalculatedField>,
    ) -> Self {
        self.calculated_fields = Some(value);
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

    pub fn reference_tables(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
    ) -> Self {
        self.reference_tables = Some(value);
        self
    }

    pub fn scheduling_options(
        mut self,
        value: Option<crate::datadogV2::model::SecurityMonitoringSchedulingOptions>,
    ) -> Self {
        self.scheduling_options = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn third_party_cases(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>,
    ) -> Self {
        self.third_party_cases = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::SecurityMonitoringRuleTypeTest) -> Self {
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

impl<'de> Deserialize<'de> for SecurityMonitoringStandardRuleTestPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringStandardRuleTestPayloadVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringStandardRuleTestPayloadVisitor {
            type Value = SecurityMonitoringStandardRuleTestPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV2::model::CalculatedField>> =
                    None;
                let mut cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
                > = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut group_signals_by: Option<Vec<String>> = None;
                let mut has_extended_title: Option<bool> = None;
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
                let mut scheduling_options: Option<
                    Option<crate::datadogV2::model::SecurityMonitoringSchedulingOptions>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut third_party_cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>,
                > = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeTest> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calculatedFields" => {
                            if v.is_null() {
                                continue;
                            }
                            calculated_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cases" => {
                            cases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "referenceTables" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_tables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedulingOptions" => {
                            scheduling_options =
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
                                    crate::datadogV2::model::SecurityMonitoringRuleTypeTest::UnparsedObject(_type_) => {
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
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = SecurityMonitoringStandardRuleTestPayload {
                    calculated_fields,
                    cases,
                    filters,
                    group_signals_by,
                    has_extended_title,
                    is_enabled,
                    message,
                    name,
                    options,
                    queries,
                    reference_tables,
                    scheduling_options,
                    tags,
                    third_party_cases,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringStandardRuleTestPayloadVisitor)
    }
}
