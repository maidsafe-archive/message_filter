// Copyright 2014 MaidSafe.net limited
// 
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
// 
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://www.maidsafe.net/licenses
// 
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
// 
// See the Licences for the specific language governing permissions and limitations relating to
// use of the MaidSafe Software.

#![crate_name = "message_filter"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/message_filter/")]
#![feature(std_misc)]

//! #message_filter limited via size or time  
//! 
//! This container allows time or size to be the limiting factor for any key types.
//!
//!#Use
//!
//!##To use as size based MessageFilter 
//!
//!`let mut message_filter = MessageFilter::<usize>::with_capacity(size);`
//!
//!##Or as time based MessageFilter
//! 
//! `let time_to_live = chrono::duration::Duration::milliseconds(100);`
//!
//! `let mut message_filter = MessageFilter::<usize>::with_expiry_duration(time_to_live);`
//! 
//!##Or as time or size limited cache
//!
//! ` let size = 10usize;
//!     let time_to_live = chrono::duration::Duration::milliseconds(100);
//!     let mut message_filter = MessageFilter::<usize>::with_expiry_duration_and_capacity(time_to_live, size);`


extern crate chrono;

use std::usize;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// Allows message filter container which may be limited by size or time.
/// Get(value) is not required as only value is stored
pub struct MessageFilter<V> where V: PartialOrd + Ord + Clone + Hash {
    set: HashSet<V>,
    list: VecDeque<(V, chrono::DateTime<chrono::Local>)>,
    capacity: usize,
    time_to_live: chrono::duration::Duration,
}
/// constructor for size (capacity) based MessageFilter
impl<V> MessageFilter<V> where V: PartialOrd + Ord + Clone + Hash {
    pub fn with_capacity(capacity: usize) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: capacity,
            time_to_live: chrono::duration::MAX,
        }
    }
/// constructor for time based HashMap
    pub fn with_expiry_duration(time_to_live: chrono::duration::Duration) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: usize::MAX,
            time_to_live: time_to_live,
        }
    }
/// constructor for dual feature capacity or time based MessageFilter
    pub fn with_expiry_duration_and_capacity(time_to_live: chrono::duration::Duration, capacity: usize) -> MessageFilter<V> {
        MessageFilter {
            set: HashSet::new(),
            list: VecDeque::new(),
            capacity: capacity,
            time_to_live: time_to_live,
        }
    }
/// Add a value to MessageFilter
    pub fn add(&mut self, value: V) {
        if self.set.insert(value.clone()) {
            self.list.push_back((value, chrono::Local::now()));
        }
        let trimmed = std::cmp::max(0, self.set.len() - self.capacity);
        for _ in 0..trimmed {
            self.set.remove(&self.list.pop_front().unwrap().0);
        }
        let mut expiring = true;
        while expiring {
            if self.list.front().unwrap().1 + self.time_to_live < chrono::Local::now() {
                self.set.remove(&self.list.pop_front().unwrap().0);
            } else {
                expiring = false;
            }
        }
        
    }
/// Check for existance of a key
    pub fn check(&self, value: &V) -> bool {
        self.set.contains(value)
    }
/// Current size of cache
    pub fn len(&self) -> usize {
        self.set.len()
    }

}
