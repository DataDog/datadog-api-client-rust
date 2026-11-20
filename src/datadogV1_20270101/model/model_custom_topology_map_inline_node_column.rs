// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CustomTopologyMapInlineNodeColumn {
	CustomTopologyMapInlineNodeIdColumn(Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeIdColumn>),
	CustomTopologyMapInlineNodeStringColumn(Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumn>),
	CustomTopologyMapInlineNodeValueColumn(Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeValueColumn>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CustomTopologyMapInlineNodeColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeIdColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CustomTopologyMapInlineNodeColumn::CustomTopologyMapInlineNodeIdColumn(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CustomTopologyMapInlineNodeColumn::CustomTopologyMapInlineNodeStringColumn(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeValueColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CustomTopologyMapInlineNodeColumn::CustomTopologyMapInlineNodeValueColumn(_v));
            }
        }
	
        return Ok(CustomTopologyMapInlineNodeColumn::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
