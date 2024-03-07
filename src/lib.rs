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

use std::error::Error;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::generator::{Generator, SnowflakeGenerator};

// ----------------------------------------------------------------

mod generator;
#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

lazy_static! {
    static ref BUILT_IN_SNOWFLAKE: Arc<Mutex<Option<SnowflakeGenerator>>> =
        Arc::new(Mutex::new(None));
}

// ----------------------------------------------------------------

pub fn snowflake() -> Arc<Mutex<Option<SnowflakeGenerator>>> {
    let mut instance = BUILT_IN_SNOWFLAKE.lock().unwrap();
    if instance.is_none() {
        *instance = Some(SnowflakeGenerator::builtin());
    }

    Arc::clone(&BUILT_IN_SNOWFLAKE)
}

// ----------------------------------------------------------------

pub fn next_id() -> Result<u64, Box<dyn Error>> {
    snowflake().lock().unwrap().as_ref().unwrap().next_id()
}
