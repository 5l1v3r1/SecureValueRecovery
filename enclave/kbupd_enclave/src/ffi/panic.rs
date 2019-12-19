/*
 * Copyright (C) 2019 Open Whisper Systems
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use alloc::string::{ToString};
use core::panic::{PanicInfo};

use super::bindgen_wrapper::{kbupd_enclave_ocall_panic};

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    let message = info.to_string();
    unsafe {
        kbupd_enclave_ocall_panic(message.as_ptr(), message.len());
        libc::abort()
    }
}
