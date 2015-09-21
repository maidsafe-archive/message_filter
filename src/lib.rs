// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

//! # Message Filter
//!
//! A size or time based message filter that takes any generic type as a key and will drop keys
//! after a time period, or once a maximum number of messages is reached (LRU Cache pattern).  The
//! filter currently only allows adding messages; a delete function will be provided at a later
//! stage.
//!
//! This library can be used by network based systems to filter previously seen messages.
//!
//! # Examples
//!
//! ```
//! extern crate message_filter;
//! extern crate time;
//! use ::message_filter::MessageFilter;
//!
//! # fn main() {
//! // Construct a `MessageFilter` of `u8`s, limited by message count
//! let max_count = 10;
//! let message_filter = MessageFilter::<u8>::with_capacity(max_count);
//!
//! // Construct a `MessageFilter` of `String`s, limited by expiry time
//! let time_to_live = ::time::Duration::milliseconds(100);
//! let message_filter = MessageFilter::<String>::with_expiry_duration(time_to_live);
//!
//! // Construct a `MessageFilter` of `Vec<u8>`s, limited by message count and expiry time
//! let message_filter = MessageFilter::<Vec<u8>>::with_expiry_duration_and_capacity(time_to_live,
//!                                                                                  max_count);
//! # }
//! ```

#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://maidsafe.github.io/message_filter")]

#![forbid(
    bad_style,              // Includes:
                            // - non_camel_case_types:   types, variants, traits and type parameters
                            //                           should have camel case names,
                            // - non_snake_case:         methods, functions, lifetime parameters and
                            //                           modules should have snake case names
                            // - non_upper_case_globals: static constants should have uppercase
                            //                           identifiers
    exceeding_bitshifts,    // shift exceeds the type's number of bits
    mutable_transmutes,     // mutating transmuted &mut T from &T may cause undefined behavior
    no_mangle_const_items,  // const items will not have their symbols exported
    unknown_crate_types,    // unknown crate type found in #[crate_type] directive
    warnings                // mass-change the level for lints which produce warnings
    )]

#![deny(
    deprecated,                    // detects use of #[deprecated] items
    drop_with_repr_extern,         // use of #[repr(C)] on a type that implements Drop
    improper_ctypes,               // proper use of libc types in foreign modules
    missing_docs,                  // detects missing documentation for public members
    non_shorthand_field_patterns,  // using `Struct { x: x }` instead of `Struct { x }`
    overflowing_literals,          // literal out of range for its type
    plugin_as_library,             // compiler plugin used as ordinary library in non-plugin crate
    private_no_mangle_fns,         // functions marked #[no_mangle] should be exported
    private_no_mangle_statics,     // statics marked #[no_mangle] should be exported
    raw_pointer_derive,            // uses of #[derive] with raw pointers are rarely correct
    stable_features,               // stable features found in #[feature] directive
    unconditional_recursion,       // functions that cannot return without calling themselves
    unknown_lints,                 // unrecognized lint attribute
    unsafe_code,                   // usage of `unsafe` code
    unused,                        // Includes:
                                   // - unused_imports:     imports that are never used
                                   // - unused_variables:   detect variables which are not used in
                                   //                       any way
                                   // - unused_assignments: detect assignments that will never be
                                   //                       read
                                   // - dead_code:          detect unused, unexported items
                                   // - unused_mut:         detect mut variables which don't need to
                                   //                       be mutable
                                   // - unreachable_code:   detects unreachable code paths
                                   // - unused_must_use:    unused result of a type flagged as
                                   //                       #[must_use]
                                   // - unused_unsafe:      unnecessary use of an `unsafe` block
                                   // - path_statements: path statements with no effect
    unused_allocation,             // detects unnecessary allocations that can be eliminated
    unused_attributes,             // detects attributes that were not used by the compiler
    unused_comparisons,            // comparisons made useless by limits of the types involved
    unused_features,               // unused or unknown features found in crate-level #[feature]
                                   // directives
    unused_parens,                 // `if`, `match`, `while` and `return` do not need parentheses
    while_true                     // suggest using `loop { }` instead of `while true { }`
    )]

#![warn(
    trivial_casts,            // detects trivial casts which could be removed
    trivial_numeric_casts,    // detects trivial casts of numeric types which could be removed
    unused_extern_crates,     // extern crates that are never used
    unused_import_braces,     // unnecessary braces around an imported item
    unused_qualifications,    // detects unnecessarily qualified names
    unused_results,           // unused result of an expression in a statement
    variant_size_differences  // detects enums with widely varying variant sizes
    )]

#![allow(
    box_pointers,                  // use of owned (Box type) heap memory
    fat_ptr_transmutes,            // detects transmutes of fat pointers
    missing_copy_implementations,  // detects potentially-forgotten implementations of `Copy`
    missing_debug_implementations  // detects missing implementations of fmt::Debug
    )]

#[cfg(test)]
extern crate rand;
extern crate time;

