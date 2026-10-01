// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Specifies the source of the key value used for metric enrichment table lookups.
/// The lookup key can be either the metric name or a metric tag.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineMetricEnrichmentTableLookupSource {
    ObservabilityPipelineMetricEnrichmentTableMetricNameLookup(
        Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableMetricNameLookup>,
    ),
    ObservabilityPipelineMetricEnrichmentTableTagLookup(
        Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableTagLookup>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricEnrichmentTableLookupSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableMetricNameLookup,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricEnrichmentTableLookupSource::ObservabilityPipelineMetricEnrichmentTableMetricNameLookup(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableTagLookup>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineMetricEnrichmentTableLookupSource::ObservabilityPipelineMetricEnrichmentTableTagLookup(_v));
            }
        }

        return Ok(
            ObservabilityPipelineMetricEnrichmentTableLookupSource::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
