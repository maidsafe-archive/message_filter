# Message Filter - Change Log

## [0.3.0]
- Only store hash codes instead of complete messages.
- Count the number of times a message has been inserted.

## [0.2.0]
- Renamed API function `add` to `insert` and made it return the element if it pre-existed
- Renamed API function `check` to `contains`
- Refactored internals to avoid duplicating container of messages
- Created a test for adding a duplicate message
- Fixed compiler error in benchmark
- Removed deprecated lint check
- Fixed warnings in documentation test

## [0.1.5]
- Remove wildcard dependencies.

## [0.1.4]
- [#36](https://github.com/maidsafe/message_filter/pull/36) remove expired values at start of add and check
- update CI scripts

## [0.1.3]
- remove unwrap calls
- update CI scripts

## [0.1.2]
- Implement add_key  (bool return, true added, false == already exists)
- Test add_key (time and size based tests)
