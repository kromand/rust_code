use random_get::generate_numbers;
use std::time::{Duration, Instant};

mod sorted_containers;

pub use sorted_containers::{test_container, SortedArray, SortedBTree, SortedFlatMap, SortedMap};

fn remember_rust(some_var: u128) -> Option<bool> {
    if some_var > 0 {
        Some(true)
    } else {
        None
    }
}

fn print_latency(name: &str, elapsed: Duration, cycles: u64, sample_count: usize, unit: &str) {
    println!(
        "{name}: {} ns total, {:.2} ns/{unit}, {cycles} cycles total, {:.2} cycles/{unit}",
        elapsed.as_nanos(),
        elapsed.as_nanos() as f64 / sample_count as f64,
        cycles as f64 / sample_count as f64
    );
}

#[cfg(target_arch = "x86")]
fn read_cycles() -> u64 {
    unsafe {
        std::arch::x86::_mm_lfence();
        let cycles = std::arch::x86::_rdtsc();
        std::arch::x86::_mm_lfence();
        cycles
    }
}

#[cfg(target_arch = "x86_64")]
fn read_cycles() -> u64 {
    unsafe {
        std::arch::x86_64::_mm_lfence();
        let cycles = std::arch::x86_64::_rdtsc();
        std::arch::x86_64::_mm_lfence();
        cycles
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn read_cycles() -> u64 {
    0
}

/*
Available Collections:
LinkedList - Doubly-linked list

Link type: LinkedListLink
Atomic variant: LinkedListAtomicLink
SinglyLinkedList - Singly-linked list

Link type: SinglyLinkedListLink
Atomic variant: SinglyLinkedListAtomicLink
XorLinkedList - XOR doubly-linked list

Link type: XorLinkedListLink
Atomic variant: XorLinkedListAtomicLink
Memory-efficient alternative to doubly-linked lists
RBTree - Red-Black Tree

Link type: RBTreeLink
Atomic variant: RBTreeAtomicLink
Maintains elements in sorted order

*/
//
fn main() {
    let random_values = generate_numbers(1000000, 0, 100).expect("valid random value range");
    println!("Generated {} random values", random_values.len());

    let mut sorted_btree = SortedBTree::new();
    let value_count = random_values.len();
    let start = Instant::now();
    let cycle_start = read_cycles();
    for &value in &random_values {
        sorted_btree.insert(value);
    }
    let elapsed = start.elapsed();
    let cycle_elapsed = read_cycles() - cycle_start;
    print_latency("SortedBTree", elapsed, cycle_elapsed, value_count, "value");

    let mut sorted_flat_map = SortedFlatMap::new(value_count);
    let start = Instant::now();
    let cycle_start = read_cycles();
    for &value in &random_values {
        sorted_flat_map.insert(value);
    }
    let elapsed = start.elapsed();
    let cycle_elapsed = read_cycles() - cycle_start;
    print_latency(
        "SortedFlatMap",
        elapsed,
        cycle_elapsed,
        value_count,
        "value",
    );
}

#[cfg(test)]
mod tests {
    use super::{test_container, SortedArray, SortedBTree, SortedFlatMap, SortedMap};

    #[test]
    fn insert_keeps_values_sorted() {
        let mut values = SortedArray::new(3);

        assert!(values.insert(8));
        assert!(values.insert(2));
        assert!(values.insert(5));

        assert_eq!(values.as_slice(), &[2, 5, 8]);
        assert!(!values.insert(10));
    }

    #[test]
    fn remove_deletes_one_matching_value() {
        let mut values = SortedArray::new(3);
        values.insert(4);
        values.insert(4);
        values.insert(9);

        assert!(values.remove(4));
        assert_eq!(values.as_slice(), &[4, 9]);
        assert!(!values.remove(7));
    }

    #[test]
    fn map_keeps_values_sorted_and_supports_duplicates() {
        let mut values = SortedMap::new(3);

        assert!(values.insert(8));
        assert!(values.insert(2));
        assert!(values.insert(2));
        assert!(!values.insert(5));

        assert_eq!(values.values().collect::<Vec<_>>(), vec![2, 2, 8]);
        assert_eq!(values.len(), 3);
        assert!(values.remove(2));
        assert_eq!(values.values().collect::<Vec<_>>(), vec![2, 8]);
    }

    #[test]
    fn flat_map_keeps_values_sorted_and_supports_duplicates() {
        let mut values = SortedFlatMap::new(3);

        assert!(values.insert(8));
        assert!(values.insert(2));
        assert!(values.insert(2));
        assert!(!values.insert(5));

        assert_eq!(values.values().collect::<Vec<_>>(), vec![2, 2, 8]);
        assert_eq!(values.len(), 3);
        assert!(values.remove(2));
        assert_eq!(values.values().collect::<Vec<_>>(), vec![2, 8]);
    }

    #[test]
    fn btree_inserts_and_removes_values() {
        let mut values = SortedBTree::new();

        assert!(values.insert(8));
        assert!(values.insert(2));
        assert!(!values.insert(8));
        assert!(values.remove(2));
        assert!(!values.remove(2));
    }
}
