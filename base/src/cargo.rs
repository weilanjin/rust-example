// cargo new svc && cd svc
// cargo add serde --features derive

// --- 写代码 ----
// cargo check                       # 改一行就跑一下，秒级反馈
// cargo clippy -- -D warnings       # 收尾前过一遍 lint
// cargo fmt --check                 # 或者 cargo fmt 直接改
// cargo test -- --nocapture
// cargo run -- --config prod.toml
// cargo build --release             # 只在交付前编译一次
