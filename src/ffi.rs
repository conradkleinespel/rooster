use libc;
use std::ptr;

#[allow(non_camel_case_types)]
pub type time_t = libc::c_uint;

mod internal {
    extern "C" {
        pub fn time(t: *mut super::time_t) -> super::time_t;
    }
}

pub fn time() -> time_t {
    let retrieved_time = unsafe { internal::time(ptr::null_mut()) };

    if retrieved_time == !(0 as u32) {
        panic!("Could not get time from system");
    }

    retrieved_time
}

#[cfg(test)]
mod test {
    use crate::ffi::time;

    #[test]
    fn test_time() {
        let t_2030 = 1893456000;
        let t = time();
        assert!(t > 0 && t < t_2030);
    }
}
