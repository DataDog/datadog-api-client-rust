// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use the Schema Category Mapper to categorize log event into enum fields.
/// In the case of OCSF, they can be used to map sibling fields which are composed of an ID and a name.
///
/// **Notes**:
///
/// - The syntax of the query is the one of Logs Explorer search bar.
///   The query can be done on any log attribute or tag, whether it is a facet or not.
///   Wildcards can also be used inside your query.
/// - Categories are executed in order and processing stops at the first match.
///   Make sure categories are properly ordered in case a log could match multiple queries.
/// - Sibling fields always have a numerical ID field and a human-readable string name.
/// - A fallback section handles cases where the name or ID value matches a specific value.
///   If the name matches "Other" or the ID matches 99, the value of the sibling name field will be pulled from a source field from the original log.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsSchemaCategoryMapper {
    /// Array of filters to match or not a log and their
    /// corresponding `name` to assign a custom value to the log.
    #[serde(rename = "categories")]
    pub categories: Vec<crate::datadogV1::model::LogsSchemaCategoryMapperCategory>,
    /// Used to override hardcoded category values with a value pulled from a source attribute on the log.
    #[serde(rename = "fallback")]
    pub fallback: Option<crate::datadogV1::model::LogsSchemaCategoryMapperFallback>,
    /// Name of the logs schema category mapper.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the target attributes which value is defined by the matching category.
    #[serde(rename = "targets")]
    pub targets: crate::datadogV1::model::LogsSchemaCategoryMapperTargets,
    /// Type of logs schema category mapper.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsSchemaCategoryMapperType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsSchemaCategoryMapper {
    pub fn new(
        categories: Vec<crate::datadogV1::model::LogsSchemaCategoryMapperCategory>,
        name: String,
        targets: crate::datadogV1::model::LogsSchemaCategoryMapperTargets,
        type_: crate::datadogV1::model::LogsSchemaCategoryMapperType,
    ) -> LogsSchemaCategoryMapper {
        LogsSchemaCategoryMapper {
            categories,
            fallback: None,
            name,
            targets,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fallback(
        mut self,
        value: crate::datadogV1::model::LogsSchemaCategoryMapperFallback,
    ) -> Self {
        self.fallback = Some(value);
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

impl<'de> Deserialize<'de> for LogsSchemaCategoryMapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsSchemaCategoryMapperVisitor;
        impl<'a> Visitor<'a> for LogsSchemaCategoryMapperVisitor {
            type Value = LogsSchemaCategoryMapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut categories: Option<
                    Vec<crate::datadogV1::model::LogsSchemaCategoryMapperCategory>,
                > = None;
                let mut fallback: Option<
                    crate::datadogV1::model::LogsSchemaCategoryMapperFallback,
                > = None;
                let mut name: Option<String> = None;
                let mut targets: Option<crate::datadogV1::model::LogsSchemaCategoryMapperTargets> =
                    None;
                let mut type_: Option<crate::datadogV1::model::LogsSchemaCategoryMapperType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "categories" => {
                            categories = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fallback" => {
                            if v.is_null() {
                                continue;
                            }
                            fallback = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsSchemaCategoryMapperType::UnparsedObject(_type_) => {
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
                let categories = categories.ok_or_else(|| M::Error::missing_field("categories"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsSchemaCategoryMapper {
                    categories,
                    fallback,
                    name,
                    targets,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsSchemaCategoryMapperVisitor)
    }
}
