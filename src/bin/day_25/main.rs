use advent_2020::crypto;

fn main() {
    const DOOR_PUBLIC_KEY: u64 = 10441485;
    const CARD_PUBLIC_KEY: u64 = 1004920;

    let door_loop_size = crypto::find_loop_size(7, DOOR_PUBLIC_KEY);
    let card_loop_size = crypto::find_loop_size(7, CARD_PUBLIC_KEY);

    let k1 = crypto::transform(DOOR_PUBLIC_KEY, card_loop_size);
    let k2 = crypto::transform(CARD_PUBLIC_KEY, door_loop_size);

    assert_eq!(k1, k2);
    println!("Part 1: {}", k1);
}
