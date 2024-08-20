// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
/// to be used because this will sum up all request counts instead of averaging them, or taking the max or
/// min of all of those requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceLevelObjectiveQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator")]
    pub denominator: String,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator")]
    pub numerator: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceLevelObjectiveQuery {
    pub fn new(denominator: String, numerator: String) -> ServiceLevelObjectiveQuery {
        ServiceLevelObjectiveQuery {
            denominator,
            numerator,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ServiceLevelObjectiveQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceLevelObjectiveQueryVisitor;
        impl<'a> Visitor<'a> for ServiceLevelObjectiveQueryVisitor {
            type Value = ServiceLevelObjectiveQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denominator: Option<String> = None;
                let mut numerator: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denominator" => {
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let denominator =
                    denominator.ok_or_else(|| M::Error::missing_field("denominator"))?;
                let numerator = numerator.ok_or_else(|| M::Error::missing_field("numerator"))?;

                let content = ServiceLevelObjectiveQuery {
                    denominator,
                    numerator,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceLevelObjectiveQueryVisitor)
    }
}
