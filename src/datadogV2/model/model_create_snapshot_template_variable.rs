// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A template variable definition for snapshot rendering.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateSnapshotTemplateVariable {
    /// The template variable name.
    #[serde(rename = "name")]
    pub name: String,
    /// The tag prefix associated with the template variable. For example, a prefix of `host` with a value of `web-server-1` scopes the snapshot to `host:web-server-1`.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// The list of scoped values for this template variable.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateSnapshotTemplateVariable {
    pub fn new(
        name: String,
        prefix: String,
        values: Vec<String>,
    ) -> CreateSnapshotTemplateVariable {
        CreateSnapshotTemplateVariable {
            name,
            prefix,
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

impl<'de> Deserialize<'de> for CreateSnapshotTemplateVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateSnapshotTemplateVariableVisitor;
        impl<'a> Visitor<'a> for CreateSnapshotTemplateVariableVisitor {
            type Value = CreateSnapshotTemplateVariable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let prefix = prefix.ok_or_else(|| M::Error::missing_field("prefix"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = CreateSnapshotTemplateVariable {
                    name,
                    prefix,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateSnapshotTemplateVariableVisitor)
    }
}
