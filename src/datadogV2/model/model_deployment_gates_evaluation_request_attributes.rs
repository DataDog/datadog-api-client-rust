// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a deployment gate evaluation request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGatesEvaluationRequestAttributes {
    /// The environment of the deployment.
    #[serde(rename = "env")]
    pub env: String,
    /// The identifier of the deployment gate. Defaults to "default".
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,
    /// A primary tag to scope APM Faulty Deployment Detection rules.
    #[serde(rename = "primary_tag")]
    pub primary_tag: Option<String>,
    /// The service being deployed.
    #[serde(rename = "service")]
    pub service: String,
    /// The version of the deployment. Required for APM Faulty Deployment Detection rules.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGatesEvaluationRequestAttributes {
    pub fn new(env: String, service: String) -> DeploymentGatesEvaluationRequestAttributes {
        DeploymentGatesEvaluationRequestAttributes {
            env,
            identifier: None,
            primary_tag: None,
            service,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn identifier(mut self, value: String) -> Self {
        self.identifier = Some(value);
        self
    }

    pub fn primary_tag(mut self, value: String) -> Self {
        self.primary_tag = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for DeploymentGatesEvaluationRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGatesEvaluationRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentGatesEvaluationRequestAttributesVisitor {
            type Value = DeploymentGatesEvaluationRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut env: Option<String> = None;
                let mut identifier: Option<String> = None;
                let mut primary_tag: Option<String> = None;
                let mut service: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            if v.is_null() {
                                continue;
                            }
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_tag" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_tag =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;

                let content = DeploymentGatesEvaluationRequestAttributes {
                    env,
                    identifier,
                    primary_tag,
                    service,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGatesEvaluationRequestAttributesVisitor)
    }
}
