# Batch OS

### structure

- Image: OS & APP

- U-Mode: app, user lib

- S-Mode: app manager, trap handler, syscall service

### user lib

- compile together with app (under `bin`)
- provide syscall API (actually `ecall` inside) for app
- help init and exit apps

### app manager

- `os/build.rs` generate `link_app.s`, loading app binary into kernel
- a global `APP_MANAGER` (init by reading `link_app.s`) controls the loading and running of apps
- provide `USER_STACK` for app running and `KERNEL_STACK` for trap handling (different with `boot_stack`)

### trap handler

- `trap.s` provides the key handle function `__save_trap_ctx` and `__restore_ctx` in assembly
- `stvec` points to the address of `__save_trap_ctx`, which then distribute trap by calling `trap_handler`
- after `trap_handler` return, `__restore_ctx`  will be executed, which will finally execute `sret`
- handle error and exit similarly by calling `run_next_app`, which push app init context into kernel stack and restore it

### syscall service

- help simplify trap handling by providing syscall service

