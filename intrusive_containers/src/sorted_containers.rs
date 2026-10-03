// Test containers in this module:
//
// - SortedArray<T>: stores sorted values in a Vec<T>.
// - SortedMap<T>: stores values and duplicate counts in a BTreeMap<T, usize>.
// - SortedFlatMap<T>: stores values and duplicate counts in a FlatMap<T, usize>.
// - SortedBTree<T>: stores unique sorted values in a BTreeSet<T>.
//
// Each container implements test_container<T> with insert() and remove().

use flat_map::FlatMap;
use std::collections::{BTreeMap, BTreeSet};

// -----------------------------------------------------------------------------
// Container trait
// -----------------------------------------------------------------------------

#[allow(non_camel_case_types)]
pub trait test_container<T> {
    fn insert(&mut self, value: T) -> bool;
    fn remove(&mut self, value: T) -> bool;
}

// -----------------------------------------------------------------------------
// Sorted array
// -----------------------------------------------------------------------------

pub struct SortedArray<T> {
    values: Vec<T>,
    capacity: usize,
}

impl<T> SortedArray<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.values
    }
}

impl<T: Ord> test_container<T> for SortedArray<T> {
    fn insert(&mut self, value: T) -> bool {
        if self.values.len() == self.capacity {
            return false;
        }

        let position = self
            .values
            .binary_search(&value)
            .unwrap_or_else(|position| position);
        self.values.insert(position, value);
        true
    }

    fn remove(&mut self, value: T) -> bool {
        let Ok(position) = self.values.binary_search(&value) else {
            return false;
        };

        self.values.remove(position);
        true
    }
}

// -----------------------------------------------------------------------------
// Sorted map
// -----------------------------------------------------------------------------

pub struct SortedMap<T> {
    values: BTreeMap<T, usize>,
    capacity: usize,
    length: usize,
}

impl<T> SortedMap<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: BTreeMap::new(),
            capacity,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn values(&self) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
    {
        self.values
            .iter()
            .flat_map(|(value, &count)| std::iter::repeat_n(value.clone(), count))
    }
}

impl<T: Ord> test_container<T> for SortedMap<T> {
    fn insert(&mut self, value: T) -> bool {
        if self.length == self.capacity {
            return false;
        }

        *self.values.entry(value).or_insert(0) += 1;
        self.length += 1;
        true
    }

    fn remove(&mut self, value: T) -> bool {
        let Some(count) = self.values.get_mut(&value) else {
            return false;
        };

        *count -= 1;
        self.length -= 1;
        if *count == 0 {
            self.values.remove(&value);
        }
        true
    }
}

// -----------------------------------------------------------------------------
// Sorted flat map
// -----------------------------------------------------------------------------

pub struct SortedFlatMap<T> {
    values: FlatMap<T, usize>,
    capacity: usize,
    length: usize,
}

impl<T> SortedFlatMap<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: FlatMap::with_capacity(capacity),
            capacity,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn values(&self) -> impl Iterator<Item = T> + '_
    where
        T: Clone,
    {
        self.values
            .iter()
            .flat_map(|(value, &count)| std::iter::repeat_n(value.clone(), count))
    }
}

impl<T: Ord> test_container<T> for SortedFlatMap<T> {
    fn insert(&mut self, value: T) -> bool {
        if self.length == self.capacity {
            return false;
        }

        if let Some(count) = self.values.get_mut(&value) {
            *count += 1;
        } else {
            self.values.insert(value, 1);
        }
        self.length += 1;
        true
    }

    fn remove(&mut self, value: T) -> bool {
        let Some(count) = self.values.get_mut(&value) else {
            return false;
        };

        *count -= 1;
        self.length -= 1;
        if *count == 0 {
            self.values.remove(&value);
        }
        true
    }
}

// -----------------------------------------------------------------------------
// Sorted B-tree
// -----------------------------------------------------------------------------

pub struct SortedBTree<T> {
    values: BTreeSet<T>,
}

impl<T> SortedBTree<T> {
    pub fn new() -> Self {
        Self {
            values: BTreeSet::new(),
        }
    }
}

impl<T: Ord> test_container<T> for SortedBTree<T> {
    fn insert(&mut self, value: T) -> bool {
        self.values.insert(value)
    }

    fn remove(&mut self, value: T) -> bool {
        self.values.remove(&value)
    }
}
