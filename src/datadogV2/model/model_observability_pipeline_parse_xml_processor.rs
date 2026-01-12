// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `parse_xml` processor parses XML from a specified field and extracts it into the event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineParseXMLProcessor {
    /// Whether to always use a text key for element content.
    #[serde(rename = "always_use_text_key")]
    pub always_use_text_key: Option<bool>,
    /// The prefix to use for XML attributes in the parsed output.
    #[serde(rename = "attr_prefix")]
    pub attr_prefix: Option<String>,
    /// The display name for a component.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Whether this processor is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The name of the log field that contains an XML string.
    #[serde(rename = "field")]
    pub field: String,
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// Whether to include XML attributes in the parsed output.
    #[serde(rename = "include_attr")]
    pub include_attr: Option<bool>,
    /// Whether to parse boolean values from strings.
    #[serde(rename = "parse_bool")]
    pub parse_bool: Option<bool>,
    /// Whether to parse null values.
    #[serde(rename = "parse_null")]
    pub parse_null: Option<bool>,
    /// Whether to parse numeric values from strings.
    #[serde(rename = "parse_number")]
    pub parse_number: Option<bool>,
    /// The key name to use for text content within XML elements. Must be at least 1 character if specified.
    #[serde(rename = "text_key")]
    pub text_key: Option<String>,
    /// The processor type. The value should always be `parse_xml`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineParseXMLProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineParseXMLProcessor {
    pub fn new(
        enabled: bool,
        field: String,
        id: String,
        include: String,
        type_: crate::datadogV2::model::ObservabilityPipelineParseXMLProcessorType,
    ) -> ObservabilityPipelineParseXMLProcessor {
        ObservabilityPipelineParseXMLProcessor {
            always_use_text_key: None,
            attr_prefix: None,
            display_name: None,
            enabled,
            field,
            id,
            include,
            include_attr: None,
            parse_bool: None,
            parse_null: None,
            parse_number: None,
            text_key: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn always_use_text_key(mut self, value: bool) -> Self {
        self.always_use_text_key = Some(value);
        self
    }

    pub fn attr_prefix(mut self, value: String) -> Self {
        self.attr_prefix = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn include_attr(mut self, value: bool) -> Self {
        self.include_attr = Some(value);
        self
    }

    pub fn parse_bool(mut self, value: bool) -> Self {
        self.parse_bool = Some(value);
        self
    }

    pub fn parse_null(mut self, value: bool) -> Self {
        self.parse_null = Some(value);
        self
    }

    pub fn parse_number(mut self, value: bool) -> Self {
        self.parse_number = Some(value);
        self
    }

    pub fn text_key(mut self, value: String) -> Self {
        self.text_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineParseXMLProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineParseXMLProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineParseXMLProcessorVisitor {
            type Value = ObservabilityPipelineParseXMLProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut always_use_text_key: Option<bool> = None;
                let mut attr_prefix: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut field: Option<String> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut include_attr: Option<bool> = None;
                let mut parse_bool: Option<bool> = None;
                let mut parse_null: Option<bool> = None;
                let mut parse_number: Option<bool> = None;
                let mut text_key: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineParseXMLProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "always_use_text_key" => {
                            if v.is_null() {
                                continue;
                            }
                            always_use_text_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attr_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            attr_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_attr" => {
                            if v.is_null() {
                                continue;
                            }
                            include_attr =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parse_bool" => {
                            if v.is_null() {
                                continue;
                            }
                            parse_bool = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parse_null" => {
                            if v.is_null() {
                                continue;
                            }
                            parse_null = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parse_number" => {
                            if v.is_null() {
                                continue;
                            }
                            parse_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_key" => {
                            if v.is_null() {
                                continue;
                            }
                            text_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineParseXMLProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineParseXMLProcessor {
                    always_use_text_key,
                    attr_prefix,
                    display_name,
                    enabled,
                    field,
                    id,
                    include,
                    include_attr,
                    parse_bool,
                    parse_null,
                    parse_number,
                    text_key,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineParseXMLProcessorVisitor)
    }
}
