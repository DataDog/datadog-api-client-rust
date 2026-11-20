// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TraceStreamQuery20270101Sort {
	ListStreamSort(Box<crate::datadogV1::model::ListStreamSort>),
	Vec<crate::datadogV1::model::ListStreamSort>(Vec<crate::datadogV1::model::ListStreamSort>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TraceStreamQuery20270101Sort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ListStreamSort>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TraceStreamQuery20270101Sort::ListStreamSort(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Vec<crate::datadogV1::model::ListStreamSort>>(value.clone()) {
			return Ok(TraceStreamQuery20270101Sort::Vec<crate::datadogV1::model::ListStreamSort>(_v))
		}
	
        return Ok(TraceStreamQuery20270101Sort::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
