// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for converting historical job results to signals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConvertJobResultsToSignalsAttributes {
    /// Request ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Job result IDs.
    #[serde(rename = "jobResultIds")]
    pub job_result_ids: Vec<String>,
    /// Notifications sent.
    #[serde(rename = "notifications")]
    pub notifications: Vec<String>,
    /// Message of generated signals.
    #[serde(rename = "signalMessage")]
    pub signal_message: String,
    /// Severity of the Security Signal.
    #[serde(rename = "signalSeverity")]
    pub signal_severity: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConvertJobResultsToSignalsAttributes {
    pub fn new(
        job_result_ids: Vec<String>,
        notifications: Vec<String>,
        signal_message: String,
        signal_severity: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> ConvertJobResultsToSignalsAttributes {
        ConvertJobResultsToSignalsAttributes {
            id: None,
            job_result_ids,
            notifications,
            signal_message,
            signal_severity,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for ConvertJobResultsToSignalsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConvertJobResultsToSignalsAttributesVisitor;
        impl<'a> Visitor<'a> for ConvertJobResultsToSignalsAttributesVisitor {
            type Value = ConvertJobResultsToSignalsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut job_result_ids: Option<Vec<String>> = None;
                let mut notifications: Option<Vec<String>> = None;
                let mut signal_message: Option<String> = None;
                let mut signal_severity: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleSeverity,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobResultIds" => {
                            job_result_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signalMessage" => {
                            signal_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signalSeverity" => {
                            signal_severity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _signal_severity) = signal_severity {
                                match _signal_severity {
                                    crate::datadogV2::model::SecurityMonitoringRuleSeverity::UnparsedObject(_signal_severity) => {
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
                let job_result_ids =
                    job_result_ids.ok_or_else(|| M::Error::missing_field("job_result_ids"))?;
                let notifications =
                    notifications.ok_or_else(|| M::Error::missing_field("notifications"))?;
                let signal_message =
                    signal_message.ok_or_else(|| M::Error::missing_field("signal_message"))?;
                let signal_severity =
                    signal_severity.ok_or_else(|| M::Error::missing_field("signal_severity"))?;

                let content = ConvertJobResultsToSignalsAttributes {
                    id,
                    job_result_ids,
                    notifications,
                    signal_message,
                    signal_severity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConvertJobResultsToSignalsAttributesVisitor)
    }
}
