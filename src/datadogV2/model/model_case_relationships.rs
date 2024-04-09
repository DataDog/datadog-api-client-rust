// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resources related to a case
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseRelationships {
    /// Relationship to user.
    #[serde(
        rename = "assignee",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub assignee: Option<Option<crate::datadogV2::model::NullableUserRelationship>>,
    /// Relationship to user.
    #[serde(
        rename = "created_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created_by: Option<Option<crate::datadogV2::model::NullableUserRelationship>>,
    /// Relationship to user.
    #[serde(
        rename = "modified_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_by: Option<Option<crate::datadogV2::model::NullableUserRelationship>>,
    /// Relationship to project
    #[serde(rename = "project")]
    pub project: Option<crate::datadogV2::model::ProjectRelationship>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseRelationships {
    pub fn new() -> CaseRelationships {
        CaseRelationships {
            assignee: None,
            created_by: None,
            modified_by: None,
            project: None,
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

    pub fn created_by(
        mut self,
        value: Option<crate::datadogV2::model::NullableUserRelationship>,
    ) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn modified_by(
        mut self,
        value: Option<crate::datadogV2::model::NullableUserRelationship>,
    ) -> Self {
        self.modified_by = Some(value);
        self
    }

    pub fn project(mut self, value: crate::datadogV2::model::ProjectRelationship) -> Self {
        self.project = Some(value);
        self
    }
}

impl Default for CaseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseRelationshipsVisitor;
        impl<'a> Visitor<'a> for CaseRelationshipsVisitor {
            type Value = CaseRelationships;

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
                let mut created_by: Option<
                    Option<crate::datadogV2::model::NullableUserRelationship>,
                > = None;
                let mut modified_by: Option<
                    Option<crate::datadogV2::model::NullableUserRelationship>,
                > = None;
                let mut project: Option<crate::datadogV2::model::ProjectRelationship> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee" => {
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            if v.is_null() {
                                continue;
                            }
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CaseRelationships {
                    assignee,
                    created_by,
                    modified_by,
                    project,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseRelationshipsVisitor)
    }
}
