# blog_os#1301

My testing setup to fix [phil-opp/blog_os#1301](https://github.com/phil-opp/blog_os/issues/1301).

Thanks to [michihupf](https://github.com/michihupf) for insight,
and [Michael Zalla](https://github.com/MichaelZalla) for his code, that I heavily modified.

### Assembly diff

Where green code results in no bootloop, and red one does.

```diff
_start:
+    movzx  eax, byte  ptr [753664]
+    movzx  eax, byte  ptr [753665]
-    mov    rax, qword ptr [753664]
-    movzx  ecx, byte  ptr [rax]
-    movzx  eax, byte  ptr [rax + 1]
    .p2align 4, 0x90
```

### Recreating Assembly

There are only 2 files you need to modify:

- [kernel.rs](./kernel.rs): Uncomment which `Volatile` type alias you would like to test
- [Cargo.toml](./Cargo.toml): Uncomment the version you would like to test

Where the comments say "Good" that means that the code will not produce a bootloop,
and "Bad" means it will.

Also, here is command to generate rust assembly code:

```shell
cargo rustc -p blog_os-1301 --release -- --emit asm -C "llvm-args=-x86-asm-syntax=intel"
```

The assembly file will be in
[target/x86_64-rust_os/release/deps](./target/x86_64-rust_os/release/deps).
