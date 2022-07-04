use crate::patch::Patch;

pub const REHD_PATCHES: &[Patch] = &[
    Patch::new(
        "DoorLoop", 
        &[
            0x8B, 0xF1,                     // mov    esi, ecx
            0x8B, 0x46, 0x48,               // mov    eax, [esi+48h]
            0x85, 0xC0                      // test   eax, eax
        ], 
        &[
            0x8B, 0xF1,                     // mov    esi, ecx
            0xE9, 0x9F, 0x00, 0x00, 0x00    // jmp    0xa4 
        ]
    ),

    Patch::new(
        "DoorEvent",
        &[
            0x8B, 0x4C, 0x24, 0x1C,                     // mov    ecx,DWORD PTR [esp+0x1c]
            0xC7, 0x46, 0x7C, 0x00, 0x00, 0x00, 0x00    // mov    DWORD PTR [esi+0x7c],0x0
        ],
        &[
            0x8B, 0x4C, 0x24, 0x1C,                     // mov    ecx,DWORD PTR [esp+0x1c]
            0xE9, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00    // jmp    0x8a
        ]
    ),

    Patch::new(
        "DoorEventReturn",
        &[
            0x81, 0x7E, 0x78, 0x19, 0x01, 0x00, 0x00,   // DWORD PTR [esi+0x78],0x119 
            0x75, 0x07,                                 // jne    0x10
            0xC7, 0x46, 0x78, 0x03, 0x00, 0x00, 0x00,   // DWORD PTR [esi+0x78],0x3
            0x53                                        // push   ebx
        ],
        &[
            0x5F,                           // pop    edi
            0xC7, 0x86, 0x84, 0x00, 0x00, 
            0x00, 0x03, 0x00, 0x00, 0x00,   // mov    DWORD PTR [esi+0x84],0x3
            0x5E,                           // pop    esi
            0x5D,                           // pop    ebp
            0x5B,                           // pop    ebx
            0xC2, 0x10, 0x00                // ret    0x10
        ]
    ),

    Patch::new(
        "LiftFix",
        &[
            0x68, 0xFB, 0x00, 0x00, 0x00,   // push   0xfb
            0xEB, 0x1D,                     // jmp    0x24
        ],
        &[
            0x68, 0xFA, 0x00, 0x00, 0x00,   // push   0xfa
            0xEB, 0x1D,                     // jmp    0x24
        ],
    )
];
