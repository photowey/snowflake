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

// error[E0554]: `#![feature]` may not be used on the stable release channel
// #![feature(doc_cfg)]

// ----------------------------------------------------------------

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::generator::{Generator, SnowflakeError, SnowflakeGenerator};

// ----------------------------------------------------------------

/// @since 0.1.0
pub mod generator;

/// @since 0.2.0
pub mod hashcode;
/// @since 0.2.0
#[cfg(feature = "dynamic")]
#[doc(cfg(feature = "dynamic"))]
pub mod infras;

/// @since 0.3.0
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

lazy_static! {
    static ref BUILT_IN_SNOWFLAKE: Arc<Mutex<Option<SnowflakeGenerator>>> =
        Arc::new(Mutex::new(None));
}

lazy_static! {
    static ref BUILT_IN_SNOWFLAKE_DYNAMIC: Arc<Mutex<Option<SnowflakeGenerator>>> =
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

#[cfg(feature = "dynamic")]
#[doc(cfg(feature = "dynamic"))]
fn dynamic_generator() -> Arc<Mutex<Option<SnowflakeGenerator>>> {
    let mut instance = BUILT_IN_SNOWFLAKE_DYNAMIC.lock().unwrap();
    if instance.is_none() {
        *instance = Some(SnowflakeGenerator::dynamic().unwrap());
    }

    Arc::clone(&BUILT_IN_SNOWFLAKE_DYNAMIC)
}

// ----------------------------------------------------------------

/// Use builtin default [`Generator`] `impl` instance [`SnowflakeGenerator::builtin`]
/// generates and returns a unique ID based on the [`Generator::next_id`] function.
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

/// Use builtin default [`Generator`] `impl` instance [`SnowflakeGenerator::builtin`]
/// generates and returns a unique String ID.
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

// ----------------------------------------------------------------

/// Use builtin default [`Generator`] `impl` instance [`SnowflakeGenerator::dynamic`]
/// generates and returns a unique ID based on the [`Generator::next_id`] function.
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
/// use snowflaker::dynamic_next_id;
///
/// let rvt = dynamic_next_id();
/// assert!(rvt.is_ok());
/// ```
#[cfg(feature = "dynamic")]
#[doc(cfg(feature = "dynamic"))]
pub fn dynamic_next_id() -> Result<u64, SnowflakeError> {
    dynamic_generator().lock().unwrap().as_ref().unwrap().next_id()
}

/// Use builtin default [`Generator`] `impl` instance [`SnowflakeGenerator::dynamic`]
/// generates and returns a unique String ID.
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
/// use snowflaker::dynamic_next_id_string;
///
/// let rvt = dynamic_next_id_string();
/// assert!(rvt.is_ok());
/// ```
#[cfg(feature = "dynamic")]
#[doc(cfg(feature = "dynamic"))]
pub fn dynamic_next_id_string() -> Result<String, SnowflakeError> {
    dynamic_next_id().map(|v| v.to_string())
}