1. [yrm](#yrm)
2. [Tools Mentioned](#tools-mentioned)
   1. [JustFile](#justfile)
   2. [Cargo ASM](#cargo-asm)
3. [Note Section](#note-section)
4. [Understanding the Assembly Patterns](#understanding-the-assembly-patterns)
   1. [Link](#link)
   2. [Summary Differences of Stack and Heap (Rust)](#summary-differences-of-stack-and-heap-rust)
   3. [📌 Stack vs Heap in Assembly (Rust)](#-stack-vs-heap-in-assembly-rust)
   4. [📌 Stack vs Heap Pointers in Assembly (Rust)](#-stack-vs-heap-pointers-in-assembly-rust)
   5. [When Assembly Instruction is not visible](#when-assembly-instruction-is-not-visible)

# yrm

> Learning about rust memory management from yt vid

# Tools Mentioned 

## JustFile 

> This is an alternative to nmake file 

```sh
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to DEST
``` 
- Above is installation taken from [HERE](https://just.systems/man/en/pre-built-binaries.html)
- Firs run the above install then add to path 
- `fish_add_path DEST` - Note dest is where it gets installed 
- Now the program will work `justfile`

## Cargo ASM 

1. This is for getting the assembly of the program 
2. The tutorial was showin an older crate , we will use a newer crate 

[`cargo-show-asm`](https://lib.rs/crates/cargo-show-asm) - Shows the generated assembly code 

```rs 
cargo install cargo-show-asm
```
- Note this is more modern crate which addresses short comings of `cargo asm`

# Note Section 

> This will have important notes from the tutorial video 

1. Stack (Fixed size, fast) is LIFO , Heap(Dynamic, slow) is FILO
2. Arrays live directly in the stack in rust 
3. `vec![]` - Vector is array of dynamic size , doent live on the stack , but lives on the heap


# Understanding the Assembly Patterns 

## Link 

1. [x86 Assembly Instructions](https://www.cs.virginia.edu/~evans/cs216/guides/x86.html) - List of instructions , note 
2. [x86 Assembly Reference](https://www.felixcloutier.com/x86/)


## Summary Differences of Stack and Heap (Rust)

## 📌 Stack vs Heap in Assembly (Rust)

| Feature        | Stack 🟢 | Heap 🔴 |
|---------------|---------|--------|
| **Storage Type** | Fixed-size, local variables | Dynamic memory allocation |
| **Memory Access** | `[rsp + offset]` (direct access) | `mov rax, [ptr]` (pointer dereference) |
| **Allocation** | `sub rsp, X` (manual) | `call __rust_alloc` (dynamic) |
| **Deallocation** | `add rsp, X` (automatic on function return) | `call __rust_dealloc` (manual via `drop`) |
| **Ownership** | Function scope (auto cleanup) | Must be explicitly freed (`Box`, `Rc`, etc.) |
| **Lifetime** | Short (until function exits) | Long (until manually freed) |
| **Speed** | Faster (LIFO, cache-friendly) | Slower (fragmentation, extra pointer indirection) |
| **Common Uses** | Local variables, function calls | `Vec<T>`, `Box<T>`, `Rc<T>`, `Arc<T>` |
| **Example Allocation** | `sub rsp, 32` (reserve 32 bytes) | `call __rust_alloc` (allocate heap memory) |
| **Example Access** | `mov rax, [rsp + 16]` (stack var) | `mov rax, [rax]` (dereferencing heap ptr) |




## 📌 Stack vs Heap Pointers in Assembly (Rust)

| Rust Pointer Type  | Stack or Heap? | Assembly Pattern |
|--------------------|---------------|------------------|
| **`&T` (Reference)** | **Stack** ✅ | `lea rdi, [rsp + offset]` (passing address of stack var) |
| **`&mut T` (Mutable Reference)** | **Stack** ✅ | `mov rax, [rsp + offset]` (reading/modifying stack var) |
| **`Box<T>`** | **Heap** ✅ | `call __rust_alloc` (allocates memory) → `mov [rsp + offset], rax` (stores heap pointer on stack) |
| **`Rc<T>`** | **Heap** ✅ | `call __rust_alloc` + `lock inc qword ptr [rax]` (reference counting) |
| **`Arc<T>`** | **Heap** ✅ | `call __rust_alloc` + `lock inc qword ptr [rax]` (atomic reference counting) |
| **`Vec<T>`** | **Heap** ✅ | `call __rust_alloc` (allocates memory) + `mov [rsp + offset], rax` (stores heap pointer) |
| **`String`** | **Heap** ✅ | `call __rust_alloc` (allocates memory) + `mov rax, [ptr]` (accessing string) |
| **`&str` (String Slice)** | **Stack** ✅ (if `&"hello"`) / **Heap** ✅ (if from `String`) | `lea rax, [rip + .L__unnamed]` (stack) OR `mov rax, [ptr]` (heap) |
| **`*const T` / `*mut T` (Raw Pointer)** | **Stack** (points to stack) or **Heap** (points to heap) | `mov rax, [rsp + offset]` (stack) OR `mov rax, [ptr]` (heap) |

```mermaid
graph TD;
    A[Memory Allocation] -->|Fixed-size, local variables| B[Stack]
    A -->|Dynamic memory allocation| C[Heap]

    B -->|Direct memory access| B1[Fast Access]
    B -->|Manual allocation (sub rsp, X)| B2[Predefined Size]
    B -->|Auto deallocation| B3[Cleanup on Function Return]
    B -->|Short Lifetime| B4[Until Function Exits]
    B -->|Faster (LIFO, Cache-Friendly)| B5[Efficient]

    C -->|Indirect access via pointer| C1[Slower Access]
    C -->|Dynamic allocation (call rust_alloc)| C2[Flexible Size]
    C -->|Manual deallocation (call rust_dealloc)| C3[Must Be Freed Manually]
    C -->|Long Lifetime| C4[Until Explicitly Freed]
    C -->|Slower (Fragmentation, Pointer Indirection)| C5[Less Efficient]

    B -->|Used for Local Vars, Function Calls| B6[Common Uses]
    C -->|Used for Vec, Box, Rc, Arc| C6[Common Uses]

```

## When Assembly Instruction is not visible 

1. The compiler does some optimization, so the actual assembly code may not be visible in the assembly output.
2. you need to use - `#[inline(never)]` this will force it to be a seperate function