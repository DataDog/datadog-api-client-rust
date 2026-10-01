// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// GitHub Actions OIDC claims to match against. Each field is a regular expression.
/// The `sub` claim is required; all other claims are optional. A token matches only when
/// all provided patterns match simultaneously (AND semantics).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GitHubOIDCClaimPatterns {
    /// Regular expression matched against the `actor` claim.
    #[serde(rename = "actor")]
    pub actor: Option<String>,
    /// Regular expression matched against the `actor_id` claim.
    #[serde(rename = "actor_id")]
    pub actor_id: Option<String>,
    /// Regular expression matched against the `enterprise` claim.
    #[serde(rename = "enterprise")]
    pub enterprise: Option<String>,
    /// Regular expression matched against the `enterprise_id` claim.
    #[serde(rename = "enterprise_id")]
    pub enterprise_id: Option<String>,
    /// Regular expression matched against the `environment` claim.
    #[serde(rename = "environment")]
    pub environment: Option<String>,
    /// Regular expression matched against the `event_name` claim.
    #[serde(rename = "event_name")]
    pub event_name: Option<String>,
    /// Regular expression matched against the `job_workflow_ref` claim.
    #[serde(rename = "job_workflow_ref")]
    pub job_workflow_ref: Option<String>,
    /// Regular expression matched against the `ref` claim.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Regular expression matched against the `ref_type` claim.
    #[serde(rename = "ref_type")]
    pub ref_type: Option<String>,
    /// Regular expression matched against the `repository` claim.
    #[serde(rename = "repository")]
    pub repository: Option<String>,
    /// Regular expression matched against the `repository_id` claim.
    #[serde(rename = "repository_id")]
    pub repository_id: Option<String>,
    /// Regular expression matched against the `repository_owner` claim.
    #[serde(rename = "repository_owner")]
    pub repository_owner: Option<String>,
    /// Regular expression matched against the `repository_owner_id` claim.
    #[serde(rename = "repository_owner_id")]
    pub repository_owner_id: Option<String>,
    /// Regular expression matched against the `repository_visibility` claim.
    #[serde(rename = "repository_visibility")]
    pub repository_visibility: Option<String>,
    /// Regular expression matched against the `runner_environment` claim.
    #[serde(rename = "runner_environment")]
    pub runner_environment: Option<String>,
    /// Regular expression matched against the entire `sub` (subject) claim, the primary GitHub Actions OIDC
    /// identifier (for example, `repo:<OWNER>/<REPO>:ref:refs/heads/main`). The pattern must begin with
    /// `repo:<OWNER>/`, where `<OWNER>` is a literal repository-owner name rather than a regular expression.
    #[serde(rename = "sub")]
    pub sub: String,
    /// Regular expression matched against the `workflow` claim.
    #[serde(rename = "workflow")]
    pub workflow: Option<String>,
    /// Regular expression matched against the `workflow_ref` claim.
    #[serde(rename = "workflow_ref")]
    pub workflow_ref: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GitHubOIDCClaimPatterns {
    pub fn new(sub: String) -> GitHubOIDCClaimPatterns {
        GitHubOIDCClaimPatterns {
            actor: None,
            actor_id: None,
            enterprise: None,
            enterprise_id: None,
            environment: None,
            event_name: None,
            job_workflow_ref: None,
            ref_: None,
            ref_type: None,
            repository: None,
            repository_id: None,
            repository_owner: None,
            repository_owner_id: None,
            repository_visibility: None,
            runner_environment: None,
            sub,
            workflow: None,
            workflow_ref: None,
            _unparsed: false,
        }
    }

    pub fn actor(mut self, value: String) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn actor_id(mut self, value: String) -> Self {
        self.actor_id = Some(value);
        self
    }

    pub fn enterprise(mut self, value: String) -> Self {
        self.enterprise = Some(value);
        self
    }

    pub fn enterprise_id(mut self, value: String) -> Self {
        self.enterprise_id = Some(value);
        self
    }

    pub fn environment(mut self, value: String) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn event_name(mut self, value: String) -> Self {
        self.event_name = Some(value);
        self
    }

    pub fn job_workflow_ref(mut self, value: String) -> Self {
        self.job_workflow_ref = Some(value);
        self
    }

    pub fn ref_(mut self, value: String) -> Self {
        self.ref_ = Some(value);
        self
    }

    pub fn ref_type(mut self, value: String) -> Self {
        self.ref_type = Some(value);
        self
    }

    pub fn repository(mut self, value: String) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn repository_id(mut self, value: String) -> Self {
        self.repository_id = Some(value);
        self
    }

    pub fn repository_owner(mut self, value: String) -> Self {
        self.repository_owner = Some(value);
        self
    }

    pub fn repository_owner_id(mut self, value: String) -> Self {
        self.repository_owner_id = Some(value);
        self
    }

    pub fn repository_visibility(mut self, value: String) -> Self {
        self.repository_visibility = Some(value);
        self
    }

    pub fn runner_environment(mut self, value: String) -> Self {
        self.runner_environment = Some(value);
        self
    }

    pub fn workflow(mut self, value: String) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn workflow_ref(mut self, value: String) -> Self {
        self.workflow_ref = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for GitHubOIDCClaimPatterns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GitHubOIDCClaimPatternsVisitor;
        impl<'a> Visitor<'a> for GitHubOIDCClaimPatternsVisitor {
            type Value = GitHubOIDCClaimPatterns;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actor: Option<String> = None;
                let mut actor_id: Option<String> = None;
                let mut enterprise: Option<String> = None;
                let mut enterprise_id: Option<String> = None;
                let mut environment: Option<String> = None;
                let mut event_name: Option<String> = None;
                let mut job_workflow_ref: Option<String> = None;
                let mut ref_: Option<String> = None;
                let mut ref_type: Option<String> = None;
                let mut repository: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut repository_owner: Option<String> = None;
                let mut repository_owner_id: Option<String> = None;
                let mut repository_visibility: Option<String> = None;
                let mut runner_environment: Option<String> = None;
                let mut sub: Option<String> = None;
                let mut workflow: Option<String> = None;
                let mut workflow_ref: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actor" => {
                            if v.is_null() {
                                continue;
                            }
                            actor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "actor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            actor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enterprise" => {
                            if v.is_null() {
                                continue;
                            }
                            enterprise = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enterprise_id" => {
                            if v.is_null() {
                                continue;
                            }
                            enterprise_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment" => {
                            if v.is_null() {
                                continue;
                            }
                            environment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_name" => {
                            if v.is_null() {
                                continue;
                            }
                            event_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "job_workflow_ref" => {
                            if v.is_null() {
                                continue;
                            }
                            job_workflow_ref =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ref" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ref_type" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository" => {
                            if v.is_null() {
                                continue;
                            }
                            repository = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_owner" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_owner =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_owner_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_owner_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_visibility" => {
                            if v.is_null() {
                                continue;
                            }
                            repository_visibility =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_environment" => {
                            if v.is_null() {
                                continue;
                            }
                            runner_environment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub" => {
                            sub = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workflow_ref" => {
                            if v.is_null() {
                                continue;
                            }
                            workflow_ref =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let sub = sub.ok_or_else(|| M::Error::missing_field("sub"))?;

                let content = GitHubOIDCClaimPatterns {
                    actor,
                    actor_id,
                    enterprise,
                    enterprise_id,
                    environment,
                    event_name,
                    job_workflow_ref,
                    ref_,
                    ref_type,
                    repository,
                    repository_id,
                    repository_owner,
                    repository_owner_id,
                    repository_visibility,
                    runner_environment,
                    sub,
                    workflow,
                    workflow_ref,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GitHubOIDCClaimPatternsVisitor)
    }
}
