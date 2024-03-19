// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing the new assignee of a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalAssigneeUpdateAttributes {
    /// Object representing a given user entity.
    #[serde(rename = "assignee")]
    pub assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalAssigneeUpdateAttributes {
    pub fn new(
        assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> SecurityMonitoringSignalAssigneeUpdateAttributes {
        SecurityMonitoringSignalAssigneeUpdateAttributes {
            assignee,
            version: None,
            _unparsed: false,
        }
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalAssigneeUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalAssigneeUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalAssigneeUpdateAttributesVisitor {
            type Value = SecurityMonitoringSignalAssigneeUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee: Option<crate::datadogV2::model::SecurityMonitoringTriageUser> =
                    None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee" => {
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let assignee = assignee.ok_or_else(|| M::Error::missing_field("assignee"))?;

                let content = SecurityMonitoringSignalAssigneeUpdateAttributes {
                    assignee,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalAssigneeUpdateAttributesVisitor)
    }
}
