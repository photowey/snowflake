/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

/// [`HASH_BASE`] 31
pub const HASH_BASE: u64 = (1 << 5) - 1;

// ----------------------------------------------------------------

/// Defines a function to compute a hash-code for types.
///
/// Implementors of this trait can produce a consistent 64-bit hash-value that
/// is unique for instances with the same content (ideally).
///
/// @since 0.2.0
pub trait HashCode {
    /// Generates and returns a hash-value based on the contents of `self`.
    ///
    /// # Returns
    ///
    /// - `u64`: A 64-bit unsigned integer representing the hash code.
    fn hashcode(&self) -> u64;
}

/// Implement the [`HashCode`] trait for the [`String`] type.
impl HashCode for String {
    fn hashcode(&self) -> u64 {
        let mut hash: u64 = 0;
        for ch in self.chars() {
            hash = HASH_BASE * hash + ch as u64;
        }
        hash
    }
}
