#[inline(always)]
pub fn hash3(a: u32, b: u32, c: u32) -> u32 {
    let mut h = a.wrapping_mul(0x8541_0000);
    h ^= b.wrapping_mul(0x9E37_79B9);
    h = h.rotate_left(13);
    h ^= c.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2545_F491);
    h ^= h >> 13;
    h
}

// uses hash3 to generate a random f32 from 0..1
#[inline(always)]
pub fn normal_hash3(a: u32, b: u32, c: u32) -> f32 {
    let hash = hash3(a, b, c);
    (hash & 0xff) as f32 / 255.0
}
