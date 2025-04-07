// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A processor for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineConfigProcessorItem {
    ObservabilityPipelineFilterProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineFilterProcessor>,
    ),
    ObservabilityPipelineParseJSONProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineParseJSONProcessor>,
    ),
    ObservabilityPipelineQuotaProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineQuotaProcessor>,
    ),
    ObservabilityPipelineAddFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineAddFieldsProcessor>,
    ),
    ObservabilityPipelineRemoveFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessor>,
    ),
    ObservabilityPipelineRenameFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineRenameFieldsProcessor>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineConfigProcessorItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineFilterProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineFilterProcessor(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineParseJSONProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineParseJSONProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineQuotaProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineQuotaProcessor(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAddFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineAddFieldsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineRemoveFieldsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineRenameFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineRenameFieldsProcessor(_v));
            }
        }

        return Ok(ObservabilityPipelineConfigProcessorItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
