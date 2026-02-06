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
pub struct JiraIssueFindingId {
    /// Timestamp when the finding was discovered.
    #[serde(rename = "discovered")]
    pub discovered: i64,
    /// Identifier of the finding.
    #[serde(rename = "id")]
    pub id: String,
    /// Resource associated with the finding.
    #[serde(rename = "resource")]
    pub resource: String,
    /// Tags associated with the finding.
    #[serde(rename = "tags")]
    pub tags: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIssueFindingId {
    pub fn new(discovered: i64, id: String, resource: String, tags: String) -> JiraIssueFindingId {
        JiraIssueFindingId {
            discovered,
            id,
            resource,
            tags,
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

impl<'de> Deserialize<'de> for JiraIssueFindingId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIssueFindingIdVisitor;
        impl<'a> Visitor<'a> for JiraIssueFindingIdVisitor {
            type Value = JiraIssueFindingId;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut discovered: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut resource: Option<String> = None;
                let mut tags: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "discovered" => {
                            discovered = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let discovered = discovered.ok_or_else(|| M::Error::missing_field("discovered"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let resource = resource.ok_or_else(|| M::Error::missing_field("resource"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = JiraIssueFindingId {
                    discovered,
                    id,
                    resource,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIssueFindingIdVisitor)
    }
}
