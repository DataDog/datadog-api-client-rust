// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    /// The relative file path from the GCS bucket root to the CSV file.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    /// The name of the GCP bucket.
    #[serde(rename = "gcp_bucket_name")]
    pub gcp_bucket_name: Option<String>,
    /// The ID of the GCP project.
    #[serde(rename = "gcp_project_id")]
    pub gcp_project_id: Option<String>,
    /// The email of the GCP service account.
    #[serde(rename = "gcp_service_account_email")]
    pub gcp_service_account_email: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    pub fn new() -> TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
        TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
            file_path: None,
            gcp_bucket_name: None,
            gcp_project_id: None,
            gcp_service_account_email: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    pub fn gcp_bucket_name(mut self, value: String) -> Self {
        self.gcp_bucket_name = Some(value);
        self
    }

    pub fn gcp_project_id(mut self, value: String) -> Self {
        self.gcp_project_id = Some(value);
        self
    }

    pub fn gcp_service_account_email(mut self, value: String) -> Self {
        self.gcp_service_account_email = Some(value);
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

impl Default for TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor;
        impl<'a> Visitor<'a> for TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor {
            type Value = TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file_path: Option<String> = None;
                let mut gcp_bucket_name: Option<String> = None;
                let mut gcp_project_id: Option<String> = None;
                let mut gcp_service_account_email: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_bucket_name" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_project_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_service_account_email" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_service_account_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
                    file_path,
                    gcp_bucket_name,
                    gcp_project_id,
                    gcp_service_account_email,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            TableResultV2DataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor,
        )
    }
}
