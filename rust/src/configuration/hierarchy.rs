use std::fmt;
use std::convert::TryFrom;
use tomlenv::Error;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Hierarchy {
    Prod,
    Stage,
    Dev2Prod,
    Dev,
    Local
}

impl fmt::Display for Hierarchy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let env = match *self {
            Hierarchy::Prod => "prod",
            Hierarchy::Stage => "stage",
            Hierarchy::Dev2Prod => "dev2prod",
            Hierarchy::Dev => "dev",
            Hierarchy::Local => "local",
        };
        write!(f, "{}", env)
    }
}

impl<'a> TryFrom<&'a str> for Hierarchy {
    type Error = Error;

    fn try_from(env: &str) -> Result<Self, Error> {
        match env {
            "prod" => Ok(Hierarchy::Prod),
            "stage" => Ok(Hierarchy::Stage),
            "dev2prod" => Ok(Hierarchy::Dev2Prod),
            "dev" => Ok(Hierarchy::Dev),
            "local" => Ok(Hierarchy::Local),
            _ => Err(Error::invalid_runtime_environment(env)),
        }
    }
}

impl TryFrom<String> for Hierarchy {
    type Error = Error;

    fn try_from(env: String) -> Result<Self, Error> {
        match &env[..] {
            "prod" => Ok(Hierarchy::Prod),
            "stage" => Ok(Hierarchy::Stage),
            "dev2prod" => Ok(Hierarchy::Dev2Prod),
            "dev" => Ok(Hierarchy::Dev),
            "local" => Ok(Hierarchy::Local),
            _ => Err(Error::invalid_runtime_environment(&env)),
        }
    }
}

impl Serialize for Hierarchy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Hierarchy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Hierarchy, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct MyHierarchyVisitor;

        impl<'de> de::Visitor<'de> for MyHierarchyVisitor {
            type Value = Hierarchy;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid environment")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Hierarchy, E>
                where
                    E: de::Error,
            {
                TryFrom::try_from(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_string(MyHierarchyVisitor)
    }
}