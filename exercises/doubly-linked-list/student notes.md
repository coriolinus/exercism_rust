# Doubly Linked List Notes

- `Option<NonNull<*mut Node<T>>>` is a bit verbose, but was fairly straightforward to research
- `(**next.as_ref())` feels like a weird pattern, but appears to work
- there are some rustc warnings about deprecated idioms; that's fine, this branch hasn't been updated in a while; we just need to fix them
- there are some rustfmt changes to the test files; no big deal
- it took longer to get the first test passing than I'd initially expected. The README notes on breaking down the implementation in phases seem good and useful; I just hadn't known to expect that `Drop` would need to be implemented for the first test to pass. Once I'd figured that out and implemented it reasonably, I needed to either bypass the cursor stuff, or implement a suitable subset of the cursor in order to move forward. However, I'm not sure at this time if there's a better way to do things than what is already in place.
