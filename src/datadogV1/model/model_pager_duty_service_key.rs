// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// PagerDuty service object key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PagerDutyServiceKey {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PagerDutyServiceKey {
    pub fn new(service_key: String) -> PagerDutyServiceKey {
        PagerDutyServiceKey {
            service_key,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for PagerDutyServiceKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PagerDutyServiceKeyVisitor;
        impl<'a> Visitor<'a> for PagerDutyServiceKeyVisitor {
            type Value = PagerDutyServiceKey;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut service_key: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "service_key" => {
                            service_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let service_key =
                    service_key.ok_or_else(|| M::Error::missing_field("service_key"))?;

                let content = PagerDutyServiceKey {
                    service_key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PagerDutyServiceKeyVisitor)
    }
}
