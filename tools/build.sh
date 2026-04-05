#!/bin/bash
# defense-os 构建脚本

# 1. 工具链
./setup-riscv-toolchain.sh
source environment.rc

# 2. 内核
make seL4_defconfig
make -j$(nproc)

# 3. 运行时
cd runtime
cargo build --release --target=riscv64imac-unknown-none-elf

# 4. 镜像
cd ..
./mkimage.sh -o defense-os.img