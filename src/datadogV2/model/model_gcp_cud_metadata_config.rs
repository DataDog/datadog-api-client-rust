// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The BigQuery table the committed use discount (CUD) metadata is read from. Only returned when CUD metadata is configured for the account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPCudMetadataConfig {
    /// The BigQuery dataset ID of the CUD metadata export.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// The Google Cloud project ID of the CUD metadata export.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The BigQuery table ID of the CUD metadata export.
    #[serde(rename = "table_id")]
    pub table_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPCudMetadataConfig {
    pub fn new(dataset_id: String, project_id: String, table_id: String) -> GCPCudMetadataConfig {
        GCPCudMetadataConfig {
            dataset_id,
            project_id,
            table_id,
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

impl<'de> Deserialize<'de> for GCPCudMetadataConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPCudMetadataConfigVisitor;
        impl<'a> Visitor<'a> for GCPCudMetadataConfigVisitor {
            type Value = GCPCudMetadataConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_id: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut table_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_id" => {
                            table_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let table_id = table_id.ok_or_else(|| M::Error::missing_field("table_id"))?;

                let content = GCPCudMetadataConfig {
                    dataset_id,
                    project_id,
                    table_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPCudMetadataConfigVisitor)
    }
}
