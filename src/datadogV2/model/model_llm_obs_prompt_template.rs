// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A text template, a list of chat messages, or an authored chat object. Text can include an exact prompt version with `{{>prompt-id version=N}}`. Use an authored chat object when including prompts as chat messages.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LLMObsPromptTemplate {
    LLMObsPromptTextTemplate(String),
    LLMObsPromptChatTemplate(Vec<crate::datadogV2::model::LLMObsPromptChatMessage>),
    LLMObsPromptAuthoringMessagesTemplate(
        Box<crate::datadogV2::model::LLMObsPromptAuthoringMessagesTemplate>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LLMObsPromptTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(LLMObsPromptTemplate::LLMObsPromptTextTemplate(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::LLMObsPromptChatMessage>,
        >(value.clone())
        {
            return Ok(LLMObsPromptTemplate::LLMObsPromptChatTemplate(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LLMObsPromptAuthoringMessagesTemplate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LLMObsPromptTemplate::LLMObsPromptAuthoringMessagesTemplate(
                    _v,
                ));
            }
        }

        return Ok(LLMObsPromptTemplate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
