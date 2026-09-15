// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A chat message or a named message placeholder in a prompt template.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LLMObsPromptChatTemplateItem {
    LLMObsPromptChatMessage(Box<crate::datadogV2::model::LLMObsPromptChatMessage>),
    LLMObsPromptMessagePlaceholder(Box<crate::datadogV2::model::LLMObsPromptMessagePlaceholder>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LLMObsPromptChatTemplateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LLMObsPromptChatMessage>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LLMObsPromptChatTemplateItem::LLMObsPromptChatMessage(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LLMObsPromptMessagePlaceholder>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LLMObsPromptChatTemplateItem::LLMObsPromptMessagePlaceholder(_v));
            }
        }

        return Ok(LLMObsPromptChatTemplateItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
