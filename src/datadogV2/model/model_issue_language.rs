// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueLanguage {
    BRIGHTSCRIPT,
    C,
    C_PLUS_PLUS,
    C_SHARP,
    CLOJURE,
    DOT_NET,
    ELIXIR,
    ERLANG,
    GO,
    GROOVY,
    HASKELL,
    HCL,
    JAVA,
    JAVASCRIPT,
    JVM,
    KOTLIN,
    OBJECTIVE_C,
    PERL,
    PHP,
    PYTHON,
    RUBY,
    RUST,
    SCALA,
    SWIFT,
    TERRAFORM,
    TYPESCRIPT,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssueLanguage {
    fn to_string(&self) -> String {
        match self {
            Self::BRIGHTSCRIPT => String::from("BRIGHTSCRIPT"),
            Self::C => String::from("C"),
            Self::C_PLUS_PLUS => String::from("C_PLUS_PLUS"),
            Self::C_SHARP => String::from("C_SHARP"),
            Self::CLOJURE => String::from("CLOJURE"),
            Self::DOT_NET => String::from("DOT_NET"),
            Self::ELIXIR => String::from("ELIXIR"),
            Self::ERLANG => String::from("ERLANG"),
            Self::GO => String::from("GO"),
            Self::GROOVY => String::from("GROOVY"),
            Self::HASKELL => String::from("HASKELL"),
            Self::HCL => String::from("HCL"),
            Self::JAVA => String::from("JAVA"),
            Self::JAVASCRIPT => String::from("JAVASCRIPT"),
            Self::JVM => String::from("JVM"),
            Self::KOTLIN => String::from("KOTLIN"),
            Self::OBJECTIVE_C => String::from("OBJECTIVE_C"),
            Self::PERL => String::from("PERL"),
            Self::PHP => String::from("PHP"),
            Self::PYTHON => String::from("PYTHON"),
            Self::RUBY => String::from("RUBY"),
            Self::RUST => String::from("RUST"),
            Self::SCALA => String::from("SCALA"),
            Self::SWIFT => String::from("SWIFT"),
            Self::TERRAFORM => String::from("TERRAFORM"),
            Self::TYPESCRIPT => String::from("TYPESCRIPT"),
            Self::UNKNOWN => String::from("UNKNOWN"),
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
            "BRIGHTSCRIPT" => Self::BRIGHTSCRIPT,
            "C" => Self::C,
            "C_PLUS_PLUS" => Self::C_PLUS_PLUS,
            "C_SHARP" => Self::C_SHARP,
            "CLOJURE" => Self::CLOJURE,
            "DOT_NET" => Self::DOT_NET,
            "ELIXIR" => Self::ELIXIR,
            "ERLANG" => Self::ERLANG,
            "GO" => Self::GO,
            "GROOVY" => Self::GROOVY,
            "HASKELL" => Self::HASKELL,
            "HCL" => Self::HCL,
            "JAVA" => Self::JAVA,
            "JAVASCRIPT" => Self::JAVASCRIPT,
            "JVM" => Self::JVM,
            "KOTLIN" => Self::KOTLIN,
            "OBJECTIVE_C" => Self::OBJECTIVE_C,
            "PERL" => Self::PERL,
            "PHP" => Self::PHP,
            "PYTHON" => Self::PYTHON,
            "RUBY" => Self::RUBY,
            "RUST" => Self::RUST,
            "SCALA" => Self::SCALA,
            "SWIFT" => Self::SWIFT,
            "TERRAFORM" => Self::TERRAFORM,
            "TYPESCRIPT" => Self::TYPESCRIPT,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
