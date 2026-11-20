// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DatabaseRecommendationsStreamRequestColumnsItems20270101Width {
	F64(f64),
	DashboardInlineSchema16(Box<crate::datadogV1_20270101::model::DashboardInlineSchema16>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DatabaseRecommendationsStreamRequestColumnsItems20270101Width {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
			return Ok(DatabaseRecommendationsStreamRequestColumnsItems20270101Width::F64(_v))
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DashboardInlineSchema16>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::DashboardInlineSchema16::UnparsedObject(_v) => {},
				_ => {
					return Ok(DatabaseRecommendationsStreamRequestColumnsItems20270101Width::DashboardInlineSchema16(_v))
				}
			}
		}
	
        return Ok(DatabaseRecommendationsStreamRequestColumnsItems20270101Width::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
