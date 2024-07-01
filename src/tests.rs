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

use crate::generator::{Constants, Generator, SnowflakeGenerator};

use super::*;

// ----------------------------------------------------------------

#[test]
fn test_bits() {
    assert_eq!(31, Constants::MAX_DATA_CENTER_ID);
    assert_eq!(31, Constants::MAX_WORKER_ID);
    assert_eq!(4095, Constants::SEQUENCE_MASK);

    assert_eq!(12, Constants::WORKER_ID_SHIFT);
    assert_eq!(17, Constants::CENTER_ID_SHIFT);
    assert_eq!(22, Constants::TIMESTAMP_SHIFT);
}

#[test]
fn test_next_id() {
    // 122235238222008321
    let rvt = next_id();
    assert!(rvt.is_ok());
}

#[test]
fn test_next_id_string() {
    // 122256588529602560
    let rvt = next_id_string();
    assert!(rvt.is_ok());
}

#[test]
fn test_generator_new_failed() {
    let gen = SnowflakeGenerator::new(32, 32);
    assert!(gen.is_err());
}

#[test]
fn test_generator_new_ok() {
    let gen = SnowflakeGenerator::new(31, 31);
    assert!(gen.is_ok());
}

#[test]
fn test_generator_builtin_ok() {
    let gen = SnowflakeGenerator::builtin();
    assert!(gen.is_ok());
}

#[test]
fn test_generator_next_id() {
    // 122235451737247745
    // 122_235_451_737_247_745 -> 18
    let rvt = generator().lock().unwrap().as_ref().unwrap().next_id();
    assert!(rvt.is_ok());
}

#[test]
fn test_custom_new_next_id() {
    let center_id = 16;
    let worker_id = 16;

    let gen = SnowflakeGenerator::new(center_id, worker_id);
    assert!(gen.is_ok());
    let rvt = gen.unwrap().next_id();
    assert!(rvt.is_ok());
}

// ----------------------------------------------------------------

#[test]
fn test_hash_base() {
    assert_eq!(31, hashcode::HASH_BASE);
}

// ---------------------------------------------------------------- macros

#[test]
fn test_macro_snowflake_builtin() {
    let rvt = snowflake_builtin!();
    assert!(rvt.is_ok());
}

#[test]
fn test_macro_snowflake_builtin_string() {
    let rvt = snowflake_builtin_string!();
    assert!(rvt.is_ok());
}

// ----------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "dynamic")]
mod feature_dynamic_tests {
    use std::thread;

    use crate::{dynamic_next_id, dynamic_next_id_string, infras};
    // @since 0.3.0
    use crate::generator::{Constants, Generator, SnowflakeGenerator};

    #[test]
    fn test_try_get_data_center_id() {
        let center_id = infras::try_get_data_center_id();
        assert!(center_id <= Constants::MAX_DATA_CENTER_ID);
    }

    #[test]
    fn test_try_get_worker_id() {
        let center_id = infras::try_get_data_center_id();
        let worker_id = infras::try_get_worker_id(center_id);
        assert!(worker_id <= Constants::MAX_WORKER_ID);
    }

    #[test]
    fn test_generator_dynamic() {
        let gen = SnowflakeGenerator::dynamic();
        assert!(gen.is_ok());
        let rvt = gen.unwrap().next_id();
        assert!(rvt.is_ok());
    }

    #[test]
    fn test_dynamic_next_id() {
        let rvt = dynamic_next_id();
        assert!(rvt.is_ok());
    }

    #[test]
    fn test_dynamic_next_id_string() {
        let rvt = dynamic_next_id_string();
        assert!(rvt.is_ok());
    }

    // ---------------------------------------------------------------- macros

    #[test]
    fn test_macro_snowflake_dynamic() {
        let rvt = snowflake_dynamic!();
        assert!(rvt.is_ok());
    }

    #[test]
    fn test_macro_snowflake_dynamic_string() {
        let rvt = snowflake_dynamic_string!();
        assert!(rvt.is_ok());
    }

    // ---------------------------------------------------------------- multi-thread
    #[test]
    fn test_multi_thread_sequence() {
        let generator = SnowflakeGenerator::builtin().unwrap();
        let generator_clone = generator.clone();

        assert_eq!(generator.get_sequence(), 0);
        assert_eq!(generator_clone.get_sequence(), 0);

        let h1 = thread::spawn(move || {
            for _ in 0..10 {
                generator_clone.set_sequence(generator_clone.get_sequence() + 1);
                // println!("h1: {}", generator_clone.get_sequence())
            }
        });

        let generator_clone = generator.clone();
        let h2 = thread::spawn(move || {
            for _ in 0..10 {
                generator_clone.set_sequence(generator_clone.get_sequence() + 1);
                // println!("h2: {}", generator_clone.get_sequence())
            }
        });

        h1.join().unwrap();
        h2.join().unwrap();

        assert_eq!(20, generator.get_sequence());

        // value borrowed here after move
        //assert_eq!(20, generator_clone.get_sequence());
    }
}

