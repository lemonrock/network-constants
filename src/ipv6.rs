// This file is part of network-constants. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network-constants/master/COPYRIGHT. No part of network-constants, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of network-constants. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network-constants/master/COPYRIGHT.


use std::net::Ipv6Addr;


// It would be nice to avoid the cost of constructing this, but it's not allowed in a static...
#[inline(always)]
pub fn localhost() -> Ipv6Addr
{
	Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
}

#[inline(always)]
pub fn unspecified() -> Ipv6Addr
{
	Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
}
