// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Faulty deployment detection options returned in deployment rule responses. The `excluded_resources` field is always present, which allows disambiguating this type from monitor options when both share a `duration` field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
    /// The duration for faulty deployment detection.
    #[serde(rename = "duration")]
    pub duration: Option<i64>,
    /// Resources to exclude from faulty deployment detection.
    #[serde(rename = "excluded_resources")]
    pub excluded_resources: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
    pub fn new(
        excluded_resources: Vec<String>,
    ) -> DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
        DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
            duration: None,
            excluded_resources,
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentRuleOptionsFaultyDeploymentDetectionResponseVisitor;
        impl<'a> Visitor<'a> for DeploymentRuleOptionsFaultyDeploymentDetectionResponseVisitor {
            type Value = DeploymentRuleOptionsFaultyDeploymentDetectionResponse;

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
                let excluded_resources = excluded_resources
                    .ok_or_else(|| M::Error::missing_field("excluded_resources"))?;

                let content = DeploymentRuleOptionsFaultyDeploymentDetectionResponse {
                    duration,
                    excluded_resources,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentRuleOptionsFaultyDeploymentDetectionResponseVisitor)
    }
}
