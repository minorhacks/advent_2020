use advent_2020::jigsaw;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_20/input.txt").unwrap();
    let tiles = input.trim().parse::<jigsaw::Tiles>().unwrap();
    println!(
        "Part 1: {}",
        tiles.assemble(12).corner_ids().iter().product::<usize>()
    );

    let mut images = tiles.assemble(12).images();
    let image = images
        .iter_mut()
        .filter_map(|i| match i.mark_sea_monsters() {
            0 => None,
            _ => Some(i),
        })
        .next()
        .unwrap();
    println!("Part 2: {}", image.roughness());
}
