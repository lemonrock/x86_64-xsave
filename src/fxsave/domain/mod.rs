// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


use super::*;


include!("AbridgedFpuTagWord.rs");
include!("FloatingPointOrMMRegisterValue.rs");
include!("FloatingPointUnitOpcode.rs");
include!("PrecisionControl.rs");
include!("X87FloatingPointUnitControlWord.rs");
include!("X87FloatingPointUnitStatusWord.rs");


/// Floating Point Unit (FPU) data point offset.
pub mod floating_point_unit_data_pointer_offset;


/// Floating Point Unit (FPU) instruction point offset.
pub mod floating_point_unit_instruction_pointer_offset;
