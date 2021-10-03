// TODO:
//      - [ ] implement a circular buffer
//      - [ ] implement a buffer
//      - [x] implement a queue
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

struct CircularBuffer<T> {
    queue: Vec<T>,
}

impl<T> CircularBuffer<T> {
    fn new(size: usize) -> Self {
        CircularBuffer {
            queue: Vec::with_capacity(size),
        }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        let len = self.length();

        if len == self.queue.capacity() {
            self.queue.rotate_right(1);

            self.queue[0] = item;
        } else {
            self.queue.push(item);
        }
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
        assert_eq!(queue.is_empty(), false);

        let mut circular_queue: CircularBuffer<isize> = CircularBuffer::new(4);

        circular_queue.enqueue(1);
        circular_queue.enqueue(2);
        circular_queue.enqueue(3);
        circular_queue.enqueue(4);
        circular_queue.enqueue(5);

        // [5, 1, 2, 3]

        assert_eq!(circular_queue.length(), 4);
        assert_eq!(circular_queue.queue[0], 5);
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
