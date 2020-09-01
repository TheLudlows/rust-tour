use faster::*;

fn main() {
    let lots_of_10s = [-10i8; 3000].simd_iter(i8s(0))
        .simd_map(|v| v.abs())
        .scalar_collect();
    assert_eq!(lots_of_10s, vec![10u8; 3000]);
}
