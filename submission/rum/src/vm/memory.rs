use anyhow::{anyhow, Result};
use dashmap::DashMap;
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Segment {
    pub id: u32,
    pub words: Vec<u32>,
}

impl Segment {
    pub fn new(id: u32, size: u32) -> Self {
        Self {
            id,
            words: vec![0; size as usize],
        }
    }
}

pub struct SegmentTable {
    zero_segment: Arc<[u32]>,
    mapped: DashMap<u32, Segment>,
    free_ids: ArrayQueue<u32>,
}

impl SegmentTable {
    // Constructor to initialize the SegmentTable with a program
    pub fn new(program: Vec<u32>) -> Self {
        Self {
            zero_segment: Arc::from(program.into_boxed_slice()),
            mapped: DashMap::new(),
            free_ids: ArrayQueue::new(1024),
        }
    }

    // Fetch the value from the zero segment by address
    pub fn fetch(&self, addr: u32) -> Result<u32> {
        self.zero_segment
            .get(addr as usize)
            .copied()
            .ok_or_else(|| anyhow!("Invalid address: {}", addr))
    }

    // Get the value from a specified segment at an offset
    pub fn get(&self, segment: u32, offset: u32) -> Result<u32> {
        if segment == 0 {
            return self.fetch(offset);
        }
        self.mapped.get(&segment)
            .ok_or_else(|| anyhow!("Invalid segment"))?
            .words.get(offset as usize)
            .copied()
            .ok_or_else(|| anyhow!("Invalid offset"))
    }

    // Set a value in a specified segment at an offset
    pub fn set(&self, segment: u32, offset: u32, value: u32) -> Result<()> {
        if segment == 0 {
            return Err(anyhow!("Cannot modify zero segment"));
        }
        self.mapped.get_mut(&segment)
            .ok_or_else(|| anyhow!("Invalid segment"))?
            .words
            .get_mut(offset as usize)
            .ok_or_else(|| anyhow!("Invalid offset"))?
            .clone_from(&value);
        Ok(())
    }

    // Map a new segment with the given size and return its ID
    pub fn map_segment(&self, size: u32) -> Result<u32> {
        let id = self.free_ids.pop().unwrap_or_else(|| 
            rand::random::<u32>() | 1  // Ensure non-zero ID
        );
        self.mapped.insert(id, Segment::new(id, size));
        Ok(id)
    }

    // Unmap a segment given its ID
    pub fn unmap_segment(&self, id: u32) -> Result<()> {
        if id == 0 {
            return Err(anyhow!("Cannot unmap zero segment"));
        }
        if self.mapped.remove(&id).is_some() {
            self.free_ids.push(id).map_err(|_| anyhow!("Free ID queue full"))?;
            Ok(())
        } else {
            Err(anyhow!("Attempted to unmap nonexistent segment {}", id))
        }
    }

    // Check if a segment with the given ID exists
    pub fn contains_segment(&self, id: u32) -> bool {
        self.mapped.contains_key(&id)
    }

    // Load a program into the zero segment from the specified segment ID
    pub fn load_program(&mut self, id: u32) -> Result<()> {
        let segment = self
            .mapped
            .get(&id)
            .ok_or_else(|| anyhow!("Segment {} not found", id))?;
    
        // Clone the program into a new Arc
        let new_program = Arc::from(segment.words.clone().into_boxed_slice());
    
        // Replace the zero segment
        self.zero_segment = new_program;
    
        Ok(())
    }

    // Get a clone of the zero segment (used to retrieve the program)
    pub fn get_zero_segment(&self) -> Arc<[u32]> {
        self.zero_segment.clone()
    }
}
