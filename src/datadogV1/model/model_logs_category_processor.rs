// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use the Category Processor to add a new attribute (without spaces or special characters in the new attribute name)
/// to a log matching a provided search query. Use categories to create groups for an analytical view.
/// For example, URL groups, machine groups, environments, and response time buckets.
///
/// **Notes**:
///
/// - The syntax of the query is the one of Logs Explorer search bar.
///   The query can be done on any log attribute or tag, whether it is a facet or not.
///   Wildcards can also be used inside your query.
/// - Once the log has matched one of the Processor queries, it stops.
///   Make sure they are properly ordered in case a log could match several queries.
/// - The names of the categories must be unique.
/// - Once defined in the Category Processor, you can map categories to log status using the Log Status Remapper.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsCategoryProcessor {
    /// Array of filters to match or not a log and their
    /// corresponding `name` to assign a custom value to the log.
    #[serde(rename = "categories")]
    pub categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Name of the target attribute which value is defined by the matching category.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs category processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsCategoryProcessorType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsCategoryProcessor {
    pub fn new(
        categories: Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
        target: String,
        type_: crate::datadogV1::model::LogsCategoryProcessorType,
    ) -> LogsCategoryProcessor {
        LogsCategoryProcessor {
            categories,
            is_enabled: None,
            name: None,
            target,
            type_,
            _unparsed: false,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsCategoryProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsCategoryProcessorVisitor;
        impl<'a> Visitor<'a> for LogsCategoryProcessorVisitor {
            type Value = LogsCategoryProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut categories: Option<
                    Vec<crate::datadogV1::model::LogsCategoryProcessorCategory>,
                > = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsCategoryProcessorType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "categories" => {
                            categories = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsCategoryProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let categories = categories.ok_or_else(|| M::Error::missing_field("categories"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsCategoryProcessor {
                    categories,
                    is_enabled,
                    name,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsCategoryProcessorVisitor)
    }
}
