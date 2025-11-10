// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Google Cloud Platform storage access configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    /// The relative file path from the GCS bucket root to the CSV file.
    #[serde(rename = "file_path")]
    pub file_path: String,
    /// GCP bucket containing the CSV file.
    #[serde(rename = "gcp_bucket_name")]
    pub gcp_bucket_name: String,
    /// GCP project ID where the bucket is located.
    #[serde(rename = "gcp_project_id")]
    pub gcp_project_id: String,
    /// Service account email with read permissions for the GCS bucket.
    #[serde(rename = "gcp_service_account_email")]
    pub gcp_service_account_email: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
    pub fn new(
        file_path: String,
        gcp_bucket_name: String,
        gcp_project_id: String,
        gcp_service_account_email: String,
    ) -> CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
        CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
            file_path,
            gcp_bucket_name,
            gcp_project_id,
            gcp_service_account_email,
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

impl<'de> Deserialize<'de>
    for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor;
        impl<'a> Visitor<'a>
            for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor
        {
            type Value = CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail;

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
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_bucket_name" => {
                            gcp_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_project_id" => {
                            gcp_project_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_service_account_email" => {
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
                let file_path = file_path.ok_or_else(|| M::Error::missing_field("file_path"))?;
                let gcp_bucket_name =
                    gcp_bucket_name.ok_or_else(|| M::Error::missing_field("gcp_bucket_name"))?;
                let gcp_project_id =
                    gcp_project_id.ok_or_else(|| M::Error::missing_field("gcp_project_id"))?;
                let gcp_service_account_email = gcp_service_account_email
                    .ok_or_else(|| M::Error::missing_field("gcp_service_account_email"))?;

                let content =
                    CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail {
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
            CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetailVisitor,
        )
    }
}
