pub(super) fn avif_dimensions_from_bmff(bytes: &[u8]) -> Option<(u32, u32)> {
    if !has_avif_brand(bytes) {
        return None;
    }

    let meta = find_box_payload(bytes, b"meta")?;
    if meta.len() < 4 {
        return None;
    }

    let iprp = find_box_payload(&meta[4..], b"iprp")?;
    let ipco = find_box_payload(iprp, b"ipco")?;
    let ispe = find_box_payload(ipco, b"ispe")?;
    ispe_dimensions(ispe)
}

fn has_avif_brand(bytes: &[u8]) -> bool {
    let Some(ftyp) = find_box_payload(bytes, b"ftyp") else {
        return false;
    };
    if ftyp.len() < 8 {
        return false;
    }

    is_avif_brand(&ftyp[..4]) || ftyp[8..].chunks_exact(4).any(is_avif_brand)
}

fn is_avif_brand(brand: &[u8]) -> bool {
    matches!(brand, b"avif" | b"avis")
}

fn ispe_dimensions(payload: &[u8]) -> Option<(u32, u32)> {
    if payload.len() < 12 || !has_zero_full_box_header(payload) {
        return None;
    }

    let width = read_u32_be(&payload[4..8]);
    let height = read_u32_be(&payload[8..12]);
    if width == 0 || height == 0 {
        return None;
    }
    Some((width, height))
}

fn has_zero_full_box_header(payload: &[u8]) -> bool {
    payload.get(..4) == Some([0, 0, 0, 0].as_slice())
}

fn find_box_payload<'a>(bytes: &'a [u8], kind: &[u8; 4]) -> Option<&'a [u8]> {
    let mut offset = 0usize;
    while offset < bytes.len() {
        let Some(bmff_box) = read_box(bytes, offset) else {
            break;
        };
        if &bmff_box.kind == kind {
            return Some(bmff_box.payload);
        }
        if bmff_box.end <= offset {
            break;
        }
        offset = bmff_box.end;
    }
    None
}

struct BmffBox<'a> {
    kind: [u8; 4],
    payload: &'a [u8],
    end: usize,
}

fn read_box(bytes: &[u8], offset: usize) -> Option<BmffBox<'_>> {
    if offset + 8 > bytes.len() {
        return None;
    }

    let size = read_u32_be(&bytes[offset..offset + 4]) as usize;
    let mut kind = [0u8; 4];
    kind.copy_from_slice(&bytes[offset + 4..offset + 8]);

    let (payload_start, end) = match size {
        0 => (offset + 8, bytes.len()),
        1 => {
            if offset + 16 > bytes.len() {
                return None;
            }
            let extended_size = read_u64_be(&bytes[offset + 8..offset + 16]);
            let end = usize::try_from(extended_size).ok()?.checked_add(offset)?;
            (offset + 16, end)
        }
        2..=7 => return None,
        _ => (offset + 8, offset.checked_add(size)?),
    };

    if payload_start > end || end > bytes.len() {
        return None;
    }

    Some(BmffBox {
        kind,
        payload: &bytes[payload_start..end],
        end,
    })
}

fn read_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn read_u64_be(bytes: &[u8]) -> u64 {
    u64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}
