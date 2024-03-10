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

/// Use the builtin default generator[`crate::generator::SnowflakeGenerator::builtin`].
///
#[macro_export]
macro_rules! snowflake_builtin {
    () => {
        next_id()
    };
}

/// Use the builtin default generator[`super::generator::SnowflakeGenerator::builtin`].
///
#[macro_export]
macro_rules! snowflake_builtin_string {
    () => {
        next_id_string()
    };
}

/// Use the builtin dynamic generator[`super::generator::SnowflakeGenerator::dynamic`] by features = [`"dynamic"`].
///
#[macro_export]
#[cfg(feature = "dynamic")]
macro_rules! snowflake_dynamic {
    () => {
        dynamic_next_id()
    };
}

/// Use the builtin dynamic generator[`crate::generator::SnowflakeGenerator::dynamic`] by features = [`"dynamic"`].
#[macro_export]
#[cfg(feature = "dynamic")]
macro_rules! snowflake_dynamic_string {
    () => {
        dynamic_next_id_string()
    };
}