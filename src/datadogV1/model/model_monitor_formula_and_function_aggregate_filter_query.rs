// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Filter query for aggregate filtered queries. Can be an events query or a reference table query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MonitorFormulaAndFunctionAggregateFilterQuery {
    MonitorFormulaAndFunctionEventQueryDefinition(
        Box<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinition>,
    ),
    MonitorFormulaAndFunctionReferenceTableQueryDefinition(
        Box<crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableQueryDefinition>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionAggregateFilterQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MonitorFormulaAndFunctionAggregateFilterQuery::MonitorFormulaAndFunctionEventQueryDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MonitorFormulaAndFunctionAggregateFilterQuery::MonitorFormulaAndFunctionReferenceTableQueryDefinition(_v));
            }
        }

        return Ok(
            MonitorFormulaAndFunctionAggregateFilterQuery::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
