# 问题排查

## 问题1：延迟超标
sudo systemctl status defense-os
检查：CPU占用、内存使用、调度延迟

## 问题2：误报率高
defense-analyzer --debug --log-level=verbose
分析：特征提取、阈值设置、模型版本

## 问题3：更新失败
defense-updater --rollback --verify
操作：回滚版本、验证签名、检查磁盘