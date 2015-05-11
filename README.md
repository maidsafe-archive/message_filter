# message_filter 

A size or time based message filter that takes any generic type as a key and will drop keys after a time period, or once size of container is reached (LRU Cache pattern). The filter has Add key method only, a delete function will be added at a later stage. This is a handy container for network based systems to filter previously seen messages.

|Crate|Travis|Appveyor|Coverage|
|:------:|:-------:|:-------:|:-------:|
|[![](http://meritbadge.herokuapp.com/message_filter)](https://crates.io/crates/message_filter)|[![Build Status](https://travis-ci.org/maidsafe/message_filter.svg?branch=master)](https://travis-ci.org/maidsafe/message_filter)|[![Build status](https://ci.appveyor.com/api/projects/status/xj1muadwnd1ysmmc/branch/master?svg=true)](https://ci.appveyor.com/project/dirvine/lru-time-cache-o9t28/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/message_filter/badge.svg)](https://coveralls.io/r/maidsafe/message_filter)|

| [ API Documentation](http://maidsafe.github.io/message_filter/) | [MaidSafe System Documention](http://systemdocs.maidsafe.net/) | [MaidSafe web site](http://www.maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |

#Todo
- [x] Implement add_key  (bool return, true added, false == already exists)
- [x] Test add_key (time and size based tests)
- [x] API version 0.1.0
