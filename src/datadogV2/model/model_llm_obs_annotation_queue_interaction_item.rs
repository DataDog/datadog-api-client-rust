// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A single interaction to add to an annotation queue.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LLMObsAnnotationQueueInteractionItem {
    LLMObsTraceInteractionItem(Box<crate::datadogV2::model::LLMObsTraceInteractionItem>),
    LLMObsDisplayBlockInteractionItem(
        Box<crate::datadogV2::model::LLMObsDisplayBlockInteractionItem>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LLMObsAnnotationQueueInteractionItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LLMObsTraceInteractionItem>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LLMObsAnnotationQueueInteractionItem::LLMObsTraceInteractionItem(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LLMObsDisplayBlockInteractionItem>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    LLMObsAnnotationQueueInteractionItem::LLMObsDisplayBlockInteractionItem(_v),
                );
            }
        }

        return Ok(LLMObsAnnotationQueueInteractionItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
