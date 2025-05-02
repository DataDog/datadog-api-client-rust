// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Holds time zone information and a list of time restrictions for a routing rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeRestrictions {
    /// Defines the list of time-based restrictions.
    #[serde(rename = "restrictions")]
    pub restrictions: Vec<crate::datadogV2::model::TimeRestriction>,
    /// Specifies the time zone applicable to the restrictions.
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeRestrictions {
    pub fn new(
        restrictions: Vec<crate::datadogV2::model::TimeRestriction>,
        time_zone: String,
    ) -> TimeRestrictions {
        TimeRestrictions {
            restrictions,
            time_zone,
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

impl<'de> Deserialize<'de> for TimeRestrictions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeRestrictionsVisitor;
        impl<'a> Visitor<'a> for TimeRestrictionsVisitor {
            type Value = TimeRestrictions;

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
                let restrictions =
                    restrictions.ok_or_else(|| M::Error::missing_field("restrictions"))?;
                let time_zone = time_zone.ok_or_else(|| M::Error::missing_field("time_zone"))?;

                let content = TimeRestrictions {
                    restrictions,
                    time_zone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeRestrictionsVisitor)
    }
}
