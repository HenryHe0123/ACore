# Process Management

### Process Control Block (U)

- pid
- status
- parent
- children
- exit code

### Task Control Block (S)

- pid
- kernel stack
- memory space
- task context
- trap context

### Fork

sys_fork

Process Manager:
- allocate new pid
- prepare PCB

Task Manager:
- copy memory space
- prepare TCB

### Exit

sys_exit -> exit_current_and_run_next

Process Manager:
- record exit code and change status
- move children to initproc

Task Manager:
- recycle resources (TCB)
- run next task

### Wait

sys_waitpid

Process Manager:
- find zombie child by pid
- get exit code and remove from children list

Task Manager:
- store exit code
