use pretty_assertions::assert_eq;

use rocksdb::Cache;
use rocksdb::WriteBufferManager;

#[test]
fn test_write_buffer_manager_with_cache() {
    let wmb = WriteBufferManager::new(10, Some(&Cache::new_lru_cache(10)));
    assert_eq!(wmb.buffer_size(), 10);
}

#[test]
fn test_write_buffer_manager_with_no_cache() {
    let wmb = WriteBufferManager::new(10, None);
    assert_eq!(wmb.buffer_size(), 10);
}
