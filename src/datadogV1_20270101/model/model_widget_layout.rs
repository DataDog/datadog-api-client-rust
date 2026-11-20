// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WidgetLayout {
	FreeLayout(Box<crate::datadogV1_20270101::model::FreeLayout>),
	MSLWidgetLayout(Box<crate::datadogV1_20270101::model::MSLWidgetLayout>),
	MSLBackendWidgetLayout(Box<crate::datadogV1_20270101::model::MSLBackendWidgetLayout>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WidgetLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FreeLayout>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetLayout::FreeLayout(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MSLWidgetLayout>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetLayout::MSLWidgetLayout(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MSLBackendWidgetLayout>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetLayout::MSLBackendWidgetLayout(_v));
            }
        }
	
        return Ok(WidgetLayout::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
