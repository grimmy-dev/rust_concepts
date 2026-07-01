# Arena

A simple version of modern arena. Arena is a single, unified owner for multiple values, they live together and die together along with arena.

## Implementation

```rs
let mut arena = Arena::new();

let t1:&mut T = arena.alloc(String::new("hello and this is owned by arena"));
```

| Methods | Description |
| ------- | ----------- |
| `fn new() -> Arena<T>` | Initializes a storage vec |
| `fn alloc(value:T) ->&mut T` | pushes the value to storage and give mutable reference of that value |
| `fn len(&self) -> usize` | Returns count of stored values |
| `fn is_empty(&self) -> bool` | Check whether the storage is empty |

**Disclaimer**: The tests are written intentionally to fail and not compile so we can get to know about compile time and run time safety and `cargo build` will fail (I do not care as long as concepts are clear).
