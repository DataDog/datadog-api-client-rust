// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The S3 Archive's integration destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArchiveIntegrationS3 {
    /// The account ID for the integration.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The path of the integration.
    #[serde(rename = "role_name")]
    pub role_name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArchiveIntegrationS3 {
    pub fn new(account_id: String, role_name: String) -> LogsArchiveIntegrationS3 {
        LogsArchiveIntegrationS3 {
            account_id,
            role_name,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LogsArchiveIntegrationS3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArchiveIntegrationS3Visitor;
        impl<'a> Visitor<'a> for LogsArchiveIntegrationS3Visitor {
            type Value = LogsArchiveIntegrationS3;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut role_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role_name" => {
                            role_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let role_name = role_name.ok_or_else(|| M::Error::missing_field("role_name"))?;

                let content = LogsArchiveIntegrationS3 {
                    account_id,
                    role_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArchiveIntegrationS3Visitor)
    }
}
