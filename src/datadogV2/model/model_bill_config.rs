// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bill config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillConfig {
    /// The name of the configured Azure Export.
    #[serde(rename = "export_name")]
    pub export_name: String,
    /// The path where the Azure Export is saved.
    #[serde(rename = "export_path")]
    pub export_path: String,
    /// The name of the storage account where the Azure Export is saved.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// The name of the storage container where the Azure Export is saved.
    #[serde(rename = "storage_container")]
    pub storage_container: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BillConfig {
    pub fn new(
        export_name: String,
        export_path: String,
        storage_account: String,
        storage_container: String,
    ) -> BillConfig {
        BillConfig {
            export_name,
            export_path,
            storage_account,
            storage_container,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for BillConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BillConfigVisitor;
        impl<'a> Visitor<'a> for BillConfigVisitor {
            type Value = BillConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut export_name: Option<String> = None;
                let mut export_path: Option<String> = None;
                let mut storage_account: Option<String> = None;
                let mut storage_container: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "export_name" => {
                            export_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_path" => {
                            export_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_account" => {
                            storage_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_container" => {
                            storage_container =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let export_name =
                    export_name.ok_or_else(|| M::Error::missing_field("export_name"))?;
                let export_path =
                    export_path.ok_or_else(|| M::Error::missing_field("export_path"))?;
                let storage_account =
                    storage_account.ok_or_else(|| M::Error::missing_field("storage_account"))?;
                let storage_container = storage_container
                    .ok_or_else(|| M::Error::missing_field("storage_container"))?;

                let content = BillConfig {
                    export_name,
                    export_path,
                    storage_account,
                    storage_container,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BillConfigVisitor)
    }
}