/// Allows message filter container which may be limited by size or time.
/// Get(value) is not required as only value is stored
pub struct MessageFilter<V>
    where V: PartialOrd + Ord + Clone + ::std::hash::Hash
{
    set: ::std::collections::HashSet<V>,
    list: ::std::collections::VecDeque<(V, ::time::SteadyTime)>,
    capacity: usize,
    time_to_live: ::time::Duration,
}
/// Constructor for size (capacity) based MessageFilter
impl<V> MessageFilter<V> where V: PartialOrd + Ord + Clone + ::std::hash::Hash {
    /// Constructor for capacity based MessageFilter
    pub fn with_capacity(capacity: usize) -> MessageFilter<V> {
        MessageFilter {
            set: ::std::collections::HashSet::new(),
            list: ::std::collections::VecDeque::new(),
            capacity: capacity,
            time_to_live: ::time::Duration::max_value(),
        }
    }
    /// Constructor for time based MessageFilter
    pub fn with_expiry_duration(time_to_live: ::time::Duration) -> MessageFilter<V> {
        MessageFilter {
            set: ::std::collections::HashSet::new(),
            list: ::std::collections::VecDeque::new(),
            capacity: ::std::usize::MAX,
            time_to_live: time_to_live,
        }
    }
    /// Constructor for dual feature capacity or time based MessageFilter
    pub fn with_expiry_duration_and_capacity(time_to_live: ::time::Duration,
                                             capacity: usize)
                                             -> MessageFilter<V> {
        MessageFilter {
            set: ::std::collections::HashSet::new(),
            list: ::std::collections::VecDeque::new(),
            capacity: capacity,
            time_to_live: time_to_live,
        }
    }
    /// Add a value to MessageFilter
    pub fn add(&mut self, value: V) {
        self.remove_expired();

        if self.set.insert(value.clone()) {
            self.list.push_back((value, ::time::SteadyTime::now()));
        }

        let mut trimmed = 0;
        if self.set.len() > self.capacity {
            trimmed = self.set.len() - self.capacity;
        }
        for _ in 0..trimmed {
            let _ = match self.list.pop_front() {
                Some(item) => self.set.remove(&item.0),
                None => false,
            };
        }
    }
    /// Check for existence of a key
    pub fn check(&mut self, value: &V) -> bool {
        self.remove_expired();
        self.set.contains(value)
    }
    /// Current size of cache
    pub fn len(&self) -> usize {
        self.set.len()
    }

    fn remove_expired(&mut self) {
        loop {
            let pop = match self.list.front() {
                Some(item) => if self.time_to_live != ::time::Duration::max_value() &&
                                 item.1 + self.time_to_live < ::time::SteadyTime::now() {
                    true
                } else {
                    break
                },
                None => break,
            };
            if pop {
                match self.list.pop_front() {
                    Some(item) => self.set.remove(&item.0),
                    None => false,
                };
            }
        }
    }
}



#[cfg(test)]
mod test {
    fn generate_random_vec<T>(len: usize) -> Vec<T>
        where T: ::rand::Rand {
        let mut vec = Vec::<T>::with_capacity(len);
        for _ in 0..len {
            vec.push(::rand::random::<T>());
        }
        vec
    }

    #[test]
    fn size_only() {
        let size = 10usize;
        let mut msg_filter = super::MessageFilter::<usize>::with_capacity(size);

        for i in 0..10 {
            println!("i : {} ", i);
            assert_eq!(msg_filter.len(), i);
            msg_filter.add(i);
            assert_eq!(msg_filter.len(), i + 1);
        }

        for i in 10..1000 {
            msg_filter.add(i);
            assert_eq!(msg_filter.len(), size);
        }

        for _ in (0..1000).rev() {
            assert!(msg_filter.check(&(1000 - 1)));
        }
    }

    #[test]
    fn time_only() {
        let time_to_live = ::time::Duration::milliseconds(100);
        let mut msg_filter = super::MessageFilter::<usize>::with_expiry_duration(time_to_live);

        for i in 0..10 {
            assert_eq!(msg_filter.len(), i);
            msg_filter.add(i);
            assert_eq!(msg_filter.len(), i + 1);
        }

        ::std::thread::sleep_ms(100);
        msg_filter.add(11);

        assert_eq!(msg_filter.len(), 1);

        for i in 0..10 {
            assert_eq!(msg_filter.len(), i + 1);
            msg_filter.add(i);
            assert_eq!(msg_filter.len(), i + 2);
        }
    }

    #[test]
    fn time_and_size() {
        let size = 10usize;
        let time_to_live = ::time::Duration::milliseconds(100);
        let mut msg_filter =
            super::MessageFilter::<usize>::with_expiry_duration_and_capacity(time_to_live, size);

        for i in 0..1000 {
            if i < size {
                assert_eq!(msg_filter.len(), i);
            }

            msg_filter.add(i);

            if i < size {
                assert_eq!(msg_filter.len(), i + 1);
            } else {
                assert_eq!(msg_filter.len(), size);
            }
        }

        ::std::thread::sleep_ms(100);
        msg_filter.add(1);

        assert_eq!(msg_filter.len(), 1);
    }

    #[test]
    fn time_size_struct_value() {
        let size = 100usize;
        let time_to_live = ::time::Duration::milliseconds(100);

        #[derive(PartialEq, PartialOrd, Ord, Clone, Eq, Hash)]
        struct Temp {
            id: Vec<u8>,
        }

        let mut msg_filter =
            super::MessageFilter::<Temp>::with_expiry_duration_and_capacity(time_to_live, size);

        for i in 0..1000 {
            if i < size {
                assert_eq!(msg_filter.len(), i);
            }

            msg_filter.add(Temp { id: generate_random_vec::<u8>(64), });

            if i < size {
                assert_eq!(msg_filter.len(), i + 1);
            } else {
                assert_eq!(msg_filter.len(), size);
            }
        }

        ::std::thread::sleep_ms(100);
        msg_filter.add(Temp { id: generate_random_vec::<u8>(64), });

        assert_eq!(msg_filter.len(), 1);
    }
}
