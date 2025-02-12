// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The spec defines what the workflow does.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Spec {
    /// A list of annotations used in the workflow. These are like sticky notes for your workflow!
    #[serde(rename = "annotations")]
    pub annotations: Option<Vec<crate::datadogV2::model::Annotation>>,
    /// A list of connections or connection groups used in the workflow.
    #[serde(rename = "connectionEnvs")]
    pub connection_envs: Option<Vec<crate::datadogV2::model::ConnectionEnv>>,
    /// Unique identifier used to trigger workflows automatically in Datadog.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// A list of input parameters for the workflow. These can be used as dynamic runtime values in your workflow.
    #[serde(rename = "inputSchema")]
    pub input_schema: Option<crate::datadogV2::model::InputSchema>,
    /// A list of output parameters for the workflow.
    #[serde(rename = "outputSchema")]
    pub output_schema: Option<crate::datadogV2::model::OutputSchema>,
    /// A `Step` is a sub-component of a workflow. Each `Step` performs an action.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::Step>>,
    /// The list of triggers that activate this workflow. At least one trigger is required, and each trigger type may appear at most once.
    #[serde(rename = "triggers")]
    pub triggers: Option<Vec<crate::datadogV2::model::Trigger>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Spec {
    pub fn new() -> Spec {
        Spec {
            annotations: None,
            connection_envs: None,
            handle: None,
            input_schema: None,
            output_schema: None,
            steps: None,
            triggers: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn annotations(mut self, value: Vec<crate::datadogV2::model::Annotation>) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn connection_envs(mut self, value: Vec<crate::datadogV2::model::ConnectionEnv>) -> Self {
        self.connection_envs = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn input_schema(mut self, value: crate::datadogV2::model::InputSchema) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn output_schema(mut self, value: crate::datadogV2::model::OutputSchema) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<crate::datadogV2::model::Step>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn triggers(mut self, value: Vec<crate::datadogV2::model::Trigger>) -> Self {
        self.triggers = Some(value);
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

impl Default for Spec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Spec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpecVisitor;
        impl<'a> Visitor<'a> for SpecVisitor {
            type Value = Spec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotations: Option<Vec<crate::datadogV2::model::Annotation>> = None;
                let mut connection_envs: Option<Vec<crate::datadogV2::model::ConnectionEnv>> = None;
                let mut handle: Option<String> = None;
                let mut input_schema: Option<crate::datadogV2::model::InputSchema> = None;
                let mut output_schema: Option<crate::datadogV2::model::OutputSchema> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::Step>> = None;
                let mut triggers: Option<Vec<crate::datadogV2::model::Trigger>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotations" => {
                            if v.is_null() {
                                continue;
                            }
                            annotations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connectionEnvs" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_envs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            input_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outputSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            output_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triggers" => {
                            if v.is_null() {
                                continue;
                            }
                            triggers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = Spec {
                    annotations,
                    connection_envs,
                    handle,
                    input_schema,
                    output_schema,
                    steps,
                    triggers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpecVisitor)
    }
}
