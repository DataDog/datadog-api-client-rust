// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata field key and values to set on the experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems {
    /// Selected values for an enumerated metadata field.
    #[serde(rename = "enum_values")]
    pub enum_values: Option<Vec<String>>,
    /// Key that identifies the metadata field.
    #[serde(rename = "field_key")]
    pub field_key: String,
    /// Text value for a free-text metadata field.
    #[serde(rename = "freetext_value")]
    pub freetext_value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems {
    pub fn new(
        field_key: String,
    ) -> ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems {
        ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems {
            enum_values: None,
            field_key,
            freetext_value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enum_values(mut self, value: Vec<String>) -> Self {
        self.enum_values = Some(value);
        self
    }

    pub fn freetext_value(mut self, value: String) -> Self {
        self.freetext_value = Some(value);
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

impl<'de> Deserialize<'de>
    for ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItemsVisitor
        {
            type Value = ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enum_values: Option<Vec<String>> = None;
                let mut field_key: Option<String> = None;
                let mut freetext_value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enum_values" => {
                            if v.is_null() {
                                continue;
                            }
                            enum_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field_key" => {
                            field_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "freetext_value" => {
                            if v.is_null() {
                                continue;
                            }
                            freetext_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let field_key = field_key.ok_or_else(|| M::Error::missing_field("field_key"))?;

                let content =
                    ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems {
                        enum_values,
                        field_key,
                        freetext_value,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItemsVisitor,
        )
    }
}
