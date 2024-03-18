// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The PagerDuty service that is available for integration with Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PagerDutyService {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
    /// Your service name associated with a service key in PagerDuty.
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PagerDutyService {
    pub fn new(service_key: String, service_name: String) -> PagerDutyService {
        PagerDutyService {
            service_key,
            service_name,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for PagerDutyService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PagerDutyServiceVisitor;
        impl<'a> Visitor<'a> for PagerDutyServiceVisitor {
            type Value = PagerDutyService;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut service_key: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "service_key" => {
                            service_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let service_key =
                    service_key.ok_or_else(|| M::Error::missing_field("service_key"))?;
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;

                let content = PagerDutyService {
                    service_key,
                    service_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PagerDutyServiceVisitor)
    }
}
