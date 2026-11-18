// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Selects users or accounts with a segment template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentTemplateBlock {
    /// Start of the time range, in epoch milliseconds.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// Name of the block, used in `combination`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Template parameter values, keyed by parameter name.
    #[serde(rename = "parameters")]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
    /// ID of the segment template, such as `time-spent-v1`.
    #[serde(rename = "template_id")]
    pub template_id: String,
    /// End of the time range, in epoch milliseconds.
    #[serde(rename = "to")]
    pub to: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentTemplateBlock {
    pub fn new(
        parameters: std::collections::BTreeMap<String, serde_json::Value>,
        template_id: String,
    ) -> SegmentTemplateBlock {
        SegmentTemplateBlock {
            from: None,
            name: None,
            parameters,
            template_id,
            to: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
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

impl<'de> Deserialize<'de> for SegmentTemplateBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentTemplateBlockVisitor;
        impl<'a> Visitor<'a> for SegmentTemplateBlockVisitor {
            type Value = SegmentTemplateBlock;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<String> = None;
                let mut name: Option<String> = None;
                let mut parameters: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut template_id: Option<String> = None;
                let mut to: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parameters" => {
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_id" => {
                            template_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let parameters = parameters.ok_or_else(|| M::Error::missing_field("parameters"))?;
                let template_id =
                    template_id.ok_or_else(|| M::Error::missing_field("template_id"))?;

                let content = SegmentTemplateBlock {
                    from,
                    name,
                    parameters,
                    template_id,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentTemplateBlockVisitor)
    }
}
