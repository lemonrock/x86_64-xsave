// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(extern_types)]


//! #x86_64-xsave
//! 
//! This is a rust library modelling the structures used in `XSAVE` and related x86 CPU operations.


use self::state_component_bitmaps::*;
use self::state_components::*;
use self::fxsave::*;
use self::fxsave::domain::*;
use self::fxsave::domain::floating_point_unit_data_pointer_offset::*;
use self::fxsave::domain::floating_point_unit_instruction_pointer_offset::*;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__cpuid;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__cpuid_count;
#[cfg(all(target_arch = "x86_64", target_feature = "fxsr"))] use ::std::arch::x86_64::_fxrstor64;
#[cfg(all(target_arch = "x86_64", target_feature = "fxsr"))] use ::std::arch::x86_64::_fxsave64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xrstor64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xsave64;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::CpuidResult;
use ::std::alloc::Alloc;
use ::std::alloc::Layout;
use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use ::std::slice::from_raw_parts;


include!("ExtendedStateInformation.rs");
include!("MxcsrRegisterValue.rs");
include!("RoundingControl.rs");
include!("SseXmmRegisterValue.rs");
include!("XSaveArea.rs");
include!("XSaveAreaLayout.rs");
include!("XSaveExtendedRegion.rs");
include!("XSaveHeader.rs");


/// State component bitmaps.
pub mod state_component_bitmaps;


/// State component variations.
pub mod state_components;


/// Legacy x87 and `SSE` state saving using `FXSAVE`.
pub mod fxsave;
