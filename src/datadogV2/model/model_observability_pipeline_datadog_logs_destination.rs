// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `datadog_logs` destination forwards logs to Datadog Log Management.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineDatadogLogsDestination {
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// A list of routing rules that forward matching logs to Datadog using dedicated API keys.
    #[serde(rename = "routes")]
    pub routes:
        Option<Vec<crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationRoute>>,
    /// The destination type. The value should always be `datadog_logs`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineDatadogLogsDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType,
    ) -> ObservabilityPipelineDatadogLogsDestination {
        ObservabilityPipelineDatadogLogsDestination {
            id,
            inputs,
            routes: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn routes(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationRoute>,
    ) -> Self {
        self.routes = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineDatadogLogsDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineDatadogLogsDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineDatadogLogsDestinationVisitor {
            type Value = ObservabilityPipelineDatadogLogsDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut routes: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationRoute>,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "routes" => {
                            if v.is_null() {
                                continue;
                            }
                            routes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestinationType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineDatadogLogsDestination {
                    id,
                    inputs,
                    routes,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineDatadogLogsDestinationVisitor)
    }
}
