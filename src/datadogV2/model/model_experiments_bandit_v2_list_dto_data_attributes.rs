// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of a bandit. Use Get bandit for its setup, active policy, and latest refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditV2ListDTODataAttributes {
    /// When the bandit was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The bandit's description.
    #[serde(rename = "description")]
    pub description: String,
    /// When the bandit was stopped or reached its scheduled end. Null while it hasn't ended.
    #[serialize_always]
    #[serde(rename = "ended_at")]
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The metric the bandit optimizes. Null until set on a draft.
    #[serialize_always]
    #[serde(rename = "metric_id")]
    pub metric_id: Option<uuid::Uuid>,
    /// The bandit's name.
    #[serde(rename = "name")]
    pub name: String,
    /// The key of the arm most likely to be best, when one arm leads and the bandit is outside warmup. Null otherwise.
    #[serialize_always]
    #[serde(rename = "optimal_variant_key")]
    pub optimal_variant_key: Option<String>,
    /// Why no optimal variant is reported. Absent when one is, or when the cause isn't known.
    #[serde(rename = "optimal_variant_unavailable_reason")]
    pub optimal_variant_unavailable_reason:
        Option<crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason>,
    /// When the bandit started. Null until it starts.
    #[serialize_always]
    #[serde(rename = "started_at")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The bandit's status. STOPPED means the bandit was stopped and serves the variant chosen then. COMPLETED means it reached its scheduled end and serves its fallback.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsBanditStatus,
    /// The subject type the bandit randomizes and measures by. Null until set on a draft.
    #[serialize_always]
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<uuid::Uuid>,
    /// The bandit's tags, without team tags.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Handles of the teams that own the bandit.
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
    /// When the bandit was last changed.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditV2ListDTODataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        ended_at: Option<chrono::DateTime<chrono::Utc>>,
        metric_id: Option<uuid::Uuid>,
        name: String,
        optimal_variant_key: Option<String>,
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        status: crate::datadogV2::model::ExperimentsBanditStatus,
        subject_type_id: Option<uuid::Uuid>,
        tags: Vec<String>,
        teams: Vec<String>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> ExperimentsBanditV2ListDTODataAttributes {
        ExperimentsBanditV2ListDTODataAttributes {
            created_at,
            description,
            ended_at,
            metric_id,
            name,
            optimal_variant_key,
            optimal_variant_unavailable_reason: None,
            started_at,
            status,
            subject_type_id,
            tags,
            teams,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn optimal_variant_unavailable_reason(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
    ) -> Self {
        self.optimal_variant_unavailable_reason = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsBanditV2ListDTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditV2ListDTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditV2ListDTODataAttributesVisitor {
            type Value = ExperimentsBanditV2ListDTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut ended_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut metric_id: Option<Option<uuid::Uuid>> = None;
                let mut name: Option<String> = None;
                let mut optimal_variant_key: Option<Option<String>> = None;
                let mut optimal_variant_unavailable_reason: Option<
                    crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
                > = None;
                let mut started_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut status: Option<crate::datadogV2::model::ExperimentsBanditStatus> = None;
                let mut subject_type_id: Option<Option<uuid::Uuid>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ended_at" => {
                            ended_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "optimal_variant_key" => {
                            optimal_variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "optimal_variant_unavailable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            optimal_variant_unavailable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _optimal_variant_unavailable_reason) =
                                optimal_variant_unavailable_reason
                            {
                                match _optimal_variant_unavailable_reason {
                                    crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason::UnparsedObject(_optimal_variant_unavailable_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsBanditStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let ended_at = ended_at.ok_or_else(|| M::Error::missing_field("ended_at"))?;
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let optimal_variant_key = optimal_variant_key
                    .ok_or_else(|| M::Error::missing_field("optimal_variant_key"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let subject_type_id =
                    subject_type_id.ok_or_else(|| M::Error::missing_field("subject_type_id"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let teams = teams.ok_or_else(|| M::Error::missing_field("teams"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = ExperimentsBanditV2ListDTODataAttributes {
                    created_at,
                    description,
                    ended_at,
                    metric_id,
                    name,
                    optimal_variant_key,
                    optimal_variant_unavailable_reason,
                    started_at,
                    status,
                    subject_type_id,
                    tags,
                    teams,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditV2ListDTODataAttributesVisitor)
    }
}
