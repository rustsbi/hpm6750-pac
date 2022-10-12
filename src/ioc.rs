#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - Controller for input/output pad A"]
    pub pad_a: [PAD_A; 32],
    #[doc = "0x100..0x200 - Controller for input/output pad B"]
    pub pad_b: [PAD_B; 32],
    #[doc = "0x200..0x300 - Controller for input/output pad C"]
    pub pad_c: [PAD_C; 32],
    #[doc = "0x300..0x400 - Controller for input/output pad D"]
    pub pad_d: [PAD_D; 32],
    #[doc = "0x400..0x500 - Controller for input/output pad E"]
    pub pad_e: [PAD_E; 32],
    #[doc = "0x500..0x558 - Controller for input/output pad F"]
    pub pad_f: [PAD_F; 11],
    _reserved6: [u8; 0x07a8],
    #[doc = "0xd00..0xd60 - Controller for input/output pad X"]
    pub pad_x: [PAD_X; 12],
    _reserved7: [u8; 0xa0],
    #[doc = "0xe00..0xe60 - Controller for input/output pad Y"]
    pub pad_y: [PAD_Y; 12],
    _reserved8: [u8; 0xa0],
    #[doc = "0xf00..0xf60 - Controller for input/output pad Z"]
    pub pad_z: [PAD_Z; 12],
}
#[doc = "Controller for input/output pad"]
pub use self::pad::PAD;
#[doc = r"Cluster"]
#[doc = "Controller for input/output pad"]
pub mod pad;
pub use self::pad as pad_a;
pub use self::pad as pad_b;
pub use self::pad as pad_c;
pub use self::pad as pad_d;
pub use self::pad as pad_e;
pub use self::pad as pad_f;
pub use self::pad as pad_x;
pub use self::pad as pad_y;
pub use self::pad as pad_z;
#[doc = "Controller for input/output pad A"]
pub use self::PAD as PAD_A;
#[doc = "Controller for input/output pad B"]
pub use self::PAD as PAD_B;
#[doc = "Controller for input/output pad C"]
pub use self::PAD as PAD_C;
#[doc = "Controller for input/output pad D"]
pub use self::PAD as PAD_D;
#[doc = "Controller for input/output pad E"]
pub use self::PAD as PAD_E;
#[doc = "Controller for input/output pad F"]
pub use self::PAD as PAD_F;
#[doc = "Controller for input/output pad X"]
pub use self::PAD as PAD_X;
#[doc = "Controller for input/output pad Y"]
pub use self::PAD as PAD_Y;
#[doc = "Controller for input/output pad Z"]
pub use self::PAD as PAD_Z;
