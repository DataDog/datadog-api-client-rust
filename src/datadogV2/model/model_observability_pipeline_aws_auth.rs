// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS authentication credentials used for accessing AWS services such as S3.
/// If omitted, the system’s default credentials are used (for example, the IAM role and environment variables).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAwsAuth {
    /// The Amazon Resource Name (ARN) of the role to assume.
    #[serde(rename = "assume_role")]
    pub assume_role: Option<String>,
    /// A unique identifier for cross-account role assumption.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// A session identifier used for logging and tracing the assumed role session.
    #[serde(rename = "session_name")]
    pub session_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAwsAuth {
    pub fn new() -> ObservabilityPipelineAwsAuth {
        ObservabilityPipelineAwsAuth {
            assume_role: None,
            external_id: None,
            session_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assume_role(mut self, value: String) -> Self {
        self.assume_role = Some(value);
        self
    }

    pub fn external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn session_name(mut self, value: String) -> Self {
        self.session_name = Some(value);
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

impl Default for ObservabilityPipelineAwsAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAwsAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAwsAuthVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAwsAuthVisitor {
            type Value = ObservabilityPipelineAwsAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assume_role: Option<String> = None;
                let mut external_id: Option<String> = None;
                let mut session_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assume_role" => {
                            if v.is_null() {
                                continue;
                            }
                            assume_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_name" => {
                            if v.is_null() {
                                continue;
                            }
                            session_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineAwsAuth {
                    assume_role,
                    external_id,
                    session_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAwsAuthVisitor)
    }
}
