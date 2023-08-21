// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScatterplotDimension {
    #[serde(rename = "x")]
	X,
    #[serde(rename = "y")]
	Y,
    #[serde(rename = "radius")]
	RADIUS,
    #[serde(rename = "color")]
	COLOR,
}

impl ToString for ScatterplotDimension {
    fn to_string(&self) -> String {
        match self {
            Self::X => String::from("x"),
            Self::Y => String::from("y"),
            Self::RADIUS => String::from("radius"),
            Self::COLOR => String::from("color"),
        }
    }
}

impl Default for ScatterplotDimension {
    fn default() -> ScatterplotDimension {
        Self::X
    }
}
