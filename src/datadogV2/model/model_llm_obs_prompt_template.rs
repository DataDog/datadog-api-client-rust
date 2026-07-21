// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A text template or a list of chat messages.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LLMObsPromptTemplate {
    LLMObsPromptTextTemplate(String),
    LLMObsPromptChatTemplate(Vec<crate::datadogV2::model::LLMObsPromptChatMessage>),
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

        return Ok(LLMObsPromptTemplate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
