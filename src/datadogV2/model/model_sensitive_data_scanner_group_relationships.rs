// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerGroupRelationships {
    /// A Sensitive Data Scanner configuration data.
    #[serde(rename = "configuration")]
    pub configuration: Option<crate::datadogV2::model::SensitiveDataScannerConfigurationData>,
    /// Rules included in the group.
    #[serde(rename = "rules")]
    pub rules: Option<crate::datadogV2::model::SensitiveDataScannerRuleData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerGroupRelationships {
    pub fn new() -> SensitiveDataScannerGroupRelationships {
        SensitiveDataScannerGroupRelationships {
            configuration: None,
            rules: None,
            _unparsed: false,
        }
    }

    pub fn configuration(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationData,
    ) -> Self {
        self.configuration = Some(value);
        self
    }

    pub fn rules(mut self, value: crate::datadogV2::model::SensitiveDataScannerRuleData) -> Self {
        self.rules = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerGroupRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerGroupRelationshipsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerGroupRelationshipsVisitor {
            type Value = SensitiveDataScannerGroupRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration: Option<
                    crate::datadogV2::model::SensitiveDataScannerConfigurationData,
                > = None;
                let mut rules: Option<crate::datadogV2::model::SensitiveDataScannerRuleData> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules" => {
                            if v.is_null() {
                                continue;
                            }
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerGroupRelationships {
                    configuration,
                    rules,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerGroupRelationshipsVisitor)
    }
}
