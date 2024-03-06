use super::PriorityQueue;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();

        assert!(pq.is_empty());
        pq.push(3);
        pq.push(5);
        pq.push(1);

        assert_eq!(pq.size(), 3);
        assert_eq!(pq.peek(), Some(&5));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.peek(), Some(&3));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.peek(), Some(&1));
        assert_eq!(pq.pop(), Some(1));
        assert!(pq.is_empty());
    }
}