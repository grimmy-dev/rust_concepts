# arena

a typed arena allocator library - you hand it values, it owns them in
one contiguous store and hands back a way to reach them. the whole point
is to learn ownership, borrowing, and lifetimes by fighting the borrow
checker, not by routing around it. library crate, not a binary.

`cargo new arena --lib`

## required Behaviour

an `Arena<T>` owns a backing store. you `alloc` a value into it and get a
handle back. the arena owns every value until the arena itself is dropped,
at which point everything inside dies with it. references into the arena
must not be able to outlive the arena - the compiler enforces this, that
is the lesson.

## API

- `Arena::new()` - empty arena
- `alloc(&mut self, value: T) -> &mut T` - store a value, get a reference into the store
- `len(&self) -> usize`
- `is_empty(&self) -> bool`
- `get(&self, index: usize) -> Option<&T>` - added in the index phase below

## Constraints

- no `unsafe`
- no `.clone()` in the core API (tests may clone for setup)
- no `Rc` / `Arc` / `RefCell` - win with the borrow checker, don't escape it
- `Vec` is the backing store, that's fine - not building allocation from raw memory

## Exercises (these ARE the project)

1. alloc one value, read it back. proves the happy path.
2. alloc two values, try to hold both `&mut` references at once.
   does NOT compile. do not fix it - write a comment explaining WHY
   `&mut self` -> `&mut T` makes a second alloc illegal while the first
   ref is alive.
3. try to make a reference outlive the arena (hold a ref, drop the arena).
   does NOT compile. this is a use-after-free caught at compile time.
   write down what the error was.
4. switch alloc to return a `usize` index instead of `&mut T`, add
   `get(index) -> Option<&T>`. now exercises 2 and 3 compile. explain
   why the index version is freer (no borrow held) but weaker (no
   compile-time guarantee the index is valid).

## Stretch (only if flying)

- make the arena hold borrowed data: `Arena<&'a str>`, thread the `'a`
  lifetime through by hand. explicit lifetime annotations become
  load-bearing here, not decoration.

## Checklist

- [x] `Arena<T>` owns its values, drops them with itself
- [x] alloc returning `&mut T` works for the single-reference case
- [x] exercise 2 fails to compile and I wrote down why
- [x] exercise 3 fails to compile and I wrote down why
- [x] index version (alloc -> usize, get -> Option) works
- [x] `get` returns `Option`, never panics on a bad index
- [x] zero unsafe, zero clone in core API, no Rc/Arc/RefCell
- [x] I can explain OUT LOUD why `&mut T` alloc blocks a second alloc
      while the ref is held, and why the index version doesn't
