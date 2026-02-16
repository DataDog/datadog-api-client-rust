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
pub struct PickRemediationFromInvestigationResponse {
    /// The keywords generated and used for finding actions.
    #[serde(rename = "keywords_used")]
    pub keywords_used: Vec<String>,
    /// The matching actions.
    #[serde(rename = "matches")]
    pub matches: Vec<crate::datadogV2::model::ActionMatch>,
    /// The request ID.
    #[serde(rename = "request_id")]
    pub request_id: String,
    /// The total number of matches.
    #[serde(rename = "total_matches")]
    pub total_matches: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PickRemediationFromInvestigationResponse {
    pub fn new(
        keywords_used: Vec<String>,
        matches: Vec<crate::datadogV2::model::ActionMatch>,
        request_id: String,
        total_matches: i64,
    ) -> PickRemediationFromInvestigationResponse {
        PickRemediationFromInvestigationResponse {
            keywords_used,
            matches,
            request_id,
            total_matches,
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

impl<'de> Deserialize<'de> for PickRemediationFromInvestigationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PickRemediationFromInvestigationResponseVisitor;
        impl<'a> Visitor<'a> for PickRemediationFromInvestigationResponseVisitor {
            type Value = PickRemediationFromInvestigationResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut keywords_used: Option<Vec<String>> = None;
                let mut matches: Option<Vec<crate::datadogV2::model::ActionMatch>> = None;
                let mut request_id: Option<String> = None;
                let mut total_matches: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "keywords_used" => {
                            keywords_used =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "matches" => {
                            matches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_matches" => {
                            total_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let keywords_used =
                    keywords_used.ok_or_else(|| M::Error::missing_field("keywords_used"))?;
                let matches = matches.ok_or_else(|| M::Error::missing_field("matches"))?;
                let request_id = request_id.ok_or_else(|| M::Error::missing_field("request_id"))?;
                let total_matches =
                    total_matches.ok_or_else(|| M::Error::missing_field("total_matches"))?;

                let content = PickRemediationFromInvestigationResponse {
                    keywords_used,
                    matches,
                    request_id,
                    total_matches,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PickRemediationFromInvestigationResponseVisitor)
    }
}
