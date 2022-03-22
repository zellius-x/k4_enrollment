
Tạo tài khoản sub-account
    near create-account x.billgate.testnet --masterAccount billgate.testnet --initialBalance 20

Buil wasm file & deploy
    env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
    near deploy --wasmFile target/wasm32-unknown-unknown/release/k4_enrollment.wasm --accountId x.billgate.testnet

Đăng ký khoá học
    near call x.billgate.testnet enroll '{"account_id": "billgate.testnet", "enroll_message": "Khoá học NEAR K4 do VBI-GFS tổ chức tháng 3/2022."}' --accountId billgate.testnet

Kiểm tra nội dung với tài khoản đã đăng ký
    near view x.billgate.testnet get_enroll_status '{"account_id": "billgate.testnet"}' --accountId billgate.testnet

Huỷ đăng ký
    near call x.billgate.testnet delete_enroll_record '{"account_id": "billgate.testnet"}' --accountId billgate.testnet