use std::collections::HashMap;

pub struct Disk {
    pub blocks: Vec<DiskBlock>,
}

impl Disk {
    pub fn new(blocks: Vec<DiskBlock>) -> Self {
        Self { blocks }
    }

    pub fn defragment(&mut self) {
        let empty_blocks: Vec<_> = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == DiskBlock::Empty)
            .map(|(i, _)| i)
            .collect();

        let file_blocks: Vec<_> = self
            .blocks
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| matches!(b, DiskBlock::File(_)))
            .map(|(i, _)| i)
            .collect();

        for (&i, &j) in file_blocks.iter().zip(empty_blocks.iter()) {
            if i < j {
                break;
            }

            self.blocks.swap(i, j);
        }
    }

    pub fn defragment_v2(&mut self) {
        let mut files = Vec::from_iter(self.find_files());
        files.sort_unstable_by_key(|&(id, _)| id);
        files.reverse();

        let mut last_indices = HashMap::new();

        for (_, file) in files {
            let last_index = last_indices.get(&file.size).copied().unwrap_or(0);
            if let Some(empty_index) = self.find_empty_block(file.size, last_index) {
                last_indices.insert(file.size, empty_index + file.size);
                if empty_index < file.index {
                    let (start, end) = self.blocks.split_at_mut(file.index);
                    start[empty_index..empty_index + file.size]
                        .swap_with_slice(&mut end[..file.size]);
                }
            }
        }
    }

    pub fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, &b)| match b {
                DiskBlock::Empty => 0,
                DiskBlock::File(id) => i as u64 * id as u64,
            })
            .sum()
    }

    fn find_empty_block(&self, file_len: usize, start_index: usize) -> Option<usize> {
        let mut empty_start_index = None;

        for (i, &block) in self.blocks[start_index..].iter().enumerate() {
            if let DiskBlock::Empty = block {
                if empty_start_index.is_none() {
                    empty_start_index = Some(i);
                }
            } else {
                empty_start_index = None;
            }

            if let Some(empty_start_index) = empty_start_index {
                if empty_start_index + file_len - 1 == i {
                    return Some(start_index + empty_start_index);
                }
            }
        }

        None
    }

    fn find_files(&self) -> HashMap<usize, FileStat> {
        let mut files = HashMap::new();

        for (index, &block) in self.blocks.iter().enumerate() {
            if let DiskBlock::File(id) = block {
                files
                    .entry(id)
                    .or_insert_with(|| FileStat { index, size: 0 })
                    .size += 1;
            }
        }

        files
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiskBlock {
    Empty,
    File(usize),
}

struct FileStat {
    index: usize,
    size: usize,
}
