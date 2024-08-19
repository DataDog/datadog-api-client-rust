// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Anonymous assignee entity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTodoAnonymousAssignee {
    /// URL for assignee's icon.
    #[serde(rename = "icon")]
    pub icon: String,
    /// Anonymous assignee's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Assignee's name.
    #[serde(rename = "name")]
    pub name: String,
    /// The source of the anonymous assignee.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTodoAnonymousAssignee {
    pub fn new(
        icon: String,
        id: String,
        name: String,
        source: crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource,
    ) -> IncidentTodoAnonymousAssignee {
        IncidentTodoAnonymousAssignee {
            icon,
            id,
            name,
            source,
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

impl<'de> Deserialize<'de> for IncidentTodoAnonymousAssignee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTodoAnonymousAssigneeVisitor;
        impl<'a> Visitor<'a> for IncidentTodoAnonymousAssigneeVisitor {
            type Value = IncidentTodoAnonymousAssignee;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut icon: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut source: Option<
                    crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "icon" => {
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::IncidentTodoAnonymousAssigneeSource::UnparsedObject(_source) => {
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
                let icon = icon.ok_or_else(|| M::Error::missing_field("icon"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = IncidentTodoAnonymousAssignee {
                    icon,
                    id,
                    name,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTodoAnonymousAssigneeVisitor)
    }
}
