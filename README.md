# ACore

This is a simple RISC-V microkernel built from scratch, written in Rust.

### Features

- Microkernel Architecture (Process Manager in U-mode)
- Virtual Memory with SV39
- Bash-like Shell

### Quick Start

Run the following commands to build and test the kernel:

```
$ git clone git@github.com:HenryHe0123/ACore.git
$ cd ACore/os
$ make run
```

Then you can see the initial output of the OS:

```
 ___  ___  ____    ____  _______  ______    __
|   \/   | \   \  /   / /       ||   _  \  |  |
|  \  /  |  \   \/   / |   (----`|  |_)  | |  |
|  |\/|  |   \_    _/   \   \    |   _  <  |  |
|  |  |  |     |  | .----)   |   |  |_)  | |  |
|__|  |__|     |__| |_______/    |______/  |__|
[mysbi] Hello, kernel!
[kernel] .text [0x80000000, 0x8000f000)
[kernel] .rodata [0x8000f000, 0x80013000)
[kernel] .data [0x80013000, 0x80162000)
[kernel] .bss [0x80162000, 0x80473000)
[kernel] mapping .text section
[kernel] mapping .rodata section
[kernel] mapping .data section
[kernel] mapping .bss section
[kernel] mapping physical memory
[kernel] mapping MMIO
[kernel] Hello, MMU!
[kernel] ----- APPS -----
[kernel] exit
[kernel] fantastic_text
[kernel] forkexec
[kernel] forktest
[kernel] forktest2
[kernel] forktest_simple
[kernel] forktree
[kernel] hello_world
[kernel] matrix
[kernel] sleep
[kernel] sleep_simple
[kernel] stack_overflow
[kernel] usertests
[kernel] usertests-simple
[kernel] yield
[kernel] ----------------
[initproc] Start running.
[process manager] Start running.
Welcome to Shell!
henryhe@ACore:~$
```

The supported applications are listed above. You can use the built-in `ls` command to display them again. To run an application, just type its name and press Enter. `usertests` can run a bunch of applications, thus it is recommended.

There are several ways to exit the OS. You can gracefully shut down by typing `shutdown` in the shell and pressing Enter. Or you can type `Ctrl+a` then `x` to terminate Qemu. We also support using `Ctrl+c` to terminate the shell and then exit the system.

### Tutorial

- [ACore-Guide](https://acore-guide.sjtu.app)
- [rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3)
