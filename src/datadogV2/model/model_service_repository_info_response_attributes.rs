// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the service repository information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceRepositoryInfoResponseAttributes {
    /// The SHA of the commit associated with the service version.
    #[serde(rename = "commit_sha")]
    pub commit_sha: Option<String>,
    /// The URL of the source code repository.
    #[serde(rename = "repository_url")]
    pub repository_url: Option<String>,
    /// The status of the service repository info lookup.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ServiceRepositoryInfoStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceRepositoryInfoResponseAttributes {
    pub fn new(
        status: crate::datadogV2::model::ServiceRepositoryInfoStatus,
    ) -> ServiceRepositoryInfoResponseAttributes {
        ServiceRepositoryInfoResponseAttributes {
            commit_sha: None,
            repository_url: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn commit_sha(mut self, value: String) -> Self {
        self.commit_sha = Some(value);
        self
    }

    pub fn repository_url(mut self, value: String) -> Self {
        self.repository_url = Some(value);
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

impl<'de> Deserialize<'de> for ServiceRepositoryInfoResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceRepositoryInfoResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceRepositoryInfoResponseAttributesVisitor {
            type Value = ServiceRepositoryInfoResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commit_sha: Option<String> = None;
                let mut repository_url: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::ServiceRepositoryInfoStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commit_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            commit_sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_url" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ServiceRepositoryInfoStatus::UnparsedObject(_status) => {
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
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = ServiceRepositoryInfoResponseAttributes {
                    commit_sha,
                    repository_url,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceRepositoryInfoResponseAttributesVisitor)
    }
}
