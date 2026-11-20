// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Platform-specific settings for a data quality monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualityPlatformSettings {
    /// Snowflake-specific settings for a data quality monitor.
    #[serde(rename = "snowflake")]
    pub snowflake: Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySnowflakeSettings>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl MonitorFormulaAndFunctionDataQualityPlatformSettings {
    pub fn new() -> MonitorFormulaAndFunctionDataQualityPlatformSettings {
        MonitorFormulaAndFunctionDataQualityPlatformSettings {
            snowflake: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn snowflake(mut self, value: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySnowflakeSettings) -> Self {
        self.snowflake = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for MonitorFormulaAndFunctionDataQualityPlatformSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualityPlatformSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualityPlatformSettingsVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualityPlatformSettingsVisitor {
            type Value = MonitorFormulaAndFunctionDataQualityPlatformSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut snowflake: Option<crate::datadogV1::model::MonitorFormulaAndFunctionDataQualitySnowflakeSettings> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "snowflake" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = MonitorFormulaAndFunctionDataQualityPlatformSettings {
                    snowflake,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionDataQualityPlatformSettingsVisitor)
    }
}
