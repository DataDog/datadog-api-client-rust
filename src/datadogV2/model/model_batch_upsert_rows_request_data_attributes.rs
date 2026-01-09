// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes containing row data values for row creation or update operations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BatchUpsertRowsRequestDataAttributes {
    /// Key-value pairs representing row data, where keys are schema field names and values match the corresponding column types.
    #[serde(rename = "values")]
    pub values: std::collections::BTreeMap<
        String,
        crate::datadogV2::model::BatchUpsertRowsRequestDataAttributesValue,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BatchUpsertRowsRequestDataAttributes {
    pub fn new(
        values: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::BatchUpsertRowsRequestDataAttributesValue,
        >,
    ) -> BatchUpsertRowsRequestDataAttributes {
        BatchUpsertRowsRequestDataAttributes {
            values,
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

impl<'de> Deserialize<'de> for BatchUpsertRowsRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BatchUpsertRowsRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for BatchUpsertRowsRequestDataAttributesVisitor {
            type Value = BatchUpsertRowsRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut values: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::BatchUpsertRowsRequestDataAttributesValue,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = BatchUpsertRowsRequestDataAttributes {
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BatchUpsertRowsRequestDataAttributesVisitor)
    }
}
