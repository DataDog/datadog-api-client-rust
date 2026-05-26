// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a dataset version history response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetVersionHistoryAttributes {
    /// The total number of versions available for this dataset.
    #[serde(rename = "count")]
    pub count: i64,
    /// A map from version number (as a string) to the dataset state at that version.
    #[serde(rename = "data")]
    pub data: std::collections::BTreeMap<
        String,
        crate::datadogV2::model::SecurityMonitoringDatasetVersionEntry,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetVersionHistoryAttributes {
    pub fn new(
        count: i64,
        data: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::SecurityMonitoringDatasetVersionEntry,
        >,
    ) -> SecurityMonitoringDatasetVersionHistoryAttributes {
        SecurityMonitoringDatasetVersionHistoryAttributes {
            count,
            data,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetVersionHistoryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetVersionHistoryAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetVersionHistoryAttributesVisitor {
            type Value = SecurityMonitoringDatasetVersionHistoryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut data: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::SecurityMonitoringDatasetVersionEntry,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = SecurityMonitoringDatasetVersionHistoryAttributes {
                    count,
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetVersionHistoryAttributesVisitor)
    }
}
