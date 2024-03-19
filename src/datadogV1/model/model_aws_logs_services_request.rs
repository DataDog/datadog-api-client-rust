// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of current AWS services for which Datadog offers automatic log collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsServicesRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Array of services IDs set to enable automatic log collection. Discover the list of available services with the get list of AWS log ready services API endpoint.
    #[serde(rename = "services")]
    pub services: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsServicesRequest {
    pub fn new(account_id: String, services: Vec<String>) -> AWSLogsServicesRequest {
        AWSLogsServicesRequest {
            account_id,
            services,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AWSLogsServicesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsServicesRequestVisitor;
        impl<'a> Visitor<'a> for AWSLogsServicesRequestVisitor {
            type Value = AWSLogsServicesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let services = services.ok_or_else(|| M::Error::missing_field("services"))?;

                let content = AWSLogsServicesRequest {
                    account_id,
                    services,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsServicesRequestVisitor)
    }
}
