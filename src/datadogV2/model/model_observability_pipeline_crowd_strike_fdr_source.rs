// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `crowdstrike_fdr` source ingests CrowdStrike Falcon Data Replicator (FDR) data.
/// It reads file notifications from the CrowdStrike-managed SQS queue for your FDR feed and downloads the referenced objects from the CrowdStrike-managed S3 bucket.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineCrowdStrikeFdrSource {
    /// Name of the environment variable or secret that holds the AWS access key ID that CrowdStrike generated for the FDR feed.
    #[serde(rename = "access_key_id_key")]
    pub access_key_id_key: Option<String>,
    /// Number of concurrent tasks polling the SQS queue. Defaults to the number of Worker threads.
    #[serde(rename = "client_concurrency")]
    pub client_concurrency: Option<i64>,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// Maximum number of SQS messages to receive per poll. Each message can reference multiple files. Defaults to 1.
    #[serde(rename = "max_number_of_messages")]
    pub max_number_of_messages: Option<i64>,
    /// AWS region of the FDR feed's SQS queue and S3 bucket.
    #[serde(rename = "region")]
    pub region: String,
    /// Name of the environment variable or secret that holds the AWS secret access key that CrowdStrike generated for the FDR feed.
    #[serde(rename = "secret_access_key_key")]
    pub secret_access_key_key: Option<String>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The source type. Always `crowdstrike_fdr`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineCrowdStrikeFdrSourceType,
    /// Name of the environment variable or secret that holds the URL of the FDR feed's SQS queue.
    #[serde(rename = "url_key")]
    pub url_key: Option<String>,
    /// Number of seconds a received SQS message is hidden from other consumers. Set it higher than the time needed to download and process the largest FDR file. Defaults to 300.
    #[serde(rename = "visibility_timeout_secs")]
    pub visibility_timeout_secs: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineCrowdStrikeFdrSource {
    pub fn new(
        id: String,
        region: String,
        type_: crate::datadogV2::model::ObservabilityPipelineCrowdStrikeFdrSourceType,
    ) -> ObservabilityPipelineCrowdStrikeFdrSource {
        ObservabilityPipelineCrowdStrikeFdrSource {
            access_key_id_key: None,
            client_concurrency: None,
            id,
            max_number_of_messages: None,
            region,
            secret_access_key_key: None,
            tls: None,
            type_,
            url_key: None,
            visibility_timeout_secs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn access_key_id_key(mut self, value: String) -> Self {
        self.access_key_id_key = Some(value);
        self
    }

    pub fn client_concurrency(mut self, value: i64) -> Self {
        self.client_concurrency = Some(value);
        self
    }

    pub fn max_number_of_messages(mut self, value: i64) -> Self {
        self.max_number_of_messages = Some(value);
        self
    }

    pub fn secret_access_key_key(mut self, value: String) -> Self {
        self.secret_access_key_key = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
        self
    }

    pub fn url_key(mut self, value: String) -> Self {
        self.url_key = Some(value);
        self
    }

    pub fn visibility_timeout_secs(mut self, value: i64) -> Self {
        self.visibility_timeout_secs = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineCrowdStrikeFdrSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineCrowdStrikeFdrSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineCrowdStrikeFdrSourceVisitor {
            type Value = ObservabilityPipelineCrowdStrikeFdrSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key_id_key: Option<String> = None;
                let mut client_concurrency: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut max_number_of_messages: Option<i64> = None;
                let mut region: Option<String> = None;
                let mut secret_access_key_key: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineCrowdStrikeFdrSourceType,
                > = None;
                let mut url_key: Option<String> = None;
                let mut visibility_timeout_secs: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_key_id_key" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_id_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_concurrency" => {
                            if v.is_null() {
                                continue;
                            }
                            client_concurrency =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_number_of_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            max_number_of_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_access_key_key" => {
                            if v.is_null() {
                                continue;
                            }
                            secret_access_key_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelineCrowdStrikeFdrSourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url_key" => {
                            if v.is_null() {
                                continue;
                            }
                            url_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visibility_timeout_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            visibility_timeout_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineCrowdStrikeFdrSource {
                    access_key_id_key,
                    client_concurrency,
                    id,
                    max_number_of_messages,
                    region,
                    secret_access_key_key,
                    tls,
                    type_,
                    url_key,
                    visibility_timeout_secs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineCrowdStrikeFdrSourceVisitor)
    }
}
