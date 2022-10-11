svd2rust -i HPM6750.svd --target riscv --const_generic --nightly
Remove-Item -Path "src" -Recurse
form -i lib.rs -o src/ 
Remove-Item lib.rs
cargo fmt
