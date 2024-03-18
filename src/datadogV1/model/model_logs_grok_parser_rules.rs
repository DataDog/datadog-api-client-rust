// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Set of rules for the grok parser.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsGrokParserRules {
    /// List of match rules for the grok parser, separated by a new line.
    #[serde(rename = "match_rules")]
    pub match_rules: String,
    /// List of support rules for the grok parser, separated by a new line.
    #[serde(rename = "support_rules")]
    pub support_rules: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsGrokParserRules {
    pub fn new(match_rules: String) -> LogsGrokParserRules {
        LogsGrokParserRules {
            match_rules,
            support_rules: None,
            _unparsed: false,
        }
    }

    pub fn support_rules(mut self, value: String) -> Self {
        self.support_rules = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsGrokParserRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsGrokParserRulesVisitor;
        impl<'a> Visitor<'a> for LogsGrokParserRulesVisitor {
            type Value = LogsGrokParserRules;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_rules: Option<String> = None;
                let mut support_rules: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match_rules" => {
                            match_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            support_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let match_rules =
                    match_rules.ok_or_else(|| M::Error::missing_field("match_rules"))?;

                let content = LogsGrokParserRules {
                    match_rules,
                    support_rules,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsGrokParserRulesVisitor)
    }
}
