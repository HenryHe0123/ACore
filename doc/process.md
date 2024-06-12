# Process

### 输入支持
- **UART输入**：支持逐字符输入。如果没有字符输入，则等待或将控制权交给其他任务。

### ELF加载器支持
- **`build.rs`**：在内核中嵌入每个应用程序的名称。
- **`elf-loader`**：根据名称加载ELF文件。

### 进程结构

- **PID分配器**：RAII（资源获取即初始化）
- **内核堆栈分配器**：RAII
- **`TaskStruct`**：任务结构体，包含以下信息：
  - PID
  - 内核堆栈资源
  - 任务状态
  - 陷阱上下文
  - 任务上下文
  - 用户空间
  - 该进程的父进程
  - 该进程的子进程

`TaskStruct`存储在堆中，可通过`Arc`或`Weak`访问。
- `Arc<TaskStruct>`。
- `TaskStruct {inner : Mutex<RefCell<TaskStructInner>>}` 或 `TaskStruct {inner : UPSafeCell<TaskStructInner>}}`。
- `Mutex`或`UPSafeCell`：为`Arc`提供`Sync`特性。
- `RefCell`：提供内部可变性。

### 任务管理
- **`TaskManager`**：管理所有任务（准备运行的任务）。
- **`Processor`**：管理当前运行任务，支持多核。
  - `Processor`中的当前任务。
  - `idle_task_ctx`：空闲任务上下文。
  - 调度函数。

示例流程：`task A -> idle -> another task B`，此时有一个空闲控制流，未运行在任何任务的内核堆栈上，而是运行在该核心的引导堆栈上。

### 资源管理
- 支持资源释放。

### 用户程序
- **`initproc`**：初始化进程。
- **`user_shell`**：用户shell。

### 进程系统调用
- **`sys_fork()`**：复制当前任务。
- **`sys_exec()`**：从ELF文件加载。
- **`sys_waitpid(pid, *exit_code)`**：等待子进程并获取退出代码，**释放任务结构体空间**。
  - `sys_waitpid(-1)`：等待任意子进程。
  - `sys_waitpid()`返回`-1`：没有子进程（指定PID）。
  - `sys_waitpid()`返回`-2`：子进程正在运行。
  - `sys_waitpid(id) -> id`：子进程`id`退出。

### 用户API：系统调用的封装
- **`wait`**：`sys_waitpid(-1, *exit_code)`。
- **`waitpid`**：`sys_waitpid(id, *exit_code)`。