// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CString;
use std::os::raw::c_char;

use ffi_support::FfiStr;
use glean::ErrorType;

use crate::ffi::helpers;

#[no_mangle]
pub extern "C" fn glean_custom_distribution_accumulate_sample(id: u32, sample: i64) {
    with_metric!(
        CUSTOM_DISTRIBUTION_MAP,
        id,
        metric,
        metric.accumulate_samples(vec![sample])
    );
}

#[no_mangle]
pub extern "C" fn glean_custom_distribution_test_get_value(
    id: u32,
    ping_name: FfiStr
) -> *mut c_char {
    let value = with_metric!(
        CUSTOM_DISTRIBUTION_MAP,
        id,
        metric,
        metric.test_get_value(helpers::option_from_ffi(ping_name))
    );

    let value_as_json = match value {
        Some(v) => serde_json::to_string(&v).unwrap(),
        None => "".to_string(),
    };

    CString::new(value_as_json)
        .expect("Unable to create CString.")
        .into_raw()
}

#[no_mangle]
pub extern "C" fn glean_custom_distribution_test_get_num_recorded_errors(
    id: u32,
    error_type: ErrorType,
) -> i32 {
    with_metric!(
        CUSTOM_DISTRIBUTION_MAP,
        id,
        metric,
        metric.test_get_num_recorded_errors(error_type)
    )
}
