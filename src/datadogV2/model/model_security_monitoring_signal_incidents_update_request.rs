// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request body for changing the related incidents of a given security monitoring signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateRequest {
    /// Data containing the patch for changing the related incidents of a signal.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateData,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalIncidentsUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateData,
    ) -> SecurityMonitoringSignalIncidentsUpdateRequest {
        SecurityMonitoringSignalIncidentsUpdateRequest {
            data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalIncidentsUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalIncidentsUpdateRequestVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalIncidentsUpdateRequestVisitor {
            type Value = SecurityMonitoringSignalIncidentsUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateData,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = SecurityMonitoringSignalIncidentsUpdateRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalIncidentsUpdateRequestVisitor)
    }
}
