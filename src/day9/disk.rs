use std::collections::HashMap;

pub struct Disk {
    pub blocks: Vec<DiskBlock>,
}

impl Disk {
    pub fn new(blocks: Vec<DiskBlock>) -> Self {
        Self { blocks }
    }

    pub fn defragment(&mut self) {
        let empty_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b == DiskBlock::Empty)
            .map(|(i, _)| i);

        let file_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| match b {
                DiskBlock::File(_) => true,
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
        let mut files: Vec<_> = self.find_files().into_iter().collect();
        files.sort_by_key(|(id, _)| *id);
        files.reverse();

        for (_, file) in files {
            if let Some(empty_index) = self.find_empty_block(file.size) {
                if empty_index < file.index {
                    for i in 0..file.size {
                        self.blocks.swap(empty_index + i, file.index + i);
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
                DiskBlock::Empty => 0,
                DiskBlock::File(id) => i as u64 * id as u64,
            })
            .sum()
    }

    fn find_empty_block(&self, file_len: usize) -> Option<usize> {
        self.blocks
            .windows(file_len)
            .position(|window| window.iter().all(|b| *b == DiskBlock::Empty))
    }

    fn find_files(&self) -> HashMap<usize, FileStat> {
        let mut counts: HashMap<usize, FileStat> = HashMap::new();

        for (index, block) in self.blocks.iter().enumerate() {
            if let DiskBlock::File(id) = block {
                let entry = counts.entry(*id).or_insert(FileStat { index, size: 0 });
                entry.size += 1;
            }
        }

        counts
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
