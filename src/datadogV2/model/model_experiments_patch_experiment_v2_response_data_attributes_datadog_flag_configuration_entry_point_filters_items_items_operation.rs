// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation
{
    EQ,
    IN,
    NEQ,
    NOT_IN,
    GTE,
    LTE,
    GT,
    LT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation {
    fn to_string(&self) -> String {
        match self {
            Self::EQ => String::from("eq"),
            Self::IN => String::from("in"),
            Self::NEQ => String::from("neq"),
            Self::NOT_IN => String::from("not_in"),
            Self::GTE => String::from("gte"),
            Self::LTE => String::from("lte"),
            Self::GT => String::from("gt"),
            Self::LT => String::from("lt"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation {
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

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"eq" => Self::EQ,"in" => Self::IN,"neq" => Self::NEQ,"not_in" => Self::NOT_IN,"gte" => Self::GTE,"lte" => Self::LTE,"gt" => Self::GT,"lt" => Self::LT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
