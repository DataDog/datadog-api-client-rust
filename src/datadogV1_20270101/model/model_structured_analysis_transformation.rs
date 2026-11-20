// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StructuredAnalysisTransformation {
	StructuredAnalysisFilter(Box<crate::datadogV1_20270101::model::StructuredAnalysisFilter>),
	StructuredAnalysisColumnExtraction(Box<crate::datadogV1_20270101::model::StructuredAnalysisColumnExtraction>),
	StructuredAnalysisAggregation(Box<crate::datadogV1_20270101::model::StructuredAnalysisAggregation>),
	StructuredAnalysisSort(Box<crate::datadogV1_20270101::model::StructuredAnalysisSort>),
	StructuredAnalysisLimit(Box<crate::datadogV1_20270101::model::StructuredAnalysisLimit>),
	StructuredAnalysisCalculatedField(Box<crate::datadogV1_20270101::model::StructuredAnalysisCalculatedField>),
	StructuredAnalysisCast(Box<crate::datadogV1_20270101::model::StructuredAnalysisCast>),
	StructuredAnalysisJoin(Box<crate::datadogV1_20270101::model::StructuredAnalysisJoin>),
	StructuredAnalysisProjection(Box<crate::datadogV1_20270101::model::StructuredAnalysisProjection>),
	StructuredAnalysisDestructure(Box<crate::datadogV1_20270101::model::StructuredAnalysisDestructure>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for StructuredAnalysisTransformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisFilter>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisFilter(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisColumnExtraction>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisColumnExtraction(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisAggregation>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisAggregation(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisSort>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisSort(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisLimit>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisLimit(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisCalculatedField>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisCalculatedField(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisCast>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisCast(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisJoin>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisJoin(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisProjection>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisProjection(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisDestructure>>(value.clone()) {
			if !_v._unparsed {
                return Ok(StructuredAnalysisTransformation::StructuredAnalysisDestructure(_v));
            }
        }
	
        return Ok(StructuredAnalysisTransformation::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
