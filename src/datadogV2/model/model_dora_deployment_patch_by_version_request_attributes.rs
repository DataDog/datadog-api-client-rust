// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for patching a DORA deployment event identified by service, environment, and version.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORADeploymentPatchByVersionRequestAttributes {
    /// Indicates whether the deployment resulted in a change failure.
    #[serde(rename = "change_failure")]
    pub change_failure: bool,
    /// The environment the deployment was performed in.
    #[serde(rename = "env")]
    pub env: String,
    /// Remediation details for the deployment. Optional, but required to calculate failed deployment recovery time. Specify either `id` or `version` to identify the remediation deployment, but not both.
    #[serde(rename = "remediation")]
    pub remediation: Option<crate::datadogV2::model::DORADeploymentPatchByVersionRemediation>,
    /// The name of the service that was deployed.
    #[serde(rename = "service")]
    pub service: String,
    /// The version deployed. This can be seen in the Service Catalog or in the APM Deployment Tracking.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORADeploymentPatchByVersionRequestAttributes {
    pub fn new(
        change_failure: bool,
        env: String,
        service: String,
        version: String,
    ) -> DORADeploymentPatchByVersionRequestAttributes {
        DORADeploymentPatchByVersionRequestAttributes {
            change_failure,
            env,
            remediation: None,
            service,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn remediation(
        mut self,
        value: crate::datadogV2::model::DORADeploymentPatchByVersionRemediation,
    ) -> Self {
        self.remediation = Some(value);
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

impl<'de> Deserialize<'de> for DORADeploymentPatchByVersionRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORADeploymentPatchByVersionRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DORADeploymentPatchByVersionRequestAttributesVisitor {
            type Value = DORADeploymentPatchByVersionRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_failure: Option<bool> = None;
                let mut env: Option<String> = None;
                let mut remediation: Option<
                    crate::datadogV2::model::DORADeploymentPatchByVersionRemediation,
                > = None;
                let mut service: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_failure" => {
                            change_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation" => {
                            if v.is_null() {
                                continue;
                            }
                            remediation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _remediation) = remediation {
                                match _remediation {
                                    crate::datadogV2::model::DORADeploymentPatchByVersionRemediation::UnparsedObject(_remediation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let change_failure =
                    change_failure.ok_or_else(|| M::Error::missing_field("change_failure"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = DORADeploymentPatchByVersionRequestAttributes {
                    change_failure,
                    env,
                    remediation,
                    service,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORADeploymentPatchByVersionRequestAttributesVisitor)
    }
}
