// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema for an On-Call-based trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallTriggerWrapper {
    /// Trigger a workflow from an On-Call Page or On-Call Handover. For automatic triggering a handle must be configured and the workflow must be published.
    #[serde(rename = "onCallTrigger")]
    pub on_call_trigger: crate::datadogV2::model::OnCallTrigger,
    /// A list of steps that run first after a trigger fires.
    #[serde(rename = "startStepNames")]
    pub start_step_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallTriggerWrapper {
    pub fn new(on_call_trigger: crate::datadogV2::model::OnCallTrigger) -> OnCallTriggerWrapper {
        OnCallTriggerWrapper {
            on_call_trigger,
            start_step_names: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn start_step_names(mut self, value: Vec<String>) -> Self {
        self.start_step_names = Some(value);
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

impl<'de> Deserialize<'de> for OnCallTriggerWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallTriggerWrapperVisitor;
        impl<'a> Visitor<'a> for OnCallTriggerWrapperVisitor {
            type Value = OnCallTriggerWrapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut on_call_trigger: Option<crate::datadogV2::model::OnCallTrigger> = None;
                let mut start_step_names: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "onCallTrigger" => {
                            on_call_trigger =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startStepNames" => {
                            if v.is_null() {
                                continue;
                            }
                            start_step_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let on_call_trigger =
                    on_call_trigger.ok_or_else(|| M::Error::missing_field("on_call_trigger"))?;

                let content = OnCallTriggerWrapper {
                    on_call_trigger,
                    start_step_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallTriggerWrapperVisitor)
    }
}
