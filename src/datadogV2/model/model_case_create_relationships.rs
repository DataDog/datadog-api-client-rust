// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships formed with the case on creation
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseCreateRelationships {
    /// Relationship to user.
    #[serde(
        rename = "assignee",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub assignee: Option<Option<crate::datadogV2::model::NullableUserRelationship>>,
    /// Relationship to project
    #[serde(rename = "project")]
    pub project: crate::datadogV2::model::ProjectRelationship,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseCreateRelationships {
    pub fn new(project: crate::datadogV2::model::ProjectRelationship) -> CaseCreateRelationships {
        CaseCreateRelationships {
            assignee: None,
            project,
            _unparsed: false,
        }
    }

    pub fn assignee(
        mut self,
        value: Option<crate::datadogV2::model::NullableUserRelationship>,
    ) -> Self {
        self.assignee = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CaseCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for CaseCreateRelationshipsVisitor {
            type Value = CaseCreateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee: Option<
                    Option<crate::datadogV2::model::NullableUserRelationship>,
                > = None;
                let mut project: Option<crate::datadogV2::model::ProjectRelationship> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee" => {
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let project = project.ok_or_else(|| M::Error::missing_field("project"))?;

                let content = CaseCreateRelationships {
                    assignee,
                    project,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseCreateRelationshipsVisitor)
    }
}
