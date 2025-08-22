// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueLanguage {
    JAVASCRIPT,
    JVM,
    RUBY,
    TYPESCRIPT,
    JAVA,
    KOTLIN,
    SCALA,
    GROOVY,
    CLOJURE,
    GO,
    PYTHON,
    PHP,
    DOT_NET,
    C_SHARP,
    C_PLUS_PLUS,
    OBJECTIVE_C,
    SWIFT,
    BRIGHTSCRIPT,
    C,
    ELIXIR,
    ERLANG,
    PERL,
    HASKELL,
    RUST,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueLanguage {
    fn to_string(&self) -> String {
        match self {
            Self::JAVASCRIPT => String::from("javascript"),
            Self::JVM => String::from("jvm"),
            Self::RUBY => String::from("ruby"),
            Self::TYPESCRIPT => String::from("typescript"),
            Self::JAVA => String::from("java"),
            Self::KOTLIN => String::from("kotlin"),
            Self::SCALA => String::from("scala"),
            Self::GROOVY => String::from("groovy"),
            Self::CLOJURE => String::from("clojure"),
            Self::GO => String::from("go"),
            Self::PYTHON => String::from("python"),
            Self::PHP => String::from("php"),
            Self::DOT_NET => String::from("dot_net"),
            Self::C_SHARP => String::from("c_sharp"),
            Self::C_PLUS_PLUS => String::from("c_plus_plus"),
            Self::OBJECTIVE_C => String::from("objective_c"),
            Self::SWIFT => String::from("swift"),
            Self::BRIGHTSCRIPT => String::from("brightscript"),
            Self::C => String::from("c"),
            Self::ELIXIR => String::from("elixir"),
            Self::ERLANG => String::from("erlang"),
            Self::PERL => String::from("perl"),
            Self::HASKELL => String::from("haskell"),
            Self::RUST => String::from("rust"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssueLanguage {
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

impl<'de> Deserialize<'de> for IssueLanguage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "javascript" => Self::JAVASCRIPT,
            "jvm" => Self::JVM,
            "ruby" => Self::RUBY,
            "typescript" => Self::TYPESCRIPT,
            "java" => Self::JAVA,
            "kotlin" => Self::KOTLIN,
            "scala" => Self::SCALA,
            "groovy" => Self::GROOVY,
            "clojure" => Self::CLOJURE,
            "go" => Self::GO,
            "python" => Self::PYTHON,
            "php" => Self::PHP,
            "dot_net" => Self::DOT_NET,
            "c_sharp" => Self::C_SHARP,
            "c_plus_plus" => Self::C_PLUS_PLUS,
            "objective_c" => Self::OBJECTIVE_C,
            "swift" => Self::SWIFT,
            "brightscript" => Self::BRIGHTSCRIPT,
            "c" => Self::C,
            "elixir" => Self::ELIXIR,
            "erlang" => Self::ERLANG,
            "perl" => Self::PERL,
            "haskell" => Self::HASKELL,
            "rust" => Self::RUST,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
