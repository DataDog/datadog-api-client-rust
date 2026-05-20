// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Support hours during which the escalation policy will be executed. Outside of these hours, the escalation policy will be on hold and triggered once the next support hours window starts. This is mutually exclusive with the top-level `time_restriction` field on the routing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoutingRuleEscalationPolicyActionSupportHours {
    /// The list of support hours time windows.
    #[serde(rename = "restrictions")]
    pub restrictions: Option<Vec<crate::datadogV2::model::TimeRestriction>>,
    /// The time zone in which the support hours are expressed.
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoutingRuleEscalationPolicyActionSupportHours {
    pub fn new(time_zone: String) -> RoutingRuleEscalationPolicyActionSupportHours {
        RoutingRuleEscalationPolicyActionSupportHours {
            restrictions: None,
            time_zone,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn restrictions(mut self, value: Vec<crate::datadogV2::model::TimeRestriction>) -> Self {
        self.restrictions = Some(value);
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

impl<'de> Deserialize<'de> for RoutingRuleEscalationPolicyActionSupportHours {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoutingRuleEscalationPolicyActionSupportHoursVisitor;
        impl<'a> Visitor<'a> for RoutingRuleEscalationPolicyActionSupportHoursVisitor {
            type Value = RoutingRuleEscalationPolicyActionSupportHours;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut restrictions: Option<Vec<crate::datadogV2::model::TimeRestriction>> = None;
                let mut time_zone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "restrictions" => {
                            if v.is_null() {
                                continue;
                            }
                            restrictions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_zone" => {
                            time_zone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let time_zone = time_zone.ok_or_else(|| M::Error::missing_field("time_zone"))?;

                let content = RoutingRuleEscalationPolicyActionSupportHours {
                    restrictions,
                    time_zone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RoutingRuleEscalationPolicyActionSupportHoursVisitor)
    }
}
