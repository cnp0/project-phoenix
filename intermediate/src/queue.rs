use ringbuf::RingBuffer;

const BUFFER_SIZE: usize = 2;

fn create_queue() -> RingBuffer<i32> {
    return RingBuffer::<i32>::new(BUFFER_SIZE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let queue = create_queue();

        let (mut prod, mut cons) = queue.split();

        // test up to BUFFER_SIZE
        prod.push(0).unwrap();
        prod.push(1).unwrap();
        assert_eq!(prod.push(2), Err(2));

        assert_eq!(cons.pop().unwrap(), 0);

        prod.push(2).unwrap();

        assert_eq!(cons.pop().unwrap(), 1);
        assert_eq!(cons.pop().unwrap(), 2);
        assert_eq!(cons.pop(), None);
    }
}
