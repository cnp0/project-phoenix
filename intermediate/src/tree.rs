struct Node {
    val: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, val: u32) {
        if self.val == val {
            return;
        }

        let target_child = if val > self.val {
            &mut self.right
        } else {
            &mut self.left
        };

        match target_child {
            &mut Some(ref mut child) => child.insert(val),
            &mut None => {
                let new_child = Node {
                    val: val,
                    left: None,
                    right: None,
                };
                let boxed_child = Some(Box::new(new_child));
                *target_child = boxed_child;
            }
        }
    }

    pub fn value(self) -> u32 {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trees() {
        let mut root = Node {
            val: 0,
            left: None,
            right: None,
        };

        root.insert(1);
        root.insert(2);
        root.insert(3);

        assert_eq!(root.val, 0);
        assert_eq!(root.right.unwrap().right.unwrap().right.unwrap().value(), 3);
        assert!(root.left.is_none());
    }
}
