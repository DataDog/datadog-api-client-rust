// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// The Datadog side of a model's zero data retention eligibility. The customer side is the provider's
/// `zdr_eligible` setting.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelZdr {
    /// Datadog's coverage of the model's operations under zero data retention. A model is eligible when
/// coverage is `full` and its provider's `zdr_eligible` is `true`. `partial` means only the
/// `covered_operations` can be served: requests on other operations are not dispatched to it.
    #[serde(rename = "coverage")]
    pub coverage: crate::datadogV2::model::AiGatewayModelZdrCoverage,
    /// Operations covered by zero data retention.
    #[serde(rename = "covered_operations")]
    pub covered_operations: Option<Vec<String>>,
    /// Operations not covered by zero data retention.
    #[serde(rename = "uncovered_operations")]
    pub uncovered_operations: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayModelZdr {
    pub fn new(coverage: crate::datadogV2::model::AiGatewayModelZdrCoverage) -> AiGatewayModelZdr {
        AiGatewayModelZdr {
            coverage,
            covered_operations: None,
            uncovered_operations: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn covered_operations(mut self, value: Vec<String>) -> Self {
        self.covered_operations = Some(value);
        self
    }
    
    pub fn uncovered_operations(mut self, value: Vec<String>) -> Self {
        self.uncovered_operations = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayModelZdr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelZdrVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelZdrVisitor {
            type Value = AiGatewayModelZdr;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut coverage: Option<crate::datadogV2::model::AiGatewayModelZdrCoverage> = None;
                let mut covered_operations: Option<Vec<String>> = None;
                let mut uncovered_operations: Option<Vec<String>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "coverage" => {
                            coverage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _coverage) = coverage {
                                match _coverage {
                                    crate::datadogV2::model::AiGatewayModelZdrCoverage::UnparsedObject(_coverage) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "covered_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            covered_operations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "uncovered_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            uncovered_operations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let coverage = coverage.ok_or_else(|| M::Error::missing_field("coverage"))?;

                let content = AiGatewayModelZdr {
                    coverage,
                    covered_operations,
                    uncovered_operations,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelZdrVisitor)
    }
}
