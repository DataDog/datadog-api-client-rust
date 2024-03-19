// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerConfigurationRelationships {
    /// List of groups, ordered.
    #[serde(rename = "groups")]
    pub groups: Option<crate::datadogV2::model::SensitiveDataScannerGroupList>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerConfigurationRelationships {
    pub fn new() -> SensitiveDataScannerConfigurationRelationships {
        SensitiveDataScannerConfigurationRelationships {
            groups: None,
            _unparsed: false,
        }
    }

    pub fn groups(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupList) -> Self {
        self.groups = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerConfigurationRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerConfigurationRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerConfigurationRelationshipsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerConfigurationRelationshipsVisitor {
            type Value = SensitiveDataScannerConfigurationRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut groups: Option<crate::datadogV2::model::SensitiveDataScannerGroupList> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerConfigurationRelationships { groups, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerConfigurationRelationshipsVisitor)
    }
}
