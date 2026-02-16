// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataTransformationDescriptionResponse {
    /// Detailed description of the transformation.
    #[serde(rename = "details")]
    pub details: String,
    /// The generation ID.
    #[serde(rename = "id")]
    pub id: String,
    /// A brief summary of the transformation.
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataTransformationDescriptionResponse {
    pub fn new(
        details: String,
        id: String,
        summary: String,
    ) -> DataTransformationDescriptionResponse {
        DataTransformationDescriptionResponse {
            details,
            id,
            summary,
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

impl<'de> Deserialize<'de> for DataTransformationDescriptionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataTransformationDescriptionResponseVisitor;
        impl<'a> Visitor<'a> for DataTransformationDescriptionResponseVisitor {
            type Value = DataTransformationDescriptionResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut details: Option<String> = None;
                let mut id: Option<String> = None;
                let mut summary: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "details" => {
                            details = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let details = details.ok_or_else(|| M::Error::missing_field("details"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let summary = summary.ok_or_else(|| M::Error::missing_field("summary"))?;

                let content = DataTransformationDescriptionResponse {
                    details,
                    id,
                    summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataTransformationDescriptionResponseVisitor)
    }
}
