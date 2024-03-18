// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data containing the patch for changing the related incidents of a signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateData {
    /// Attributes describing the new list of related signals for a security signal.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateAttributes,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalIncidentsUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateAttributes,
    ) -> SecurityMonitoringSignalIncidentsUpdateData {
        SecurityMonitoringSignalIncidentsUpdateData {
            attributes,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalIncidentsUpdateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalIncidentsUpdateDataVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalIncidentsUpdateDataVisitor {
            type Value = SecurityMonitoringSignalIncidentsUpdateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateAttributes,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;

                let content = SecurityMonitoringSignalIncidentsUpdateData {
                    attributes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalIncidentsUpdateDataVisitor)
    }
}
