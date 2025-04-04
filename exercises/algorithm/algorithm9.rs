/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Copy> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
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
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut id = self.count;
        let mut pa = self.parent_idx(id);
        while pa >= 1 {
            let x = &self.items[id];
            let y = &self.items[pa];
            //println!("xx id: {}, son: {} {} {}", id, pa, x, y);
            if (self.comparator)(x, y) {
                //println!("xx id: {}, son: {} {} {}", id, pa, x, y);
                self.items.swap(id, pa);
                id = pa;
                pa = self.parent_idx(id);
            } else {
                //println!("add {:?}", self.items.clone());
                return;
            }
        }
        //println!("add {:?}", self.items.clone());
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

    fn down(&self, idx: usize) -> (bool, usize) {
        if !self.children_present(idx) {
            return (false, 0);
        }
        let mut idx = idx;
        let rson = self.right_child_idx(idx);
        if rson <= self.count && (self.comparator)(&self.items[rson], &self.items[rson - 1]) {
            idx = rson;
        } else {
            idx = rson - 1;
        }
        if (self.comparator)(&self.items[idx], &self.items[idx / 2]) {
            return (true, idx);
        }
        return (false, 0);
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            //println!("rr {:?}", self.items.clone());
            return None;
        }
        let res = self.items[1];
        self.items[1] = self.items[self.count];
        self.count -= 1;
        if self.count == 0 || self.count == 1 {
            self.items.pop();
            //println!("rr {:?}", self.items.clone());
            return Some(res);
        }
        let mut id = 1;
        while id < self.count {
            let (ok, id2) = self.down(id);
            if !ok {
                break;
            }
            //println!("id: {}, son: {} {} {}", id, id2, self.items[id], self.items[id2]);
            self.items.swap(id, id2);
            id = id2;
        }
        self.items.pop();
        //println!("rr {:?}", self.items.clone());
        return Some(res);
    }
}

impl<T: Copy> Heap<T>
where
    T: Default + Ord,
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

impl<T: Copy> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }
        return self.pop();
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Copy>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Copy>() -> Heap<T>
    where
        T: Default + Ord,
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
        // println!("aa{}", heap.next().unwrap());
        // println!("{}", heap.next().unwrap());
        // println!("{}", heap.next().unwrap());
        // println!("{}", heap.next().unwrap());
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
    }
}