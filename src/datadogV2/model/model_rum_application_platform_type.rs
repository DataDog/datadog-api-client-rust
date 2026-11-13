// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RUMApplicationPlatformType {
    BROWSER,
    IOS,
    ANDROID,
    REACT_NATIVE,
    FLUTTER,
    ROKU,
    ELECTRON,
    UNITY,
    KOTLIN_MULTIPLATFORM,
    REACT,
    NEXTJS,
    ANGULAR,
    VUEJS,
    NUXT,
    TANSTACK,
    MAUI,
    SALESFORCE_LWC,
    SHOPIFY,
    CPP,
    UNREAL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RUMApplicationPlatformType {
    fn to_string(&self) -> String {
        match self {
            Self::BROWSER => String::from("browser"),
            Self::IOS => String::from("ios"),
            Self::ANDROID => String::from("android"),
            Self::REACT_NATIVE => String::from("react-native"),
            Self::FLUTTER => String::from("flutter"),
            Self::ROKU => String::from("roku"),
            Self::ELECTRON => String::from("electron"),
            Self::UNITY => String::from("unity"),
            Self::KOTLIN_MULTIPLATFORM => String::from("kotlin-multiplatform"),
            Self::REACT => String::from("react"),
            Self::NEXTJS => String::from("nextjs"),
            Self::ANGULAR => String::from("angular"),
            Self::VUEJS => String::from("vuejs"),
            Self::NUXT => String::from("nuxt"),
            Self::TANSTACK => String::from("tanstack"),
            Self::MAUI => String::from("maui"),
            Self::SALESFORCE_LWC => String::from("salesforce-lwc"),
            Self::SHOPIFY => String::from("shopify"),
            Self::CPP => String::from("cpp"),
            Self::UNREAL => String::from("unreal"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RUMApplicationPlatformType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for RUMApplicationPlatformType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "browser" => Self::BROWSER,
            "ios" => Self::IOS,
            "android" => Self::ANDROID,
            "react-native" => Self::REACT_NATIVE,
            "flutter" => Self::FLUTTER,
            "roku" => Self::ROKU,
            "electron" => Self::ELECTRON,
            "unity" => Self::UNITY,
            "kotlin-multiplatform" => Self::KOTLIN_MULTIPLATFORM,
            "react" => Self::REACT,
            "nextjs" => Self::NEXTJS,
            "angular" => Self::ANGULAR,
            "vuejs" => Self::VUEJS,
            "nuxt" => Self::NUXT,
            "tanstack" => Self::TANSTACK,
            "maui" => Self::MAUI,
            "salesforce-lwc" => Self::SALESFORCE_LWC,
            "shopify" => Self::SHOPIFY,
            "cpp" => Self::CPP,
            "unreal" => Self::UNREAL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
