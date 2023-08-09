use std::ffi::c_void;
use windows_sys::Win32::Foundation::NTSTATUS;

use crate::pattern::Pattern;

const BCRYPT_PATTERN: Pattern = Pattern {
    name: "BCrypt",
    start: 0x0000000140100000,
    end: 0x0000000200000000,
    mask: "xx????xxxxxxx",
    op: &[
        0xFF, 0x15, 0x00, 0x00, 0x00, 0x00, // call QWORD PTR [rip + {ADDRESS TO JUMP TO}]
        0x48, 0x8B, 0x4C, 0x24, 0x40, // mov rcx, QWORD PTR [rsp + 0x40]
        0x85, 0xC0, // test eax, eax
    ],
};

const VERIFY_CERTIFICATE_PATTERN: Pattern = Pattern {
    name: "VerifyCertificate",
    start: 0x0000000140100000,
    end: 0x0000000200000000,
    mask: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
    ????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxxxxxxxxxxxxxx?????\
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxxxx\
    xxxxxxxxxxxxxxxx????xxxx????xxxxxxxxxxx?????xxxxxxxx????xxxxxx?????xxxxxxxx????xxxx????xxxx\
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx?????x????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxxxxxx\
    xxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxx\
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxx????xxxxxxxxxxxxxxxxxx",
    op: &[
        0x48, 0x89, 0x5C, 0x24, 0x08, 0x44, 0x88, 0x44, 0x24, 0x18, 0x55, 0x56, 0x57, 0x48, 0x83,
        0xEC, 0x30, 0x33, 0xED, 0x41, 0x0F, 0xB6, 0xC0, 0x48, 0x8B, 0xFA, 0x48, 0x8B, 0xF1, 0x89,
        0x6C, 0x24, 0x68, 0x41, 0x80, 0xF8, 0x01, 0x75, 0x5C, 0x48, 0x8D, 0x8A, 0xC0, 0x01, 0x00,
        0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x75, 0x47, 0x8B, 0x87, 0x10, 0x08, 0x00,
        0x00, 0x44, 0x8B, 0x8F, 0x0C, 0x06, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x14, 0x08, 0x00, 0x00,
        0x89, 0x44, 0x24, 0x28, 0x48, 0x89, 0x4C, 0x24, 0x20, 0x4C, 0x8D, 0x87, 0x10, 0x06, 0x00,
        0x00, 0x48, 0x8B, 0xCE, 0x48, 0x8B, 0xD7, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F,
        0x84, 0x00, 0x00, 0x00, 0x00, 0x8D, 0x45, 0xCE, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83,
        0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x0F, 0xB6, 0x44, 0x24, 0x60, 0x48, 0x8D, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00, 0x84, 0xC0, 0x75, 0x0D, 0x83, 0xBF,
        0x24, 0x08, 0x00, 0x00, 0x00, 0x75, 0x04, 0x33, 0xC0, 0xEB, 0x05, 0xB8, 0x01, 0x00, 0x00,
        0x00, 0x44, 0x0F, 0xB6, 0xC0, 0x48, 0x8B, 0xD7, 0x48, 0x8B, 0xCB, 0xE8, 0x00, 0x00, 0x00,
        0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00, 0x44, 0x8B, 0x8B, 0xC4, 0x01, 0x00,
        0x00, 0x44, 0x3B, 0x8F, 0x04, 0x04, 0x00, 0x00, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00, 0x8B,
        0x83, 0xD0, 0x01, 0x00, 0x00, 0x4C, 0x8B, 0x83, 0xC8, 0x01, 0x00, 0x00, 0x48, 0x8D, 0x8B,
        0xD4, 0x01, 0x00, 0x00, 0x89, 0x44, 0x24, 0x28, 0x48, 0x89, 0x4C, 0x24, 0x20, 0x48, 0x8B,
        0xD7, 0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00,
        0x00, 0x00, 0xF6, 0x83, 0xC0, 0x01, 0x00, 0x00, 0x01, 0x74, 0x52, 0x48, 0x8D, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x40, 0x03, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00,
        0x85, 0xC0, 0x74, 0x1B, 0x48, 0x8D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x40,
        0x03, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x0F, 0x85, 0x00, 0x00, 0x00,
        0x00, 0x8B, 0x6C, 0x24, 0x68, 0x48, 0x85, 0xF6, 0x74, 0x17, 0x48, 0x8B, 0x86, 0x70, 0x01,
        0x00, 0x00, 0x80, 0xB8, 0x98, 0x00, 0x00, 0x00, 0x00, 0x74, 0x07, 0xC6, 0x80, 0x98, 0x00,
        0x00, 0x00, 0x00, 0x48, 0x8D, 0x8F, 0x40, 0x03, 0x00, 0x00, 0x48, 0x8D, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x85, 0xC0, 0x75, 0x09, 0xF6, 0x83, 0xC0, 0x01,
        0x00, 0x00, 0x02, 0x74, 0x58, 0x48, 0x8B, 0x93, 0xF0, 0x01, 0x00, 0x00, 0x48, 0x85, 0xD2,
        0x74, 0x75, 0x41, 0xB0, 0x01, 0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x8B, 0xE8,
        0x89, 0x44, 0x24, 0x68, 0x85, 0xC0, 0x74, 0x49, 0x48, 0x8B, 0x93, 0xF0, 0x01, 0x00, 0x00,
        0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x9B, 0xF8, 0x01, 0x00, 0x00,
        0x48, 0x85, 0xDB, 0x74, 0x54, 0x0F, 0xB6, 0x44, 0x24, 0x60, 0xE9, 0x00, 0x00, 0x00, 0x00,
        0xB8, 0x9C, 0xFF, 0xFF, 0xFF, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83, 0xC4, 0x30, 0x5F,
        0x5E, 0x5D, 0xC3, 0xB8, 0x9A, 0xFF, 0xFF, 0xFF, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83,
        0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x48, 0x8B, 0x8B, 0xF0, 0x01, 0x00, 0x00, 0xE8, 0x00,
        0x00, 0x00, 0x00, 0x48, 0xC7, 0x83, 0xF0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8B,
        0xC5, 0x48, 0x8B, 0x5C, 0x24, 0x50, 0x48, 0x83, 0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3, 0x48,
        0x8B, 0xD7, 0x48, 0x8B, 0xCE, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x5C, 0x24, 0x50,
        0xB8, 0xCD, 0xFF, 0xFF, 0xFF, 0x48, 0x83, 0xC4, 0x30, 0x5F, 0x5E, 0x5D, 0xC3,
    ],
};

