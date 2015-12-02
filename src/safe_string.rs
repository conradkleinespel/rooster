// Copyright 2014 The Rooster Developers
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

use std::ops::Drop;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SafeString {
    inner: String,
}

impl SafeString {
    pub fn new(inner: String) -> SafeString {
        SafeString {
            inner: inner,
        }
    }
}

impl Drop for SafeString {
    fn drop(&mut self) {
        self.inner.clear();
        for _ in 0 .. self.inner.capacity() {
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
