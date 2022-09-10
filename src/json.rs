use std::borrow::Borrow;

use num::BigInt;

/// A string that definitely contains a number.
pub struct StrNum {
    representation: String,
}

impl StrNum {
    pub fn representation(&self) -> &str {
        self.representation.as_ref()
    }
}

// Allowing `fallible_impl_from` because `StrNum` contains a number by definition.
#[allow(clippy::fallible_impl_from)]
impl From<StrNum> for BigInt {
    fn from(number: StrNum) -> Self {
        number.representation.parse::<Self>().unwrap()
    }
}

impl<Num: Borrow<serde_json::Number>> From<Num> for StrNum {
    fn from(number: Num) -> Self {
        Self {
            representation: number.borrow().to_string(),
        }
    }
}

pub trait Json
where
    Self: Sized,
{
    fn number(&self) -> Option<StrNum>;
    fn string(&self) -> Option<&str>;
    fn get(&self, key: &str) -> Option<&serde_json::Value>;
    fn index(&self, index: usize) -> Option<&serde_json::Value>;
}

impl<V: Borrow<serde_json::Value>> Json for Option<V> {
    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        match self {
            Some(object) => {
                match object.borrow().as_object() {
                    Some(map) => map.get(key),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn index(&self, index: usize) -> Option<&serde_json::Value> {
        match self {
            Some(object) => {
                match object.borrow().as_array() {
                    Some(array) => array.get(index),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn number(&self) -> Option<StrNum> {
        match self {
            Some(object) => {
                match object.borrow() {
                    serde_json::Value::Number(number) => Some(number.into()),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn string(&self) -> Option<&str> {
        match self {
            Some(object) => object.borrow().as_str(),
            None => None,
        }
    }
}

impl Json for serde_json::Value {
    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.get(key)
    }

    fn index(&self, index: usize) -> Option<&serde_json::Value> {
        self.as_array().and_then(|array| {
            if index >= array.len() {
                None
            } else {
                Some(&array[index])
            }
        })
    }

    fn number(&self) -> Option<StrNum> {
        match self {
            Self::Number(number) => Some(number.into()),
            _ => None,
        }
    }

    fn string(&self) -> Option<&str> {
        self.as_str()
    }
}
