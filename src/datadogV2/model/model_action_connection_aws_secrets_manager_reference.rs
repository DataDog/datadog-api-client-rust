// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reference to a string value in AWS Secrets Manager. The connection must also specify `external_secrets_manager` settings. AWS Secrets Manager references are supported for token-based credentials and HTTP authentication tokens on `DATACENTER` connections.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionConnectionAWSSecretsManagerReference {
    /// Top-level JSON key to extract from the secret string. Omit this field to use the entire secret string.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// Name or ARN of the AWS Secrets Manager secret.
    #[serde(rename = "secret_id")]
    pub secret_id: String,
    /// AWS Secrets Manager secret backend.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
    /// ID of the secret version to retrieve. Cannot be used together with `version_stage`. If both are omitted, the current version is used.
    #[serde(rename = "version_id")]
    pub version_id: Option<String>,
    /// Staging label of the secret version to retrieve. Cannot be used together with `version_id`. If both are omitted, the current version is used.
    #[serde(rename = "version_stage")]
    pub version_stage: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionConnectionAWSSecretsManagerReference {
    pub fn new(
        secret_id: String,
        type_: crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
    ) -> ActionConnectionAWSSecretsManagerReference {
        ActionConnectionAWSSecretsManagerReference {
            key: None,
            secret_id,
            type_,
            version_id: None,
            version_stage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn version_id(mut self, value: String) -> Self {
        self.version_id = Some(value);
        self
    }

    pub fn version_stage(mut self, value: String) -> Self {
        self.version_stage = Some(value);
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

impl<'de> Deserialize<'de> for ActionConnectionAWSSecretsManagerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionConnectionAWSSecretsManagerReferenceVisitor;
        impl<'a> Visitor<'a> for ActionConnectionAWSSecretsManagerReferenceVisitor {
            type Value = ActionConnectionAWSSecretsManagerReference;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<String> = None;
                let mut secret_id: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
                > = None;
                let mut version_id: Option<String> = None;
                let mut version_stage: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_id" => {
                            secret_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ActionConnectionAWSSecretsManagerType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version_id" => {
                            if v.is_null() {
                                continue;
                            }
                            version_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_stage" => {
                            if v.is_null() {
                                continue;
                            }
                            version_stage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let secret_id = secret_id.ok_or_else(|| M::Error::missing_field("secret_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ActionConnectionAWSSecretsManagerReference {
                    key,
                    secret_id,
                    type_,
                    version_id,
                    version_stage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionConnectionAWSSecretsManagerReferenceVisitor)
    }
}
