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

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![feature(test)]

extern crate time;
extern crate test;
extern crate rand;
extern crate message_filter;

fn generate_random_vec<T>(len: usize) -> Vec<T>
    where T: rand::Rand {
    let mut vec = Vec::<T>::with_capacity(len);
    for _ in 0..len {
        vec.push(rand::random::<T>());
    }
    vec
}

#[bench]
fn bench_add_1000_1kb_messages_to_100_capacity(b: &mut ::test::Bencher) {
    let mut my_cache = ::message_filter::MessageFilter::<Vec<u8>>::with_capacity(100);
    let mut contents = Vec::<Vec<u8>>::new();
    let bytes_len = 1024;
    for _ in 0..1000 {
        contents.push(generate_random_vec::<u8>(bytes_len));
    }

    b.iter(|| {
        for i in 0..1000 {
            // Each value is unique so return from insert is None.
            let _ = my_cache.insert(contents[i].clone());
        }
    });
    b.bytes = 1000 * bytes_len as u64;
    assert_eq!(my_cache.len(), 100);
}

#[bench]
fn bench_add_10000_1kb_messages_to_1000_capacity(b: &mut ::test::Bencher) {
    let mut my_cache = ::message_filter::MessageFilter::<Vec<u8>>::with_capacity(1000);
    let mut contents = Vec::<Vec<u8>>::new();
    let bytes_len = 1024;
    for _ in 0..10000 {
        contents.push(generate_random_vec::<u8>(bytes_len));
    }

    b.iter(|| {
        for i in 0..10000 {
            // Each value is unique so return from insert is None.
            let _ = my_cache.insert(contents[i].clone());
        }
    });
    b.bytes = 10000 * bytes_len as u64;
    assert_eq!(my_cache.len(), 1000);
}

#[bench]
fn bench_add_1000_1kb_messages_timeout(b: &mut ::test::Bencher) {
    let time_to_live = time::Duration::milliseconds(100);
    let mut my_cache =
        ::message_filter::MessageFilter::<Vec<u8>>::with_expiry_duration(time_to_live);

    let bytes_len = 1024;
    for _ in 0..1000 {
        // Each value is probably unique so return from insert will probably be None.
        let _ = my_cache.insert(generate_random_vec::<u8>(bytes_len));
    }
    let content = generate_random_vec::<u8>(bytes_len);
    ::std::thread::sleep(::std::time::Duration::from_millis(100));

    b.iter(|| {
        // Each value is probably unique so return from insert will probably be None.
        let _ = my_cache.insert(content.clone());
    });
    b.bytes = bytes_len as u64;
    assert_eq!(my_cache.len(), 1);
}

// the following test can not achieve a convergence on performance
// #[bench]
// fn bench_add_1000_1mb_messages_timeout (b: &mut ::test::Bencher) {
//     let time_to_live = time::Duration::milliseconds(100);
//     let mut my_cache =
//         ::message_filter::MessageFilter::<Vec<u8>>::with_expiry_duration(time_to_live);

//     let mut contents = Vec::<Vec<u8>>::new();
//     let bytes_len = 1024 * 1024;
//     for _ in 0..1000 {
//         contents.push(generate_random_vec::<u8>(bytes_len));
//     }

//     b.iter(|| {
//         for i in 0..1000 {
//             let _ = my_cache.insert(contents[i].clone());
//         }
//     });
//     b.bytes = 1000 * bytes_len as u64;
// }
