// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CohortScope20270101Target {
	CohortScope20270101TargetOneOf(Box<crate::datadogV1::model::CohortScope20270101TargetOneOf>),
	CohortScope20270101TargetOneOf4454865216(Box<crate::datadogV1::model::CohortScope20270101TargetOneOf4454865216>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CohortScope20270101Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::CohortScope20270101TargetOneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CohortScope20270101Target::CohortScope20270101TargetOneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::CohortScope20270101TargetOneOf4454865216>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CohortScope20270101Target::CohortScope20270101TargetOneOf4454865216(_v));
            }
        }
	
        return Ok(CohortScope20270101Target::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
