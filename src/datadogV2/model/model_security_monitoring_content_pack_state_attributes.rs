// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a content pack state
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringContentPackStateAttributes {
    /// Whether the cloud SIEM index configuration is incorrect (only applies to certain pricing models)
    #[serde(rename = "cloud_siem_index_incorrect")]
    pub cloud_siem_index_incorrect: bool,
    /// The activation status of a content pack
    #[serde(rename = "cp_activation")]
    pub cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
    /// Whether filters (Security Filters or Index Query depending on the pricing model) are configured for logs
    #[serde(rename = "filters_configured_for_logs")]
    pub filters_configured_for_logs: bool,
    /// The installation status of the related integration
    #[serde(rename = "integration_installed_status")]
    pub integration_installed_status:
        Option<crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus>,
    /// Timestamp bucket indicating when logs were last collected
    #[serde(rename = "logs_last_collected")]
    pub logs_last_collected: crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
    /// Whether logs have been seen from any index
    #[serde(rename = "logs_seen_from_any_index")]
    pub logs_seen_from_any_index: bool,
    /// The current status of a content pack
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityMonitoringContentPackStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringContentPackStateAttributes {
    pub fn new(
        cloud_siem_index_incorrect: bool,
        cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
        filters_configured_for_logs: bool,
        logs_last_collected: crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
        logs_seen_from_any_index: bool,
        state: crate::datadogV2::model::SecurityMonitoringContentPackStatus,
    ) -> SecurityMonitoringContentPackStateAttributes {
        SecurityMonitoringContentPackStateAttributes {
            cloud_siem_index_incorrect,
            cp_activation,
            filters_configured_for_logs,
            integration_installed_status: None,
            logs_last_collected,
            logs_seen_from_any_index,
            state,
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackStateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringContentPackStateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringContentPackStateAttributesVisitor {
            type Value = SecurityMonitoringContentPackStateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud_siem_index_incorrect: Option<bool> = None;
                let mut cp_activation: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackActivation,
                > = None;
                let mut filters_configured_for_logs: Option<bool> = None;
                let mut integration_installed_status: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
                > = None;
                let mut logs_last_collected: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
                > = None;
                let mut logs_seen_from_any_index: Option<bool> = None;
                let mut state: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud_siem_index_incorrect" => {
                            cloud_siem_index_incorrect =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cp_activation" => {
                            cp_activation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cp_activation) = cp_activation {
                                match _cp_activation {
                                    crate::datadogV2::model::SecurityMonitoringContentPackActivation::UnparsedObject(_cp_activation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filters_configured_for_logs" => {
                            filters_configured_for_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "logs_last_collected" => {
                            logs_last_collected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _logs_last_collected) = logs_last_collected {
                                match _logs_last_collected {
                                    crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket::UnparsedObject(_logs_last_collected) => {
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
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::SecurityMonitoringContentPackStatus::UnparsedObject(_state) => {
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
                let cloud_siem_index_incorrect = cloud_siem_index_incorrect
                    .ok_or_else(|| M::Error::missing_field("cloud_siem_index_incorrect"))?;
                let cp_activation =
                    cp_activation.ok_or_else(|| M::Error::missing_field("cp_activation"))?;
                let filters_configured_for_logs = filters_configured_for_logs
                    .ok_or_else(|| M::Error::missing_field("filters_configured_for_logs"))?;
                let logs_last_collected = logs_last_collected
                    .ok_or_else(|| M::Error::missing_field("logs_last_collected"))?;
                let logs_seen_from_any_index = logs_seen_from_any_index
                    .ok_or_else(|| M::Error::missing_field("logs_seen_from_any_index"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = SecurityMonitoringContentPackStateAttributes {
                    cloud_siem_index_incorrect,
                    cp_activation,
                    filters_configured_for_logs,
                    integration_installed_status,
                    logs_last_collected,
                    logs_seen_from_any_index,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringContentPackStateAttributesVisitor)
    }
}
