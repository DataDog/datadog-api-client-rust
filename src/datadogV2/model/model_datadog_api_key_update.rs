// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the `DatadogAPIKey` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatadogAPIKeyUpdate {
    /// The `DatadogAPIKeyUpdate` `api_key`.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    /// The `DatadogAPIKeyUpdate` `app_key`.
    #[serde(rename = "app_key")]
    pub app_key: Option<String>,
    /// The `DatadogAPIKeyUpdate` `datacenter`.
    #[serde(rename = "datacenter")]
    pub datacenter: Option<String>,
    /// Custom subdomain used for Datadog URLs generated with this Connection. For example, if this org uses `<https://acme.datadoghq.com`> to access Datadog, set this field to `acme`. If this field is omitted, generated URLs will use the default site URL for its datacenter (see [<https://docs.datadoghq.com/getting_started/site]>(<https://docs.datadoghq.com/getting_started/site>)).
    #[serde(rename = "subdomain")]
    pub subdomain: Option<String>,
    /// The definition of the `DatadogAPIKey` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DatadogAPIKeyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatadogAPIKeyUpdate {
    pub fn new(type_: crate::datadogV2::model::DatadogAPIKeyType) -> DatadogAPIKeyUpdate {
        DatadogAPIKeyUpdate {
            api_key: None,
            app_key: None,
            datacenter: None,
            subdomain: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn app_key(mut self, value: String) -> Self {
        self.app_key = Some(value);
        self
    }

    pub fn datacenter(mut self, value: String) -> Self {
        self.datacenter = Some(value);
        self
    }

    pub fn subdomain(mut self, value: String) -> Self {
        self.subdomain = Some(value);
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

impl<'de> Deserialize<'de> for DatadogAPIKeyUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatadogAPIKeyUpdateVisitor;
        impl<'a> Visitor<'a> for DatadogAPIKeyUpdateVisitor {
            type Value = DatadogAPIKeyUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut app_key: Option<String> = None;
                let mut datacenter: Option<String> = None;
                let mut subdomain: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::DatadogAPIKeyType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "app_key" => {
                            if v.is_null() {
                                continue;
                            }
                            app_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datacenter" => {
                            if v.is_null() {
                                continue;
                            }
                            datacenter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subdomain" => {
                            if v.is_null() {
                                continue;
                            }
                            subdomain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DatadogAPIKeyType::UnparsedObject(
                                        _type_,
                                    ) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DatadogAPIKeyUpdate {
                    api_key,
                    app_key,
                    datacenter,
                    subdomain,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatadogAPIKeyUpdateVisitor)
    }
}
