
pub struct SegCache {
    entries: [Option<(u32, Arc<[u32]>)>; 8],
}

impl SegCache {
    pub fn access(&mut self, id: u32) -> Option<Arc<[u32]>> {
        // ... lookup and update LRU order
    }
}