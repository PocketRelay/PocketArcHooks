#![allow(clippy::missing_safety_doc)]

use std::ffi::c_void;
use windows_sys::Win32::{
    Foundation::FALSE,
    System::{
        LibraryLoader::FreeLibrary,
        Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

pub mod proxy;

/// The mask to match the opcodes with
const VERIFY_CERTIFICATE_STR_MASK: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxxxxxxxxxxxxxx?????\
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxxxx\
xxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxx?????xxxxxxxx????xxxxxx?????xxxxxxxx????xxxx????xxxx\
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx?????x????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxxxxxx\
xxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx\
xxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxx";

/// The OP codes matched by the mask above
const VERIFY_CERTIFICATE_OP_MASK: &[u8] = &[
    0x48, 0x89, 0x5C, 0x24, 0x08, 0x44, 0x88, 0x44, 0x24, 0x18, 0x55, 0x56, 0x57, 0x48, 0x83, 0xEC,
    0x30, 0x33, 0xED, 0x41, 0x0F, 0xB6, 0xC0, 0x48, 0x8B, 0xFA, 0x48, 0x8B, 0xF1, 0x89, 0x6C, 0x24,
    0x68, 0x41, 0x80, 0xF8, 0x01, 0x75, 0x5C, 0x48, 0x8D, 0x8A, 0xC0, 0x01, 0x00, 0x00, 0xE8, 0x00,
    0x00, 0x00, 0x00, 0x85, 0xC0, 0x75, 0x47, 0x8B, 0x87, 0x10, 0x08, 0x00, 0x00, 0x44, 0x8B, 0x8F,
    0x0C, 0x06, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x14, 0x08, 0x00, 0x00, 0x89, 0x44, 0x24, 0x28, 0x48,
    0x89, 0x4C, 0x24, 0x20, 0x4C, 0x8D, 0x87, 0x10, 0x06, 0x00, 0x00, 0x48, 0x8B, 0xCE, 0x48, 0x8B,
    0xD7, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x8D, 0x45,
    0xCE, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83, 0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x0F, 0xB6,
    0x44, 0x24, 0x60, 0x48, 0x8D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00,
    0x84, 0xC0, 0x75, 0x0D, 0x83, 0xBF, 0x24, 0x08, 0x00, 0x00, 0x00, 0x75, 0x04, 0x33, 0xC0, 0xEB,
    0x05, 0xB8, 0x01, 0x00, 0x00, 0x00, 0x44, 0x0F, 0xB6, 0xC0, 0x48, 0x8B, 0xD7, 0x48, 0x8B, 0xCB,
    0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00, 0x44, 0x8B, 0x8B,
    0xC4, 0x01, 0x00, 0x00, 0x44, 0x3B, 0x8F, 0x04, 0x04, 0x00, 0x00, 0x0F, 0x85, 0x00, 0x00, 0x00,
    0x00, 0x8B, 0x83, 0xD0, 0x01, 0x00, 0x00, 0x4C, 0x8B, 0x83, 0xC8, 0x01, 0x00, 0x00, 0x48, 0x8D,
    0x8B, 0xD4, 0x01, 0x00, 0x00, 0x89, 0x44, 0x24, 0x28, 0x48, 0x89, 0x4C, 0x24, 0x20, 0x48, 0x8B,
    0xD7, 0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00, 0x00,
    0x00, 0xF6, 0x83, 0xC0, 0x01, 0x00, 0x00, 0x01, 0x74, 0x52, 0x48, 0x8D, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x48, 0x8D, 0x8F, 0x40, 0x03, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x74,
    0x1B, 0x48, 0x8D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x40, 0x03, 0x00, 0x00, 0xE8,
    0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00, 0x8B, 0x6C, 0x24, 0x68,
    0x48, 0x85, 0xF6, 0x74, 0x17, 0x48, 0x8B, 0x86, 0x70, 0x01, 0x00, 0x00, 0x80, 0xB8, 0x98, 0x00,
    0x00, 0x00, 0x00, 0x74, 0x07, 0xC6, 0x80, 0x98, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x40,
    0x03, 0x00, 0x00, 0x48, 0x8D, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85,
    0xC0, 0x75, 0x09, 0xF6, 0x83, 0xC0, 0x01, 0x00, 0x00, 0x02, 0x74, 0x58, 0x48, 0x8B, 0x93, 0xF0,
    0x01, 0x00, 0x00, 0x48, 0x85, 0xD2, 0x74, 0x75, 0x41, 0xB0, 0x01, 0x48, 0x8B, 0xCE, 0xE8, 0x00,
    0x00, 0x00, 0x00, 0x8B, 0xE8, 0x89, 0x44, 0x24, 0x68, 0x85, 0xC0, 0x74, 0x49, 0x48, 0x8B, 0x93,
    0xF0, 0x01, 0x00, 0x00, 0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x9B, 0xF8,
    0x01, 0x00, 0x00, 0x48, 0x85, 0xDB, 0x74, 0x54, 0x0F, 0xB6, 0x44, 0x24, 0x60, 0xE9, 0x00, 0x00,
    0x00, 0x00, 0xB8, 0x9C, 0xFF, 0xFF, 0xFF, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83, 0xC4, 0x30,
    0x5F, 0x5E, 0x5D, 0xC3, 0xB8, 0x9A, 0xFF, 0xFF, 0xFF, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83,
    0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x48, 0x8B, 0x8B, 0xF0, 0x01, 0x00, 0x00, 0xE8, 0x00, 0x00,
    0x00, 0x00, 0x48, 0xC7, 0x83, 0xF0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8B, 0xC5, 0x48,
    0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83, 0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x48, 0x8B, 0xD7, 0x48,
    0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0xB8, 0xCD, 0xFF, 0xFF,
    0xFF, 0x48, 0x83, 0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3,
];

/// Compares the opcodes after the provided address using the provided
/// opcode and pattern
///
/// # Arguments
/// * addr - The address to start matching from
/// * op_mask - The opcodes to match against
/// * str_mask - The str pattern defining how to match the opcodes
unsafe fn compare_mask(addr: *const u8, op_mask: &[u8], str_mask: &str) -> bool {
    str_mask
        .chars()
        .enumerate()
        .zip(op_mask.iter())
        .all(|((offset, mask), op)| mask == '?' || *addr.add(offset) == *op)
}

/// Attempts to find a matching pattern anywhere between the start and
/// end address
///
/// # Arguments
/// * start - The address to start searching from
/// * end - The address to end searching at
/// * op_mask - The opcodes to match against
/// * str_mask - The str pattern defining how to match the opcodes
unsafe fn find_pattern(start: u64, end: u64, op_mask: &[u8], str_mask: &str) -> Option<*const u64> {
    (start..=end)
        .map(|addr| addr as *const u64)
        .find(|addr| compare_mask(addr.cast(), op_mask, str_mask))
}

/// Finds and hooks the VerifyCertificate function replacing it with
/// something that will always return zero aka the success value
unsafe fn hook_verify_certificate() {
    // Last known addr: 0x0000000140CDCB30

    let start_addr: u64 = 0x0000000140100000;
    let end_addr: u64 = 0x0000000160000000;

    // Find the pattern for VerifyCertificate
    let call_addr = find_pattern(
        start_addr,
        end_addr,
        VERIFY_CERTIFICATE_OP_MASK,
        VERIFY_CERTIFICATE_STR_MASK,
    );

    let call_addr = match call_addr {
        Some(value) => value,
        None => {
            println!("Failed to find VerifyCertificate hook position");
            return;
        }
    };

    println!("Found VerifyCertificate @ {:#016x}", call_addr as usize);

    let mut old_protect: PAGE_PROTECTION_FLAGS = 0;
    // Protect the memory region
    if VirtualProtect(
        call_addr as *const c_void,
        16,
        PAGE_READWRITE,
        &mut old_protect,
    ) == FALSE
    {
        println!("Failed to protect memory region while hooking VerifyCertificate");
        return;
    }

    // Replacement opcodes for just returning always zero
    let new_ops: [u8; 9] = [
        0xb8, 0x0, 0x0, 0x0, 0x0,  // mov eax, 0
        0xc3, // ret
        0x90, // nop
        0x90, // nop
        0x90, // nop
    ];

    // Iterate the opcodes and write them to the ptr
    let mut op_ptr: *mut u8 = call_addr as *mut u8;
    for op in new_ops {
        *op_ptr = op;
        op_ptr = op_ptr.add(1);
    }

    // Unprotect the memory region
    VirtualProtect(
        call_addr as *const c_void,
        16,
        old_protect,
        &mut old_protect,
    );
}

/// Handles the DLL being attached to the game
fn attach() {
    unsafe { hook_verify_certificate() };
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllMain(dll_module: usize, call_reason: u32, _: *mut ()) -> bool {
    let handle = proxy::init();

    if call_reason == DLL_PROCESS_ATTACH {
        attach();
    }

    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => {
            if let Some(handle) = handle {
                FreeLibrary(handle);
            }
        }
        _ => {}
    }

    true
}
