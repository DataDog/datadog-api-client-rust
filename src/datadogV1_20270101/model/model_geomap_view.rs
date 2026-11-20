// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GeomapView {
	GeomapView20270101OneOf(Box<crate::datadogV1::model::GeomapView20270101OneOf>),
	GeomapView20270101OneOf4457650336(Box<crate::datadogV1::model::GeomapView20270101OneOf4457650336>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GeomapView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GeomapView20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapView::GeomapView20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GeomapView20270101OneOf4457650336>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapView::GeomapView20270101OneOf4457650336(_v));
            }
        }
	
        return Ok(GeomapView::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
