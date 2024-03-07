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
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// ----------------------------------------------------------------
// ----------------------------------------------------------------

#[derive(Debug)]
pub enum SnowflakeError {
    CenterIdInvalid,
    WorkerIdInvalid,
    SystemTimeError,
    ClockMovedBackwards,
}

impl Display for SnowflakeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            SnowflakeError::CenterIdInvalid => write!(f, "Data Center ID out of range"),
            SnowflakeError::WorkerIdInvalid => write!(f, "Worker ID out of range"),
            SnowflakeError::SystemTimeError => write!(f, "SystemTime before UNIX EPOCH!"),
            SnowflakeError::ClockMovedBackwards => {
                write!(f, "Clock moved backwards. Refusing to generate id")
            }
        }
    }
}

impl Error for SnowflakeError {}

// ----------------------------------------------------------------

pub struct Constants;

impl Constants {
    pub const EPOCH: u64 = 1680646028000; // 2023-04-05 06:07:08

    pub const DATA_CENTER_ID_BITS: u64 = 5;
    pub const WORKER_ID_BITS: u64 = 5;
    pub const SEQUENCE_BITS: u64 = 12;

    pub const MAX_DATA_CENTER_ID: u64 = !(!0 << Constants::DATA_CENTER_ID_BITS);
    pub const MAX_WORKER_ID: u64 = !(!0 << Constants::WORKER_ID_BITS);
    pub const SEQUENCE_MASK: u64 = !(!0 << Constants::SEQUENCE_BITS);

    pub const WORKER_ID_SHIFT: u64 = Constants::SEQUENCE_BITS;
    pub const CENTER_ID_SHIFT: u64 = Constants::SEQUENCE_BITS + Constants::WORKER_ID_BITS;
    pub const TIMESTAMP_LEFT_SHIFT: u64 =
        Constants::DATA_CENTER_ID_BITS + Constants::WORKER_ID_BITS + Constants::SEQUENCE_BITS;
}

// ----------------------------------------------------------------

pub trait Generator {
    fn next_id(&self) -> Result<u64, SnowflakeError>;

    fn time_gen() -> Result<u64, SnowflakeError>;

    fn til_next_millis(last_timestamp: u64) -> Result<u64, SnowflakeError>;
}

// ----------------------------------------------------------------

pub struct SnowflakeGenerator {
    center_id: u64,
    worker_id: u64,
    sequence: AtomicU64,
    last_timestamp: AtomicU64,
}

impl SnowflakeGenerator {
    pub fn builtin() -> Result<Self, SnowflakeError> {
        SnowflakeGenerator::new(1, 1)
    }
    pub fn new(center_id: u64, worker_id: u64) -> Result<Self, SnowflakeError> {
        if center_id > Constants::MAX_DATA_CENTER_ID {
            return Err(SnowflakeError::CenterIdInvalid);
        }

        if center_id > Constants::MAX_WORKER_ID {
            return Err(SnowflakeError::WorkerIdInvalid);
        }

        Ok(SnowflakeGenerator {
            center_id,
            worker_id,
            sequence: AtomicU64::new(0),
            last_timestamp: AtomicU64::new(0),
        })
    }
}

impl Generator for SnowflakeGenerator {
    fn next_id(&self) -> Result<u64, SnowflakeError> {
        let mut timestamp = Self::time_gen().unwrap();

        loop {
            let last_timestamp = self.last_timestamp.load(Ordering::Relaxed);

            if timestamp < last_timestamp {
                timestamp = last_timestamp;
            }

            // TODO ?
            if timestamp != last_timestamp {
                self.sequence.store(0, Ordering::Relaxed);
            }

            let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);

            if sequence <= Constants::SEQUENCE_MASK {
                let id = ((timestamp - Constants::EPOCH) << Constants::TIMESTAMP_LEFT_SHIFT)
                    | (self.center_id << Constants::CENTER_ID_SHIFT)
                    | (self.worker_id << Constants::WORKER_ID_SHIFT)
                    | sequence;

                self.last_timestamp.store(timestamp, Ordering::Relaxed);

                return Ok(id);
            }

            timestamp = Self::til_next_millis(timestamp).unwrap();
        }
    }

    fn time_gen() -> Result<u64, SnowflakeError> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => Ok(now.as_millis() as u64),
            Err(_) => Err(SnowflakeError::SystemTimeError),
        }
    }

    fn til_next_millis(last_timestamp: u64) -> Result<u64, SnowflakeError> {
        let mut next = Self::time_gen().unwrap();
        while next <= last_timestamp {
            next = Self::time_gen().unwrap();
        }

        Ok(next)
    }
}
