// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Secrets Manager settings shared by the credential references on a `DATACENTER` connection. These settings are required whenever a field uses an AWS Secrets Manager reference.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionConnectionAWSSecretsManager {
    /// ID of the AWS connection used to retrieve secrets. This connection must be in the same Datadog organization.
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    /// AWS region containing the referenced secrets.
    #[serde(rename = "region")]
    pub region: String,
    /// AWS Secrets Manager secret backend.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionConnectionAWSSecretsManager {
    pub fn new(
        connection_id: String,
        region: String,
        type_: crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
    ) -> ActionConnectionAWSSecretsManager {
        ActionConnectionAWSSecretsManager {
            connection_id,
            region,
            type_,
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

impl<'de> Deserialize<'de> for ActionConnectionAWSSecretsManager {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionConnectionAWSSecretsManagerVisitor;
        impl<'a> Visitor<'a> for ActionConnectionAWSSecretsManagerVisitor {
            type Value = ActionConnectionAWSSecretsManager;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connection_id: Option<String> = None;
                let mut region: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ActionConnectionAWSSecretsManagerType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connection_id" => {
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let connection_id =
                    connection_id.ok_or_else(|| M::Error::missing_field("connection_id"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ActionConnectionAWSSecretsManager {
                    connection_id,
                    region,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionConnectionAWSSecretsManagerVisitor)
    }
}
