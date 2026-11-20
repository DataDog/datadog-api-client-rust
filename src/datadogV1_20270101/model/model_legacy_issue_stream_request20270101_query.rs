// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LegacyIssueStreamRequest20270101Query {
	APMIssueStreamQuery(Box<crate::datadogV1_20270101::model::APMIssueStreamQuery>),
	RUMIssueStreamQuery(Box<crate::datadogV1_20270101::model::RUMIssueStreamQuery>),
	LogsIssueStreamQuery(Box<crate::datadogV1_20270101::model::LogsIssueStreamQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LegacyIssueStreamRequest20270101Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::APMIssueStreamQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(LegacyIssueStreamRequest20270101Query::APMIssueStreamQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RUMIssueStreamQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(LegacyIssueStreamRequest20270101Query::RUMIssueStreamQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LogsIssueStreamQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(LegacyIssueStreamRequest20270101Query::LogsIssueStreamQuery(_v));
            }
        }
	
        return Ok(LegacyIssueStreamRequest20270101Query::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
