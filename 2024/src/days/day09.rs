use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{fs, io};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day9/input.txt") {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
    };

    // Convert data (a single string) to a Vec of i64s, there is no spacing it's a single long string
    let mut vals: Vec<usize> = Vec::new();
    for c in data.chars() {
        match c.to_digit(10) {
            Some(d) => vals.push(d as usize),
            None => continue,
        }
    }

    let disk = vals;

    // Start at the first free block and the last file.
    let mut left = 0;
    let mut right = disk.len() - 2 + disk.len() % 2;
    let mut needed = disk[right];
    let mut block = 0;
    let mut checksum = 0;

    while left < right {
        // When moving to the next free block, add the checksum for the file we're skipping over.
        (checksum, block) = update(checksum, block, left, disk[left]);
        let mut available = disk[left + 1];
        left += 2;

        while available > 0 {
            if needed == 0 {
                if left == right {
                    break;
                }
                right -= 2;
                needed = disk[right];
            }

            // Take as much space as possible from the current free block range.
            let size = needed.min(available);
            (checksum, block) = update(checksum, block, right, size);
            available -= size;
            needed -= size;
        }
    }

    // Account for any remaining file blocks left over.
    (checksum, _) = update(checksum, block, right, needed);
    println!("Day 9 Part 1: {:?}", checksum);
    println!("Day 9 Part 2: {:?}", part2(&disk));

    Ok(())
}

fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block * size + EXTRA[size];
    (checksum + id * extra, block + size)
}

pub fn part2(disk: &[usize]) -> usize {
    let mut block = 0;
    let mut checksum = 0;
    let mut free: Vec<_> = (0..10).map(|_| MinHeap::with_capacity(1_000)).collect();

    // Build a min-heap (leftmost free block first) where the size of each block is
    // implicit in the index of the array.
    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(block, ());
        }

        block += size;
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        block -= size;

        // Count any previous free blocks to decrement block offset correctly.
        if index % 2 == 1 {
            continue;
        }

        // Find the leftmost free block that can fit the file (if any).
        let mut next_block = block;
        let mut next_index = usize::MAX;

        #[allow(clippy::needless_range_loop)]
        for i in size..free.len() {
            if let Some((&first, ())) = free[i].peek() {
                if first < next_block {
                    next_block = first;
                    next_index = i;
                }
            }
        }

        // We can make smaller free block from bigger blocks but not the other way around.
        // As an optimization if all blocks of the biggest size are after our position then
        // we can ignore them.
        if !free.is_empty() {
            let last = free.len() - 1;
            if let Some((&first, ())) = free[last].peek() {
                if first > block {
                    free.pop();
                }
            }
        }

        // Update the checksum with the file's location (possibly unchanged).
        let id = index / 2;
        let extra = next_block * size + EXTRA[size];
        checksum += id * extra;

        // If we used a free block, remove then add back any leftover space.
        if next_index != usize::MAX {
            free[next_index].pop();
            if size < next_index {
                free[next_index - size].push(next_block + size, ());
            }
        }
    }

    checksum
}

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn _new() -> Self {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Wrapper { key, value });
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }

    #[inline]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.heap.peek().map(|w| (&w.key, &w.value))
    }
}
