// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema for an App-based trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AppTriggerWrapper {
    /// Trigger a workflow VIA an App.
    #[serde(rename = "appTrigger")]
    pub app_trigger: std::collections::BTreeMap<String, serde_json::Value>,
    /// A list of steps that run first after a trigger fires.
    #[serde(rename = "startStepNames")]
    pub start_step_names: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AppTriggerWrapper {
    pub fn new(
        app_trigger: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> AppTriggerWrapper {
        AppTriggerWrapper {
            app_trigger,
            start_step_names: None,
            _unparsed: false,
        }
    }

    pub fn start_step_names(mut self, value: Vec<String>) -> Self {
        self.start_step_names = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AppTriggerWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AppTriggerWrapperVisitor;
        impl<'a> Visitor<'a> for AppTriggerWrapperVisitor {
            type Value = AppTriggerWrapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut app_trigger: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut start_step_names: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "appTrigger" => {
                            app_trigger =
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let app_trigger =
                    app_trigger.ok_or_else(|| M::Error::missing_field("app_trigger"))?;

                let content = AppTriggerWrapper {
                    app_trigger,
                    start_step_names,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AppTriggerWrapperVisitor)
    }
}
