// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a data observability monitor run status response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetDataObservabilityMonitorRunStatusResponseAttributes {
    /// Error message describing why the monitor run failed. Only present when status is error.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// The status of a data observability monitor run.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::DataObservabilityMonitorRunStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetDataObservabilityMonitorRunStatusResponseAttributes {
    pub fn new(
        status: crate::datadogV2::model::DataObservabilityMonitorRunStatus,
    ) -> GetDataObservabilityMonitorRunStatusResponseAttributes {
        GetDataObservabilityMonitorRunStatusResponseAttributes {
            error_message: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
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

impl<'de> Deserialize<'de> for GetDataObservabilityMonitorRunStatusResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetDataObservabilityMonitorRunStatusResponseAttributesVisitor;
        impl<'a> Visitor<'a> for GetDataObservabilityMonitorRunStatusResponseAttributesVisitor {
            type Value = GetDataObservabilityMonitorRunStatusResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_message: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::DataObservabilityMonitorRunStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DataObservabilityMonitorRunStatus::UnparsedObject(_status) => {
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
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = GetDataObservabilityMonitorRunStatusResponseAttributes {
                    error_message,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetDataObservabilityMonitorRunStatusResponseAttributesVisitor)
    }
}
