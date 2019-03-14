// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// Extended state information, as defined in Section 13.2 of the Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 1 (Basic Architecture).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedStateInformation
{
	/// State components that can be enabled in the `XCR0` register.
	///
	/// To access the `XCR0` register use the struct `StateComponentBitmap`.
	xsave_state_components_present: StateComponentBitmap,

	/// Maximum size (in bytes) of an `XSAVE` area if all supported features in the processor are set as feature bits in the `XCR0` register.
	///
	/// To access the `XCR0` register use the struct `StateComponentBitmap`.
	pub xsave_area_size_supported_features: usize,

	/// Maximum size (in bytes) of an `XSAVE` area required by the feature bits enabled in the `XCR0` register.
	///
	/// May be less than `xsave_area_size_supported_features` if compaction is used.
	pub xsave_area_size_enabled_features: usize,

	/// State components that can be enabled in the `IA32_XSS` `MCR` register.
	xsaves_state_components_present: StateComponentBitmap,

	/// Maximum size (in bytes) of an `XSAVE` area saved by the `XSAVES` instruction required by the feature bits enabled in the `XCR0` and `IA32_XSS` `MCR` registers.
	///
	/// May be less than `xsave_area_size_supported_features` if compaction is used.
	///
	/// To access the `XCR0` register use the struct `StateComponentBitmap`.
	pub xsaves_area_size_enabled_features: usize,

	/// Has the `XSAVEOPT` instruction (`xsaveopt` feature).
	pub has_xsaveopt_feature: bool,

	/// Has `XSAVE` compaction extensions (`xsavec` feature):-
	///
	/// * The compacted format of the extended region of `XSAVE` areas;
	/// * Has the `XSAVEC` instruction;
	/// * Execution of the compacted form of `XRSTOR`.
	pub has_xsavec_feature: bool,

	/// The `XGETBV` instruction supports execution with `XGETBV == 1`.
	pub xgetbv_supports_ecx_equals_one: bool,

	/// Has `XSAVE` supervisor extensions (`xsaves` feature):-
	///
	/// * Has the `XSAVES` instruction;
	/// * Has the `XRSTORS` instruction;
	/// * Has the `IA32_XSS` `MSR`.
	///
	/// If this is `true` then `has_xsavec_feature` is also `true`.
	pub has_xsaves_feature: bool,
}

impl ExtendedStateInformation
{
	/// Is an user state component present?
	#[inline(always)]
	pub fn user_state_component_possible(&self, user_state_component: UserStateComponent) -> bool
	{
		self.xsave_state_components_present.is_present(user_state_component.to_state_component())
	}

	/// Is a supervisor state component present?
	#[inline(always)]
	pub fn supervisor_state_component_possible(&self, supervisor_state_component: SupervisorStateComponent) -> bool
	{
		self.xsave_state_components_present.is_present(supervisor_state_component.to_state_component())
	}

	/// Creates a new instance.
	///
	/// Returns `None` if unsupported by the CPU.
	#[inline(always)]
	pub fn new() -> Option<Self>
	{
		match Self::eax_extended_state_information(0)
		{
			None => return None,

			Some(sub_function_0) =>
			{
				let sub_function_1 = Self::eax_extended_state_information(1).unwrap();
				let sub_function_1_eax = sub_function_1.eax;

				Some
				(
					Self
					{
						xsave_state_components_present: StateComponentBitmap((sub_function_0.edx as u64) << 32 | sub_function_0.eax as u64),

						xsave_area_size_supported_features: sub_function_0.ecx as usize,

						xsave_area_size_enabled_features: sub_function_0.ebx as usize,

						xsaves_state_components_present: StateComponentBitmap((sub_function_1.edx as u64) << 32 | sub_function_1.ecx as u64),

						xsaves_area_size_enabled_features: sub_function_1.ebx as usize,

						has_xsaveopt_feature: sub_function_1_eax & 0b0001 != 0,

						has_xsavec_feature: sub_function_1_eax & 0b0010 != 0,

						xgetbv_supports_ecx_equals_one: sub_function_1_eax & 0b0100 != 0,

						has_xsaves_feature: sub_function_1_eax & 0b1000 != 0,
					}
				)
			}
		}
	}

	#[inline(always)]
	fn eax_extended_state_information(ecx_or_sub_leaf: u32) -> Option<CpuidResult>
	{
		const EAX_EXTENDED_STATE_INFO: u32 = 0x0D;
		if !Self::is_extended_functionality_type_supported(EAX_EXTENDED_STATE_INFO)
		{
			return None
		}

		if has_cpuid()
		{
			Some(unsafe { __cpuid_count(EAX_EXTENDED_STATE_INFO, ecx_or_sub_leaf) })
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn is_extended_functionality_type_supported(eax_or_leaf: u32) -> bool
	{
		const EAX_EXTENDED_FUNCTION_INFO: u32 = 0x80000000;

		if has_cpuid()
		{
			let result = unsafe { __cpuid(EAX_EXTENDED_FUNCTION_INFO) };

			let eax = result.eax;

			if eax == 0
			{
				return false
			}

			let maximum_eax_value = eax - EAX_EXTENDED_FUNCTION_INFO;
			eax_or_leaf <= maximum_eax_value
		}
		else
		{
			false
		}
	}
}
