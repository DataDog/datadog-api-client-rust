// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Guardrail metric details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GuardrailMetric {
    /// The metric ID to monitor.
    #[serde(rename = "metric_id")]
    pub metric_id: String,
    /// Action to perform when a guardrail threshold is triggered.
    #[serde(rename = "trigger_action")]
    pub trigger_action: crate::datadogV2::model::GuardrailTriggerAction,
    /// The signal or system that triggered the action.
    #[serde(
        rename = "triggered_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub triggered_by: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GuardrailMetric {
    pub fn new(
        metric_id: String,
        trigger_action: crate::datadogV2::model::GuardrailTriggerAction,
    ) -> GuardrailMetric {
        GuardrailMetric {
            metric_id,
            trigger_action,
            triggered_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn triggered_by(mut self, value: Option<String>) -> Self {
        self.triggered_by = Some(value);
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

impl<'de> Deserialize<'de> for GuardrailMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GuardrailMetricVisitor;
        impl<'a> Visitor<'a> for GuardrailMetricVisitor {
            type Value = GuardrailMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metric_id: Option<String> = None;
                let mut trigger_action: Option<crate::datadogV2::model::GuardrailTriggerAction> =
                    None;
                let mut triggered_by: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_action" => {
                            trigger_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trigger_action) = trigger_action {
                                match _trigger_action {
                                    crate::datadogV2::model::GuardrailTriggerAction::UnparsedObject(_trigger_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "triggered_by" => {
                            triggered_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;
                let trigger_action =
                    trigger_action.ok_or_else(|| M::Error::missing_field("trigger_action"))?;

                let content = GuardrailMetric {
                    metric_id,
                    trigger_action,
                    triggered_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GuardrailMetricVisitor)
    }
}
