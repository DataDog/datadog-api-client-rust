// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Payload to test a rule query with the expected result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleQueryPayload {
    /// Expected result of the test.
    #[serde(rename = "expectedResult")]
    pub expected_result: Option<bool>,
    /// Index of the query under test.
    #[serde(rename = "index")]
    pub index: Option<i64>,
    /// Payload used to test the rule query.
    #[serde(rename = "payload")]
    pub payload: Option<crate::datadogV2::model::SecurityMonitoringRuleQueryPayloadData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleQueryPayload {
    pub fn new() -> SecurityMonitoringRuleQueryPayload {
        SecurityMonitoringRuleQueryPayload {
            expected_result: None,
            index: None,
            payload: None,
            _unparsed: false,
        }
    }

    pub fn expected_result(mut self, value: bool) -> Self {
        self.expected_result = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn payload(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleQueryPayloadData,
    ) -> Self {
        self.payload = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleQueryPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleQueryPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleQueryPayloadVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleQueryPayloadVisitor {
            type Value = SecurityMonitoringRuleQueryPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expected_result: Option<bool> = None;
                let mut index: Option<i64> = None;
                let mut payload: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleQueryPayloadData,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expectedResult" => {
                            if v.is_null() {
                                continue;
                            }
                            expected_result =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            if v.is_null() {
                                continue;
                            }
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringRuleQueryPayload {
                    expected_result,
                    index,
                    payload,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleQueryPayloadVisitor)
    }
}
