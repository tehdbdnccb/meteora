use std::env;
use std::fs;
use std::path::PathBuf;

// FFI bindings are written by hand rather than generated with bindgen to avoid
// a libclang build dependency. They must match src/engine/orderbook.h exactly.
const BINDINGS: &str = r#"#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Order {
    pub order_id: u64,
    pub price: u64,
    pub size: u32,
    pub side: u8,
    pub _padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BookSnapshot {
    pub best_bid: u64,
    pub best_ask: u64,
    pub bid_volume: u32,
    pub ask_volume: u32,
}

extern "C" {
    pub fn engine_init();
    pub fn process_orders(orders: *const Order, count: u32);
    pub fn get_book_snapshot() -> BookSnapshot;
}
"#;

fn main() {
    println!("cargo:rerun-if-changed=src/engine/avx512_orderbook.cpp");
    println!("cargo:rerun-if-changed=src/engine/orderbook.h");

    // No -mavx512f: the vector loop body is currently empty, and the flag
    // risks SIGILL on hosts without AVX-512.
    cc::Build::new()
        .cpp(true)
        .file("src/engine/avx512_orderbook.cpp")
        .include("src/engine")
        .flag("-std=c++20")
        .flag("-O3")
        .compile("orderbook");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(out_dir.join("bindings.rs"), BINDINGS).expect("Failed to write bindings.rs");
}
