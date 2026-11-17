// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a custom recommendation rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationCustomRuleAttributes {
    /// The cloud provider of the recommender, such as `aws`, `azure`, or `gcp`.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: String,
    /// When the rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the rule.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Whether the rule is applied when recommendations are generated.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Tag filters that select the resources the rule applies to. An empty list selects every resource.
    #[serde(rename = "filters")]
    pub filters: Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>,
    /// When the rule was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who last modified the rule.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Free-form notes, such as the reason for the customization.
    #[serde(rename = "notes")]
    pub notes: String,
    /// The order in which the rules of a recommender are evaluated, lowest first. It is assigned when the rule is created.
    #[serde(rename = "priority")]
    pub priority: i64,
    /// The recommender the rule customizes.
    #[serde(rename = "recommender_id")]
    pub recommender_id: String,
    /// Setting overrides, keyed by setting name. Settings left out keep the recommender default. The accepted
    /// settings and their ranges depend on the recommender.
    #[serde(rename = "settings")]
    pub settings: std::collections::BTreeMap<String, f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationCustomRuleAttributes {
    pub fn new(
        cloud_provider: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        enabled: bool,
        filters: Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
        name: String,
        notes: String,
        priority: i64,
        recommender_id: String,
        settings: std::collections::BTreeMap<String, f64>,
    ) -> CostRecommendationCustomRuleAttributes {
        CostRecommendationCustomRuleAttributes {
            cloud_provider,
            created_at,
            created_by,
            enabled,
            filters,
            modified_at,
            modified_by,
            name,
            notes,
            priority,
            recommender_id,
            settings,
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

impl<'de> Deserialize<'de> for CostRecommendationCustomRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationCustomRuleAttributesVisitor;
        impl<'a> Visitor<'a> for CostRecommendationCustomRuleAttributesVisitor {
            type Value = CostRecommendationCustomRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud_provider: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut filters: Option<
                    Vec<crate::datadogV2::model::CostRecommendationCustomRuleFilter>,
                > = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut notes: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut recommender_id: Option<String> = None;
                let mut settings: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud_provider" => {
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notes" => {
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommender_id" => {
                            recommender_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cloud_provider =
                    cloud_provider.ok_or_else(|| M::Error::missing_field("cloud_provider"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let notes = notes.ok_or_else(|| M::Error::missing_field("notes"))?;
                let priority = priority.ok_or_else(|| M::Error::missing_field("priority"))?;
                let recommender_id =
                    recommender_id.ok_or_else(|| M::Error::missing_field("recommender_id"))?;
                let settings = settings.ok_or_else(|| M::Error::missing_field("settings"))?;

                let content = CostRecommendationCustomRuleAttributes {
                    cloud_provider,
                    created_at,
                    created_by,
                    enabled,
                    filters,
                    modified_at,
                    modified_by,
                    name,
                    notes,
                    priority,
                    recommender_id,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationCustomRuleAttributesVisitor)
    }
}
