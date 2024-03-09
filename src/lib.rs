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

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::generator::{Generator, SnowflakeError, SnowflakeGenerator};

// ----------------------------------------------------------------

pub mod generator;

#[cfg(feature = "dynamic")]
pub mod infras;

mod hashcode;
#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

lazy_static! {
    static ref BUILT_IN_SNOWFLAKE: Arc<Mutex<Option<SnowflakeGenerator>>> =
        Arc::new(Mutex::new(None));
}

// ----------------------------------------------------------------

fn generator() -> Arc<Mutex<Option<SnowflakeGenerator>>> {
    let mut instance = BUILT_IN_SNOWFLAKE.lock().unwrap();
    if instance.is_none() {
        *instance = Some(SnowflakeGenerator::builtin().unwrap());
    }

    Arc::clone(&BUILT_IN_SNOWFLAKE)
}

// ----------------------------------------------------------------

/// [`next_id`]
///
/// Generates and returns a unique ID based on the [`Generator::next_id`] function.
///
/// ## Return
///
/// Returns a `Result<u64, SnowflakeError>` where:
///
/// - `Ok(u64)`: Represents a successfully generated unique ID.
/// - `Err(SnowflakeError)`: Indicates an error occurred, such as the system clock moved backwards.
///
/// # Examples
///
/// ```rust
/// use snowflaker::next_id;
///
/// let rvt = next_id();
/// assert!(rvt.is_ok());
/// ```
pub fn next_id() -> Result<u64, SnowflakeError> {
    generator().lock().unwrap().as_ref().unwrap().next_id()
}

/// [`next_id_string`]
///
/// Generates and returns a unique String ID.
///
/// ## Return
///
/// Returns a `Result<u64, SnowflakeError>` where:
///
/// - `Ok(u64)`: Represents a successfully generated unique ID.
/// - `Err(SnowflakeError)`: Indicates an error occurred, such as the system clock moved backwards.
///
/// # Examples
///
/// ```rust
/// use snowflaker::next_id_string;
///
/// let rvt = next_id_string();
/// assert!(rvt.is_ok());
/// ```
pub fn next_id_string() -> Result<String, SnowflakeError> {
    next_id().map(|v| v.to_string())
}
