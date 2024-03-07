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

use crate::generator::{Constants, Generator};

#[test]
fn test_bits() {
    assert_eq!(31, Constants::MAX_DATA_CENTER_ID);
    assert_eq!(31, Constants::MAX_WORKER_ID);
    assert_eq!(4095, Constants::SEQUENCE_MASK);

    assert_eq!(12, Constants::WORKER_ID_SHIFT);
    assert_eq!(17, Constants::CENTER_ID_SHIFT);
    assert_eq!(22, Constants::TIMESTAMP_LEFT_SHIFT);
}

#[test]
fn test_next_id() {
    // 122235238222008321
    assert!(super::next_id().is_ok());
}

#[test]
fn test_snowflake_next_id() {
    // 122235451737247745
    // 122_235_451_737_247_745 -> 18
    assert!(super::snowflake()
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .next_id()
        .is_ok());
}
