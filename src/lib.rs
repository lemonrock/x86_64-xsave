// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(extern_types)]
#![feature(stdsimd)]


//! #x86_64-xsave
//! 
//! This is a rust library modelling the structures used in `XSAVE` and related x86 CPU operations.


#[macro_use] extern crate arrayref;


use self::state_component_bitmaps::*;
use self::state_components::*;
use self::fxsave::*;
use self::fxsave::domain::*;
use self::fxsave::domain::floating_point_unit_data_pointer_offset::*;
use self::fxsave::domain::floating_point_unit_instruction_pointer_offset::*;

#[cfg(target_arch = "x86")] use ::std::arch::x86::__cpuid;
#[cfg(target_arch = "x86")] use ::std::arch::x86::__cpuid_count;
#[cfg(all(target_arch = "x86", target_feature = "sse"))] use ::std::arch::x86::_mm_getcsr;
#[cfg(all(target_arch = "x86", target_feature = "sse"))] use ::std::arch::x86::_mm_setcsr;
#[cfg(all(target_arch = "x86", target_feature = "fxsr"))] use ::std::arch::x86::_fxrstor;
#[cfg(all(target_arch = "x86", target_feature = "fxsr"))] use ::std::arch::x86::_fxsave;
#[cfg(all(target_arch = "x86", target_feature = "xsave"))] use ::std::arch::x86::_XCR_XFEATURE_ENABLED_MASK;
#[cfg(all(target_arch = "x86", target_feature = "xsave"))] use ::std::arch::x86::_xgetbv;
#[cfg(all(target_arch = "x86", target_feature = "xsave"))] use ::std::arch::x86::_xrstor;
#[cfg(all(target_arch = "x86", target_feature = "xsave"))] use ::std::arch::x86::_xsave;
#[cfg(all(target_arch = "x86", target_feature = "xsave", target_feature = "xsavec"))] use ::std::arch::x86::_xsavec;
#[cfg(all(target_arch = "x86", target_feature = "xsave", target_feature = "xsaveopt"))] use ::std::arch::x86::_xsaveopt;
#[cfg(all(target_arch = "x86", target_feature = "xsave"))] use ::std::arch::x86::_xsetbv;
#[cfg(target_arch = "x86")] use ::std::arch::x86::CpuidResult;
#[cfg(target_arch = "x86")] use ::std::arch::x86::has_cpuid;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__cpuid;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__cpuid_count;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))] use ::std::arch::x86_64::_mm_getcsr;
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))] use ::std::arch::x86_64::_mm_setcsr;
#[cfg(all(target_arch = "x86_64", target_feature = "fxsr"))] use ::std::arch::x86_64::_fxrstor64;
#[cfg(all(target_arch = "x86_64", target_feature = "fxsr"))] use ::std::arch::x86_64::_fxsave64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_XCR_XFEATURE_ENABLED_MASK;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xgetbv;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xrstor64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xsave64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave", target_feature = "xsavec"))] use ::std::arch::x86_64::_xsavec64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave", target_feature = "xsaveopt"))] use ::std::arch::x86_64::_xsaveopt64;
#[cfg(all(target_arch = "x86_64", target_feature = "xsave"))] use ::std::arch::x86_64::_xsetbv;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::CpuidResult;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::has_cpuid;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "xsave"))] use ::std::alloc::AllocErr;
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
use ::std::ops::DerefMut;
use ::std::ptr::NonNull;
use ::std::slice::from_raw_parts;

// target_env sgx


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

// TODO: Load / store x87 control (?and status)? word
//
// FLDCW: Load x87 FPU control word from memory.
// FSTSW/FNSTSW: Store x87 FPU status word to memory or another register.
