# ***This repository is no longer maintained***
# It has been moved to the maidsafe-archive organisation for reference only
#
#
#
#
# message_filter

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/message_filter/blob/master/COPYING)

**Primary Maintainer:** Fraser Hutchison (fraser.hutchison@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/message_filter)](https://crates.io/crates/message_filter)|[![Build Status](https://travis-ci.org/maidsafe/message_filter.svg?branch=master)](https://travis-ci.org/maidsafe/message_filter)|[![Build status](https://ci.appveyor.com/api/projects/status/433nw77iac2cjo9r/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/message-filter/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/message_filter/badge.svg)](https://coveralls.io/r/maidsafe/message_filter)|[![Stories in Ready](https://badge.waffle.io/maidsafe/message_filter.png?label=ready&title=Ready)](https://waffle.io/maidsafe/message_filter)|

| [API Documentation - master branch](http://maidsafe.net/message_filter/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [SAFE Network Forum](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

## Overview

A size or time based message filter that takes any generic type as a key and will drop keys after a time period, or once a maximum number of messages is reached (LRU Cache pattern).  The filter currently only allows adding messages; a delete function will be provided at a later stage.  This library can be used by network based systems to filter previously seen messages.
