// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A processor for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineConfigProcessorItem {
    ObservabilityPipelineAddEnvVarsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineAddEnvVarsProcessor>,
    ),
    ObservabilityPipelineAddFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineAddFieldsProcessor>,
    ),
    ObservabilityPipelineAddHostnameProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineAddHostnameProcessor>,
    ),
    ObservabilityPipelineCustomProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineCustomProcessor>,
    ),
    ObservabilityPipelineDatadogTagsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineDatadogTagsProcessor>,
    ),
    ObservabilityPipelineDedupeProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineDedupeProcessor>,
    ),
    ObservabilityPipelineEnrichmentTableProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessor>,
    ),
    ObservabilityPipelineFilterProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineFilterProcessor>,
    ),
    ObservabilityPipelineGenerateMetricsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineGenerateMetricsProcessor>,
    ),
    ObservabilityPipelineOcsfMapperProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessor>,
    ),
    ObservabilityPipelineParseGrokProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessor>,
    ),
    ObservabilityPipelineParseJSONProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineParseJSONProcessor>,
    ),
    ObservabilityPipelineParseXMLProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineParseXMLProcessor>,
    ),
    ObservabilityPipelineQuotaProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineQuotaProcessor>,
    ),
    ObservabilityPipelineReduceProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineReduceProcessor>,
    ),
    ObservabilityPipelineRemoveFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessor>,
    ),
    ObservabilityPipelineRenameFieldsProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineRenameFieldsProcessor>,
    ),
    ObservabilityPipelineSampleProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineSampleProcessor>,
    ),
    ObservabilityPipelineSensitiveDataScannerProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessor>,
    ),
    ObservabilityPipelineSplitArrayProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessor>,
    ),
    ObservabilityPipelineThrottleProcessor(
        Box<crate::datadogV2::model::ObservabilityPipelineThrottleProcessor>,
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
            Box<crate::datadogV2::model::ObservabilityPipelineAddEnvVarsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineAddEnvVarsProcessor(_v));
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
            Box<crate::datadogV2::model::ObservabilityPipelineAddHostnameProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineAddHostnameProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineCustomProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineCustomProcessor(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineDatadogTagsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineDatadogTagsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineDedupeProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineDedupeProcessor(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineEnrichmentTableProcessor(_v));
            }
        }
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
            Box<crate::datadogV2::model::ObservabilityPipelineGenerateMetricsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineGenerateMetricsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineOcsfMapperProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineParseGrokProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineParseGrokProcessor(_v));
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
            Box<crate::datadogV2::model::ObservabilityPipelineParseXMLProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineParseXMLProcessor(_v));
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
            Box<crate::datadogV2::model::ObservabilityPipelineReduceProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineReduceProcessor(
                        _v,
                    ),
                );
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
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSampleProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineSampleProcessor(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineSensitiveDataScannerProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineSplitArrayProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineThrottleProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigProcessorItem::ObservabilityPipelineThrottleProcessor(_v));
            }
        }

        return Ok(ObservabilityPipelineConfigProcessorItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
