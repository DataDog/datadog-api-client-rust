// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationSecurityWafCustomRuleConditionOperator {
    MATCH_REGEX,
    NOT_MATCH_REGEX,
    PHRASE_MATCH,
    NOT_PHRASE_MATCH,
    IS_XSS,
    IS_SQLI,
    EXACT_MATCH,
    NOT_EXACT_MATCH,
    IP_MATCH,
    NOT_IP_MATCH,
    CAPTURE_DATA,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ApplicationSecurityWafCustomRuleConditionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::MATCH_REGEX => String::from("match_regex"),
            Self::NOT_MATCH_REGEX => String::from("!match_regex"),
            Self::PHRASE_MATCH => String::from("phrase_match"),
            Self::NOT_PHRASE_MATCH => String::from("!phrase_match"),
            Self::IS_XSS => String::from("is_xss"),
            Self::IS_SQLI => String::from("is_sqli"),
            Self::EXACT_MATCH => String::from("exact_match"),
            Self::NOT_EXACT_MATCH => String::from("!exact_match"),
            Self::IP_MATCH => String::from("ip_match"),
            Self::NOT_IP_MATCH => String::from("!ip_match"),
            Self::CAPTURE_DATA => String::from("capture_data"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ApplicationSecurityWafCustomRuleConditionOperator {
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleConditionOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "match_regex" => Self::MATCH_REGEX,
            "!match_regex" => Self::NOT_MATCH_REGEX,
            "phrase_match" => Self::PHRASE_MATCH,
            "!phrase_match" => Self::NOT_PHRASE_MATCH,
            "is_xss" => Self::IS_XSS,
            "is_sqli" => Self::IS_SQLI,
            "exact_match" => Self::EXACT_MATCH,
            "!exact_match" => Self::NOT_EXACT_MATCH,
            "ip_match" => Self::IP_MATCH,
            "!ip_match" => Self::NOT_IP_MATCH,
            "capture_data" => Self::CAPTURE_DATA,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
