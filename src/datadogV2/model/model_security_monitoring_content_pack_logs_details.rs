// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details for a logs-based content pack.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringContentPackLogsDetails {
    /// The activation status of a content pack.
    #[serde(rename = "cp_activation")]
    pub cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
    /// Timestamp bucket indicating when logs were last collected.
    #[serde(rename = "data_last_seen")]
    pub data_last_seen: crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
    /// Whether filters (Security Filters or Index Query depending on the pricing model) are
    /// present and correctly configured to route logs into Cloud SIEM.
    #[serde(rename = "filters_configured")]
    pub filters_configured: bool,
    /// The installation status of the related integration.
    #[serde(rename = "integration_installed_status")]
    pub integration_installed_status:
        crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
    /// Whether logs for this content pack have been seen in any Datadog index in the last 72 hours.
    #[serde(rename = "logs_seen_from_any_index")]
    pub logs_seen_from_any_index: bool,
    /// Whether the Cloud SIEM index configuration is incorrect (only applies to certain pricing models).
    #[serde(rename = "siem_index_incorrect")]
    pub siem_index_incorrect: bool,
    /// The filtered data type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityFilterFilteredDataType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringContentPackLogsDetails {
    pub fn new(
        cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
        data_last_seen: crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
        filters_configured: bool,
        integration_installed_status: crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
        logs_seen_from_any_index: bool,
        siem_index_incorrect: bool,
        type_: crate::datadogV2::model::SecurityFilterFilteredDataType,
    ) -> SecurityMonitoringContentPackLogsDetails {
        SecurityMonitoringContentPackLogsDetails {
            cp_activation,
            data_last_seen,
            filters_configured,
            integration_installed_status,
            logs_seen_from_any_index,
            siem_index_incorrect,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackLogsDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringContentPackLogsDetailsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringContentPackLogsDetailsVisitor {
            type Value = SecurityMonitoringContentPackLogsDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cp_activation: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackActivation,
                > = None;
                let mut data_last_seen: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket,
                > = None;
                let mut filters_configured: Option<bool> = None;
                let mut integration_installed_status: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackIntegrationStatus,
                > = None;
                let mut logs_seen_from_any_index: Option<bool> = None;
                let mut siem_index_incorrect: Option<bool> = None;
                let mut type_: Option<crate::datadogV2::model::SecurityFilterFilteredDataType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "data_last_seen" => {
                            data_last_seen =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_last_seen) = data_last_seen {
                                match _data_last_seen {
                                    crate::datadogV2::model::SecurityMonitoringContentPackTimestampBucket::UnparsedObject(_data_last_seen) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "filters_configured" => {
                            filters_configured =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_installed_status" => {
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
                        "siem_index_incorrect" => {
                            siem_index_incorrect =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityFilterFilteredDataType::UnparsedObject(_type_) => {
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
                let cp_activation =
                    cp_activation.ok_or_else(|| M::Error::missing_field("cp_activation"))?;
                let data_last_seen =
                    data_last_seen.ok_or_else(|| M::Error::missing_field("data_last_seen"))?;
                let filters_configured = filters_configured
                    .ok_or_else(|| M::Error::missing_field("filters_configured"))?;
                let integration_installed_status = integration_installed_status
                    .ok_or_else(|| M::Error::missing_field("integration_installed_status"))?;
                let logs_seen_from_any_index = logs_seen_from_any_index
                    .ok_or_else(|| M::Error::missing_field("logs_seen_from_any_index"))?;
                let siem_index_incorrect = siem_index_incorrect
                    .ok_or_else(|| M::Error::missing_field("siem_index_incorrect"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringContentPackLogsDetails {
                    cp_activation,
                    data_last_seen,
                    filters_configured,
                    integration_installed_status,
                    logs_seen_from_any_index,
                    siem_index_incorrect,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringContentPackLogsDetailsVisitor)
    }
}
