// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// PagerDuty service object name.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PagerDutyServiceName {
    /// Your service name associated service key in PagerDuty.
    #[serde(rename = "service_name")]
    pub service_name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PagerDutyServiceName {
    pub fn new(service_name: String) -> PagerDutyServiceName {
        PagerDutyServiceName {
            service_name,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for PagerDutyServiceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PagerDutyServiceNameVisitor;
        impl<'a> Visitor<'a> for PagerDutyServiceNameVisitor {
            type Value = PagerDutyServiceName;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut service_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "service_name" => {
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let service_name =
                    service_name.ok_or_else(|| M::Error::missing_field("service_name"))?;

                let content = PagerDutyServiceName {
                    service_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PagerDutyServiceNameVisitor)
    }
}
