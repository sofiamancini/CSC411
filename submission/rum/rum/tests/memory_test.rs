use rum::vm::memory::SegmentTable;
use anyhow::Result;

#[test]
fn test_basic_allocation() -> Result<()> {
    let mem = SegmentTable::new(vec![0; 10]);
    let id = mem.map_segment(5)?;
    assert!(mem.contains_segment(id));
    Ok(())
}

#[test]
fn test_zero_segment_protection() {
    let mem = SegmentTable::new(vec![0; 10]);
    // Verify we can't unmap segment 0
    assert!(mem.unmap_segment(0).is_err());
}

#[test]
fn test_segment_reuse() -> Result<()> {
    let mem = SegmentTable::new(vec![0; 10]);
    let id1 = mem.map_segment(100)?;
    mem.unmap_segment(id1)?;
    let id2 = mem.map_segment(100)?;
    assert_eq!(id1, id2, "IDs should be recycled");
    Ok(())
}

#[test]
fn test_segment_access() -> Result<()> {
    let mem = SegmentTable::new(vec![0; 10]);
    let id = mem.map_segment(3)?;
    assert_eq!(mem.get(id, 0)?, 0);
    assert_eq!(mem.get(id, 2)?, 0);
    Ok(())
}

#[test]
fn test_invalid_access() {
    let mem = SegmentTable::new(vec![0; 10]);
    assert!(mem.get(999, 0).is_err());
}

#[test]
fn test_program_loading() -> Result<()> {
    let program = vec![1, 2, 3, 4, 5];
    let mem = SegmentTable::new(program.clone());
    assert_eq!(mem.fetch(0)?, 1);
    assert_eq!(mem.fetch(4)?, 5);
    assert!(mem.fetch(5).is_err());
    Ok(())
}

#[test]
fn test_segment_write_and_read() -> Result<()> {
    let mem = SegmentTable::new(vec![0; 10]);
    let id = mem.map_segment(2)?;
    mem.set(id, 0, 42)?;
    assert_eq!(mem.get(id, 0)?, 42);
    Ok(())
}

#[test]
fn test_fetch_non_zero_segment() -> Result<()> {
    let mem = SegmentTable::new(vec![10, 20, 30, 40, 50]);
    // `fetch` only works on segment 0
    assert_eq!(mem.fetch(2)?, 30);
    assert!(mem.fetch(5).is_err());
    Ok(())
}
