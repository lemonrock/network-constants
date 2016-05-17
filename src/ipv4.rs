// This file is part of network-constants. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network-constants/master/COPYRIGHT. No part of network-constants, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of network-constants. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network-constants/master/COPYRIGHT.


use std::net::Ipv4Addr;


// It would be nice to avoid the cost of constructing this, but it's not allowed in a static...
#[inline(always)]
pub fn localhost() -> Ipv4Addr
{
	Ipv4Addr::new(127, 0, 0, 1)
}

#[inline(always)]
pub fn unspecified() -> Ipv4Addr
{
	Ipv4Addr::new(0, 0, 0, 0)
}

#[inline(always)]
pub fn broadcast() -> Ipv4Addr
{
	Ipv4Addr::new(255, 255, 255, 255)
}
