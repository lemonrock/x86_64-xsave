// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


use super::*;


include!("NotSizingQueryableUserStateComponent.rs");
include!("SizingQueryableUserStateComponent.rs");
include!("StateComponent.rs");
include!("StateComponentSizing.rs");
include!("SupervisorStateComponent.rs");
include!("SupportedIn.rs");
include!("UserStateComponent.rs");
