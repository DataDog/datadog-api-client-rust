// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Azure archive's integration destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveIntegrationAzure {
    /// A client ID.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// A tenant ID.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveIntegrationAzure {
    pub fn new(client_id: String, tenant_id: String) -> LogsArchiveIntegrationAzure {
        LogsArchiveIntegrationAzure {
            client_id,
            tenant_id,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LogsArchiveIntegrationAzure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveIntegrationAzureVisitor;
        impl<'a> Visitor<'a> for LogsArchiveIntegrationAzureVisitor {
            type Value = LogsArchiveIntegrationAzure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = LogsArchiveIntegrationAzure {
                    client_id,
                    tenant_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveIntegrationAzureVisitor)
    }
}
