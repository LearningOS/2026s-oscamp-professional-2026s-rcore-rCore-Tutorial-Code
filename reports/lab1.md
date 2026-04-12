# 实验一报告

## 实验环境

- 仓库：`2026s-rcore-yuanssssss`
- 检查章节：`CHAPTER=3`
- 运行平台：QEMU `riscv64`

## 工作概述

本次实验通过主要包含内核中的两部分修复：

1. 实现了 `sys_trace` 及其所需的配套逻辑。
2. 修复了时间与休眠相关行为，使 chapter 3 的 sleep 测试能够正常完成。

## 实现内容

### trace 系统调用

在内核中实现了 `sys_trace`，支持以下功能：

- `TraceRequest::Syscall`：返回当前任务的系统调用次数统计
- `TraceRequest::Read`：从合法的用户地址读取 1 个字节
- `TraceRequest::Write`：向合法的用户地址写入 1 个字节

同时增加了按任务维度统计 syscall 次数的能力，以及查询当前任务信息的辅助函数。

### 时间与 sleep

调整了内核时间返回路径，避免 chapter 3 测试中前期调用 `get_time()` 时意外返回 `0ms`。

另外增加了一个由时钟中断推进的软件毫秒时间源，并将其与硬件时间值结合，
从而在测试环境中提供更稳定的 sleep 行为。

## 实验结果

chapter 3 检查中的相关模式已经全部通过：

- `get_time OK`
- `Test sleep OK`
- `Test sleep1 passed`
- `string from task trace test`
- `Test trace OK`

## 备注

在调试过程中，由于工作区挂载到 Docker 环境，部分文件一度被 `root` 持有。
之后已经修正所有权，使内核源码可以正常修改并重新构建。
