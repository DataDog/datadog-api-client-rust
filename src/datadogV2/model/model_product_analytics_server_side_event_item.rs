// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Product Analytics server-side event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsServerSideEventItem {
    /// The account linked to your event.
    #[serde(rename = "account")]
    pub account: Option<crate::datadogV2::model::ProductAnalyticsServerSideEventItemAccount>,
    /// The application in which you want to send your events.
    #[serde(rename = "application")]
    pub application: crate::datadogV2::model::ProductAnalyticsServerSideEventItemApplication,
    /// Fields used for the event.
    #[serde(rename = "event")]
    pub event: crate::datadogV2::model::ProductAnalyticsServerSideEventItemEvent,
    /// The session linked to your event.
    #[serde(rename = "session")]
    pub session: Option<crate::datadogV2::model::ProductAnalyticsServerSideEventItemSession>,
    /// The type of Product Analytics event. Must be `server` for server-side events.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ProductAnalyticsServerSideEventItemType,
    /// The user linked to your event.
    #[serde(rename = "usr")]
    pub usr: Option<crate::datadogV2::model::ProductAnalyticsServerSideEventItemUsr>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsServerSideEventItem {
    pub fn new(
        application: crate::datadogV2::model::ProductAnalyticsServerSideEventItemApplication,
        event: crate::datadogV2::model::ProductAnalyticsServerSideEventItemEvent,
        type_: crate::datadogV2::model::ProductAnalyticsServerSideEventItemType,
    ) -> ProductAnalyticsServerSideEventItem {
        ProductAnalyticsServerSideEventItem {
            account: None,
            application,
            event,
            session: None,
            type_,
            usr: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsServerSideEventItemAccount,
    ) -> Self {
        self.account = Some(value);
        self
    }

    pub fn session(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsServerSideEventItemSession,
    ) -> Self {
        self.session = Some(value);
        self
    }

    pub fn usr(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsServerSideEventItemUsr,
    ) -> Self {
        self.usr = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsServerSideEventItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsServerSideEventItemVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsServerSideEventItemVisitor {
            type Value = ProductAnalyticsServerSideEventItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemAccount,
                > = None;
                let mut application: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemApplication,
                > = None;
                let mut event: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemEvent,
                > = None;
                let mut session: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemSession,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemType,
                > = None;
                let mut usr: Option<
                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemUsr,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account" => {
                            if v.is_null() {
                                continue;
                            }
                            account = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application" => {
                            application =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event" => {
                            event = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session" => {
                            if v.is_null() {
                                continue;
                            }
                            session = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProductAnalyticsServerSideEventItemType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "usr" => {
                            if v.is_null() {
                                continue;
                            }
                            usr = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application =
                    application.ok_or_else(|| M::Error::missing_field("application"))?;
                let event = event.ok_or_else(|| M::Error::missing_field("event"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ProductAnalyticsServerSideEventItem {
                    account,
                    application,
                    event,
                    session,
                    type_,
                    usr,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsServerSideEventItemVisitor)
    }
}
