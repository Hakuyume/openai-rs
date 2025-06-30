macro_rules! impl_serde {
    ($ty:ident, $value:literal) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                if value == $value {
                    Ok(Self)
                } else {
                    Err(<D::Error as serde::de::Error>::invalid_value(
                        serde::de::Unexpected::Str(&value),
                        &$value,
                    ))
                }
            }
        }
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                $value.serialize(serializer)
            }
        }
    };
}

mod generated;
pub use generated::*;

#[cfg(test)]
mod tests;
