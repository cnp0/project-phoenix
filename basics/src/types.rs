// example custom type
struct ExampleStruct {
    name: String,
    id: u32,
}

enum ExampleEnum {
    One,
    Two,
    Three,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let name = String::from("Cool Thing");
        let cloned_name = name.clone();

        let id = 1111111;
        let cloned_id = id.clone();

        let test_thing = ExampleStruct { name, id };

        assert_eq!(cloned_name, test_thing.name);
        assert_eq!(cloned_id, test_thing.id);
    }

    #[test]
    fn test_enums() {
        let thing = ExampleEnum::One;

        let result = match thing {
            ExampleEnum::One => 1,
            ExampleEnum::Two => 2,
            ExampleEnum::Three => 3,
        };

        assert_eq!(result, 1);
    }
}
