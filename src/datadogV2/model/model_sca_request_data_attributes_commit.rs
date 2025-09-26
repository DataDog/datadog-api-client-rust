// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ScaRequestDataAttributesCommit` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesCommit {
    /// The `commit` `author_date`.
    #[serde(rename = "author_date")]
    pub author_date: Option<String>,
    /// The `commit` `author_email`.
    #[serde(rename = "author_email")]
    pub author_email: Option<String>,
    /// The `commit` `author_name`.
    #[serde(rename = "author_name")]
    pub author_name: Option<String>,
    /// The `commit` `branch`.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// The `commit` `committer_email`.
    #[serde(rename = "committer_email")]
    pub committer_email: Option<String>,
    /// The `commit` `committer_name`.
    #[serde(rename = "committer_name")]
    pub committer_name: Option<String>,
    /// The `commit` `sha`.
    #[serde(rename = "sha")]
    pub sha: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesCommit {
    pub fn new() -> ScaRequestDataAttributesCommit {
        ScaRequestDataAttributesCommit {
            author_date: None,
            author_email: None,
            author_name: None,
            branch: None,
            committer_email: None,
            committer_name: None,
            sha: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author_date(mut self, value: String) -> Self {
        self.author_date = Some(value);
        self
    }

    pub fn author_email(mut self, value: String) -> Self {
        self.author_email = Some(value);
        self
    }

    pub fn author_name(mut self, value: String) -> Self {
        self.author_name = Some(value);
        self
    }

    pub fn branch(mut self, value: String) -> Self {
        self.branch = Some(value);
        self
    }

    pub fn committer_email(mut self, value: String) -> Self {
        self.committer_email = Some(value);
        self
    }

    pub fn committer_name(mut self, value: String) -> Self {
        self.committer_name = Some(value);
        self
    }

    pub fn sha(mut self, value: String) -> Self {
        self.sha = Some(value);
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

impl Default for ScaRequestDataAttributesCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesCommit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesCommitVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesCommitVisitor {
            type Value = ScaRequestDataAttributesCommit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_date: Option<String> = None;
                let mut author_email: Option<String> = None;
                let mut author_name: Option<String> = None;
                let mut branch: Option<String> = None;
                let mut committer_email: Option<String> = None;
                let mut committer_name: Option<String> = None;
                let mut sha: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author_date" => {
                            if v.is_null() {
                                continue;
                            }
                            author_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "author_email" => {
                            if v.is_null() {
                                continue;
                            }
                            author_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "author_name" => {
                            if v.is_null() {
                                continue;
                            }
                            author_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "branch" => {
                            if v.is_null() {
                                continue;
                            }
                            branch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committer_email" => {
                            if v.is_null() {
                                continue;
                            }
                            committer_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committer_name" => {
                            if v.is_null() {
                                continue;
                            }
                            committer_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sha" => {
                            if v.is_null() {
                                continue;
                            }
                            sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesCommit {
                    author_date,
                    author_email,
                    author_name,
                    branch,
                    committer_email,
                    committer_name,
                    sha,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesCommitVisitor)
    }
}
