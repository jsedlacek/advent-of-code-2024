use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    Empty,
    File(usize),
}

pub struct Disk {
    pub blocks: Vec<Block>,
}

impl Disk {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }

    pub fn defragment(&mut self) {
        let empty_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b == Block::Empty)
            .map(|(i, _)| i);

        let file_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| match b {
                Block::File(_) => true,
                _ => false,
            })
            .map(|(i, _)| i);

        for (i, j) in file_blocks.zip(empty_blocks) {
            if i < j {
                break;
            }

            self.blocks.swap(i, j);
        }
    }

    pub fn defragment_v2(&mut self) {
        let mut index_counts: Vec<_> = self.find_index_counts().into_iter().collect();
        index_counts.sort_by_key(|(id, _)| *id);
        index_counts.reverse();

        for (id, file_len) in index_counts {
            if let Some(file_index) = self.blocks.iter().position(|b| *b == Block::File(id)) {
                if let Some(empty_index) = self.find_empty_block(file_len) {
                    if empty_index < file_index {
                        for i in 0..file_len {
                            self.blocks.swap(empty_index + i, file_index + i);
                        }
                    }
                }
            }
        }
    }

    pub fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, &b)| match b {
                Block::Empty => 0,
                Block::File(id) => i as u64 * id as u64,
            })
            .sum()
    }

    fn find_empty_block(&self, file_len: usize) -> Option<usize> {
        self.blocks
            .windows(file_len)
            .position(|window| window.iter().all(|b| *b == Block::Empty))
    }

    fn find_index_counts(&self) -> HashMap<usize, usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();

        for block in self.blocks.iter() {
            if let Block::File(id) = block {
                *counts.entry(*id).or_insert(0) += 1;
            }
        }

        counts
    }
}
