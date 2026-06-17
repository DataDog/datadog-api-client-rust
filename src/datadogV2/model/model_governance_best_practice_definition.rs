// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The best practice associated with an insight. Populated with the first active best practice
/// matched to the insight; `null` when no best practice is attached.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceBestPracticeDefinition {
    /// The value driver the best practice is grouped under, such as `access_governance`,
    /// `security`, `compliance`, or `operational_hygiene`.
    #[serde(rename = "category")]
    pub category: String,
    /// A relative link to the configuration page where the best practice can be acted upon.
    #[serde(rename = "deep_link")]
    pub deep_link: String,
    /// The full rationale and guidance for the best practice.
    #[serde(rename = "description")]
    pub description: String,
    /// An optional association to a control's detection type. `null` when not associated with a control.
    #[serde(
        rename = "detection_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub detection_type: Option<Option<String>>,
    /// The unique identifier of the best practice.
    #[serde(rename = "id")]
    pub id: String,
    /// The expected impact of following the best practice.
    #[serde(rename = "impact")]
    pub impact: String,
    /// A priority hint for ordering best practices by expected impact. Lower values indicate
    /// higher priority.
    #[serde(rename = "impact_hint")]
    pub impact_hint: i64,
    /// The permissions required for the user to act on the best practice.
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
    /// Whether the best practice is currently `active` or `deprecated`.
    #[serde(rename = "status")]
    pub status: String,
    /// A one-line explanation of why this best practice matters.
    #[serde(rename = "summary")]
    pub summary: String,
    /// A short, human-readable name for the best practice.
    #[serde(rename = "title")]
    pub title: String,
    /// The condition that surfaces the best practice. For an `insight` trigger, the insight
    /// slug; for a `static` trigger, a descriptive condition key.
    #[serde(rename = "trigger_condition")]
    pub trigger_condition: String,
    /// How the best practice is surfaced. `insight` ties it to an insight; `static` surfaces it
    /// unless its condition is met.
    #[serde(rename = "trigger_type")]
    pub trigger_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceBestPracticeDefinition {
    pub fn new(
        category: String,
        deep_link: String,
        description: String,
        id: String,
        impact: String,
        impact_hint: i64,
        permissions: Vec<String>,
        status: String,
        summary: String,
        title: String,
        trigger_condition: String,
        trigger_type: String,
    ) -> GovernanceBestPracticeDefinition {
        GovernanceBestPracticeDefinition {
            category,
            deep_link,
            description,
            detection_type: None,
            id,
            impact,
            impact_hint,
            permissions,
            status,
            summary,
            title,
            trigger_condition,
            trigger_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detection_type(mut self, value: Option<String>) -> Self {
        self.detection_type = Some(value);
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

impl<'de> Deserialize<'de> for GovernanceBestPracticeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceBestPracticeDefinitionVisitor;
        impl<'a> Visitor<'a> for GovernanceBestPracticeDefinitionVisitor {
            type Value = GovernanceBestPracticeDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut deep_link: Option<String> = None;
                let mut description: Option<String> = None;
                let mut detection_type: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut impact: Option<String> = None;
                let mut impact_hint: Option<i64> = None;
                let mut permissions: Option<Vec<String>> = None;
                let mut status: Option<String> = None;
                let mut summary: Option<String> = None;
                let mut title: Option<String> = None;
                let mut trigger_condition: Option<String> = None;
                let mut trigger_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deep_link" => {
                            deep_link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detection_type" => {
                            detection_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact" => {
                            impact = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_hint" => {
                            impact_hint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permissions" => {
                            permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_condition" => {
                            trigger_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_type" => {
                            trigger_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let deep_link = deep_link.ok_or_else(|| M::Error::missing_field("deep_link"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let impact = impact.ok_or_else(|| M::Error::missing_field("impact"))?;
                let impact_hint =
                    impact_hint.ok_or_else(|| M::Error::missing_field("impact_hint"))?;
                let permissions =
                    permissions.ok_or_else(|| M::Error::missing_field("permissions"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let summary = summary.ok_or_else(|| M::Error::missing_field("summary"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let trigger_condition = trigger_condition
                    .ok_or_else(|| M::Error::missing_field("trigger_condition"))?;
                let trigger_type =
                    trigger_type.ok_or_else(|| M::Error::missing_field("trigger_type"))?;

                let content = GovernanceBestPracticeDefinition {
                    category,
                    deep_link,
                    description,
                    detection_type,
                    id,
                    impact,
                    impact_hint,
                    permissions,
                    status,
                    summary,
                    title,
                    trigger_condition,
                    trigger_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceBestPracticeDefinitionVisitor)
    }
}
