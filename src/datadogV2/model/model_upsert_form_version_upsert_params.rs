// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Concurrency control parameters for the form version upsert operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertFormVersionUpsertParams {
    /// The ETag of the latest version. Required when `match_policy` is `if_etag_match`.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option")]
    pub etag: Option<Option<String>>,
    /// If true, only a new version may be inserted; updating the current draft is not allowed.
    #[serde(rename = "insert_only")]
    pub insert_only: Option<bool>,
    /// The policy for matching the latest form version during an upsert operation.
    #[serde(rename = "match_policy")]
    pub match_policy: crate::datadogV2::model::LatestVersionMatchPolicy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertFormVersionUpsertParams {
    pub fn new(
        match_policy: crate::datadogV2::model::LatestVersionMatchPolicy,
    ) -> UpsertFormVersionUpsertParams {
        UpsertFormVersionUpsertParams {
            etag: None,
            insert_only: None,
            match_policy,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn etag(mut self, value: Option<String>) -> Self {
        self.etag = Some(value);
        self
    }

    pub fn insert_only(mut self, value: bool) -> Self {
        self.insert_only = Some(value);
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

impl<'de> Deserialize<'de> for UpsertFormVersionUpsertParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertFormVersionUpsertParamsVisitor;
        impl<'a> Visitor<'a> for UpsertFormVersionUpsertParamsVisitor {
            type Value = UpsertFormVersionUpsertParams;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut etag: Option<Option<String>> = None;
                let mut insert_only: Option<bool> = None;
                let mut match_policy: Option<crate::datadogV2::model::LatestVersionMatchPolicy> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "etag" => {
                            etag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "insert_only" => {
                            if v.is_null() {
                                continue;
                            }
                            insert_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_policy" => {
                            match_policy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _match_policy) = match_policy {
                                match _match_policy {
                                    crate::datadogV2::model::LatestVersionMatchPolicy::UnparsedObject(_match_policy) => {
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
                let match_policy =
                    match_policy.ok_or_else(|| M::Error::missing_field("match_policy"))?;

                let content = UpsertFormVersionUpsertParams {
                    etag,
                    insert_only,
                    match_policy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertFormVersionUpsertParamsVisitor)
    }
}
