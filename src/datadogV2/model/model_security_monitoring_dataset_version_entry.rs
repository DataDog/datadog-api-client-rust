// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single entry in the version history of a dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetVersionEntry {
    /// The list of field changes between this version of the dataset and the previous one.
    #[serde(rename = "changes")]
    pub changes: Vec<crate::datadogV2::model::SecurityMonitoringDatasetVersionFieldChange>,
    /// The attributes of a Cloud SIEM dataset.
    #[serde(rename = "dataset")]
    pub dataset: crate::datadogV2::model::SecurityMonitoringDatasetAttributesResponse,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetVersionEntry {
    pub fn new(
        changes: Vec<crate::datadogV2::model::SecurityMonitoringDatasetVersionFieldChange>,
        dataset: crate::datadogV2::model::SecurityMonitoringDatasetAttributesResponse,
    ) -> SecurityMonitoringDatasetVersionEntry {
        SecurityMonitoringDatasetVersionEntry {
            changes,
            dataset,
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetVersionEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetVersionEntryVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetVersionEntryVisitor {
            type Value = SecurityMonitoringDatasetVersionEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut changes: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringDatasetVersionFieldChange>,
                > = None;
                let mut dataset: Option<
                    crate::datadogV2::model::SecurityMonitoringDatasetAttributesResponse,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "changes" => {
                            changes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset" => {
                            dataset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let changes = changes.ok_or_else(|| M::Error::missing_field("changes"))?;
                let dataset = dataset.ok_or_else(|| M::Error::missing_field("dataset"))?;

                let content = SecurityMonitoringDatasetVersionEntry {
                    changes,
                    dataset,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetVersionEntryVisitor)
    }
}
