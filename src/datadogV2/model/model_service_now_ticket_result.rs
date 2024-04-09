// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// ServiceNow ticket information
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceNowTicketResult {
    /// Link to the Incident created on ServiceNow
    #[serde(rename = "sys_target_link")]
    pub sys_target_link: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceNowTicketResult {
    pub fn new() -> ServiceNowTicketResult {
        ServiceNowTicketResult {
            sys_target_link: None,
            _unparsed: false,
        }
    }

    pub fn sys_target_link(mut self, value: String) -> Self {
        self.sys_target_link = Some(value);
        self
    }
}

impl Default for ServiceNowTicketResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceNowTicketResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceNowTicketResultVisitor;
        impl<'a> Visitor<'a> for ServiceNowTicketResultVisitor {
            type Value = ServiceNowTicketResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut sys_target_link: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "sys_target_link" => {
                            if v.is_null() {
                                continue;
                            }
                            sys_target_link =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ServiceNowTicketResult {
                    sys_target_link,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceNowTicketResultVisitor)
    }
}
