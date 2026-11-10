// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship between the issue and other resources, such as an assignee, case, teams, pull requests, watches, samples, or a Bits AI investigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueRelationships {
    /// Relationship between the issue and assignee.
    #[serde(rename = "assignee")]
    pub assignee: Option<crate::datadogV2::model::IssueAssigneeRelationship>,
    /// Bits AI investigation linked to the issue, returned when `bits_investigation` is requested
    /// in the `include` query parameter.
    #[serde(rename = "bits_investigation")]
    pub bits_investigation: Option<crate::datadogV2::model::IssueBitsInvestigationRelationship>,
    /// Relationship between the issue and case.
    #[serde(rename = "case")]
    pub case: Option<crate::datadogV2::model::IssueCaseRelationship>,
    /// Relationship between the issue and its linked pull requests. Returned when `pull_requests` is included and the issue has linked pull requests.
    #[serde(rename = "pull_requests")]
    pub pull_requests: Option<crate::datadogV2::model::IssuePullRequestsRelationship>,
    /// Relationship between the issue and its curated error samples.
    #[serde(rename = "samples")]
    pub samples: Option<crate::datadogV2::model::IssueSamplesRelationship>,
    /// Relationship between the issue and teams.
    #[serde(rename = "team_owners")]
    pub team_owners: Option<crate::datadogV2::model::IssueTeamOwnersRelationship>,
    /// Relationship between the issue and its active watches. Returned when `watches` is included and the issue has an active watch.
    #[serde(rename = "watches")]
    pub watches: Option<crate::datadogV2::model::IssueWatchesRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueRelationships {
    pub fn new() -> IssueRelationships {
        IssueRelationships {
            assignee: None,
            bits_investigation: None,
            case: None,
            pull_requests: None,
            samples: None,
            team_owners: None,
            watches: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignee(mut self, value: crate::datadogV2::model::IssueAssigneeRelationship) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn bits_investigation(
        mut self,
        value: crate::datadogV2::model::IssueBitsInvestigationRelationship,
    ) -> Self {
        self.bits_investigation = Some(value);
        self
    }

    pub fn case(mut self, value: crate::datadogV2::model::IssueCaseRelationship) -> Self {
        self.case = Some(value);
        self
    }

    pub fn pull_requests(
        mut self,
        value: crate::datadogV2::model::IssuePullRequestsRelationship,
    ) -> Self {
        self.pull_requests = Some(value);
        self
    }

    pub fn samples(mut self, value: crate::datadogV2::model::IssueSamplesRelationship) -> Self {
        self.samples = Some(value);
        self
    }

    pub fn team_owners(
        mut self,
        value: crate::datadogV2::model::IssueTeamOwnersRelationship,
    ) -> Self {
        self.team_owners = Some(value);
        self
    }

    pub fn watches(mut self, value: crate::datadogV2::model::IssueWatchesRelationship) -> Self {
        self.watches = Some(value);
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

impl Default for IssueRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueRelationshipsVisitor;
        impl<'a> Visitor<'a> for IssueRelationshipsVisitor {
            type Value = IssueRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignee: Option<crate::datadogV2::model::IssueAssigneeRelationship> = None;
                let mut bits_investigation: Option<
                    crate::datadogV2::model::IssueBitsInvestigationRelationship,
                > = None;
                let mut case: Option<crate::datadogV2::model::IssueCaseRelationship> = None;
                let mut pull_requests: Option<
                    crate::datadogV2::model::IssuePullRequestsRelationship,
                > = None;
                let mut samples: Option<crate::datadogV2::model::IssueSamplesRelationship> = None;
                let mut team_owners: Option<crate::datadogV2::model::IssueTeamOwnersRelationship> =
                    None;
                let mut watches: Option<crate::datadogV2::model::IssueWatchesRelationship> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignee" => {
                            if v.is_null() {
                                continue;
                            }
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bits_investigation" => {
                            if v.is_null() {
                                continue;
                            }
                            bits_investigation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "case" => {
                            if v.is_null() {
                                continue;
                            }
                            case = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pull_requests" => {
                            if v.is_null() {
                                continue;
                            }
                            pull_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "samples" => {
                            if v.is_null() {
                                continue;
                            }
                            samples = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_owners" => {
                            if v.is_null() {
                                continue;
                            }
                            team_owners =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "watches" => {
                            if v.is_null() {
                                continue;
                            }
                            watches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssueRelationships {
                    assignee,
                    bits_investigation,
                    case,
                    pull_requests,
                    samples,
                    team_owners,
                    watches,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueRelationshipsVisitor)
    }
}
