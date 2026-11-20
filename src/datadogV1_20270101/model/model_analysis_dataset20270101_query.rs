// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AnalysisDataset20270101Query {
	SqlAnalysisQuery(Box<crate::datadogV1_20270101::model::SqlAnalysisQuery>),
	StructuredAnalysisQuery(Box<crate::datadogV1_20270101::model::StructuredAnalysisQuery>),
	SampledAnalysisQuery(Box<crate::datadogV1_20270101::model::SampledAnalysisQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AnalysisDataset20270101Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SqlAnalysisQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AnalysisDataset20270101Query::SqlAnalysisQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StructuredAnalysisQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AnalysisDataset20270101Query::StructuredAnalysisQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SampledAnalysisQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AnalysisDataset20270101Query::SampledAnalysisQuery(_v));
            }
        }
	
        return Ok(AnalysisDataset20270101Query::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
