// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cloud storage access configuration for the reference table data file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
    /// Amazon Web Services S3 storage access configuration.
    #[serde(rename = "aws_detail")]
    pub aws_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail>,
    /// Azure Blob Storage access configuration.
    #[serde(rename = "azure_detail")]
    pub azure_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail>,
    /// Google Cloud Platform storage access configuration.
    #[serde(rename = "gcp_detail")]
    pub gcp_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
    pub fn new() -> CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
        CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
            aws_detail: None,
            azure_detail: None,
            gcp_detail: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aws_detail(
        mut self,
        value: crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail,
    ) -> Self {
        self.aws_detail = Some(value);
        self
    }

    pub fn azure_detail(
        mut self,
        value: crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail,
    ) -> Self {
        self.azure_detail = Some(value);
        self
    }

    pub fn gcp_detail(
        mut self,
        value: crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail,
    ) -> Self {
        self.gcp_detail = Some(value);
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

impl Default for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsVisitor;
        impl<'a> Visitor<'a> for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsVisitor {
            type Value = CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail> = None;
                let mut azure_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail> = None;
                let mut gcp_detail: Option<crate::datadogV2::model::CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsGcpDetail> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws_detail" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_detail" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_detail =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_detail" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateTableRequestDataAttributesFileMetadataOneOfAccessDetails {
                    aws_detail,
                    azure_detail,
                    gcp_detail,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsVisitor)
    }
}
