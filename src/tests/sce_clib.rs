use crate::{sceClibStrncmp, sceClibTolower, sceClibToupper};

#[test]
fn tolower() {
    unsafe {
        assert_eq!(sceClibTolower(b'a' as i8), b'a' as i32);
        assert_eq!(sceClibTolower(b'A' as i8), b'a' as i32);
        assert_eq!(sceClibTolower(b'Z' as i8), b'z' as i32);
        assert_eq!(sceClibTolower(b';' as i8), b';' as i32);
        assert_eq!(sceClibTolower(b'\n' as i8), b'\n' as i32);
        assert_eq!(sceClibTolower(b'\0' as i8), b'\0' as i32);
    }
}

#[test]
fn toupper() {
    unsafe {
        assert_eq!(sceClibToupper(b'A' as i8), b'A' as i32);
        assert_eq!(sceClibToupper(b'a' as i8), b'A' as i32);
        assert_eq!(sceClibToupper(b'Z' as i8), b'Z' as i32);
        assert_eq!(sceClibToupper(b';' as i8), b';' as i32);
        assert_eq!(sceClibToupper(b'\n' as i8), b'\n' as i32);
        assert_eq!(sceClibToupper(b'\0' as i8), b'\0' as i32);
    }
}

#[test]
fn strncmp() {
    let reference = "Much doge, wow!";

    unsafe {
        assert_eq!(
            sceClibStrncmp(
                reference.as_ptr() as *const i8,
                "Much doge, wow!".as_ptr() as *const i8,
                15
            ),
            0
        );
        assert_eq!(
            sceClibStrncmp(
                reference.as_ptr() as *const i8,
                "Much".as_ptr() as *const i8,
                4
            ),
            0
        );
        assert!(
            sceClibStrncmp(
                reference.as_ptr() as *const i8,
                "much".as_ptr() as *const i8,
                4
            ) < 0
        );
        assert!(
            sceClibStrncmp(
                reference.as_ptr() as *const i8,
                "MUCH".as_ptr() as *const i8,
                4
            ) > 0
        );
    }
}
