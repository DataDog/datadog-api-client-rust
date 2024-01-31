// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// The original widget we are splitting on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SplitGraphSourceWidgetDefinition {
    ChangeWidgetDefinition(crate::datadogV1::model::ChangeWidgetDefinition),
    GeomapWidgetDefinition(crate::datadogV1::model::GeomapWidgetDefinition),
    QueryValueWidgetDefinition(crate::datadogV1::model::QueryValueWidgetDefinition),
    ScatterPlotWidgetDefinition(crate::datadogV1::model::ScatterPlotWidgetDefinition),
    SunburstWidgetDefinition(crate::datadogV1::model::SunburstWidgetDefinition),
    TableWidgetDefinition(crate::datadogV1::model::TableWidgetDefinition),
    TimeseriesWidgetDefinition(crate::datadogV1::model::TimeseriesWidgetDefinition),
    ToplistWidgetDefinition(crate::datadogV1::model::ToplistWidgetDefinition),
    TreeMapWidgetDefinition(crate::datadogV1::model::TreeMapWidgetDefinition),
}
