// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the Jira issue to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateJiraIssueRequestDataRelationships {
    /// Case linked to the Jira issue.
    #[serde(rename = "case")]
    pub case: crate::datadogV2::model::CreateJiraIssueRequestDataRelationshipsCase,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateJiraIssueRequestDataRelationships {
    pub fn new(
        case: crate::datadogV2::model::CreateJiraIssueRequestDataRelationshipsCase,
    ) -> CreateJiraIssueRequestDataRelationships {
        CreateJiraIssueRequestDataRelationships {
            case,
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

impl<'de> Deserialize<'de> for CreateJiraIssueRequestDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateJiraIssueRequestDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for CreateJiraIssueRequestDataRelationshipsVisitor {
            type Value = CreateJiraIssueRequestDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case: Option<
                    crate::datadogV2::model::CreateJiraIssueRequestDataRelationshipsCase,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "case" => {
                            case = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let case = case.ok_or_else(|| M::Error::missing_field("case"))?;

                let content = CreateJiraIssueRequestDataRelationships {
                    case,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateJiraIssueRequestDataRelationshipsVisitor)
    }
}
