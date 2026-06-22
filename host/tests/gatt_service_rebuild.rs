//! A `#[gatt_service]` with a characteristic larger than `MAX_SMALL_DATA_SIZE`
//! must be buildable more than once. The macro kept the value buffer in a
//! one-shot `StaticCell`, so the second build panicked ("StaticCell already full").

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use trouble_host::prelude::*;

#[gatt_service(uuid = "7e701cf1-b1df-42a1-bb5f-6a1028c793b0")]
struct BigService {
    // 32 > MAX_SMALL_DATA_SIZE (20), so the value needs external 'static storage.
    #[characteristic(uuid = "2a37", read, write)]
    big: [u8; 32],
}

#[test]
fn gatt_service_can_be_built_twice() {
    for _ in 0..2 {
        let mut table: AttributeTable<NoopRawMutex, 8> = AttributeTable::new();
        let _ = BigService::new(&mut table);
    }
}
