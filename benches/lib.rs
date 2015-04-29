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

#![feature(test)]
#![allow(non_snake_case, deprecated)]
extern crate time;
extern crate test;
extern crate rand;
extern crate message_filter;

use std::thread;
use test::Bencher;
use message_filter::MessageFilter;

fn generate_random_vec<T>(len: usize) -> Vec<T> where T: rand::Rand {
    let mut vec = Vec::<T>::with_capacity(len);
    for _ in 0..len {
        vec.push(rand::random::<T>());
    }
    vec
}

#[bench]
fn bench_add_1000_1KB_msgs_to_100_capacity (b: &mut Bencher) {
  let mut my_cache = MessageFilter::<Vec<u8>>::with_capacity(100);
  let mut contents = Vec::<Vec<u8>>::new();
  let bytes_len = 1024;
  for _ in 0..1000 {
    contents.push(generate_random_vec::<u8>(bytes_len));
  }

  b.iter(|| {
    for i in 0..1000 {
  	  my_cache.add(contents[i].clone());
    }
  });
  b.bytes = 1000 * bytes_len as u64;
  assert_eq!(my_cache.len(), 100);
}

#[bench]
fn bench_add_10000_1KB_msgs_to_1000_capacity (b: &mut Bencher) {
  let mut my_cache = MessageFilter::<Vec<u8>>::with_capacity(1000);
  let mut contents = Vec::<Vec<u8>>::new();
  let bytes_len = 1024;
  for _ in 0..10000 {
    contents.push(generate_random_vec::<u8>(bytes_len));
  }

  b.iter(|| {
    for i in 0..10000 {
      my_cache.add(contents[i].clone());
    }
  });
  b.bytes = 10000 * bytes_len as u64;
  assert_eq!(my_cache.len(), 1000);
}


#[bench]
fn bench_add_1000_1KB_msgs_timeout (b: &mut Bencher) {
  let time_to_live = time::Duration::milliseconds(100);
  let mut my_cache = MessageFilter::<Vec<u8>>::with_expiry_duration(time_to_live);

  let bytes_len = 1024;
  for _ in 0..1000 {
    my_cache.add(generate_random_vec::<u8>(bytes_len));
  }
  let content = generate_random_vec::<u8>(bytes_len);
  thread::sleep_ms(100);

  b.iter(|| {
      my_cache.add(content.clone());
  });
  b.bytes = bytes_len as u64;
  assert_eq!(my_cache.len(), 1);
}

// the following test can not achieve a convengence on performance
// #[bench]
// fn bench_add_1000_1MB_msgs_timeout (b: &mut Bencher) {
//   let time_to_live = time::Duration::milliseconds(100);
//   let mut my_cache = MessageFilter::<Vec<u8>>::with_expiry_duration(time_to_live);

//   let mut contents = Vec::<Vec<u8>>::new();
//   let bytes_len = 1024 * 1024;
//   for _ in 0..1000 {
//     contents.push(generate_random_vec::<u8>(bytes_len));
//   }

//   b.iter(|| {
//     for i in 0..1000 {
//       my_cache.add(contents[i].clone());
//     }
//   });
//   b.bytes = 1000 * bytes_len as u64;
// }
