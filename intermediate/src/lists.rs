// NOTE: It is almost always better to use Vec or VecDeque because array-based
// containers are generally faster, more memory efficient, and make better use of CPU cache.
// TODO: maybe do something more linked-list-unique
const FIRST_ELEMENT: u8 = 0;
const SECOND_ELEMENT: u8 = 1;
const THIRD_ELEMENT: u8 = 2;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn test_linked_lists() {
        let mut example_linked_list: LinkedList<u8> = LinkedList::new();

        example_linked_list.push_back(FIRST_ELEMENT);
        example_linked_list.push_back(SECOND_ELEMENT);
        example_linked_list.push_back(THIRD_ELEMENT);

        assert_eq!(example_linked_list.front(), Some(&FIRST_ELEMENT));
        assert_eq!(example_linked_list.back(), Some(&THIRD_ELEMENT));
    }
}
