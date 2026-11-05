// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ActionConnectionAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionConnectionAttributes {
    /// AWS Secrets Manager settings shared by the credential references on a `DATACENTER` connection. These settings are required whenever a field uses an AWS Secrets Manager reference.
    #[serde(
        rename = "external_secrets_manager",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub external_secrets_manager:
        Option<Option<crate::datadogV2::model::ActionConnectionAWSSecretsManager>>,
    /// The definition of `ActionConnectionIntegration` object.
    #[serde(rename = "integration")]
    pub integration: crate::datadogV2::model::ActionConnectionIntegration,
    /// Name of the connection
    #[serde(rename = "name")]
    pub name: String,
    /// Tags associated with the connection. Each tag must follow the `key:value` format.
    /// The `default` tag key is reserved.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionConnectionAttributes {
    pub fn new(
        integration: crate::datadogV2::model::ActionConnectionIntegration,
        name: String,
    ) -> ActionConnectionAttributes {
        ActionConnectionAttributes {
            external_secrets_manager: None,
            integration,
            name,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn external_secrets_manager(
        mut self,
        value: Option<crate::datadogV2::model::ActionConnectionAWSSecretsManager>,
    ) -> Self {
        self.external_secrets_manager = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ActionConnectionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionConnectionAttributesVisitor;
        impl<'a> Visitor<'a> for ActionConnectionAttributesVisitor {
            type Value = ActionConnectionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut external_secrets_manager: Option<
                    Option<crate::datadogV2::model::ActionConnectionAWSSecretsManager>,
                > = None;
                let mut integration: Option<crate::datadogV2::model::ActionConnectionIntegration> =
                    None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "external_secrets_manager" => {
                            external_secrets_manager =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration) = integration {
                                match _integration {
                                    crate::datadogV2::model::ActionConnectionIntegration::UnparsedObject(_integration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ActionConnectionAttributes {
                    external_secrets_manager,
                    integration,
                    name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionConnectionAttributesVisitor)
    }
}
