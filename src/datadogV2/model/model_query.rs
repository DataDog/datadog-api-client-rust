// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A data query used by an app. This can take the form of an external action, a data transformation, or a state variable.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Query {
    ActionQuery(Box<crate::datadogV2::model::ActionQuery>),
    DataTransform(Box<crate::datadogV2::model::DataTransform>),
    StateVariable(Box<crate::datadogV2::model::StateVariable>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for Query {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ActionQuery>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(Query::ActionQuery(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::DataTransform>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(Query::DataTransform(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::StateVariable>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(Query::StateVariable(_v));
            }
        }

        return Ok(Query::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
