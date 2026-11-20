// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FormulaUserJourneyQuery20270101Search {
	LegacyUserJourneySearch(Box<crate::datadogV1_20270101::model::LegacyUserJourneySearch>),
	UserJourneySearch(Box<crate::datadogV1_20270101::model::UserJourneySearch>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for FormulaUserJourneyQuery20270101Search {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LegacyUserJourneySearch>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaUserJourneyQuery20270101Search::LegacyUserJourneySearch(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::UserJourneySearch>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaUserJourneyQuery20270101Search::UserJourneySearch(_v));
            }
        }
	
        return Ok(FormulaUserJourneyQuery20270101Search::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
