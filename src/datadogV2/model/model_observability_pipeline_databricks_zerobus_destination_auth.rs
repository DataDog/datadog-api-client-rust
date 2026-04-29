// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OAuth credentials for authenticating with the Databricks Zerobus ingestion API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineDatabricksZerobusDestinationAuth {
    /// Your service principal application ID (UUID).
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Name of the environment variable or secret that holds the OAuth client secret used to authenticate with the Databricks ingestion endpoint.
    #[serde(rename = "client_secret_key")]
    pub client_secret_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineDatabricksZerobusDestinationAuth {
    pub fn new(client_id: String) -> ObservabilityPipelineDatabricksZerobusDestinationAuth {
        ObservabilityPipelineDatabricksZerobusDestinationAuth {
            client_id,
            client_secret_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn client_secret_key(mut self, value: String) -> Self {
        self.client_secret_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineDatabricksZerobusDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineDatabricksZerobusDestinationAuthVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineDatabricksZerobusDestinationAuthVisitor {
            type Value = ObservabilityPipelineDatabricksZerobusDestinationAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<String> = None;
                let mut client_secret_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret_key" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;

                let content = ObservabilityPipelineDatabricksZerobusDestinationAuth {
                    client_id,
                    client_secret_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineDatabricksZerobusDestinationAuthVisitor)
    }
}
