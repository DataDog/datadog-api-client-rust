// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing a single dataset object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetResponseSingle {
    /// **Datasets Object Constraints**
    /// - **Tag Limit per Dataset**:
    ///   - Each restricted dataset supports a maximum of 10 key:value pairs per product.
    ///
    /// - **Tag Key Rules per Telemetry Type**:
    ///   - Only one tag key or attribute may be used to define access within a single telemetry type.
    ///   - The same or different tag key may be used across different telemetry types.
    ///
    /// - **Tag Value Uniqueness**:
    ///   - Tag values must be unique within a single dataset.
    ///   - A tag value used in one dataset cannot be reused in another dataset of the same telemetry type.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::DatasetResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetResponseSingle {
    pub fn new() -> DatasetResponseSingle {
        DatasetResponseSingle {
            data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::DatasetResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for DatasetResponseSingle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DatasetResponseSingle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetResponseSingleVisitor;
        impl<'a> Visitor<'a> for DatasetResponseSingleVisitor {
            type Value = DatasetResponseSingle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::DatasetResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DatasetResponseSingle {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetResponseSingleVisitor)
    }
}
