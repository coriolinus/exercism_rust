# Doubly Linked List Notes

- `Option<NonNull<*mut Node<T>>>` is a bit verbose, but was fairly straightforward to research
- `(**next.as_ref())` feels like a weird pattern, but appears to work
- there are some rustc warnings about deprecated idioms; that's fine, this branch hasn't been updated in a while; we just need to fix them
- there are some rustfmt changes to the test files; no big deal
- it took longer to get the first test passing than I'd initially expected. The README notes on breaking down the implementation in phases seem good and useful; I just hadn't known to expect that `Drop` would need to be implemented for the first test to pass. Once I'd figured that out and implemented it reasonably, I needed to either bypass the cursor stuff, or implement a suitable subset of the cursor in order to move forward. However, I'm not sure at this time if there's a better way to do things than what is already in place.
- the `Cursor::take` implementation used here seems _really_ kind of hackish; I'm not certain that this is doing exactly what I want it to do. OTOH, it casues the first test to pass, so woohoo!
- In particular, it looks like this may be the _only_ way to extract an owned value from a pointer? That's exactly what `take` requires, so I'm doing it, but I'm really not thrilled by this "shove in some uninitialized memory and yoink the real value for myself" strategy I'm using.
- Also, wasn't there a thing recently where we're supposed to use `mem::MaybeUninit` or something similar instead of `mem::uninitialized`? I think I remember reading about that, but it looks like the former isn't in autocomplete yet. TODO: look this up and do the recommended thing.
- it looks like my cursor implementation differs somewhat from the one expected; perhaps I was supposed to write something like `pub struct Cursor<T>(&mut Node<T>);`? But when I tried that, it was a hassle converting from a raw pointer to a reference all the time. Perhaps when I look at this again, I'll see a good way to abstract that conversion.

