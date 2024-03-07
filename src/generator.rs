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

use chronounit::TimeUnit;

// ----------------------------------------------------------------

/// [`SnowflakeError`] Snowflake custom enum error.
#[derive(Debug, Clone)]
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

/// [`Constants`] Generator common constants.
pub struct Constants;

impl Constants {
    /// [`EPOCH`] `2023-04-05 06:07:08`
    pub const EPOCH: u64 = 1680646028000; // 2023-04-05 06:07:08

    /// [`DATA_CENTER_ID_BITS`] data-center bits: 5
    pub const DATA_CENTER_ID_BITS: u64 = 5;
    /// [`WORKER_ID_BITS`] worker bits: 5
    pub const WORKER_ID_BITS: u64 = 5;

    /// [`SEQUENCE_BITS`] sequence bits: 12
    pub const SEQUENCE_BITS: u64 = 12;

    /// [`MAX_DATA_CENTER_ID`] max data-center ID: 31
    pub const MAX_DATA_CENTER_ID: u64 = !(!0 << Constants::DATA_CENTER_ID_BITS);
    /// [`SEQUENCE_MASK`] max worker ID: 31
    pub const MAX_WORKER_ID: u64 = !(!0 << Constants::WORKER_ID_BITS);

    /// [`SEQUENCE_MASK`] sequence mask: 4095
    pub const SEQUENCE_MASK: u64 = !(!0 << Constants::SEQUENCE_BITS);

    /// [`WORKER_ID_SHIFT`] worker ID shift: 12
    pub const WORKER_ID_SHIFT: u64 = Constants::SEQUENCE_BITS;
    /// [`CENTER_ID_SHIFT`] center ID shift: 17
    pub const CENTER_ID_SHIFT: u64 = Constants::SEQUENCE_BITS + Constants::WORKER_ID_BITS;

    /// [`TIMESTAMP_SHIFT`] timestamp left shift: 22
    pub const TIMESTAMP_SHIFT: u64 =
        Constants::DATA_CENTER_ID_BITS + Constants::WORKER_ID_BITS + Constants::SEQUENCE_BITS;

    // ----------------------------------------------------------------

    /// [`DEFAULT_DATA_CENTER_ID`] default data-center ID: 1
    pub const DEFAULT_DATA_CENTER_ID: u64 = 1;

    /// [`DEFAULT_WORKER_ID`] default worker ID: 1
    pub const DEFAULT_WORKER_ID: u64 = 1;
}

// ----------------------------------------------------------------

/// [`Generator`] Unique ID generator trait
pub trait Generator {
    /// [`next_id`] Generate next ID.
    fn next_id(&self) -> Result<u64, SnowflakeError>;

    /// [`time_gen`] Get current timestamp.
    fn time_gen() -> Result<u64, SnowflakeError>;

    /// [`til_next_millis`] Get next timestamp.
    fn til_next_millis(last_timestamp: u64) -> Result<u64, SnowflakeError>;
}

// ----------------------------------------------------------------

/// [`SnowflakeGenerator`] The builtin impl of [`Generator`]
pub struct SnowflakeGenerator {
    center_id: u64,
    worker_id: u64,
    sequence: AtomicU64,
    last_timestamp: AtomicU64,
}

impl SnowflakeGenerator {
    /// [`builtin`]
    ///
    /// Returns a new instance of [`SnowflakeGenerator`] with built-in defaults.
    ///
    /// This function, `builtin`, instantiates a `SnowflakeGenerator` using the predefined constants for
    /// data-center ID and worker ID. These constants are [`Constants::DEFAULT_DATA_CENTER_ID`] and
    /// [`Constants::DEFAULT_WORKER_ID`] respectively.
    ///
    /// The return type is a `Result` where the success variant contains the initialized
    /// `Self` (a [`SnowflakeGenerator`]) and the error variant contains a [`SnowflakeError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use snowflaker::generator::SnowflakeGenerator;
    ///
    /// let gen = SnowflakeGenerator::builtin();
    /// assert!(gen.is_ok());
    /// ```
    pub fn builtin() -> Result<Self, SnowflakeError> {
        SnowflakeGenerator::new(
            Constants::DEFAULT_DATA_CENTER_ID,
            Constants::DEFAULT_WORKER_ID,
        )
    }

    /// [`new`]
    ///
    /// Constructs a new [`SnowflakeGenerator`] instance.
    ///
    /// # Arguments
    /// - `center_id`: An identifier for the data-center, represented as a `u64`. It must be within the defined maximum limit.
    /// - `worker_id`: An identifier for the worker node within the data-center, also represented as a `u64`. This too must not exceed its predefined maximum value.

    /// # Returns
    /// - `Ok(Self)`: If both `center_id` and `worker_id` are valid, returns a new [`SnowflakeGenerator`] instance.
    /// - `Err(SnowflakeError)`: If either `center_id` or `worker_id` is invalid, returns an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use snowflaker::generator::SnowflakeGenerator;
    ///
    /// let gen = SnowflakeGenerator::new(31, 31);
    /// assert!(gen.is_ok());
    ///
    /// let gen = SnowflakeGenerator::new(32, 32);
    /// assert!(gen.is_err());
    /// ```
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
    /// [`next_id`]
    ///
    /// Generates and returns a unique ID based on the
    /// current timestamp, data-center ID, worker ID, and an incrementing sequence number.
    /// It ensures that IDs are strictly increasing and handles potential clock drift or time going backwards.
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
    /// use snowflaker::generator::{Generator, SnowflakeGenerator};
    ///
    /// let gen = SnowflakeGenerator::new(31, 31);
    /// let rvt = gen.unwrap().next_id();
    /// assert!(rvt.is_ok());
    /// ```
    fn next_id(&self) -> Result<u64, SnowflakeError> {
        let mut timestamp = Self::time_gen().unwrap();
        let last_timestamp = self.last_timestamp.load(Ordering::Relaxed);

        if timestamp < last_timestamp {
            let delta = last_timestamp - timestamp;
            if delta <= 1 << 3 {
                TimeUnit::Milliseconds.sleep(delta << 1);
                timestamp = Self::time_gen().unwrap();

                if timestamp < last_timestamp {
                    return Err(SnowflakeError::ClockMovedBackwards);
                }
            }
        }

        let mut sequence = self.sequence.fetch_add(1, Ordering::Relaxed);

        if timestamp == last_timestamp {
            sequence = (sequence + 1) & Constants::SEQUENCE_MASK;
            if sequence == 0 {
                timestamp = Self::til_next_millis(timestamp).unwrap();
            }
        } else {
            sequence &= Constants::SEQUENCE_MASK;
        }

        self.sequence.store(sequence, Ordering::Relaxed);
        self.last_timestamp.store(timestamp, Ordering::Relaxed);

        let id = ((timestamp - Constants::EPOCH) << Constants::TIMESTAMP_SHIFT)
            | (self.center_id << Constants::CENTER_ID_SHIFT)
            | (self.worker_id << Constants::WORKER_ID_SHIFT)
            | sequence;

        Ok(id)
    }

    /// [`time_gen`] get current timestamp
    fn time_gen() -> Result<u64, SnowflakeError> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => Ok(now.as_millis() as u64),
            Err(_) => Err(SnowflakeError::SystemTimeError),
        }
    }

    /// [`til_next_millis`] get next timestamp
    fn til_next_millis(last_timestamp: u64) -> Result<u64, SnowflakeError> {
        let mut next = Self::time_gen().unwrap();
        while next <= last_timestamp {
            next = Self::time_gen().unwrap();
        }

        Ok(next)
    }
}
