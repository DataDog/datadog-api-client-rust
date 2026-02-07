// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorkflowHeadlessExecutionConfig {
    /// List of connections to use for the workflow execution
    #[serde(rename = "connections")]
    pub connections: Vec<crate::datadogV2::model::WorkflowHeadlessExecutionConnection>,
    /// Input parameters for the workflow execution
    #[serde(rename = "inputs")]
    pub inputs: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WorkflowHeadlessExecutionConfig {
    pub fn new(
        connections: Vec<crate::datadogV2::model::WorkflowHeadlessExecutionConnection>,
        inputs: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> WorkflowHeadlessExecutionConfig {
        WorkflowHeadlessExecutionConfig {
            connections,
            inputs,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for WorkflowHeadlessExecutionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkflowHeadlessExecutionConfigVisitor;
        impl<'a> Visitor<'a> for WorkflowHeadlessExecutionConfigVisitor {
            type Value = WorkflowHeadlessExecutionConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connections: Option<
                    Vec<crate::datadogV2::model::WorkflowHeadlessExecutionConnection>,
                > = None;
                let mut inputs: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connections" => {
                            connections =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let connections =
                    connections.ok_or_else(|| M::Error::missing_field("connections"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;

                let content = WorkflowHeadlessExecutionConfig {
                    connections,
                    inputs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WorkflowHeadlessExecutionConfigVisitor)
    }
}
