// from https://docs.rust-embedded.org/book/peripherals/singletons.html
// using `unsafe` code

struct Thing<T> {
    value: Option<T>,
}

impl<T> Thing<T> {
    fn remove_value(&mut self) -> T {
        let value = core::mem::replace(&mut self.value, None);

        return value.unwrap();
    }
}

static mut THING: Thing<&str> = Thing {
    value: Some("SomeValue"),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
        let value = unsafe { THING.remove_value() };

        assert_eq!(value, "SomeValue");
        assert!(unsafe { THING.value.is_none() });

        // will panic
        // let another_value = unsafe { THING.remove_value() };
    }
}
