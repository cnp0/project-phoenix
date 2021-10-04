// TODO: look into performance of enqueue direction; try immutable solution
use ringbuf::RingBuffer;

const TEST_CAPACITY: usize = 6;
const TEST_ITEMS_TO_QUEUE: [u8; TEST_CAPACITY] = [0, 1, 2, 3, 4, 5];

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
    fn new(size: usize) -> Self {
        Buffer {
            queue: Vec::with_capacity(size),
        }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn enqueue(&mut self, item: T) {
        if self.length() < self.queue.capacity() {
            self.queue.push(item);
        }

        println!("Item was not added to the queue!")
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
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
        if self.length() == self.queue.capacity() {
            return true;
        }

        false
    }

    // rotate left
    fn rotate(&mut self) {
        if self.is_at_capacity() {
            self.queue.rotate_left(1);
        }
    }

    // add to queue
    fn enqueue(&mut self, item: T) {
        self.rotate();

        if self.is_at_capacity() {
            let len = self.length();
            self.queue[len - 1] = item;

            return;
        }

        self.queue.push(item);
    }

    // fifo remove from queue
    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
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
        let mut queue: Queue<u8> = Queue::new();

        for item in TEST_ITEMS_TO_QUEUE.iter() {
            queue.enqueue(*item);
        }

        let item = queue.dequeue();

        assert_eq!(item, TEST_ITEMS_TO_QUEUE[0]);
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_buffer_queue() {
        let mut buffer = Buffer::new(TEST_CAPACITY);

        for item in TEST_ITEMS_TO_QUEUE.iter() {
            buffer.enqueue(*item);
        }

        buffer.enqueue(6);

        assert_eq!(buffer.length(), TEST_CAPACITY);

        let item = buffer.dequeue();

        assert_eq!(item, TEST_ITEMS_TO_QUEUE[0]);
    }

    #[test]
    fn test_circular_queue() {
        let mut circular_queue: CircularBuffer<u8> = CircularBuffer::new(TEST_CAPACITY);
        let new_item = 6;

        // [0, 1, 2, 3, 4, 5]

        for item in TEST_ITEMS_TO_QUEUE.iter() {
            circular_queue.enqueue(*item);
        }

        circular_queue.enqueue(new_item);

        assert_eq!(circular_queue.length(), TEST_CAPACITY);
        assert_eq!(circular_queue.queue[0], TEST_ITEMS_TO_QUEUE[1]);

        let item = circular_queue.dequeue();

        // [0, 1, 2, 3, 4, 5]
        // [1, 2, 3, 4, 5, new_item]
        //  ^

        assert_eq!(item, TEST_ITEMS_TO_QUEUE[1]);
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
