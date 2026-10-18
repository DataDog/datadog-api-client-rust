// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The number of resources of a batch migration in each status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationResourceStatusCounts {
    /// Number of resources whose translation was canceled.
    #[serde(rename = "canceled")]
    pub canceled: i64,
    /// Number of resources translated and ready to be published.
    #[serde(rename = "completed")]
    pub completed: i64,
    /// Number of resources identified as duplicates of an existing rule. Reserved for future use.
    #[serde(rename = "duplicate")]
    pub duplicate: i64,
    /// Number of resources whose translation failed.
    #[serde(rename = "failed")]
    pub failed: i64,
    /// Number of resources being translated.
    #[serde(rename = "inProgress")]
    pub in_progress: i64,
    /// Number of resources waiting to be translated.
    #[serde(rename = "pending")]
    pub pending: i64,
    /// Number of resources published as Cloud SIEM detection rules.
    #[serde(rename = "published")]
    pub published: i64,
    /// Number of resources being published.
    #[serde(rename = "publishing")]
    pub publishing: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringBatchMigrationResourceStatusCounts {
    pub fn new(
        canceled: i64,
        completed: i64,
        duplicate: i64,
        failed: i64,
        in_progress: i64,
        pending: i64,
        published: i64,
        publishing: i64,
    ) -> SecurityMonitoringBatchMigrationResourceStatusCounts {
        SecurityMonitoringBatchMigrationResourceStatusCounts {
            canceled,
            completed,
            duplicate,
            failed,
            in_progress,
            pending,
            published,
            publishing,
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

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationResourceStatusCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationResourceStatusCountsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationResourceStatusCountsVisitor {
            type Value = SecurityMonitoringBatchMigrationResourceStatusCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut canceled: Option<i64> = None;
                let mut completed: Option<i64> = None;
                let mut duplicate: Option<i64> = None;
                let mut failed: Option<i64> = None;
                let mut in_progress: Option<i64> = None;
                let mut pending: Option<i64> = None;
                let mut published: Option<i64> = None;
                let mut publishing: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "canceled" => {
                            canceled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "completed" => {
                            completed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duplicate" => {
                            duplicate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed" => {
                            failed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inProgress" => {
                            in_progress =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pending" => {
                            pending = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "published" => {
                            published = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "publishing" => {
                            publishing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let canceled = canceled.ok_or_else(|| M::Error::missing_field("canceled"))?;
                let completed = completed.ok_or_else(|| M::Error::missing_field("completed"))?;
                let duplicate = duplicate.ok_or_else(|| M::Error::missing_field("duplicate"))?;
                let failed = failed.ok_or_else(|| M::Error::missing_field("failed"))?;
                let in_progress =
                    in_progress.ok_or_else(|| M::Error::missing_field("in_progress"))?;
                let pending = pending.ok_or_else(|| M::Error::missing_field("pending"))?;
                let published = published.ok_or_else(|| M::Error::missing_field("published"))?;
                let publishing = publishing.ok_or_else(|| M::Error::missing_field("publishing"))?;

                let content = SecurityMonitoringBatchMigrationResourceStatusCounts {
                    canceled,
                    completed,
                    duplicate,
                    failed,
                    in_progress,
                    pending,
                    published,
                    publishing,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringBatchMigrationResourceStatusCountsVisitor)
    }
}
