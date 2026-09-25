// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The trigger definition for starting an investigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TriggerAttributes {
    /// Attributes for a general investigation, not tied to a specific monitor alert.
    #[serde(rename = "general_investigation")]
    pub general_investigation: Option<crate::datadogV2::model::GeneralInvestigationAttributes>,
    /// Attributes for a monitor alert trigger.
    #[serde(rename = "monitor_alert_trigger")]
    pub monitor_alert_trigger: Option<crate::datadogV2::model::MonitorAlertTriggerAttributes>,
    /// The type of trigger for the investigation.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::TriggerType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TriggerAttributes {
    pub fn new() -> TriggerAttributes {
        TriggerAttributes {
            general_investigation: None,
            monitor_alert_trigger: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn general_investigation(
        mut self,
        value: crate::datadogV2::model::GeneralInvestigationAttributes,
    ) -> Self {
        self.general_investigation = Some(value);
        self
    }

    pub fn monitor_alert_trigger(
        mut self,
        value: crate::datadogV2::model::MonitorAlertTriggerAttributes,
    ) -> Self {
        self.monitor_alert_trigger = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::TriggerType) -> Self {
        self.type_ = Some(value);
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

impl Default for TriggerAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TriggerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TriggerAttributesVisitor;
        impl<'a> Visitor<'a> for TriggerAttributesVisitor {
            type Value = TriggerAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut general_investigation: Option<
                    crate::datadogV2::model::GeneralInvestigationAttributes,
                > = None;
                let mut monitor_alert_trigger: Option<
                    crate::datadogV2::model::MonitorAlertTriggerAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::TriggerType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "general_investigation" => {
                            if v.is_null() {
                                continue;
                            }
                            general_investigation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _general_investigation) = general_investigation {
                                match _general_investigation {
                                    crate::datadogV2::model::GeneralInvestigationAttributes::UnparsedObject(_general_investigation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "monitor_alert_trigger" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_alert_trigger =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TriggerType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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

                let content = TriggerAttributes {
                    general_investigation,
                    monitor_alert_trigger,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TriggerAttributesVisitor)
    }
}
