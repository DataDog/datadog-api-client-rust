// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CompositeSourceSubQuery {
	CompositeEventsPlatformSubQueryDefinition(Box<crate::datadogV1_20270101::model::CompositeEventsPlatformSubQueryDefinition>),
	CompositeReferentialDataQuery(Box<crate::datadogV1_20270101::model::CompositeReferentialDataQuery>),
	CompositeAudienceFiltersQuery(Box<crate::datadogV1_20270101::model::CompositeAudienceFiltersQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CompositeSourceSubQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CompositeEventsPlatformSubQueryDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceSubQuery::CompositeEventsPlatformSubQueryDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CompositeReferentialDataQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceSubQuery::CompositeReferentialDataQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CompositeAudienceFiltersQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CompositeSourceSubQuery::CompositeAudienceFiltersQuery(_v));
            }
        }
	
        return Ok(CompositeSourceSubQuery::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