pub unsafe fn hook() {
    verify_certificate();
    bcrypt_verify_signature();
}

/// Finds and hooks the VerifyCertificate function replacing it with
/// something that will always return zero aka the success value
unsafe fn verify_certificate() {
    Pattern::apply(&VERIFY_CERTIFICATE_PATTERN, 16, |addr| {
        // Replacement opcodes for just returning always zero
        let new_ops: [u8; 9] = [
            0xb8, 0x0, 0x0, 0x0, 0x0,  // mov eax, 0
            0xc3, // ret
            0x90, // nop
            0x90, // nop
            0x90, // nop
        ];

        // Iterate the opcodes and write them to the ptr
        let mut op_ptr: *mut u8 = addr;
        for op in new_ops {
            *op_ptr = op;
            op_ptr = op_ptr.add(1);
        }
    });
}

/// Finds and hooks the VerifyCertificate function replacing it with
/// something that will always return zero aka the success value
unsafe fn bcrypt_verify_signature() {
    Pattern::apply_with_transform(
        &BCRYPT_PATTERN,
        8,
        // Transform the address to the desired locaiton
        |addr: *const u8| {
            // Skip "call QWORD PTR"
            let call_addr = addr.add(2) as *const i32;
            // Obtain the relative call address
            let relative_addr = *call_addr as usize;
            // Move to the call address
            addr.add(6 + relative_addr)
        },
        |addr| {
            // Replace the address with the dummy function address
            let ptr = addr as *mut usize;
            *ptr = fake_bcrypt_verify_signature as usize;
        },
    );
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
fn fake_bcrypt_verify_signature(
    hKey: isize,
    pPaddingInfo: *const c_void,
    pbHash: *const u8,
    cbHash: u64,
    pbSignature: *const u8,
    cbSignature: u64,
    dwFlags: u64,
) -> NTSTATUS {
    0
}
