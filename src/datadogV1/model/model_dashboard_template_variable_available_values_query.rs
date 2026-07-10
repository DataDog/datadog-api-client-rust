// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A query that dynamically computes the list of values available for this template variable.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DashboardTemplateVariableAvailableValuesQuery {
    DashboardAvailableValuesEventsQuery(
        Box<crate::datadogV1::model::DashboardAvailableValuesEventsQuery>,
    ),
    DashboardAvailableValuesMetricsQuery(
        Box<crate::datadogV1::model::DashboardAvailableValuesMetricsQuery>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DashboardTemplateVariableAvailableValuesQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::DashboardAvailableValuesEventsQuery>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DashboardTemplateVariableAvailableValuesQuery::DashboardAvailableValuesEventsQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::DashboardAvailableValuesMetricsQuery>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DashboardTemplateVariableAvailableValuesQuery::DashboardAvailableValuesMetricsQuery(_v));
            }
        }

        return Ok(
            DashboardTemplateVariableAvailableValuesQuery::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
