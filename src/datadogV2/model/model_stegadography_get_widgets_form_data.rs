// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The form data submitted to look up widgets from a watermarked image.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StegadographyGetWidgetsFormData {
    /// A PNG image (for example, a dashboard screenshot) containing embedded widget watermarks.
    #[serde(rename = "image")]
    pub image: Option<Vec<u8>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StegadographyGetWidgetsFormData {
    pub fn new() -> StegadographyGetWidgetsFormData {
        StegadographyGetWidgetsFormData {
            image: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn image(mut self, value: Vec<u8>) -> Self {
        self.image = Some(value);
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

impl Default for StegadographyGetWidgetsFormData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StegadographyGetWidgetsFormData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StegadographyGetWidgetsFormDataVisitor;
        impl<'a> Visitor<'a> for StegadographyGetWidgetsFormDataVisitor {
            type Value = StegadographyGetWidgetsFormData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut image: Option<Vec<u8>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "image" => {
                            if v.is_null() {
                                continue;
                            }
                            image = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = StegadographyGetWidgetsFormData {
                    image,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StegadographyGetWidgetsFormDataVisitor)
    }
}
