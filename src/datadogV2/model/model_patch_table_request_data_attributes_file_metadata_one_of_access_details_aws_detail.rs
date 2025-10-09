// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
    /// The ID of the AWS account.
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: Option<String>,
    /// The name of the AWS bucket.
    #[serde(rename = "aws_bucket_name")]
    pub aws_bucket_name: Option<String>,
    /// The relative file path from the S3 bucket root to the CSV file.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
    pub fn new() -> PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
        PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
            aws_account_id: None,
            aws_bucket_name: None,
            file_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aws_account_id(mut self, value: String) -> Self {
        self.aws_account_id = Some(value);
        self
    }

    pub fn aws_bucket_name(mut self, value: String) -> Self {
        self.aws_bucket_name = Some(value);
        self
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
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

impl Default for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetailVisitor;
        impl<'a> Visitor<'a>
            for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetailVisitor
        {
            type Value = PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_account_id: Option<String> = None;
                let mut aws_bucket_name: Option<String> = None;
                let mut file_path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws_account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_bucket_name" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetail {
                        aws_account_id,
                        aws_bucket_name,
                        file_path,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAwsDetailVisitor,
        )
    }
}
