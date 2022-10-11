svd2rust -i HPM6750.svd --target riscv --const_generic --nightly
rm -rf src
form -i lib.rs -o src/ 
rm lib.rs
cargo fmt
