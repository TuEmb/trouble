//! A `#[gatt_service]` with a characteristic larger than `MAX_SMALL_DATA_SIZE`
//! keeps its value in a shared `static mut` buffer. A sequential rebuild must
//! work (the old one-shot `StaticCell` panicked on the second build), but two
//! live instances would alias that buffer, so the second build must panic.
//!
//! The two tests use different service types so their guards don't interfere
//! when run on separate threads.

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use trouble_host::prelude::*;

#[gatt_service(uuid = "7e701cf1-b1df-42a1-bb5f-6a1028c793b0")]
struct BigService {
    // 32 > MAX_SMALL_DATA_SIZE (20), so the value needs external 'static storage.
    #[characteristic(uuid = "2a37", read, write)]
    big: [u8; 32],
}

#[gatt_service(uuid = "7e701cf1-b1df-42a1-bb5f-6a1028c793b1")]
struct AliasService {
    #[characteristic(uuid = "2a38", read, write)]
    big: [u8; 32],
}

// Build, drop, build again: the rebuild reuses the same storage.
#[test]
fn gatt_service_can_be_built_twice() {
    for _ in 0..2 {
        let mut table: AttributeTable<NoopRawMutex, 8> = AttributeTable::new();
        let _ = BigService::new(&mut table);
    }
}

// Building a second instance while the first is alive aliases the buffer -> panic.
#[test]
#[should_panic]
fn gatt_service_panics_if_built_while_alive() {
    let mut t1: AttributeTable<NoopRawMutex, 8> = AttributeTable::new();
    let mut t2: AttributeTable<NoopRawMutex, 8> = AttributeTable::new();
    let _s1 = AliasService::new(&mut t1);
    let _s2 = AliasService::new(&mut t2);
}
