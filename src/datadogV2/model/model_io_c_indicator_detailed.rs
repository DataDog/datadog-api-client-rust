// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An indicator of compromise with extended context from your environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IoCIndicatorDetailed {
    /// Additional domain-specific context from threat intelligence sources.
    #[serde(rename = "additional_data")]
    pub additional_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Autonomous system CIDR block.
    #[serde(rename = "as_cidr_block")]
    pub as_cidr_block: Option<String>,
    /// Geographic location information for an IP indicator.
    #[serde(rename = "as_geo")]
    pub as_geo: Option<crate::datadogV2::model::IoCGeoLocation>,
    /// Autonomous system number.
    #[serde(rename = "as_number")]
    pub as_number: Option<String>,
    /// Autonomous system organization name.
    #[serde(rename = "as_organization")]
    pub as_organization: Option<String>,
    /// Autonomous system type.
    #[serde(rename = "as_type")]
    pub as_type: Option<String>,
    /// Threat intelligence sources that flagged this indicator as benign.
    #[serde(
        rename = "benign_sources",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub benign_sources: Option<Option<Vec<crate::datadogV2::model::IoCSource>>>,
    /// Threat categories associated with the indicator.
    #[serde(rename = "categories")]
    pub categories: Option<Vec<String>>,
    /// Critical assets associated with this indicator.
    #[serde(rename = "critical_assets")]
    pub critical_assets: Option<Vec<String>>,
    /// Timestamp when the indicator was first seen.
    #[serde(rename = "first_seen")]
    pub first_seen: Option<chrono::DateTime<chrono::Utc>>,
    /// Hosts associated with this indicator.
    #[serde(rename = "hosts")]
    pub hosts: Option<Vec<String>>,
    /// Unique identifier for the indicator.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The indicator value (for example, an IP address or domain).
    #[serde(rename = "indicator")]
    pub indicator: Option<String>,
    /// Type of indicator (for example, IP address or domain).
    #[serde(rename = "indicator_type")]
    pub indicator_type: Option<String>,
    /// Timestamp when the indicator was last seen.
    #[serde(rename = "last_seen")]
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of logs that matched this indicator.
    #[serde(rename = "log_matches")]
    pub log_matches: Option<i64>,
    /// Log sources where this indicator was observed.
    #[serde(rename = "log_sources")]
    pub log_sources: Option<Vec<String>>,
    /// Effect of a scoring factor on the indicator's threat score.
    #[serde(rename = "m_as_type")]
    pub m_as_type: Option<crate::datadogV2::model::IoCScoreEffect>,
    /// Effect of a scoring factor on the indicator's threat score.
    #[serde(rename = "m_persistence")]
    pub m_persistence: Option<crate::datadogV2::model::IoCScoreEffect>,
    /// Effect of a scoring factor on the indicator's threat score.
    #[serde(rename = "m_signal")]
    pub m_signal: Option<crate::datadogV2::model::IoCScoreEffect>,
    /// Effect of a scoring factor on the indicator's threat score.
    #[serde(rename = "m_sources")]
    pub m_sources: Option<crate::datadogV2::model::IoCScoreEffect>,
    /// Threat intelligence sources that flagged this indicator as malicious.
    #[serde(
        rename = "malicious_sources",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub malicious_sources: Option<Option<Vec<crate::datadogV2::model::IoCSource>>>,
    /// Effect of a scoring factor on the indicator's threat score.
    #[serde(rename = "max_trust_score")]
    pub max_trust_score: Option<crate::datadogV2::model::IoCScoreEffect>,
    /// Threat score for the indicator (0-100).
    #[serde(rename = "score")]
    pub score: Option<f64>,
    /// Services where this indicator was observed.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Number of security signals that matched this indicator.
    #[serde(rename = "signal_matches")]
    pub signal_matches: Option<i64>,
    /// Breakdown of security signals by severity.
    #[serde(rename = "signal_severity")]
    pub signal_severity: Option<Vec<crate::datadogV2::model::IoCSignalSeverityCount>>,
    /// Signal tier level.
    #[serde(rename = "signal_tier")]
    pub signal_tier: Option<i64>,
    /// Threat intelligence sources that flagged this indicator as suspicious.
    #[serde(
        rename = "suspicious_sources",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub suspicious_sources: Option<Option<Vec<crate::datadogV2::model::IoCSource>>>,
    /// Tags associated with the indicator.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Users associated with this indicator, grouped by category.
    #[serde(rename = "users")]
    pub users: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IoCIndicatorDetailed {
    pub fn new() -> IoCIndicatorDetailed {
        IoCIndicatorDetailed {
            additional_data: None,
            as_cidr_block: None,
            as_geo: None,
            as_number: None,
            as_organization: None,
            as_type: None,
            benign_sources: None,
            categories: None,
            critical_assets: None,
            first_seen: None,
            hosts: None,
            id: None,
            indicator: None,
            indicator_type: None,
            last_seen: None,
            log_matches: None,
            log_sources: None,
            m_as_type: None,
            m_persistence: None,
            m_signal: None,
            m_sources: None,
            malicious_sources: None,
            max_trust_score: None,
            score: None,
            services: None,
            signal_matches: None,
            signal_severity: None,
            signal_tier: None,
            suspicious_sources: None,
            tags: None,
            users: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_data(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn as_cidr_block(mut self, value: String) -> Self {
        self.as_cidr_block = Some(value);
        self
    }

    pub fn as_geo(mut self, value: crate::datadogV2::model::IoCGeoLocation) -> Self {
        self.as_geo = Some(value);
        self
    }

    pub fn as_number(mut self, value: String) -> Self {
        self.as_number = Some(value);
        self
    }

    pub fn as_organization(mut self, value: String) -> Self {
        self.as_organization = Some(value);
        self
    }

    pub fn as_type(mut self, value: String) -> Self {
        self.as_type = Some(value);
        self
    }

    pub fn benign_sources(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IoCSource>>,
    ) -> Self {
        self.benign_sources = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn critical_assets(mut self, value: Vec<String>) -> Self {
        self.critical_assets = Some(value);
        self
    }

    pub fn first_seen(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.first_seen = Some(value);
        self
    }

    pub fn hosts(mut self, value: Vec<String>) -> Self {
        self.hosts = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn indicator(mut self, value: String) -> Self {
        self.indicator = Some(value);
        self
    }

    pub fn indicator_type(mut self, value: String) -> Self {
        self.indicator_type = Some(value);
        self
    }

    pub fn last_seen(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_seen = Some(value);
        self
    }

    pub fn log_matches(mut self, value: i64) -> Self {
        self.log_matches = Some(value);
        self
    }

    pub fn log_sources(mut self, value: Vec<String>) -> Self {
        self.log_sources = Some(value);
        self
    }

    pub fn m_as_type(mut self, value: crate::datadogV2::model::IoCScoreEffect) -> Self {
        self.m_as_type = Some(value);
        self
    }

    pub fn m_persistence(mut self, value: crate::datadogV2::model::IoCScoreEffect) -> Self {
        self.m_persistence = Some(value);
        self
    }

    pub fn m_signal(mut self, value: crate::datadogV2::model::IoCScoreEffect) -> Self {
        self.m_signal = Some(value);
        self
    }

    pub fn m_sources(mut self, value: crate::datadogV2::model::IoCScoreEffect) -> Self {
        self.m_sources = Some(value);
        self
    }

    pub fn malicious_sources(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IoCSource>>,
    ) -> Self {
        self.malicious_sources = Some(value);
        self
    }

    pub fn max_trust_score(mut self, value: crate::datadogV2::model::IoCScoreEffect) -> Self {
        self.max_trust_score = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn signal_matches(mut self, value: i64) -> Self {
        self.signal_matches = Some(value);
        self
    }

    pub fn signal_severity(
        mut self,
        value: Vec<crate::datadogV2::model::IoCSignalSeverityCount>,
    ) -> Self {
        self.signal_severity = Some(value);
        self
    }

    pub fn signal_tier(mut self, value: i64) -> Self {
        self.signal_tier = Some(value);
        self
    }

    pub fn suspicious_sources(
        mut self,
        value: Option<Vec<crate::datadogV2::model::IoCSource>>,
    ) -> Self {
        self.suspicious_sources = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn users(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.users = Some(value);
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

impl Default for IoCIndicatorDetailed {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IoCIndicatorDetailed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IoCIndicatorDetailedVisitor;
        impl<'a> Visitor<'a> for IoCIndicatorDetailedVisitor {
            type Value = IoCIndicatorDetailed;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_data: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut as_cidr_block: Option<String> = None;
                let mut as_geo: Option<crate::datadogV2::model::IoCGeoLocation> = None;
                let mut as_number: Option<String> = None;
                let mut as_organization: Option<String> = None;
                let mut as_type: Option<String> = None;
                let mut benign_sources: Option<Option<Vec<crate::datadogV2::model::IoCSource>>> =
                    None;
                let mut categories: Option<Vec<String>> = None;
                let mut critical_assets: Option<Vec<String>> = None;
                let mut first_seen: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut hosts: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut indicator: Option<String> = None;
                let mut indicator_type: Option<String> = None;
                let mut last_seen: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut log_matches: Option<i64> = None;
                let mut log_sources: Option<Vec<String>> = None;
                let mut m_as_type: Option<crate::datadogV2::model::IoCScoreEffect> = None;
                let mut m_persistence: Option<crate::datadogV2::model::IoCScoreEffect> = None;
                let mut m_signal: Option<crate::datadogV2::model::IoCScoreEffect> = None;
                let mut m_sources: Option<crate::datadogV2::model::IoCScoreEffect> = None;
                let mut malicious_sources: Option<Option<Vec<crate::datadogV2::model::IoCSource>>> =
                    None;
                let mut max_trust_score: Option<crate::datadogV2::model::IoCScoreEffect> = None;
                let mut score: Option<f64> = None;
                let mut services: Option<Vec<String>> = None;
                let mut signal_matches: Option<i64> = None;
                let mut signal_severity: Option<
                    Vec<crate::datadogV2::model::IoCSignalSeverityCount>,
                > = None;
                let mut signal_tier: Option<i64> = None;
                let mut suspicious_sources: Option<
                    Option<Vec<crate::datadogV2::model::IoCSource>>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut users: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additional_data" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "as_cidr_block" => {
                            if v.is_null() {
                                continue;
                            }
                            as_cidr_block =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "as_geo" => {
                            if v.is_null() {
                                continue;
                            }
                            as_geo = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "as_number" => {
                            if v.is_null() {
                                continue;
                            }
                            as_number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "as_organization" => {
                            if v.is_null() {
                                continue;
                            }
                            as_organization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "as_type" => {
                            if v.is_null() {
                                continue;
                            }
                            as_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "benign_sources" => {
                            benign_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "categories" => {
                            if v.is_null() {
                                continue;
                            }
                            categories = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "critical_assets" => {
                            if v.is_null() {
                                continue;
                            }
                            critical_assets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indicator" => {
                            if v.is_null() {
                                continue;
                            }
                            indicator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indicator_type" => {
                            if v.is_null() {
                                continue;
                            }
                            indicator_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_matches" => {
                            if v.is_null() {
                                continue;
                            }
                            log_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_sources" => {
                            if v.is_null() {
                                continue;
                            }
                            log_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "m_as_type" => {
                            if v.is_null() {
                                continue;
                            }
                            m_as_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _m_as_type) = m_as_type {
                                match _m_as_type {
                                    crate::datadogV2::model::IoCScoreEffect::UnparsedObject(
                                        _m_as_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "m_persistence" => {
                            if v.is_null() {
                                continue;
                            }
                            m_persistence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _m_persistence) = m_persistence {
                                match _m_persistence {
                                    crate::datadogV2::model::IoCScoreEffect::UnparsedObject(
                                        _m_persistence,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "m_signal" => {
                            if v.is_null() {
                                continue;
                            }
                            m_signal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _m_signal) = m_signal {
                                match _m_signal {
                                    crate::datadogV2::model::IoCScoreEffect::UnparsedObject(
                                        _m_signal,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "m_sources" => {
                            if v.is_null() {
                                continue;
                            }
                            m_sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _m_sources) = m_sources {
                                match _m_sources {
                                    crate::datadogV2::model::IoCScoreEffect::UnparsedObject(
                                        _m_sources,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "malicious_sources" => {
                            malicious_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_trust_score" => {
                            if v.is_null() {
                                continue;
                            }
                            max_trust_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _max_trust_score) = max_trust_score {
                                match _max_trust_score {
                                    crate::datadogV2::model::IoCScoreEffect::UnparsedObject(
                                        _max_trust_score,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "score" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_matches" => {
                            if v.is_null() {
                                continue;
                            }
                            signal_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_severity" => {
                            if v.is_null() {
                                continue;
                            }
                            signal_severity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            signal_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suspicious_sources" => {
                            suspicious_sources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IoCIndicatorDetailed {
                    additional_data,
                    as_cidr_block,
                    as_geo,
                    as_number,
                    as_organization,
                    as_type,
                    benign_sources,
                    categories,
                    critical_assets,
                    first_seen,
                    hosts,
                    id,
                    indicator,
                    indicator_type,
                    last_seen,
                    log_matches,
                    log_sources,
                    m_as_type,
                    m_persistence,
                    m_signal,
                    m_sources,
                    malicious_sources,
                    max_trust_score,
                    score,
                    services,
                    signal_matches,
                    signal_severity,
                    signal_tier,
                    suspicious_sources,
                    tags,
                    users,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IoCIndicatorDetailedVisitor)
    }
}
