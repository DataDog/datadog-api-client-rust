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
pub struct WorkflowScaffoldAgenticStreamRequest {
    /// Previous chat messages for iterative workflow generation.
    #[serde(rename = "chatHistory")]
    pub chat_history: Option<Vec<crate::datadogV2::model::ChatMessage>>,
    /// The existing workflow specification to modify.
    #[serde(rename = "existingWorkflow")]
    pub existing_workflow: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The previous action taken in the workflow generation.
    #[serde(rename = "previousAction")]
    pub previous_action: Option<String>,
    #[serde(rename = "userContext")]
    pub user_context: Option<crate::datadogV2::model::UserContext>,
    /// The user's prompt for generating or modifying the workflow.
    #[serde(rename = "userPrompt")]
    pub user_prompt: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WorkflowScaffoldAgenticStreamRequest {
    pub fn new(user_prompt: String) -> WorkflowScaffoldAgenticStreamRequest {
        WorkflowScaffoldAgenticStreamRequest {
            chat_history: None,
            existing_workflow: None,
            previous_action: None,
            user_context: None,
            user_prompt,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn chat_history(mut self, value: Vec<crate::datadogV2::model::ChatMessage>) -> Self {
        self.chat_history = Some(value);
        self
    }

    pub fn existing_workflow(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.existing_workflow = Some(value);
        self
    }

    pub fn previous_action(mut self, value: String) -> Self {
        self.previous_action = Some(value);
        self
    }

    pub fn user_context(mut self, value: crate::datadogV2::model::UserContext) -> Self {
        self.user_context = Some(value);
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

impl<'de> Deserialize<'de> for WorkflowScaffoldAgenticStreamRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkflowScaffoldAgenticStreamRequestVisitor;
        impl<'a> Visitor<'a> for WorkflowScaffoldAgenticStreamRequestVisitor {
            type Value = WorkflowScaffoldAgenticStreamRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chat_history: Option<Vec<crate::datadogV2::model::ChatMessage>> = None;
                let mut existing_workflow: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut previous_action: Option<String> = None;
                let mut user_context: Option<crate::datadogV2::model::UserContext> = None;
                let mut user_prompt: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chatHistory" => {
                            if v.is_null() {
                                continue;
                            }
                            chat_history =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "existingWorkflow" => {
                            if v.is_null() {
                                continue;
                            }
                            existing_workflow =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previousAction" => {
                            if v.is_null() {
                                continue;
                            }
                            previous_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userContext" => {
                            if v.is_null() {
                                continue;
                            }
                            user_context =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "userPrompt" => {
                            user_prompt =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let user_prompt =
                    user_prompt.ok_or_else(|| M::Error::missing_field("user_prompt"))?;

                let content = WorkflowScaffoldAgenticStreamRequest {
                    chat_history,
                    existing_workflow,
                    previous_action,
                    user_context,
                    user_prompt,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WorkflowScaffoldAgenticStreamRequestVisitor)
    }
}
