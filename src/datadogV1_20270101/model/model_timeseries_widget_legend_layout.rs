// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeseriesWidgetLegendLayout {
	VERTICAL,
	HORIZONTAL,
	AUTO,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TimeseriesWidgetLegendLayout {
    fn to_string(&self) -> String {
        match self {
            Self::VERTICAL => String::from("vertical"),
            Self::HORIZONTAL => String::from("horizontal"),
            Self::AUTO => String::from("auto"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TimeseriesWidgetLegendLayout {
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

impl<'de> Deserialize<'de> for TimeseriesWidgetLegendLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"vertical" => Self::VERTICAL,"horizontal" => Self::HORIZONTAL,"auto" => Self::AUTO,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
