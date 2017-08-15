// Copyright 2014-2017 The Rooster Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::ops::Drop;
use std::ops::Deref;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafeString {
    pub inner: String,
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(String::from(v))
    }
    type Value = String;
}

impl SafeString {
    pub fn new(inner: String) -> SafeString {
        SafeString { inner: inner }
    }
}

impl Drop for SafeString {
    fn drop(&mut self) {
        self.inner.clear();
        for _ in 0..self.inner.capacity() {
            self.inner.push('0');
        }
    }
}

impl Deref for SafeString {
    type Target = str;

    fn deref(&self) -> &str {
        self.inner.deref()
    }
}

impl Serialize for SafeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.deref())
    }
}

impl<'de> Deserialize<'de> for SafeString {
    fn deserialize<D>(deserializer: D) -> Result<SafeString, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor).map(
            |parsed_value| SafeString { inner: parsed_value },
        )
    }
}

#[cfg(test)]
mod test {
    use safe_string::SafeString;
    use serde_json;
    use serde_json::Error;

    #[test]
    fn safe_string_serialization() {
        let s = SafeString { inner: String::from("blabla") };

        match serde_json::to_string(&s) {
            Ok(json) => assert_eq!("\"blabla\"", json),
            Err(_) => panic!("Serialization failed, somehow"),
        }
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    pub struct TestStruct {
        password: SafeString,
    }

    #[test]
    fn safe_string_within_struct_serialization() {
        let ts = TestStruct { password: SafeString { inner: String::from("blabla") } };

        match serde_json::to_string(&ts) {
            Ok(json) => assert_eq!("{\"password\":\"blabla\"}", json),
            Err(_) => panic!("Serialization failed, somehow"),
        }
    }

    #[test]
    fn safe_string_deserialization() {
        let s = "\"blabla\"";

        let res: Result<SafeString, Error> = serde_json::from_str(s);

        match res {
            Ok(ss) => assert_eq!(ss, SafeString { inner: String::from("blabla") }),
            Err(_) => panic!("Deserialization failed"),
        }
    }

    #[test]
    fn safe_string_within_struct_deserialization() {
        let json = "{\"password\":\"blabla\"}";
        let res: Result<TestStruct, Error> = serde_json::from_str(json);
        match res {
            Ok(ts) => {
                assert_eq!(
                    ts,
                    TestStruct { password: SafeString { inner: String::from("blabla") } }
                )
            }
            Err(_) => panic!("Deserialization failed"),
        }
    }
}
