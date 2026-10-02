// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationTargetingRulesItemsConditionsItemsOperator
{
    LT,
    LTE,
    GT,
    GTE,
    MATCHES,
    NOT_MATCHES,
    ONE_OF,
    NOT_ONE_OF,
    IS_NULL,
    EQUALS,
    SEMVER_EQ,
    SEMVER_NEQ,
    SEMVER_LT,
    SEMVER_LTE,
    SEMVER_GT,
    SEMVER_GTE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationTargetingRulesItemsConditionsItemsOperator {
    fn to_string(&self) -> String {
        match self {
            Self::LT => String::from("LT"),
            Self::LTE => String::from("LTE"),
            Self::GT => String::from("GT"),
            Self::GTE => String::from("GTE"),
            Self::MATCHES => String::from("MATCHES"),
            Self::NOT_MATCHES => String::from("NOT_MATCHES"),
            Self::ONE_OF => String::from("ONE_OF"),
            Self::NOT_ONE_OF => String::from("NOT_ONE_OF"),
            Self::IS_NULL => String::from("IS_NULL"),
            Self::EQUALS => String::from("EQUALS"),
            Self::SEMVER_EQ => String::from("SEMVER_EQ"),
            Self::SEMVER_NEQ => String::from("SEMVER_NEQ"),
            Self::SEMVER_LT => String::from("SEMVER_LT"),
            Self::SEMVER_LTE => String::from("SEMVER_LTE"),
            Self::SEMVER_GT => String::from("SEMVER_GT"),
            Self::SEMVER_GTE => String::from("SEMVER_GTE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationTargetingRulesItemsConditionsItemsOperator {
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

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationTargetingRulesItemsConditionsItemsOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"LT" => Self::LT,"LTE" => Self::LTE,"GT" => Self::GT,"GTE" => Self::GTE,"MATCHES" => Self::MATCHES,"NOT_MATCHES" => Self::NOT_MATCHES,"ONE_OF" => Self::ONE_OF,"NOT_ONE_OF" => Self::NOT_ONE_OF,"IS_NULL" => Self::IS_NULL,"EQUALS" => Self::EQUALS,"SEMVER_EQ" => Self::SEMVER_EQ,"SEMVER_NEQ" => Self::SEMVER_NEQ,"SEMVER_LT" => Self::SEMVER_LT,"SEMVER_LTE" => Self::SEMVER_LTE,"SEMVER_GT" => Self::SEMVER_GT,"SEMVER_GTE" => Self::SEMVER_GTE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
