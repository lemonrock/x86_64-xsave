// This file is part of x86_64-xsave. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT. No part of x86_64-xsave, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of x86_64-xsave. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/x86_64-xsave/master/COPYRIGHT.


/// The `SSE` state exists in two, non-contiguous areas ('parts').
///
/// This is part 2.
///
/// It contains the values in the register `XMM0` to `XMM15` inclusive.
#[derive(Default, Debug, Clone)]
#[repr(C, align(16))]
pub struct SseStatePart2
{
	/// Stored value of SSE XMM register `XMM0`.
	///
	/// Part of the `SSE` state.
	pub xmm0: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM1`.
	///
	/// Part of the `SSE` state.
	pub xmm1: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM2`.
	///
	/// Part of the `SSE` state.
	pub xmm2: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM3`.
	///
	/// Part of the `SSE` state.
	pub xmm3: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM4`.
	///
	/// Part of the `SSE` state.
	pub xmm4: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM5`.
	///
	/// Part of the `SSE` state.
	pub xmm5: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM6`.
	///
	/// Part of the `SSE` state.
	pub xmm6: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM7`.
	///
	/// Part of the `SSE` state.
	pub xmm7: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM8`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm8: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM9`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm9: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM10`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm10: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM11`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm11: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM1`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm12: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM13`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm13: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM14`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm14: SseXmmRegisterValue,

	/// Stored value of SSE XMM register `XMM15`.
	///
	/// Part of the `SSE` state.
	///
	/// These fields are used only in 64-bit mode; in other modes they are neither restored (read) or written to.
	pub xmm15: SseXmmRegisterValue,
}
