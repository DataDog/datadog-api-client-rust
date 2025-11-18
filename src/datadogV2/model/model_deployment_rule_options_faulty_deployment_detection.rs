// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Faulty deployment detection options for deployment rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsFaultyDeploymentDetection {
    /// The duration for faulty deployment detection.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Resources to exclude from faulty deployment detection.
    #[serde(rename = "excluded_resources")]
    pub excluded_resources: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsFaultyDeploymentDetection {
    pub fn new() -> DeploymentRuleOptionsFaultyDeploymentDetection {
        DeploymentRuleOptionsFaultyDeploymentDetection {
            duration: None,
            excluded_resources: None,
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn excluded_resources(mut self, value: Vec<String>) -> Self {
        self.excluded_resources = Some(value);
        self
    }
}

impl Default for DeploymentRuleOptionsFaultyDeploymentDetection {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeploymentRuleOptionsFaultyDeploymentDetection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsFaultyDeploymentDetectionVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsFaultyDeploymentDetectionVisitor {
            type Value = DeploymentRuleOptionsFaultyDeploymentDetection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut excluded_resources: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = DeploymentRuleOptionsFaultyDeploymentDetection {
                    duration,
                    excluded_resources,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsFaultyDeploymentDetectionVisitor)
    }
}
