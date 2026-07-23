// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Content pack details returned when Cloud SIEM is inactive for the requesting organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringContentPackOnboardingDetails {
    /// The installation status of the related integration.
    #[serde(rename = "integration_installed_status")]
    pub integration_installed_status:
        Option<crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus>,
    /// Whether logs for this content pack have been seen in any Datadog index in the last 72 hours.
    #[serde(rename = "logs_seen_from_any_index")]
    pub logs_seen_from_any_index: bool,
    /// Type for onboarding content pack details.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetailsType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringContentPackOnboardingDetails {
    pub fn new(
        logs_seen_from_any_index: bool,
        type_: crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetailsType,
    ) -> SecurityMonitoringContentPackOnboardingDetails {
        SecurityMonitoringContentPackOnboardingDetails {
            integration_installed_status: None,
            logs_seen_from_any_index,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn integration_installed_status(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
    ) -> Self {
        self.integration_installed_status = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackOnboardingDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringContentPackOnboardingDetailsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringContentPackOnboardingDetailsVisitor {
            type Value = SecurityMonitoringContentPackOnboardingDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut integration_installed_status: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
                > = None;
                let mut logs_seen_from_any_index: Option<bool> = None;
                let mut type_: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetailsType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "integration_installed_status" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_installed_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration_installed_status) =
                                integration_installed_status
                            {
                                match _integration_installed_status {
                                    crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus::UnparsedObject(_integration_installed_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "logs_seen_from_any_index" => {
                            logs_seen_from_any_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetailsType::UnparsedObject(_type_) => {
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
                let logs_seen_from_any_index = logs_seen_from_any_index
                    .ok_or_else(|| M::Error::missing_field("logs_seen_from_any_index"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringContentPackOnboardingDetails {
                    integration_installed_status,
                    logs_seen_from_any_index,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringContentPackOnboardingDetailsVisitor)
    }
}
