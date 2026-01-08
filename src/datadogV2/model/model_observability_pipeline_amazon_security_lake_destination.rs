// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `amazon_security_lake` destination sends your logs to Amazon Security Lake.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonSecurityLakeDestination {
    /// AWS authentication credentials used for accessing AWS services such as S3.
    /// If omitted, the system’s default credentials are used (for example, the IAM role and environment variables).
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineAwsAuth>,
    /// Name of the Amazon S3 bucket in Security Lake (3-63 characters).
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// Custom source name for the logs in Security Lake.
    #[serde(rename = "custom_source_name")]
    pub custom_source_name: String,
    /// Unique identifier for the destination component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// AWS region of the S3 bucket.
    #[serde(rename = "region")]
    pub region: String,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The destination type. Always `amazon_security_lake`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonSecurityLakeDestination {
    pub fn new(
        bucket: String,
        custom_source_name: String,
        id: String,
        inputs: Vec<String>,
        region: String,
        type_: crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestinationType,
    ) -> ObservabilityPipelineAmazonSecurityLakeDestination {
        ObservabilityPipelineAmazonSecurityLakeDestination {
            auth: None,
            bucket,
            custom_source_name,
            id,
            inputs,
            region,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(mut self, value: crate::datadogV2::model::ObservabilityPipelineAwsAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonSecurityLakeDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonSecurityLakeDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonSecurityLakeDestinationVisitor {
            type Value = ObservabilityPipelineAmazonSecurityLakeDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineAwsAuth> = None;
                let mut bucket: Option<String> = None;
                let mut custom_source_name: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut region: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket" => {
                            bucket = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_source_name" => {
                            custom_source_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestinationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bucket = bucket.ok_or_else(|| M::Error::missing_field("bucket"))?;
                let custom_source_name = custom_source_name
                    .ok_or_else(|| M::Error::missing_field("custom_source_name"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineAmazonSecurityLakeDestination {
                    auth,
                    bucket,
                    custom_source_name,
                    id,
                    inputs,
                    region,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonSecurityLakeDestinationVisitor)
    }
}
