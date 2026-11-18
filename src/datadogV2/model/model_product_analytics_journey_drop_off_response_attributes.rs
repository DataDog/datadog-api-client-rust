// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a journey drop-off analysis response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyDropOffResponseAttributes {
    /// Number of sessions that reached the step before the target but not the target step.
    #[serde(rename = "drop_off_count")]
    pub drop_off_count: i64,
    /// The issues most associated with the drop-off.
    #[serde(rename = "issues")]
    pub issues: Vec<crate::datadogV2::model::ProductAnalyticsJourneyDropOffIssue>,
    /// Number of dropped-off sessions that contained an error.
    #[serde(rename = "sessions_with_errors")]
    pub sessions_with_errors: i64,
    /// Number of dropped-off sessions that contained a frustration signal.
    #[serde(rename = "sessions_with_frustration_signals")]
    pub sessions_with_frustration_signals: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyDropOffResponseAttributes {
    pub fn new(
        drop_off_count: i64,
        issues: Vec<crate::datadogV2::model::ProductAnalyticsJourneyDropOffIssue>,
        sessions_with_errors: i64,
        sessions_with_frustration_signals: i64,
    ) -> ProductAnalyticsJourneyDropOffResponseAttributes {
        ProductAnalyticsJourneyDropOffResponseAttributes {
            drop_off_count,
            issues,
            sessions_with_errors,
            sessions_with_frustration_signals,
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyDropOffResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyDropOffResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyDropOffResponseAttributesVisitor {
            type Value = ProductAnalyticsJourneyDropOffResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut drop_off_count: Option<i64> = None;
                let mut issues: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsJourneyDropOffIssue>,
                > = None;
                let mut sessions_with_errors: Option<i64> = None;
                let mut sessions_with_frustration_signals: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "drop_off_count" => {
                            drop_off_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issues" => {
                            issues = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_with_errors" => {
                            sessions_with_errors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_with_frustration_signals" => {
                            sessions_with_frustration_signals =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let drop_off_count =
                    drop_off_count.ok_or_else(|| M::Error::missing_field("drop_off_count"))?;
                let issues = issues.ok_or_else(|| M::Error::missing_field("issues"))?;
                let sessions_with_errors = sessions_with_errors
                    .ok_or_else(|| M::Error::missing_field("sessions_with_errors"))?;
                let sessions_with_frustration_signals = sessions_with_frustration_signals
                    .ok_or_else(|| M::Error::missing_field("sessions_with_frustration_signals"))?;

                let content = ProductAnalyticsJourneyDropOffResponseAttributes {
                    drop_off_count,
                    issues,
                    sessions_with_errors,
                    sessions_with_frustration_signals,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyDropOffResponseAttributesVisitor)
    }
}
