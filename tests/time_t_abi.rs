#![cfg(all(target_env = "musl", target_pointer_width = "32"))]

use boring_sys::{
    ASN1_TIME_free, ASN1_TIME_new, ASN1_TIME_set, ASN1_TIME_to_time_t, time_t,
};
use std::mem::size_of;

#[test]
fn time_t_round_trips_after_2038() {
    assert_eq!(size_of::<time_t>(), 8);
    let timestamp: time_t = 4_102_444_800;
    let sentinel: time_t = 0x1234_5678;
    let mut output = [0, sentinel];

    // The ASN1_TIME allocation remains live until both calls finish. The
    // output points to a writable time_t followed by an overwrite sentinel.
    let (set_ok, converted) = unsafe {
        let value = ASN1_TIME_new();
        assert!(!value.is_null());
        let set_ok = !ASN1_TIME_set(value, timestamp).is_null();
        let converted = ASN1_TIME_to_time_t(value, output.as_mut_ptr());
        ASN1_TIME_free(value);
        (set_ok, converted)
    };

    assert!(set_ok);
    assert_eq!(converted, 1);
    assert_eq!(output, [timestamp, sentinel]);
}
