// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Model Lab run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModelLabRunAttributes {
    /// The date and time the run completed.
    #[serde(
        rename = "completed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The date and time the run was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The date and time the run was soft-deleted.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Whether a descendant run matched the applied filters.
    #[serde(rename = "descendant_match")]
    pub descendant_match: bool,
    /// A description of the run.
    #[serde(rename = "description")]
    pub description: String,
    /// The duration of the run in seconds.
    #[serde(
        rename = "duration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub duration: Option<Option<f64>>,
    /// An optional external URL associated with the run.
    #[serde(
        rename = "external_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub external_url: Option<Option<String>>,
    /// Whether the run has child runs.
    #[serde(rename = "has_children")]
    pub has_children: bool,
    /// Whether the run is pinned by the current user.
    #[serde(rename = "is_pinned")]
    pub is_pinned: bool,
    /// Summary statistics for metrics recorded during the run.
    #[serde(rename = "metric_summaries")]
    pub metric_summaries: Vec<crate::datadogV2::model::ModelLabMetricSummary>,
    /// The MLflow artifact storage location for this run.
    #[serde(rename = "mlflow_artifact_location")]
    pub mlflow_artifact_location: String,
    /// The name of the run.
    #[serde(rename = "name")]
    pub name: String,
    /// The UUID of the run owner.
    #[serde(
        rename = "owner_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub owner_id: Option<Option<String>>,
    /// The list of parameters used for the run.
    #[serialize_always]
    #[serde(rename = "params")]
    pub params: Option<Vec<crate::datadogV2::model::ModelLabRunParam>>,
    /// The ID of the project this run belongs to.
    #[serde(rename = "project_id")]
    pub project_id: i64,
    /// The date and time the run started.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The status of a Model Lab run.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ModelLabRunStatus,
    /// The list of tags associated with the run.
    #[serde(rename = "tags")]
    pub tags: Vec<crate::datadogV2::model::ModelLabTag>,
    /// The date and time the run was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ModelLabRunAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        descendant_match: bool,
        description: String,
        has_children: bool,
        is_pinned: bool,
        metric_summaries: Vec<crate::datadogV2::model::ModelLabMetricSummary>,
        mlflow_artifact_location: String,
        name: String,
        params: Option<Vec<crate::datadogV2::model::ModelLabRunParam>>,
        project_id: i64,
        started_at: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::ModelLabRunStatus,
        tags: Vec<crate::datadogV2::model::ModelLabTag>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> ModelLabRunAttributes {
        ModelLabRunAttributes {
            completed_at: None,
            created_at,
            deleted_at: None,
            descendant_match,
            description,
            duration: None,
            external_url: None,
            has_children,
            is_pinned,
            metric_summaries,
            mlflow_artifact_location,
            name,
            owner_id: None,
            params,
            project_id,
            started_at,
            status,
            tags,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn completed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn duration(mut self, value: Option<f64>) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn external_url(mut self, value: Option<String>) -> Self {
        self.external_url = Some(value);
        self
    }

    pub fn owner_id(mut self, value: Option<String>) -> Self {
        self.owner_id = Some(value);
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

impl<'de> Deserialize<'de> for ModelLabRunAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ModelLabRunAttributesVisitor;
        impl<'a> Visitor<'a> for ModelLabRunAttributesVisitor {
            type Value = ModelLabRunAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut descendant_match: Option<bool> = None;
                let mut description: Option<String> = None;
                let mut duration: Option<Option<f64>> = None;
                let mut external_url: Option<Option<String>> = None;
                let mut has_children: Option<bool> = None;
                let mut is_pinned: Option<bool> = None;
                let mut metric_summaries: Option<
                    Vec<crate::datadogV2::model::ModelLabMetricSummary>,
                > = None;
                let mut mlflow_artifact_location: Option<String> = None;
                let mut name: Option<String> = None;
                let mut owner_id: Option<Option<String>> = None;
                let mut params: Option<Option<Vec<crate::datadogV2::model::ModelLabRunParam>>> =
                    None;
                let mut project_id: Option<i64> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::ModelLabRunStatus> = None;
                let mut tags: Option<Vec<crate::datadogV2::model::ModelLabTag>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completed_at" => {
                            completed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "descendant_match" => {
                            descendant_match =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_url" => {
                            external_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_children" => {
                            has_children =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_pinned" => {
                            is_pinned = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_summaries" => {
                            metric_summaries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mlflow_artifact_location" => {
                            mlflow_artifact_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "owner_id" => {
                            owner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "params" => {
                            params = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ModelLabRunStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let descendant_match =
                    descendant_match.ok_or_else(|| M::Error::missing_field("descendant_match"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let has_children =
                    has_children.ok_or_else(|| M::Error::missing_field("has_children"))?;
                let is_pinned = is_pinned.ok_or_else(|| M::Error::missing_field("is_pinned"))?;
                let metric_summaries =
                    metric_summaries.ok_or_else(|| M::Error::missing_field("metric_summaries"))?;
                let mlflow_artifact_location = mlflow_artifact_location
                    .ok_or_else(|| M::Error::missing_field("mlflow_artifact_location"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let params = params.ok_or_else(|| M::Error::missing_field("params"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = ModelLabRunAttributes {
                    completed_at,
                    created_at,
                    deleted_at,
                    descendant_match,
                    description,
                    duration,
                    external_url,
                    has_children,
                    is_pinned,
                    metric_summaries,
                    mlflow_artifact_location,
                    name,
                    owner_id,
                    params,
                    project_id,
                    started_at,
                    status,
                    tags,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ModelLabRunAttributesVisitor)
    }
}
