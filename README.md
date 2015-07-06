# message_filter

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/message_filter/blob/master/COPYING)

**Primary Maintainer:**     Chandra Prakash (prakash@maidsafe.net)

|Crate|Linux|Windows|OSX|Coverage|Issues|
|:------:|:-------:|:-------:|:-------:|:-------:|:-------:|
|[![](http://meritbadge.herokuapp.com/message_filter)](https://crates.io/crates/message_filter)|[![Build Status](https://travis-ci.org/maidsafe/message_filter.svg?branch=master)](https://travis-ci.org/maidsafe/message_filter)| [![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=message_filter_win64_status_badge)](http://ci.maidsafe.net:8080/job/message_filter_win64_status_badge/)|[![Build Status](http://ci.maidsafe.net:8080/buildStatus/icon?job=message_filter_osx_status_badge)](http://ci.maidsafe.net:8080/job/message_filter_osx_status_badge/) |[![Coverage Status](https://coveralls.io/repos/maidsafe/message_filter/badge.svg)](https://coveralls.io/r/maidsafe/message_filter)|[![Stories in Ready](https://badge.waffle.io/maidsafe/message_filter.png?label=ready&title=Ready)](https://waffle.io/maidsafe/message_filter)|

| [API Documentation - master branch](http://maidsafe.net/message_filter/master) | [SAFE Network System Documention](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

#Overview
A size or time based message filter that takes any generic type as a key and will drop keys after a time period, or once size of container is reached (LRU Cache pattern). The filter has Add key method only, a delete function will be added at a later stage. This is a handy container for network based systems to filter previously seen messages.
