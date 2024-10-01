/*
    heap
    This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T:  Default + std::cmp::PartialEq<i32>,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T:   Default + std::cmp::PartialEq<i32>,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.buble_up(self.items.len() - 1);
        self.count += 1;
    }

    /// 删除堆顶元素
    pub(self) fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        if self.items.len() == 1 {
            return Some(self.items.pop().unwrap());
        }

        let min = self.items.swap_remove(0);
        
        self.bubble_down(0);
        if min ==0 {
            None
        }else{
            Some(min)
        }
        
    }

    pub(self) fn buble_up(&mut self, mut index: usize) {
        while index > 0 {
            let praentIdx = self.parent_idx(index);
            if (self.comparator)(&self.items[praentIdx], &self.items[index]) {
                break;
            }
            self.items.swap(index, praentIdx);
            index = self.parent_idx(index);
        }
    }

    /// 下沉操作
    pub(self) fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left_child_index = 2 * index + 1;
            let right_child_index = 2 * index + 2;

            let mut smallest = index;

            if left_child_index < self.items.len()
                && (self.comparator)(&self.items[left_child_index], &self.items[smallest])
            {
                smallest = left_child_index;
            }

            if right_child_index < self.items.len()
                && (self.comparator)(&self.items[right_child_index], &self.items[smallest])
            {
                smallest = right_child_index;
            }

            if smallest != index {
                self.items.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord +  std::cmp::PartialEq<i32>
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord +  std::cmp::PartialEq<i32>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord +  std::cmp::PartialEq<i32>
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord +  std::cmp::PartialEq<i32>
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
 
    }
}
