use crate::{Ptr, create_gateway, jump::{AbsoluteJump, Jump}, Hook};
use iced_x86::Decoder;

pub(crate) unsafe fn hook_absolute(p_src: Ptr, p_dst: Ptr, pp_original: Ptr<Ptr>) -> crate::Result<Hook> {
    let mut stolen_size = 0;

    let dec = Decoder::new(
        crate::BITNESS,
        std::slice::from_raw_parts(p_src, 30),
        0
    );

    for asm in dec {
        stolen_size += asm.len();
        if stolen_size >= AbsoluteJump::SIZE {
            break;
        }
    }

    let gateway = create_gateway(p_src, stolen_size, p_src.add(stolen_size))?;

    AbsoluteJump::write(p_src, p_dst)?;

    if !pp_original.is_null() {
        *pp_original = gateway;
    }

    Ok(Hook::Absolute {
        gateway: gateway as _,
        p_target: p_src as _,
        stolen_size,
    })
}