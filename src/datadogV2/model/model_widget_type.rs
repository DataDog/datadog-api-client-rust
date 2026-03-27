// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetType {
    BAR_CHART,
    CHANGE,
    CLOUD_COST_SUMMARY,
    COHORT,
    FUNNEL,
    GEOMAP,
    LIST_STREAM,
    QUERY_TABLE,
    QUERY_VALUE,
    RETENTION_CURVE,
    SANKEY,
    SUNBURST,
    TIMESERIES,
    TOPLIST,
    TREEMAP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for WidgetType {
    fn to_string(&self) -> String {
        match self {
            Self::BAR_CHART => String::from("bar_chart"),
            Self::CHANGE => String::from("change"),
            Self::CLOUD_COST_SUMMARY => String::from("cloud_cost_summary"),
            Self::COHORT => String::from("cohort"),
            Self::FUNNEL => String::from("funnel"),
            Self::GEOMAP => String::from("geomap"),
            Self::LIST_STREAM => String::from("list_stream"),
            Self::QUERY_TABLE => String::from("query_table"),
            Self::QUERY_VALUE => String::from("query_value"),
            Self::RETENTION_CURVE => String::from("retention_curve"),
            Self::SANKEY => String::from("sankey"),
            Self::SUNBURST => String::from("sunburst"),
            Self::TIMESERIES => String::from("timeseries"),
            Self::TOPLIST => String::from("toplist"),
            Self::TREEMAP => String::from("treemap"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "bar_chart" => Self::BAR_CHART,
            "change" => Self::CHANGE,
            "cloud_cost_summary" => Self::CLOUD_COST_SUMMARY,
            "cohort" => Self::COHORT,
            "funnel" => Self::FUNNEL,
            "geomap" => Self::GEOMAP,
            "list_stream" => Self::LIST_STREAM,
            "query_table" => Self::QUERY_TABLE,
            "query_value" => Self::QUERY_VALUE,
            "retention_curve" => Self::RETENTION_CURVE,
            "sankey" => Self::SANKEY,
            "sunburst" => Self::SUNBURST,
            "timeseries" => Self::TIMESERIES,
            "toplist" => Self::TOPLIST,
            "treemap" => Self::TREEMAP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
