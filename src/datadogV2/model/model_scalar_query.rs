// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An individual scalar query to one of the basic Datadog data sources.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScalarQuery {
    MetricsScalarQuery(Box<crate::datadogV2::model::MetricsScalarQuery>),
    EventsScalarQuery(Box<crate::datadogV2::model::EventsScalarQuery>),
    ApmResourceStatsQuery(Box<crate::datadogV2::model::ApmResourceStatsQuery>),
    ApmMetricsQuery(Box<crate::datadogV2::model::ApmMetricsQuery>),
    ApmDependencyStatsQuery(Box<crate::datadogV2::model::ApmDependencyStatsQuery>),
    SloQuery(Box<crate::datadogV2::model::SloQuery>),
    ProcessScalarQuery(Box<crate::datadogV2::model::ProcessScalarQuery>),
    ContainerScalarQuery(Box<crate::datadogV2::model::ContainerScalarQuery>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScalarQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricsScalarQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ScalarQuery::MetricsScalarQuery(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::EventsScalarQuery>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarQuery::EventsScalarQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ApmResourceStatsQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ScalarQuery::ApmResourceStatsQuery(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ApmMetricsQuery>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarQuery::ApmMetricsQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ApmDependencyStatsQuery>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarQuery::ApmDependencyStatsQuery(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::SloQuery>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarQuery::SloQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ProcessScalarQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ScalarQuery::ProcessScalarQuery(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ContainerScalarQuery>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ScalarQuery::ContainerScalarQuery(_v));
            }
        }

        return Ok(ScalarQuery::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
