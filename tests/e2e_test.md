# 端到端测试：验证
构建与测试流程：

1. 获取源码
git clone https://github.com/defense-os/minimal-defense-os
cd minimal-defense-os

2. 设置工具链
./setup-riscv-toolchain.sh
source environment.rc

3. 构建内核
make seL4_defconfig
make -j$(nproc)

4. 构建运行时
cd runtime
cargo build --release --target=riscv64imac-unknown-none-elf

5. 生成系统镜像
./mkimage.sh -o defense-os.img