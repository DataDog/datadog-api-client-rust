// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Column of the nodes table.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TopologyMapInlineNodeColumn {
    TopologyMapInlineNodeIdColumn(Box<crate::datadogV1::model::TopologyMapInlineNodeIdColumn>),
    TopologyMapInlineNodeStringColumn(
        Box<crate::datadogV1::model::TopologyMapInlineNodeStringColumn>,
    ),
    TopologyMapInlineNodeValueColumn(
        Box<crate::datadogV1::model::TopologyMapInlineNodeValueColumn>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TopologyMapInlineNodeColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapInlineNodeIdColumn>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyMapInlineNodeColumn::TopologyMapInlineNodeIdColumn(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapInlineNodeStringColumn>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyMapInlineNodeColumn::TopologyMapInlineNodeStringColumn(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapInlineNodeValueColumn>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyMapInlineNodeColumn::TopologyMapInlineNodeValueColumn(_v));
            }
        }

        return Ok(TopologyMapInlineNodeColumn::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
