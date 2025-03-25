// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema for a Workflow-based trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorkflowTriggerWrapper {
    /// A list of steps that run first after a trigger fires.
    #[serde(rename = "startStepNames")]
    pub start_step_names: Option<Vec<String>>,
    /// Trigger a workflow from the Datadog UI. Only required if no other trigger exists.
    #[serde(rename = "workflowTrigger")]
    pub workflow_trigger: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WorkflowTriggerWrapper {
    pub fn new(
        workflow_trigger: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> WorkflowTriggerWrapper {
        WorkflowTriggerWrapper {
            start_step_names: None,
            workflow_trigger,
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

impl<'de> Deserialize<'de> for WorkflowTriggerWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkflowTriggerWrapperVisitor;
        impl<'a> Visitor<'a> for WorkflowTriggerWrapperVisitor {
            type Value = WorkflowTriggerWrapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut start_step_names: Option<Vec<String>> = None;
                let mut workflow_trigger: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "startStepNames" => {
                            if v.is_null() {
                                continue;
                            }
                            start_step_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflowTrigger" => {
                            workflow_trigger =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let workflow_trigger =
                    workflow_trigger.ok_or_else(|| M::Error::missing_field("workflow_trigger"))?;

                let content = WorkflowTriggerWrapper {
                    start_step_names,
                    workflow_trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WorkflowTriggerWrapperVisitor)
    }
}
