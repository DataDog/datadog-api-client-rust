// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SunburstBackendRequest20270101OneOfQuery {
	StructuredAnalysisQuery(Box<crate::datadogV1::model::StructuredAnalysisQuery>),
	SampledAnalysisQuery(Box<crate::datadogV1::model::SampledAnalysisQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SunburstBackendRequest20270101OneOfQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::StructuredAnalysisQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SunburstBackendRequest20270101OneOfQuery::StructuredAnalysisQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SampledAnalysisQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SunburstBackendRequest20270101OneOfQuery::SampledAnalysisQuery(_v));
            }
        }
	
        return Ok(SunburstBackendRequest20270101OneOfQuery::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
