// The MIT License (MIT)
//
// Copyright (c) 2016 Redox
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::hash::Hasher;

/// A DJB2 hasher.
///
/// This performs _a lot_ better than the default SipHasher.
pub struct Djb2 {
    state: u64,
}

impl Default for Djb2 {
    fn default() -> Djb2 {
        Djb2 { state: 5381 }
    }
}

impl Hasher for Djb2 {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state = (self.state << 5)
                .wrapping_add(self.state)
                .wrapping_add(b as u64);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hash::Hasher;

    #[test]
    fn test_correctness() {
        let mut hasher = Djb2::default();
        hasher.write(b"Hello, I'm Ticki.");
        assert_eq!(hasher.finish(), 17086230315052214372);
    }
}
