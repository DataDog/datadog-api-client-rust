// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the notebook.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookSearchMetadata {
    /// User information.
    #[serde(rename = "author")]
    pub author: crate::datadogV2::model::NotebookSearchUser,
    /// Number of cells in the notebook.
    #[serde(rename = "cell_count")]
    pub cell_count: i64,
    /// Time at which the notebook was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Time at which the notebook was deleted, or null if not deleted.
    #[serialize_always]
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Experience type of the notebook.
    #[serialize_always]
    #[serde(rename = "experience_type")]
    pub experience_type: Option<String>,
    /// Whether the notebook has computational cells.
    #[serde(rename = "has_computational_cells")]
    pub has_computational_cells: bool,
    /// Whether the notebook is favorited by the user.
    #[serde(rename = "is_favorited")]
    pub is_favorited: bool,
    /// Whether the notebook is a template.
    #[serde(rename = "is_template")]
    pub is_template: bool,
    /// Time at which the notebook was last updated.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Status of the notebook.
    #[serde(rename = "status")]
    pub status: String,
    /// Whether the notebook can take a snapshot.
    #[serde(rename = "take_snapshots")]
    pub take_snapshots: bool,
    /// Notebook type.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookSearchMetadata {
    pub fn new(
        author: crate::datadogV2::model::NotebookSearchUser,
        cell_count: i64,
        created_at: chrono::DateTime<chrono::Utc>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
        experience_type: Option<String>,
        has_computational_cells: bool,
        is_favorited: bool,
        is_template: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        status: String,
        take_snapshots: bool,
        type_: String,
    ) -> NotebookSearchMetadata {
        NotebookSearchMetadata {
            author,
            cell_count,
            created_at,
            deleted_at,
            experience_type,
            has_computational_cells,
            is_favorited,
            is_template,
            modified_at,
            status,
            take_snapshots,
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

impl<'de> Deserialize<'de> for NotebookSearchMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookSearchMetadataVisitor;
        impl<'a> Visitor<'a> for NotebookSearchMetadataVisitor {
            type Value = NotebookSearchMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::NotebookSearchUser> = None;
                let mut cell_count: Option<i64> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut experience_type: Option<Option<String>> = None;
                let mut has_computational_cells: Option<bool> = None;
                let mut is_favorited: Option<bool> = None;
                let mut is_template: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<String> = None;
                let mut take_snapshots: Option<bool> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_count" => {
                            cell_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experience_type" => {
                            experience_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_computational_cells" => {
                            has_computational_cells =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorited" => {
                            is_favorited =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_template" => {
                            is_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "take_snapshots" => {
                            take_snapshots =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let author = author.ok_or_else(|| M::Error::missing_field("author"))?;
                let cell_count = cell_count.ok_or_else(|| M::Error::missing_field("cell_count"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let deleted_at = deleted_at.ok_or_else(|| M::Error::missing_field("deleted_at"))?;
                let experience_type =
                    experience_type.ok_or_else(|| M::Error::missing_field("experience_type"))?;
                let has_computational_cells = has_computational_cells
                    .ok_or_else(|| M::Error::missing_field("has_computational_cells"))?;
                let is_favorited =
                    is_favorited.ok_or_else(|| M::Error::missing_field("is_favorited"))?;
                let is_template =
                    is_template.ok_or_else(|| M::Error::missing_field("is_template"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let take_snapshots =
                    take_snapshots.ok_or_else(|| M::Error::missing_field("take_snapshots"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NotebookSearchMetadata {
                    author,
                    cell_count,
                    created_at,
                    deleted_at,
                    experience_type,
                    has_computational_cells,
                    is_favorited,
                    is_template,
                    modified_at,
                    status,
                    take_snapshots,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookSearchMetadataVisitor)
    }
}
