/*  Copyright 2014 MaidSafe.net limited

    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").

    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses

    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.

    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe Software.                                                                 */

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

/// Allows message filter container which may be limited by size or time.
/// The times of hit of the message during the life_time is also recorded
pub struct MessageFilter<K> where K: PartialOrd + Clone {
    vector: Vec<(K, chrono::DateTime<chrono::Local>, usize)>,
    capacity: usize,
    time_to_live: chrono::duration::Duration,
}
/// constructor for size (capacity) based MessageFilter
impl<K> MessageFilter<K> where K: PartialOrd + Ord + Clone {
    pub fn with_capacity(capacity: usize) -> MessageFilter<K> {
        MessageFilter {
            vector: Vec::new(),
            capacity: capacity,
            time_to_live: chrono::duration::MAX,
        }
    }
/// constructor for time based HashMap
    pub fn with_expiry_duration(time_to_live: chrono::duration::Duration) -> MessageFilter<K> {
        MessageFilter {
            vector: Vec::new(),
            capacity: usize::MAX,
            time_to_live: time_to_live,
        }
    }
/// constructor for dual feature capacity or time based MessageFilter
    pub fn with_expiry_duration_and_capacity(time_to_live: chrono::duration::Duration, capacity: usize) -> MessageFilter<K> {
        MessageFilter {
            vector: Vec::new(),
            capacity: capacity,
            time_to_live: time_to_live,
        }
    }
/// Add a key to MessageFilter
    pub fn add(&mut self, key: K)-> usize {
        // .retain() can also be used however this will slow the performance
        let mut expiring = false;
        let mut found = false;
        let mut index = 0usize;
        let mut expiring_index = 0usize;
        let mut num_of_hits = 1usize;
        for iter in self.vector.iter_mut() {
            if (*iter).1 + self.time_to_live < chrono::Local::now() {
                expiring_index = index;
                expiring = true;
            }
            if (*iter).0 == key {
                (*iter).2 += 1;
                found = true;
                num_of_hits = (*iter).2;
                break;
            }
            index += 1;
        }
        if !found {
            self.vector.push((key, chrono::Local::now(), 1));
        }
        if expiring {
            for _ in 0..expiring_index {
                self.vector.remove(0);
            }
        }
        let trimmed = std::cmp::max(0, self.vector.len() - self.capacity);
        for _ in 0..trimmed {
            self.vector.remove(0);
        }
        num_of_hits
    }
/// Retrieve a value from cache
    pub fn get(&mut self, key: K) -> Option<(K, usize)> {
        for iter in self.vector.iter() {
            if iter.0 == key {
                return Some((key, (*iter).2.clone()));
            }
        }
        None
    }
/// Check for existance of a key
    pub fn check(&self, key: &K) -> bool {
        for iter in self.vector.iter() {
            if iter.0 == *key {
                return true;
            }
        }
        false
    }
/// Current size of cache
    pub fn len(&self) -> usize {
        self.vector.len()
    }

}

