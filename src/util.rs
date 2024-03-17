//! Utility functions for encoding arbitrary values in a `usize` so that they
//! can be passed in and out of coroutines through a register in inline
//! assembly.
//!
//! The basic idea is that we try to place the value directly in the `usize` if
//! it fits, and otherwise pass a pointer to the value instead. This pointer can
//! safely be dereferenced while the other context is suspended.

use core::mem::{self, ManuallyDrop};
use core::ptr;
use psm::psm_stack_information;

/// Internal type for a value that has been encoded in a `usize`.
pub type EncodedValue = usize;

/// Encodes the given value in a `usize` either directly or as a pointer to the
/// argument. This function logically takes ownership of the value, so it should
/// not be dropped afterwards.
pub unsafe fn encode_val<T>(val: &mut ManuallyDrop<T>) -> EncodedValue {
    if mem::size_of::<T>() <= mem::size_of::<EncodedValue>() {
        let mut out = 0;
        ptr::write_unaligned(
            &mut out as *mut EncodedValue as *mut T,
            ManuallyDrop::take(val),
        );
        out
    } else {
        val as *const ManuallyDrop<T> as EncodedValue
    }
}

// Decodes a value produced by `encode_usize` either by converting it directly
// or by treating the `usize` as a pointer and dereferencing it.
pub unsafe fn decode_val<T>(val: EncodedValue) -> T {
    if mem::size_of::<T>() <= mem::size_of::<EncodedValue>() {
        ptr::read_unaligned(&val as *const EncodedValue as *const T)
    } else {
        ptr::read(val as *const T)
    }
}

psm_stack_information! (
    yes {
        pub fn current_stack_ptr() -> usize {
            psm::stack_pointer() as usize
        }
    }
    no {
        #[inline(always)]
        pub fn current_stack_ptr() -> usize {
            unsafe {
                let mut x = std::mem::MaybeUninit::<u8>::uninit();
                // Unlikely to be ever exercised. As a fallback we execute a volatile read to a
                // local (to hopefully defeat the optimisations that would make this local a static
                // global) and take its address. This way we get a very approximate address of the
                // current frame.
                x.as_mut_ptr().write_volatile(42);
                x.as_ptr() as usize
            }
        }
    }
);
