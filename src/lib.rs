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

#![crate_name = "message_filter"]
#![crate_type = "lib"]
#![forbid(bad_style, missing_docs, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, non_shorthand_field_patterns,
        overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
        raw_pointer_derive, stable_features, unconditional_recursion, unknown_lints,
        unsafe_code, unused, unused_allocation, unused_attributes, unused_comparisons,
        unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, variant_size_differences)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/message_filter/")]

//! #Message filter limited via size or time  
//! 
//! This container allows time or size to be the limiting factor for any key types.
//!
//!#Use
//!
//!##To use as a size based MessageFilter 
//!
//!`let mut message_filter = MessageFilter::<usize>::with_capacity(size);`
//!
//!##Or as a time based MessageFilter
//! 
//! `let time_to_live = time::Duration::milliseconds(100);`
//!
//! `let mut message_filter = MessageFilter::<usize>::with_expiry_duration(time_to_live);`
//! 
//!##Or as time or size limited cache
//!
//! ` let size = 10usize;
//!     let time_to_live = time::Duration::milliseconds(100);
//!     let mut message_filter = MessageFilter::<usize>::with_expiry_duration_and_capacity(time_to_live, size);`


extern crate time;

use std::usize;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// Allows message filter container which may be limited by size or time.
/// Get(value) is not required as only value is stored
pub struct MessageFilter<V> where V: PartialOrd + Ord + Clone + Hash {
    set: HashSet<V>,
    list: VecDeque<(V, time::SteadyTime)>,
    capacity: usize,
    time_to_live: time::Duration,
}
/// Constructor for size (capacity) based MessageFilter
impl<V> MessageFilter<V> where V: PartialOrd + Ord + Clone + Hash {
    /// Constructor for capacity based MessageFilter
    pub fn with_capacity(capacity: usize) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: capacity,
            time_to_live: time::Duration::max_value(),
        }
    }
    /// Constructor for time based MessageFilter
    pub fn with_expiry_duration(time_to_live: time::Duration) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: usize::MAX,
            time_to_live: time_to_live,
        }
    }
    /// Constructor for dual feature capacity or time based MessageFilter
    pub fn with_expiry_duration_and_capacity(time_to_live: time::Duration, capacity: usize) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: capacity,
            time_to_live: time_to_live,
        }
    }
    /// Add a value to MessageFilter
    pub fn add(&mut self, value: V) {
        self.remove_expired();

        if self.set.insert(value.clone()) {
            self.list.push_back((value, time::SteadyTime::now()));
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
                Some(item) => if self.time_to_live != time::Duration::max_value() &&
                    item.1 + self.time_to_live < time::SteadyTime::now() {
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
#![allow(deprecated)]
    extern crate time;
    extern crate rand;
    use std::thread;
    use super::MessageFilter;

    fn generate_random_vec<T>(len: usize) -> Vec<T> where T: rand::Rand {
        let mut vec = Vec::<T>::with_capacity(len);
        for _ in 0..len {
            vec.push(rand::random::<T>());
        }
        vec
    }

    #[test]
    fn size_only() {
        let size = 10usize;
        let mut msg_filter = MessageFilter::<usize>::with_capacity(size);

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
        let time_to_live = time::Duration::milliseconds(100);
        let mut msg_filter = MessageFilter::<usize>::with_expiry_duration(time_to_live);

        for i in 0..10 {
            assert_eq!(msg_filter.len(), i);
            msg_filter.add(i);
            assert_eq!(msg_filter.len(), i + 1);
        }

        thread::sleep_ms(100);
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
        let time_to_live = time::Duration::milliseconds(100);
        let mut msg_filter = MessageFilter::<usize>::with_expiry_duration_and_capacity(time_to_live, size);

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

        thread::sleep_ms(100);
        msg_filter.add(1);

        assert_eq!(msg_filter.len(), 1);
    }

    #[test]
    fn time_size_struct_value() {
        let size = 100usize;
        let time_to_live = time::Duration::milliseconds(100);

        #[derive(PartialEq, PartialOrd, Ord, Clone, Eq, Hash)]
        struct Temp {
            id: Vec<u8>,
        }

        let mut msg_filter = MessageFilter::<Temp>::with_expiry_duration_and_capacity(time_to_live, size);

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

        thread::sleep_ms(100);
        msg_filter.add(Temp { id: generate_random_vec::<u8>(64), });

        assert_eq!(msg_filter.len(), 1);
    }
}
