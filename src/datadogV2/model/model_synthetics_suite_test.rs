// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing details about a Synthetic test included in a Synthetic suite.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsSuiteTest {
    /// Alerting criticality for each the test.
    #[serde(rename = "alerting_criticality")]
    pub alerting_criticality:
        Option<crate::datadogV2::model::SyntheticsSuiteTestAlertingCriticality>,
    #[serde(rename = "public_id")]
    pub public_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsSuiteTest {
    pub fn new(public_id: String) -> SyntheticsSuiteTest {
        SyntheticsSuiteTest {
            alerting_criticality: None,
            public_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alerting_criticality(
        mut self,
        value: crate::datadogV2::model::SyntheticsSuiteTestAlertingCriticality,
    ) -> Self {
        self.alerting_criticality = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsSuiteTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsSuiteTestVisitor;
        impl<'a> Visitor<'a> for SyntheticsSuiteTestVisitor {
            type Value = SyntheticsSuiteTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alerting_criticality: Option<
                    crate::datadogV2::model::SyntheticsSuiteTestAlertingCriticality,
                > = None;
                let mut public_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alerting_criticality" => {
                            if v.is_null() {
                                continue;
                            }
                            alerting_criticality =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _alerting_criticality) = alerting_criticality {
                                match _alerting_criticality {
                                    crate::datadogV2::model::SyntheticsSuiteTestAlertingCriticality::UnparsedObject(_alerting_criticality) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "public_id" => {
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let public_id = public_id.ok_or_else(|| M::Error::missing_field("public_id"))?;

                let content = SyntheticsSuiteTest {
                    alerting_criticality,
                    public_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsSuiteTestVisitor)
    }
}
