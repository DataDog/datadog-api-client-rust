// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Description of signals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationRuleCaseCreate {
    /// Notification targets for each rule case.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationRuleCaseCreate {
    pub fn new(
        status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> CloudConfigurationRuleCaseCreate {
        CloudConfigurationRuleCaseCreate {
            notifications: None,
            status,
            _unparsed: false,
        }
    }

    pub fn notifications(mut self, value: Vec<String>) -> Self {
        self.notifications = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CloudConfigurationRuleCaseCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationRuleCaseCreateVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationRuleCaseCreateVisitor {
            type Value = CloudConfigurationRuleCaseCreate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut notifications: Option<Vec<String>> = None;
                let mut status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "notifications" => {
                            if v.is_null() {
                                continue;
                            }
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecurityMonitoringRuleSeverity::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = CloudConfigurationRuleCaseCreate {
                    notifications,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationRuleCaseCreateVisitor)
    }
}
