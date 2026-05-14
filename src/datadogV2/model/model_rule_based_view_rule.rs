// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A compliance rule along with its evaluation statistics and framework mappings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleBasedViewRule {
    /// List of compliance framework mappings associated with the rule.
    #[serde(rename = "compliance_frameworks")]
    pub compliance_frameworks: Vec<crate::datadogV2::model::RuleBasedViewComplianceFramework>,
    /// Whether the rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Unique identifier of the rule.
    #[serde(rename = "id")]
    pub id: String,
    /// Human-readable name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// List of resource attribute names exposed by the rule.
    #[serde(rename = "resourceAttributes")]
    pub resource_attributes: Vec<String>,
    /// Resource category targeted by the rule.
    #[serde(rename = "resourceCategory")]
    pub resource_category: String,
    /// Resource type targeted by the rule.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Counts of findings for the rule, grouped by their evaluation status.
    #[serde(rename = "stats")]
    pub stats: crate::datadogV2::model::RuleBasedViewRuleStats,
    /// Severity associated with the rule (for example, `info`, `low`, `medium`, `high`, or `critical`).
    #[serde(rename = "status")]
    pub status: String,
    /// List of tags attached to the rule.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The category of the security rule.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RuleBasedViewRuleCategory,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleBasedViewRule {
    pub fn new(
        compliance_frameworks: Vec<crate::datadogV2::model::RuleBasedViewComplianceFramework>,
        enabled: bool,
        id: String,
        name: String,
        resource_attributes: Vec<String>,
        resource_category: String,
        resource_type: String,
        stats: crate::datadogV2::model::RuleBasedViewRuleStats,
        status: String,
        tags: Vec<String>,
        type_: crate::datadogV2::model::RuleBasedViewRuleCategory,
    ) -> RuleBasedViewRule {
        RuleBasedViewRule {
            compliance_frameworks,
            enabled,
            id,
            name,
            resource_attributes,
            resource_category,
            resource_type,
            stats,
            status,
            tags,
            type_,
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

impl<'de> Deserialize<'de> for RuleBasedViewRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleBasedViewRuleVisitor;
        impl<'a> Visitor<'a> for RuleBasedViewRuleVisitor {
            type Value = RuleBasedViewRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compliance_frameworks: Option<
                    Vec<crate::datadogV2::model::RuleBasedViewComplianceFramework>,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut resource_attributes: Option<Vec<String>> = None;
                let mut resource_category: Option<String> = None;
                let mut resource_type: Option<String> = None;
                let mut stats: Option<crate::datadogV2::model::RuleBasedViewRuleStats> = None;
                let mut status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::RuleBasedViewRuleCategory> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compliance_frameworks" => {
                            compliance_frameworks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceAttributes" => {
                            resource_attributes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceCategory" => {
                            resource_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceType" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stats" => {
                            stats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::RuleBasedViewRuleCategory::UnparsedObject(_type_) => {
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
                let compliance_frameworks = compliance_frameworks
                    .ok_or_else(|| M::Error::missing_field("compliance_frameworks"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let resource_attributes = resource_attributes
                    .ok_or_else(|| M::Error::missing_field("resource_attributes"))?;
                let resource_category = resource_category
                    .ok_or_else(|| M::Error::missing_field("resource_category"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let stats = stats.ok_or_else(|| M::Error::missing_field("stats"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RuleBasedViewRule {
                    compliance_frameworks,
                    enabled,
                    id,
                    name,
                    resource_attributes,
                    resource_category,
                    resource_type,
                    stats,
                    status,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleBasedViewRuleVisitor)
    }
}
