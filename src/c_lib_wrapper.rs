use std::sync::OnceLock;

mod c_lib {

    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl PartialEq for parameter_t {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a && self.b == other.b
        }
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { c_lib::add(a, b) }
}

#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
}

#[allow(dead_code)]
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    let mut result: i32 = 0;
    unsafe {
        let error_code = c_lib::divide(a, b, &mut result);
        if error_code == c_lib::divide_error_DIVISION_BY_ZERO_ERROR {
            return Err(DivisionError::DivisionByZero);
        }
    }
    Ok(result)
}

#[allow(dead_code)]
static CALLBACK: OnceLock<fn(i32, i32) -> i32> = OnceLock::new();

#[allow(dead_code)]
unsafe extern "C" fn trampoline(a: i32, b: i32) -> i32 {
    CALLBACK.get().unwrap()(a, b)
}

#[allow(dead_code)]
pub fn process(a: i32, b: i32, callback: fn(i32, i32) -> i32) -> i32 {
    CALLBACK.set(callback).unwrap();

    let p = c_lib::parameter_t {
        a,
        b,
        fun: Some(trampoline),
    };

    let result = unsafe { c_lib::process(p) };

    assert_eq!(result.parameter, p);

    result.res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_add() {
        assert_eq!(add(3, 5), 8);
    }

    #[test]
    fn test_divide_ok() {
        let result = divide(8, 2);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), 4);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(8, 0);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), DivisionError::DivisionByZero);
    }

    #[test]
    fn test_process() {
        assert_eq!(process(2, 3, |a, b| a * b), 6);
    }
}
