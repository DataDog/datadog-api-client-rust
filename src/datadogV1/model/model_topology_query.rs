// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A topology data source query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TopologyQuery {
    TopologyQueryDataStreamsOrServiceMap(
        Box<crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMap>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TopologyQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyQueryDataStreamsOrServiceMap>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TopologyQuery::TopologyQueryDataStreamsOrServiceMap(_v));
            }
        }

        return Ok(TopologyQuery::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
