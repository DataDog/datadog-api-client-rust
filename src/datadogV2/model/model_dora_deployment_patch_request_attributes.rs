// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for patching a DORA deployment event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORADeploymentPatchRequestAttributes {
    /// Indicates whether the deployment resulted in a change failure.
    #[serde(rename = "change_failure")]
    pub change_failure: Option<bool>,
    /// Remediation details for the deployment.
    #[serde(rename = "remediation")]
    pub remediation: Option<crate::datadogV2::model::DORADeploymentPatchRemediation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORADeploymentPatchRequestAttributes {
    pub fn new() -> DORADeploymentPatchRequestAttributes {
        DORADeploymentPatchRequestAttributes {
            change_failure: None,
            remediation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_failure(mut self, value: bool) -> Self {
        self.change_failure = Some(value);
        self
    }

    pub fn remediation(
        mut self,
        value: crate::datadogV2::model::DORADeploymentPatchRemediation,
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

impl Default for DORADeploymentPatchRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DORADeploymentPatchRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORADeploymentPatchRequestAttributesVisitor;
        impl<'a> Visitor<'a> for DORADeploymentPatchRequestAttributesVisitor {
            type Value = DORADeploymentPatchRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_failure: Option<bool> = None;
                let mut remediation: Option<
                    crate::datadogV2::model::DORADeploymentPatchRemediation,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_failure" => {
                            if v.is_null() {
                                continue;
                            }
                            change_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remediation" => {
                            if v.is_null() {
                                continue;
                            }
                            remediation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DORADeploymentPatchRequestAttributes {
                    change_failure,
                    remediation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORADeploymentPatchRequestAttributesVisitor)
    }
}
