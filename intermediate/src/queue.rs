// TODO:
//      - [ ] implement a circular buffer
//      - [ ] implement a buffer
//      - [x] implement a queue
use ringbuf::RingBuffer;

// simple queue with enqueue, dequeue, etc.
struct Queue<T> {
    queue: Vec<T>,
}

struct Buffer<T> {
    queue: Vec<T>,
}

// simple circular queue with rotating enqueue management and with capacity
struct CircularBuffer<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    // create queue using vector with no explicit capacity
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    // get length of queue
    fn length(&self) -> usize {
        self.queue.len()
    }

    // add to queue
    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    // remove from queue
    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    // get first element of queue
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

impl<T> Buffer<T> {
    fn new() {}

    fn enqueue(&self) {}

    fn dequeue(&self) {}
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

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn is_at_capacity(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.length() == self.queue.capacity() {
            return true;
        }

        false
    }

    // rotate and return an if rotated bool
    fn rotate_and_succeed(&mut self) -> bool {
        if self.is_at_capacity() {
            self.queue.rotate_right(1);

            return true;
        }

        false
    }

    // add to queue
    fn enqueue(&mut self, item: T) {
        let rotated = self.rotate_and_succeed();

        if !rotated {
            self.queue.push(item);

            return;
        }

        self.queue[0] = item;
    }

    // fifo remove from queue
    fn dequeue(&mut self) -> T {
        let rotated = self.rotate_and_succeed();

        if rotated {
            return self.queue.remove(0);
        }

        self.queue.remove(self.length() - 1)
    }
}

// use ringbuf for circular queue
fn create_ringbuf_queue(size: usize) -> RingBuffer<isize> {
    return RingBuffer::<isize>::new(size);
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
    }

    #[test]
    fn test_circular_queue() {
        let mut circular_queue: CircularBuffer<isize> = CircularBuffer::new(4);

        circular_queue.enqueue(1);
        circular_queue.enqueue(2);
        circular_queue.enqueue(3);
        circular_queue.enqueue(4);
        circular_queue.enqueue(5);

        // [5, 1, 2, 3]

        assert_eq!(circular_queue.length(), 4);
        assert_eq!(circular_queue.queue[0], 5);

        let item = circular_queue.dequeue();

        // [3, 5, 1, 2]
        //  ^

        assert_eq!(item, 3);
    }

    #[test]
    fn test_ringbuf_queue() {
        let queue = create_ringbuf_queue(2);

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
