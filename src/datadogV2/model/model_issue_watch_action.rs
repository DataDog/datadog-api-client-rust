// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Action applied to the issue when the watch condition is met.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueWatchAction {
    /// State of the issue
    #[serde(rename = "from")]
    pub from: Option<crate::datadogV2::model::IssueState>,
    /// Kind of watch action.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV2::model::IssueWatchActionKind,
    /// State of the issue
    #[serde(rename = "to")]
    pub to: Option<crate::datadogV2::model::IssueState>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueWatchAction {
    pub fn new(kind: crate::datadogV2::model::IssueWatchActionKind) -> IssueWatchAction {
        IssueWatchAction {
            from: None,
            kind,
            to: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: crate::datadogV2::model::IssueState) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: crate::datadogV2::model::IssueState) -> Self {
        self.to = Some(value);
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

impl<'de> Deserialize<'de> for IssueWatchAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueWatchActionVisitor;
        impl<'a> Visitor<'a> for IssueWatchActionVisitor {
            type Value = IssueWatchAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<crate::datadogV2::model::IssueState> = None;
                let mut kind: Option<crate::datadogV2::model::IssueWatchActionKind> = None;
                let mut to: Option<crate::datadogV2::model::IssueState> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _from) = from {
                                match _from {
                                    crate::datadogV2::model::IssueState::UnparsedObject(_from) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::IssueWatchActionKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _to) = to {
                                match _to {
                                    crate::datadogV2::model::IssueState::UnparsedObject(_to) => {
                                        _unparsed = true;
                                    }
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
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;

                let content = IssueWatchAction {
                    from,
                    kind,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueWatchActionVisitor)
    }
}
