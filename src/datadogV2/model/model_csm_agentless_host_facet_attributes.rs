// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an agentless host facet.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmAgentlessHostFacetAttributes {
    /// Whether the facet has a bounded set of allowed values. `true` indicates a fixed value set and `false` indicates free-form values.
    #[serde(rename = "bounded")]
    pub bounded: bool,
    /// Whether the facet is bundled as part of the default facet set. `true` indicates bundled and `false` indicates custom.
    #[serde(rename = "bundled")]
    pub bundled: bool,
    /// Whether the facet is both bundled and actively used. `true` indicates in use; `false` indicates unused.
    #[serde(rename = "bundledAndUsed")]
    pub bundled_and_used: bool,
    /// The list of default filter values for the facet.
    #[serde(rename = "defaultValues")]
    pub default_values: Vec<String>,
    /// A human-readable description of what the facet represents.
    #[serde(rename = "description")]
    pub description: String,
    /// Whether the facet can be edited by users. `true` indicates editable; `false` indicates read-only.
    #[serde(rename = "editable")]
    pub editable: bool,
    /// The UI display type for the facet, such as `list`.
    #[serde(rename = "facetType")]
    pub facet_type: String,
    /// The list of UI groups that this facet belongs to.
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
    /// The display name of the facet.
    #[serde(rename = "name")]
    pub name: String,
    /// The field path used when filtering by this facet.
    #[serde(rename = "path")]
    pub path: String,
    /// The data source that provides the facet values.
    #[serde(rename = "source")]
    pub source: String,
    /// The data type of the facet values.
    #[serde(rename = "type")]
    pub type_: String,
    /// The list of allowed filter values for bounded facets. Empty for unbounded facets.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmAgentlessHostFacetAttributes {
    pub fn new(
        bounded: bool,
        bundled: bool,
        bundled_and_used: bool,
        default_values: Vec<String>,
        description: String,
        editable: bool,
        facet_type: String,
        groups: Vec<String>,
        name: String,
        path: String,
        source: String,
        type_: String,
        values: Vec<String>,
    ) -> CsmAgentlessHostFacetAttributes {
        CsmAgentlessHostFacetAttributes {
            bounded,
            bundled,
            bundled_and_used,
            default_values,
            description,
            editable,
            facet_type,
            groups,
            name,
            path,
            source,
            type_,
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

impl<'de> Deserialize<'de> for CsmAgentlessHostFacetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmAgentlessHostFacetAttributesVisitor;
        impl<'a> Visitor<'a> for CsmAgentlessHostFacetAttributesVisitor {
            type Value = CsmAgentlessHostFacetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bounded: Option<bool> = None;
                let mut bundled: Option<bool> = None;
                let mut bundled_and_used: Option<bool> = None;
                let mut default_values: Option<Vec<String>> = None;
                let mut description: Option<String> = None;
                let mut editable: Option<bool> = None;
                let mut facet_type: Option<String> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut path: Option<String> = None;
                let mut source: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bounded" => {
                            bounded = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bundled" => {
                            bundled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bundledAndUsed" => {
                            bundled_and_used =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultValues" => {
                            default_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "editable" => {
                            editable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "facetType" => {
                            facet_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                let bounded = bounded.ok_or_else(|| M::Error::missing_field("bounded"))?;
                let bundled = bundled.ok_or_else(|| M::Error::missing_field("bundled"))?;
                let bundled_and_used =
                    bundled_and_used.ok_or_else(|| M::Error::missing_field("bundled_and_used"))?;
                let default_values =
                    default_values.ok_or_else(|| M::Error::missing_field("default_values"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let editable = editable.ok_or_else(|| M::Error::missing_field("editable"))?;
                let facet_type = facet_type.ok_or_else(|| M::Error::missing_field("facet_type"))?;
                let groups = groups.ok_or_else(|| M::Error::missing_field("groups"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = CsmAgentlessHostFacetAttributes {
                    bounded,
                    bundled,
                    bundled_and_used,
                    default_values,
                    description,
                    editable,
                    facet_type,
                    groups,
                    name,
                    path,
                    source,
                    type_,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmAgentlessHostFacetAttributesVisitor)
    }
}
