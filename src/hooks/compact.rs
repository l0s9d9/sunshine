use crate::{Ptr, Hook, hook_absolute, SunshineError, ABSOLUTE_JUMP_SIZE, create_gateway, jump::{RelativeJump, Jump, AbsoluteJump}, ProtectionGuard};
use faithe::{internal::virtual_query, types::protection_flags::PAGE_EXECUTE_READWRITE};
use iced_x86::Decoder;

pub(crate) unsafe fn hook_compact(p_src: Ptr, p_dst: Ptr, pp_original: Ptr<Ptr>) -> crate::Result<Hook> {
    if crate::IS_X64 {
        let (from, to) = find_relative_range(p_src)?;

        if let Some(i) = std::slice::from_raw_parts(from as Ptr, (to - from).min(i16::MAX as _))
            .windows(ABSOLUTE_JUMP_SIZE)
            .position(|w| w == [0xCC; ABSOLUTE_JUMP_SIZE] || w == [0x00; ABSOLUTE_JUMP_SIZE])
        {
            let mut stolen_size = 0;
            
            let dec = Decoder::new(
                crate::BITNESS,
                std::slice::from_raw_parts(p_src, 30),
                0
            );

            for asm in dec {
                stolen_size += asm.len();
                if stolen_size >= RelativeJump::SIZE {
                    break;
                }
            }

            let _guard = ProtectionGuard::new(p_src, stolen_size, PAGE_EXECUTE_READWRITE)?;

            let gateway = create_gateway(p_src, stolen_size, p_src.add(stolen_size))?;

            let tunnel = (from + i) as Ptr;

            AbsoluteJump::write(tunnel, p_dst)?;
            RelativeJump::write(p_src, tunnel)?;

            if !pp_original.is_null() {
                *pp_original = gateway;
            }

            Ok(Hook::Compact {
                gateway: gateway as _,
                tunnel: tunnel as _,
                stolen_size,
                target: p_src as _,
            })

        } else {
            Err(SunshineError::NotEnoughSpace)
        }
    } else {
        hook_absolute(p_src, p_dst, pp_original)
    }
}

pub fn find_relative_range(head: Ptr) -> crate::Result<(usize, usize)> {
    let mem = virtual_query(head as _).map_err(|_| SunshineError::QueryFailed)?;

    let from = (head as usize).saturating_sub(i32::MAX as usize);
    let to = (head as usize).saturating_add(i32::MAX as usize);

    // dumb hack to get around pe header.
    let from = from.max(mem.alloc_base) + 0x1005;
    let to = to.min(mem.alloc_base + mem.region_size) - 5;

    Ok((from, to))
}