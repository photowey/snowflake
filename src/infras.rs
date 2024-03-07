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

/// [`try_get_datacenter_id`] TODO Dynamic calculate a data-center ID by local-interface
pub fn try_get_datacenter_id() -> u64 {
    0
}

/// [`try_get_local_interface`] TODO Get local interface
pub fn try_get_local_interface() {}

/// [`try_get_hardware_address`] TODO Get hardware interface
pub fn try_get_hardware_address() -> Option<Vec<u8>> {
    Some(Vec::new())
}
