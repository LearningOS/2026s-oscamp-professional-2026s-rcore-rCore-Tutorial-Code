# Lab1 实验报告（ch3）

## 一、功能总结（200字以内）
本实验在 ch3 内核中实现了 sys_trace（ID 410）三类能力：按字节读取当前任务地址、按字节写入当前任务地址、查询指定系统调用号的累计调用次数。系统调用计数在内核统一 syscall 入口更新，保证 trace 查询时“本次调用也计入统计”。另外将 syscall 计数字段内聚到 TaskControlBlock，减少并行数组维护复杂度；在读写分支加入 user_byte_accessible 检查，地址越界时返回 -1，非法 trace_request 统一返回 -1。

## 二、实现说明
1. 在 syscall 总入口对当前任务进行系统调用计数。
2. 为任务控制块维护 syscall_times 数组，按 syscall_id 记录次数。
3. 在 sys_trace 中实现三种行为：
   - trace_request = 0：将 id 视为 *const u8，读取 1 字节并返回。
   - trace_request = 1：将 id 视为 *mut u8，写入 data 的低 8 位并返回 0。
   - trace_request = 2：返回当前任务 id 对应系统调用号的调用次数。
4. 增加边界判断：读取/写入前用 user_byte_accessible 检查地址合法性。

## 三、简答作业
### 1. 进入 U 态后的特征与 bad 测例现象
- 在 U 态执行 S 态特权指令或访问 S 态 CSR 会触发异常，程序无法继续正常执行。
- 在本实验框架中，相关 bad 测例会进入非法指令异常分支，内核输出类似：
  [kernel] IllegalInstruction in application, kernel killed it.
- 本地使用的 SBI 信息（由 bootloader 字符串提取）：
  - 实现：RustSBI-QEMU
  - 兼容：RISC-V SBI v1.0.0
  - 相关源码版本字符串：rustsbi-0.3.0-alpha.2

### 2. trap.S 中 __alltraps 与 __restore 问答
#### 2.1 L40：刚进入 __restore 时，sp 代表什么？__restore 的两种使用情景？
- 刚进入 __restore 时，sp 指向内核栈上的 TrapContext（当前任务的陷入现场）。
- 两种使用情景：
  1) 处理完一次 trap 后，从 trap_handler 返回，继续执行 __restore 回到用户态。
  2) 任务首次或切换后被调度运行时，通过 TaskContext::goto_restore 把返回地址设为 __restore，直接走恢复路径进入用户态。

#### 2.2 L43-L48 特殊处理了哪些寄存器？它们对进入用户态的意义？
```
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```
- sstatus：恢复特权返回相关状态位（如 SPP/SPIE 等），决定 sret 后处理器状态。
- sepc：恢复用户态下一条将执行的 PC。
- sscratch：保存用户栈指针，供特权态/用户态栈交换使用。

#### 2.3 L50-L56：为何跳过 x2 和 x4？
- x2 是 sp，不在此处直接恢复，后续通过 csrrw sp, sscratch, sp 交换恢复。
- x4 是 tp（线程指针），本实验用户程序不依赖该寄存器，因此不在通用恢复循环中处理。

#### 2.4 L60 后，sp 和 sscratch 的意义？
```
csrrw sp, sscratch, sp
```
- sp 变为用户栈指针，供返回用户态后使用。
- sscratch 变为当前内核栈指针，供下一次 trap 进入时快速切换到内核栈。

#### 2.5 __restore 中发生状态切换的是哪条指令？为何会进入用户态？
- 指令是 sret。
- sret 会依据 sstatus 中保存的返回特权级（SPP）和中断使能位恢复执行环境，并跳转到 sepc；当 SPP=U 时即返回 U 态。

#### 2.6 L13 后，sp 和 sscratch 的意义？
```
csrrw sp, sscratch, sp
```
- 在 __alltraps 入口执行后，sp 切换为内核栈指针。
- sscratch 保存陷入前的用户栈指针。

#### 2.7 从 U 态进入 S 态是哪一条指令触发的？
- 在系统调用场景中由 ecall 触发。
- 对于非法指令、访存错误或时钟中断等场景，也会由对应 trap 事件触发硬件从 U 态进入 S 态。

