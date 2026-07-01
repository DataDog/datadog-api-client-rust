// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A single Grok parsing rule, selected by either source field or include query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineParseGrokProcessorRuleItem {
    ObservabilityPipelineParseGrokProcessorRule(
        Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRule>,
    ),
    ObservabilityPipelineParseGrokProcessorIncludeRule(
        Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorIncludeRule>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineParseGrokProcessorRuleItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorRule>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineParseGrokProcessorRuleItem::ObservabilityPipelineParseGrokProcessorRule(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessorIncludeRule>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineParseGrokProcessorRuleItem::ObservabilityPipelineParseGrokProcessorIncludeRule(_v));
            }
        }

        return Ok(
            ObservabilityPipelineParseGrokProcessorRuleItem::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
