use std::collections::BinaryHeap;
use std::marker::PhantomData;

/// Структура `PriorityQueue` представляет собой очередь с приоритетом, основанную на бинарной куче.
pub struct PriorityQueue<T>
    where
        T: Ord,
{
    container: BinaryHeap<T>,
    _marker: PhantomData<T>,
}

impl<T> PriorityQueue<T>
    where
        T: Ord,
{
    /// Создает новую пустую `PriorityQueue`.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let mut queue: PriorityQueue<i32> = PriorityQueue::new();
    /// ```
    pub fn new() -> Self {
        PriorityQueue {
            container: BinaryHeap::new(),
            _marker: PhantomData,
        }
    }

    /// Проверяет, пуста ли очередь.
    ///
    /// Возвращает `true`, если очередь пуста.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let queue: PriorityQueue<i32> = PriorityQueue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    /// Возвращает количество элементов в очереди.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let mut queue = PriorityQueue::new();
    /// queue.push(1);
    /// assert_eq!(queue.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.container.len()
    }

    /// Добавляет элемент в очередь.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let mut queue = PriorityQueue::new();
    /// queue.push(1);
    /// assert!(!queue.is_empty());
    /// ```
    pub fn push(&mut self, item: T) {
        self.container.push(item);
    }

    /// Удаляет и возвращает наибольший элемент из очереди.
    ///
    /// Возвращает `None`, если очередь пуста.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let mut queue = PriorityQueue::new();
    /// queue.push(1);
    /// assert_eq!(queue.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.container.pop()
    }

    /// Возвращает ссылку на наибольший элемент в очереди без его удаления.
    ///
    /// Возвращает `None`, если очередь пуста.
    ///
    /// # Примеры
    ///
    /// ```
    /// use priority_queue::PriorityQueue;
    /// let mut queue = PriorityQueue::new();
    /// queue.push(1);
    /// assert_eq!(*queue.peek().unwrap(), 1);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.container.peek()
    }
}
