// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeseriesWidgetLegendLayout {
    #[serde(rename = "auto")]
	AUTO,
    #[serde(rename = "horizontal")]
	HORIZONTAL,
    #[serde(rename = "vertical")]
	VERTICAL,
}

impl ToString for TimeseriesWidgetLegendLayout {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::HORIZONTAL => String::from("horizontal"),
            Self::VERTICAL => String::from("vertical"),
        }
    }
}

impl Default for TimeseriesWidgetLegendLayout {
    fn default() -> TimeseriesWidgetLegendLayout {
        Self::AUTO
    }
}
