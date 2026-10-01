// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The `enrichment_table` processor enriches metrics with tags from a static CSV file or a Datadog reference table.
/// It looks up a row using the metric name or a metric tag value. It then adds each column of the matching row as a
/// metric tag, overwriting any existing tag with the same key. Exactly one of `file` or `reference_table` must be
/// configured.
///
/// **Supported pipeline types:** metrics
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineMetricEnrichmentTableProcessor {
    ObservabilityPipelineMetricEnrichmentTableFileProcessor(Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFileProcessor>),
	ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor(Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricEnrichmentTableProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFileProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricEnrichmentTableProcessor::ObservabilityPipelineMetricEnrichmentTableFileProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricEnrichmentTableProcessor::ObservabilityPipelineMetricEnrichmentTableReferenceTableProcessor(_v));
            }
        }

        return Ok(
            ObservabilityPipelineMetricEnrichmentTableProcessor::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
