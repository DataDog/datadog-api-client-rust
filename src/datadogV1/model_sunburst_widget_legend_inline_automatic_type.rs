// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SunburstWidgetLegendInlineAutomaticType {
    #[serde(rename = "inline")]
	INLINE,
    #[serde(rename = "automatic")]
	AUTOMATIC,
}

impl ToString for SunburstWidgetLegendInlineAutomaticType {
    fn to_string(&self) -> String {
        match self {
            Self::INLINE => String::from("inline"),
            Self::AUTOMATIC => String::from("automatic"),
        }
    }
}

impl Default for SunburstWidgetLegendInlineAutomaticType {
    fn default() -> SunburstWidgetLegendInlineAutomaticType {
        Self::INLINE
    }
}
