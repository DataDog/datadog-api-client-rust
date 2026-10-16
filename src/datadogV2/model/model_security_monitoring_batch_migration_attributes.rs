// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a batch migration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationAttributes {
    /// The creation date of the batch migration, in RFC 3339 format.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user or service account who created the batch migration.
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    /// The description of the batch migration.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The origin of a batch migration. Always `batch_migration` for batch migrations
    /// created through this API.
    #[serde(rename = "inputSource")]
    pub input_source: crate::datadogV2::model::SecurityMonitoringBatchMigrationInputSource,
    /// The last update date of the batch migration, in RFC 3339 format.
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user or service account who last updated the batch migration,
    /// or the name of a system actor.
    #[serde(rename = "lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    /// The name of the batch migration.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the organization that owns the batch migration.
    #[serde(rename = "orgId")]
    pub org_id: i64,
    /// The number of resources of a batch migration in each status.
    #[serde(rename = "resourceStatusCounts")]
    pub resource_status_counts:
        crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatusCounts,
    /// The status of a batch migration. A batch migration is created as `draft` and moves to
    /// `in_progress` when started. It moves to `completed` once no resource is `pending` or
    /// `in_progress`, or to `canceled` when canceled.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringBatchMigrationStatus,
    /// Tags of the batch migration. Up to 50 tags, each up to 256 characters.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringBatchMigrationAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        input_source: crate::datadogV2::model::SecurityMonitoringBatchMigrationInputSource,
        last_updated_at: chrono::DateTime<chrono::Utc>,
        name: String,
        org_id: i64,
        resource_status_counts: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatusCounts,
        status: crate::datadogV2::model::SecurityMonitoringBatchMigrationStatus,
        tags: Vec<String>,
    ) -> SecurityMonitoringBatchMigrationAttributes {
        SecurityMonitoringBatchMigrationAttributes {
            created_at,
            created_by: None,
            description: None,
            input_source,
            last_updated_at,
            last_updated_by: None,
            name,
            org_id,
            resource_status_counts,
            status,
            tags,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: String) -> Self {
        self.last_updated_by = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationAttributesVisitor {
            type Value = SecurityMonitoringBatchMigrationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut input_source: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationInputSource,
                > = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_updated_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut resource_status_counts: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatusCounts,
                > = None;
                let mut status: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationStatus,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdBy" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputSource" => {
                            input_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input_source) = input_source {
                                match _input_source {
                                    crate::datadogV2::model::SecurityMonitoringBatchMigrationInputSource::UnparsedObject(_input_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "lastUpdatedAt" => {
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lastUpdatedBy" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "orgId" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceStatusCounts" => {
                            resource_status_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecurityMonitoringBatchMigrationStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let input_source =
                    input_source.ok_or_else(|| M::Error::missing_field("input_source"))?;
                let last_updated_at =
                    last_updated_at.ok_or_else(|| M::Error::missing_field("last_updated_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let resource_status_counts = resource_status_counts
                    .ok_or_else(|| M::Error::missing_field("resource_status_counts"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = SecurityMonitoringBatchMigrationAttributes {
                    created_at,
                    created_by,
                    description,
                    input_source,
                    last_updated_at,
                    last_updated_by,
                    name,
                    org_id,
                    resource_status_counts,
                    status,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringBatchMigrationAttributesVisitor)
    }
}
