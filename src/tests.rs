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

use crate::{Constants, BUILT_IN_SNOWFLAKE};

#[test]
fn test_id() {
    assert_eq!(31, Constants::MAX_DATA_CENTER_ID);
    assert_eq!(31, Constants::MAX_WORKER_ID);
    assert_eq!(4095, Constants::SEQUENCE_MASK);

    assert_eq!(12, Constants::WORKER_ID_SHIFT);
    assert_eq!(17, Constants::CENTER_ID_SHIFT);
    assert_eq!(22, Constants::TIMESTAMP_LEFT_SHIFT);
}

#[test]
fn test_next_id() {
    for _ in 0..10 {
        // 122066891375251465
        // 122_066_891_375_251_465 -> 18
        println!("{}", BUILT_IN_SNOWFLAKE.lock().unwrap().next_id().unwrap());
    }
}
