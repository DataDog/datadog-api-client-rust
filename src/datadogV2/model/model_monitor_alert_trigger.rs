// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A trigger created from a monitor alert.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorAlertTrigger {
    /// Attributes for a monitor alert trigger.
    #[serde(rename = "monitor_alert_trigger")]
    pub monitor_alert_trigger: crate::datadogV2::model::MonitorAlertTriggerAttributes,
    /// The type of monitor alert trigger.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MonitorAlertTriggerType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorAlertTrigger {
    pub fn new(
        monitor_alert_trigger: crate::datadogV2::model::MonitorAlertTriggerAttributes,
        type_: crate::datadogV2::model::MonitorAlertTriggerType,
    ) -> MonitorAlertTrigger {
        MonitorAlertTrigger {
            monitor_alert_trigger,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorAlertTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorAlertTriggerVisitor;
        impl<'a> Visitor<'a> for MonitorAlertTriggerVisitor {
            type Value = MonitorAlertTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut monitor_alert_trigger: Option<
                    crate::datadogV2::model::MonitorAlertTriggerAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::MonitorAlertTriggerType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "monitor_alert_trigger" => {
                            monitor_alert_trigger =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MonitorAlertTriggerType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let monitor_alert_trigger = monitor_alert_trigger
                    .ok_or_else(|| M::Error::missing_field("monitor_alert_trigger"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = MonitorAlertTrigger {
                    monitor_alert_trigger,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorAlertTriggerVisitor)
    }
}
