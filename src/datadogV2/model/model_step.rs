// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Step is a sub-component of a workflow. Each Step performs an action.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Step {
    /// The unique identifier of an action.
    #[serde(rename = "actionId")]
    pub action_id: String,
    /// Used to create conditions before running subsequent actions.
    #[serde(rename = "completionGate")]
    pub completion_gate: Option<crate::datadogV2::model::CompletionGate>,
    /// The unique identifier of a connection defined in the spec.
    #[serde(rename = "connectionLabel")]
    pub connection_label: Option<String>,
    /// The definition of `StepDisplay` object.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV2::model::StepDisplay>,
    /// The `Step` `errorHandlers`.
    #[serde(rename = "errorHandlers")]
    pub error_handlers: Option<Vec<crate::datadogV2::model::ErrorHandler>>,
    /// Name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of subsequent actions to run.
    #[serde(rename = "outboundEdges")]
    pub outbound_edges: Option<Vec<crate::datadogV2::model::OutboundEdge>>,
    /// A list of inputs for an action.
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<crate::datadogV2::model::Parameter>>,
    /// Used to merge multiple branches into a single branch.
    #[serde(rename = "readinessGate")]
    pub readiness_gate: Option<crate::datadogV2::model::ReadinessGate>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Step {
    pub fn new(action_id: String, name: String) -> Step {
        Step {
            action_id,
            completion_gate: None,
            connection_label: None,
            display: None,
            error_handlers: None,
            name,
            outbound_edges: None,
            parameters: None,
            readiness_gate: None,
            _unparsed: false,
        }
    }

    pub fn completion_gate(mut self, value: crate::datadogV2::model::CompletionGate) -> Self {
        self.completion_gate = Some(value);
        self
    }

    pub fn connection_label(mut self, value: String) -> Self {
        self.connection_label = Some(value);
        self
    }

    pub fn display(mut self, value: crate::datadogV2::model::StepDisplay) -> Self {
        self.display = Some(value);
        self
    }

    pub fn error_handlers(mut self, value: Vec<crate::datadogV2::model::ErrorHandler>) -> Self {
        self.error_handlers = Some(value);
        self
    }

    pub fn outbound_edges(mut self, value: Vec<crate::datadogV2::model::OutboundEdge>) -> Self {
        self.outbound_edges = Some(value);
        self
    }

    pub fn parameters(mut self, value: Vec<crate::datadogV2::model::Parameter>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn readiness_gate(mut self, value: crate::datadogV2::model::ReadinessGate) -> Self {
        self.readiness_gate = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StepVisitor;
        impl<'a> Visitor<'a> for StepVisitor {
            type Value = Step;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_id: Option<String> = None;
                let mut completion_gate: Option<crate::datadogV2::model::CompletionGate> = None;
                let mut connection_label: Option<String> = None;
                let mut display: Option<crate::datadogV2::model::StepDisplay> = None;
                let mut error_handlers: Option<Vec<crate::datadogV2::model::ErrorHandler>> = None;
                let mut name: Option<String> = None;
                let mut outbound_edges: Option<Vec<crate::datadogV2::model::OutboundEdge>> = None;
                let mut parameters: Option<Vec<crate::datadogV2::model::Parameter>> = None;
                let mut readiness_gate: Option<crate::datadogV2::model::ReadinessGate> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actionId" => {
                            action_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "completionGate" => {
                            if v.is_null() {
                                continue;
                            }
                            completion_gate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connectionLabel" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_label =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display" => {
                            if v.is_null() {
                                continue;
                            }
                            display = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errorHandlers" => {
                            if v.is_null() {
                                continue;
                            }
                            error_handlers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outboundEdges" => {
                            if v.is_null() {
                                continue;
                            }
                            outbound_edges =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "readinessGate" => {
                            if v.is_null() {
                                continue;
                            }
                            readiness_gate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let action_id = action_id.ok_or_else(|| M::Error::missing_field("action_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = Step {
                    action_id,
                    completion_gate,
                    connection_label,
                    display,
                    error_handlers,
                    name,
                    outbound_edges,
                    parameters,
                    readiness_gate,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StepVisitor)
    }
}
