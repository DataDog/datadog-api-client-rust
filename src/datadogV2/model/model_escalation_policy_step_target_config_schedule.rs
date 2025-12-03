// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schedule-specific configuration for an escalation target.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyStepTargetConfigSchedule {
    /// Specifies the position of a schedule target (example `previous`, `current`, or `next`).
    #[serde(rename = "position")]
    pub position: Option<crate::datadogV2::model::ScheduleTargetPosition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyStepTargetConfigSchedule {
    pub fn new() -> EscalationPolicyStepTargetConfigSchedule {
        EscalationPolicyStepTargetConfigSchedule {
            position: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn position(mut self, value: crate::datadogV2::model::ScheduleTargetPosition) -> Self {
        self.position = Some(value);
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

impl Default for EscalationPolicyStepTargetConfigSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EscalationPolicyStepTargetConfigSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyStepTargetConfigScheduleVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyStepTargetConfigScheduleVisitor {
            type Value = EscalationPolicyStepTargetConfigSchedule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut position: Option<crate::datadogV2::model::ScheduleTargetPosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "position" => {
                            if v.is_null() {
                                continue;
                            }
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _position) = position {
                                match _position {
                                    crate::datadogV2::model::ScheduleTargetPosition::UnparsedObject(_position) => {
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

                let content = EscalationPolicyStepTargetConfigSchedule {
                    position,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyStepTargetConfigScheduleVisitor)
    }
}
