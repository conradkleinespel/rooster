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

pub struct SafeVec {
    inner: Vec<u8>,
}

impl SafeVec {
    pub fn new(inner: Vec<u8>) -> SafeVec {
        SafeVec {
            inner: inner,
        }
    }
}

impl Drop for SafeVec {
    fn drop(&mut self) {
        self.inner.clear();
        for _ in 0 .. self.inner.capacity() {
            self.inner.push(0u8);
        }
    }
}

impl Deref for SafeVec {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.inner.deref()
    }
}
