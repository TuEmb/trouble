//! A `#[gatt_service]` with a characteristic larger than `MAX_SMALL_DATA_SIZE`
//! keeps its value in a caller-owned `<Service>Storage`. A sequential rebuild
//! reuses the same storage (the old one-shot `StaticCell` panicked on the second
//! build). Building two live instances from one storage is rejected by the borrow
//! checker (`cannot borrow `storage` as mutable more than once`), so the value
//! buffer can never be aliased.

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use trouble_host::prelude::*;

#[gatt_service(uuid = "7e701cf1-b1df-42a1-bb5f-6a1028c793b0")]
struct BigService {
    // 32 > MAX_SMALL_DATA_SIZE (20), so the value needs external storage.
    #[characteristic(uuid = "2a37", read, write)]
    big: [u8; 32],
}

// Build, drop, build again from the same storage.
#[test]
fn gatt_service_can_be_built_twice() {
    let mut storage = BigServiceStorage::new();
    for _ in 0..2 {
        let mut table: AttributeTable<NoopRawMutex, 8> = AttributeTable::new();
        let _svc = BigService::new(&mut table, &mut storage);
    }
}
