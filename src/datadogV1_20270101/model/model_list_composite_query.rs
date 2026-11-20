// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ListCompositeQuery {
	ListFilteredQuery(Box<crate::datadogV1_20270101::model::ListFilteredQuery>),
	ListAugmentedQuery(Box<crate::datadogV1_20270101::model::ListAugmentedQuery>),
	ListSegmentQuery(Box<crate::datadogV1_20270101::model::ListSegmentQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ListCompositeQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ListFilteredQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListCompositeQuery::ListFilteredQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ListAugmentedQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListCompositeQuery::ListAugmentedQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ListSegmentQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListCompositeQuery::ListSegmentQuery(_v));
            }
        }
	
        return Ok(ListCompositeQuery::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
