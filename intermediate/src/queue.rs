use ringbuf::RingBuffer;

const RING_BUF_SIZE: usize = 2;

struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

fn create_ringbuf_queue() -> RingBuffer<i32> {
    return RingBuffer::<i32>::new(RING_BUF_SIZE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue: Queue<isize> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);

        let item = queue.dequeue();

        assert_eq!(item, 1);
        assert_eq!(queue.is_empty(), false)
    }

    #[test]
    fn test_ringbuf_queue() {
        let queue = create_ringbuf_queue();

        let (mut prod, mut cons) = queue.split();

        // test up to RING_BUF_SIZE
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
