// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeseriesFormulaRequestType {
    #[serde(rename = "timeseries_request")]
	TIMESERIES_REQUEST,
}

impl ToString for TimeseriesFormulaRequestType {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESERIES_REQUEST => String::from("timeseries_request"),
        }
    }
}

impl Default for TimeseriesFormulaRequestType {
    fn default() -> TimeseriesFormulaRequestType {
        Self::TIMESERIES_REQUEST
    }
}
