// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an enrollment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnPremManagementServiceEnrollmentAttributes {
    /// List of allowed actions for the runner.
    #[serde(rename = "actions_allowlist")]
    pub actions_allowlist: Vec<String>,
    /// The hostname for the runner. Required when push mode is enabled.
    #[serde(rename = "runner_host")]
    pub runner_host: Option<String>,
    /// The modes the runner should operate in.
    #[serde(rename = "runner_modes")]
    pub runner_modes:
        Vec<crate::datadogV2::model::OnPremManagementServiceEnrollmentAttributesRunnerModesItems>,
    /// The name of the on-prem runner.
    #[serde(rename = "runner_name")]
    pub runner_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnPremManagementServiceEnrollmentAttributes {
    pub fn new(
        actions_allowlist: Vec<String>,
        runner_modes: Vec<
            crate::datadogV2::model::OnPremManagementServiceEnrollmentAttributesRunnerModesItems,
        >,
        runner_name: String,
    ) -> OnPremManagementServiceEnrollmentAttributes {
        OnPremManagementServiceEnrollmentAttributes {
            actions_allowlist,
            runner_host: None,
            runner_modes,
            runner_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn runner_host(mut self, value: String) -> Self {
        self.runner_host = Some(value);
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

impl<'de> Deserialize<'de> for OnPremManagementServiceEnrollmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnPremManagementServiceEnrollmentAttributesVisitor;
        impl<'a> Visitor<'a> for OnPremManagementServiceEnrollmentAttributesVisitor {
            type Value = OnPremManagementServiceEnrollmentAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions_allowlist: Option<Vec<String>> = None;
                let mut runner_host: Option<String> = None;
                let mut runner_modes: Option<Vec<crate::datadogV2::model::OnPremManagementServiceEnrollmentAttributesRunnerModesItems>> = None;
                let mut runner_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions_allowlist" => {
                            actions_allowlist =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_host" => {
                            if v.is_null() {
                                continue;
                            }
                            runner_host =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_modes" => {
                            runner_modes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_name" => {
                            runner_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let actions_allowlist = actions_allowlist
                    .ok_or_else(|| M::Error::missing_field("actions_allowlist"))?;
                let runner_modes =
                    runner_modes.ok_or_else(|| M::Error::missing_field("runner_modes"))?;
                let runner_name =
                    runner_name.ok_or_else(|| M::Error::missing_field("runner_name"))?;

                let content = OnPremManagementServiceEnrollmentAttributes {
                    actions_allowlist,
                    runner_host,
                    runner_modes,
                    runner_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnPremManagementServiceEnrollmentAttributesVisitor)
    }
}
