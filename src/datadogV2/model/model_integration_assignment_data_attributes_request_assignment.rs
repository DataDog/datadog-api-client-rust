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
pub struct IntegrationAssignmentDataAttributesRequestAssignment {
    /// Map of Jira issue URLs to lists of finding IDs.
    #[serde(rename = "jira")]
    pub jira: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAssignmentDataAttributesRequestAssignment {
    pub fn new(
        jira: std::collections::BTreeMap<String, Vec<String>>,
    ) -> IntegrationAssignmentDataAttributesRequestAssignment {
        IntegrationAssignmentDataAttributesRequestAssignment {
            jira,
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

impl<'de> Deserialize<'de> for IntegrationAssignmentDataAttributesRequestAssignment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAssignmentDataAttributesRequestAssignmentVisitor;
        impl<'a> Visitor<'a> for IntegrationAssignmentDataAttributesRequestAssignmentVisitor {
            type Value = IntegrationAssignmentDataAttributesRequestAssignment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut jira: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "jira" => {
                            jira = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let jira = jira.ok_or_else(|| M::Error::missing_field("jira"))?;

                let content = IntegrationAssignmentDataAttributesRequestAssignment {
                    jira,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAssignmentDataAttributesRequestAssignmentVisitor)
    }
}
