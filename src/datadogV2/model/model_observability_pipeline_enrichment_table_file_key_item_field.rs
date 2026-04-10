// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Specifies the source of the key value used for enrichment table lookups.
/// Can be a plain field path string or an object specifying `event`, `vrl`, or `secret`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineEnrichmentTableFileKeyItemField {
    ObservabilityPipelineEnrichmentTableFieldStringPath(String),
    ObservabilityPipelineEnrichmentTableFieldEventLookup(
        Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldEventLookup>,
    ),
    ObservabilityPipelineEnrichmentTableFieldVrlLookup(
        Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldVrlLookup>,
    ),
    ObservabilityPipelineEnrichmentTableFieldSecretLookup(
        Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldSecretLookup>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineEnrichmentTableFileKeyItemField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ObservabilityPipelineEnrichmentTableFileKeyItemField::ObservabilityPipelineEnrichmentTableFieldStringPath(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldEventLookup>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineEnrichmentTableFileKeyItemField::ObservabilityPipelineEnrichmentTableFieldEventLookup(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldVrlLookup>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineEnrichmentTableFileKeyItemField::ObservabilityPipelineEnrichmentTableFieldVrlLookup(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineEnrichmentTableFieldSecretLookup>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineEnrichmentTableFileKeyItemField::ObservabilityPipelineEnrichmentTableFieldSecretLookup(_v));
            }
        }

        return Ok(
            ObservabilityPipelineEnrichmentTableFileKeyItemField::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
