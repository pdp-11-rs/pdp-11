# word_mut vs write_word Refactoring Options

## Problem Statement

The PDP-11 emulator has two different patterns for modifying memory operands:

1. **`word_mut()`** - Returns `&mut Word`
   - Used by: CLR, ASL, SWAB, BIC, BIS, ADD, SUB, COM, INC, DEC, NEG, ADC, SBC, ROR, ROL, ASR (16 instructions)
   - **Problem**: Only works with RAM, bypasses MMIO entirely
   - Pattern: `*self.word_mut(dst) = result;` or `self.word_mut(dst).clear()`

2. **`write_word()`** - Takes `value: Word`
   - Used by: MOV (1 instruction)
   - **Benefit**: Properly routes writes through MMIO subsystem
   - Pattern: `self.write_word(dst, word);`

This inconsistency means most instructions will **fail silently when operating on MMIO registers**, which is a serious bug.

## Solution Branches

Two approaches have been implemented in separate branches for comparison:

### Branch 1: `refactor/unified-write-only`

**Strategy**: Eliminate `word_mut()` entirely, use only `write_word()`

**Commit**: `1628d82`

**Changes**:
- Convert all 16 instructions to read-modify-write pattern
- Remove dependency on `word_mut()`

**Example**:
```rust
// Before
fn clr(&mut self, dst: Operand) {
    self.word_mut(dst).clear();
    self.psw[Z] = true;
}

// After
fn clr(&mut self, dst: Operand) {
    self.write_word(dst, Word::zero());
    self.psw[Z] = true;
}
```

**Pros**:
- ✅ Simple, consistent API
- ✅ All instructions handle MMIO correctly
- ✅ Easy to understand and maintain
- ✅ Clear separation: read vs write

**Cons**:
- ⚠️ Slightly less efficient for RAM (read + write vs direct mutation)
- ⚠️ More verbose for some instructions
- ⚠️ Must read value even if just clearing to zero

**Code Impact**: 18 insertions(+), 17 deletions(-)

### Branch 2: `refactor/closure-based`

**Strategy**: Add `modify_word()` method that takes a closure

**Commit**: `ce44786`

**Implementation**:
```rust
pub(super) fn modify_word<F>(&mut self, operand: Operand, f: F)
where
    F: FnOnce(&mut Word),
{
    let mut value = *self.word(operand);
    f(&mut value);
    self.write_word(operand, value);
}
```

**Example**:
```rust
// Before
fn clr(&mut self, dst: Operand) {
    self.word_mut(dst).clear();
    self.psw[Z] = true;
}

// After
fn clr(&mut self, dst: Operand) {
    self.modify_word(dst, |w| w.clear());
    self.psw[Z] = true;
}
```

**Pros**:
- ✅ Handles MMIO correctly
- ✅ Functional style, clear data flow
- ✅ Can keep mutable reference syntax inside closure
- ✅ More flexible than write-only

**Cons**:
- ⚠️ Closure syntax less familiar to some developers
- ⚠️ Must capture values needed outside closure (for flags)
- ⚠️ Slightly more complex implementation

**Code Impact**: 44 insertions(+), 17 deletions(-)

### Branch 3 (Not Implemented): Proxy Object

**Strategy**: Return a smart proxy that handles MMIO on drop

**Why Skipped**: Borrow checker issues make this approach impractical in Rust without significant complexity. The proxy would need to either:
1. Borrow `&mut Cpu` (preventing any other CPU access)
2. Use unsafe code or complex callback mechanisms
3. Require manual `commit()` calls (defeating the purpose)

## Performance Considerations

### RAM Operations

| Approach | Pattern | Cost |
|----------|---------|------|
| Current (word_mut) | Direct mutation | 0 overhead |
| Unified write-only | Read + Write | 1 extra read |
| Closure-based | Read + Write | 1 extra read |

**Impact**: Negligible. Modern CPUs have fast L1 cache, and the bottleneck in emulation is instruction dispatch, not memory access.

### MMIO Operations

| Approach | Correctness |
|----------|-------------|
| Current (word_mut) | ❌ **Broken** - bypasses devices |
| Unified write-only | ✅ Correct - routes through devices |
| Closure-based | ✅ Correct - routes through devices |

## Recommendation

**Use `refactor/unified-write-only`** (Branch 1)

**Rationale**:
1. **Simplicity**: Easiest to understand and maintain
2. **Consistency**: One pattern for all write operations
3. **Correctness**: Fixes the MMIO bug completely
4. **Familiarity**: No new concepts (closures, proxies)
5. **Minimal changes**: Smallest code delta

The closure-based approach is elegant but adds conceptual overhead without significant benefit. The extra explicitness of "read value, compute result, write back" is actually a feature, not a bug - it makes the data flow clearer.

## Migration Path

If unified-write-only is chosen:

1. Merge `refactor/unified-write-only` into `develop`
2. Remove `word_mut()` and `byte_mut()` from `impls.rs`
3. Verify all 181 tests still pass
4. Close the refactoring branches

## Testing

Both branches pass all 181 tests:
```
test result: ok. 181 passed; 0 failed; 0 ignored; 0 measured
```

The tests verify:
- ✅ All instruction implementations work correctly
- ✅ PSW flags are set properly
- ✅ Register addressing modes function
- ✅ Memory-mapped I/O (console, RK11, KW11) operates

## Related Issues

This refactoring prepares for:
- MMU implementation (memory protection requires write interception)
- Additional MMIO devices (TM11 tape, etc.)
- DMA transfers (which need consistent write routing)
