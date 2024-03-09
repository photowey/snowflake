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

use ifcfg::IfCfg;

use crate::generator::Constants;
use crate::hashcode::HashCode;

// ----------------------------------------------------------------

const LOOPBACK: &str = "Loopback";

// ----------------------------------------------------------------

/// [`InterfaceError`]
///
/// @since 0.2.0
#[derive(Debug)]
pub enum InterfaceError {
    IfCfgError,
    NonLoopbackNotFound,
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterfaceError::IfCfgError => write!(f, "IfCfgError error"),
            InterfaceError::NonLoopbackNotFound => write!(f, "Non-Loopback interface not found"),
        }
    }
}

impl Error for InterfaceError {}

// ----------------------------------------------------------------

/// [`try_get_worker_id`]
///
/// # Examples
///
/// ```rust
/// use snowflaker::generator::Constants;
/// use snowflaker::infras;
///
/// let center_id = infras::try_get_data_center_id();
/// let worker_id = infras::try_get_worker_id(center_id);
/// assert!(worker_id <= Constants::MAX_WORKER_ID);
/// ```
/// @since 0.2.0
pub fn try_get_worker_id(center_id: u64) -> u64 {
    let mut buf = center_id.to_string();
    let pid = std::process::id().to_string();
    buf.push_str(&pid);

    let hashcode = buf.hashcode();
    (hashcode & 0xFFFF) & Constants::MAX_WORKER_ID
}

// ----------------------------------------------------------------

/// [`try_get_data_center_id`]
///
/// # Examples
///
/// ```rust
/// use snowflaker::generator::Constants;
/// use snowflaker::infras;
///
/// let center_id = infras::try_get_data_center_id();
/// assert!(center_id <= Constants::MAX_DATA_CENTER_ID);
/// ```
/// @since 0.2.0
#[rustfmt::skip]
pub fn try_get_data_center_id() -> u64 {
    let mut id = Constants::DEFAULT_DATA_CENTER_ID;

    if let Ok(mac) = try_get_local_first_non_loopback_interface() {
        let tail = mac.len() - 1;
        let lower_bits = (0x000000FF & (mac[tail - 1] as u64)) | (0x0000FF00 & ((mac[tail] as u64) << 8));

        id = lower_bits >> 8;
        if id == 0 {
            id = lower_bits >> 6;
        }
    }

    id & Constants::MAX_DATA_CENTER_ID
}

/// [`try_get_local_first_non_loopback_interface`]
///
/// Attempts to retrieve the MAC address of the first non-loopback network interface on the local host.
///
/// This function queries the system's network interface configuration and returns the MAC address
/// of the first non-loopback interface. If no non-loopback interface is found, or an error occurs during
/// the querying process, it will return an appropriate error message.
///
/// @since 0.2.0
fn try_get_local_first_non_loopback_interface() -> Result<Vec<u8>, Box<dyn Error>> {
    let interfaces = match IfCfg::get() {
        Ok(interfaces) => interfaces,
        Err(_) => return Err(Box::new(InterfaceError::IfCfgError)),
    };

    // Notes: does not consider whether the interface is up?
    let mac_bytes = interfaces
        .iter()
        .find(|conf| !conf.name.contains(LOOPBACK))
        .map(|conf| {
            conf.mac
                .split('-')
                .map(|hex| u8::from_str_radix(hex, 16))
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|err| Box::new(err) as Box<dyn Error>)
        })
        .ok_or_else(|| Box::new(InterfaceError::NonLoopbackNotFound))??;

    Ok(mac_bytes)
}
