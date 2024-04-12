# Memory Management

Q：How does my kernel address space achieve identical mapping?

A:

- When kernel address space init, it will first create a page table and identical mapping areas.
- A map area contains VPN range and mapping messages, no frame allocation in new.
- When adding new map areas to kernel address space, they will be mapped to the page table.
- This means creating identical mapping for each VPN in page table, no frame allocation.
