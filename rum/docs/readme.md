# Universal Machine Optimizations

## Hot Routine and Assembly Analysis

Based on Samply profiling of the final `rum um` build on the **midmark.um** input, the routine consuming the most time is:

```
rum::vm::VirtualMachine::run
```

This fetch–decode–dispatch loop accounts for 100% of running samples (with \~75% in self-samples).

### Assembly Improvement Opportunities

- **Hoist invariant loads into registers**: Cache the `segment0` pointer, its length, and the program counter (`pc`) in local variables (registers) before entering the loop. This removes repeated memory loads (`ldr`) on each iteration.
- **Finalize a lean dispatch loop**: Keep the main loop body minimal (just one compare, one load, one branch table jump). All opcode bodies should live in non-inlined functions (`#[inline(never)]`) to minimize instruction cache pressure.

With these changes, the assembly reduces to a handful of instructions per iteration, minimizes I-cache misses, and avoids unnecessary memory accesses.

## Time Spent

* **Analysis**: \~6 hours
  Profiling, interpreting flamegraphs, reading disassembly, designing optimizations, and planning changes.
* **Implementation & Benchmarking**: \~4 hours
  Refactoring code, applying LTO, removing `HashMap`, adding segment pool, adding `#[inline(never)]`, running hyperfine/hyperfine on both inputs, and validating results.
