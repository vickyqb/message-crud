cargo build --release --target wasm32-unknown-unknown --package message_board_backend

candid-extractor target/wasm32-unknown-unknown/release/message_board_backend.wasm >src/message_board_backend/message_board_backend.did