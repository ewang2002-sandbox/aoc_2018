use std::collections::HashMap;

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (i32, String) {
    let initial_state = input[0].replace("initial state: ", "");
    let mut plant_mapping: HashMap<String, String> = HashMap::new();

    input.iter().skip(2).for_each(|line| {
        let plant_layout = line.split(" => ").collect::<Vec<_>>();
        plant_mapping.insert(plant_layout[0].to_string(), plant_layout[1].to_string());
    });

    return (part1(&initial_state, &plant_mapping), part2());
}

// --- Day 12: Subterranean Sustainability ---
// The year 518 is significantly more underground than your history books implied. Either that, or
// you've arrived in a vast cavern network under the North Pole.
//
// After exploring a little, you discover a long tunnel that contains a row of small pots as far as
// you can see to your left and right. A few of them contain plants - someone is trying to grow
// things in these geothermally-heated caves.
//
// The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3,
// and so on; to the right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the
// right and whether they do (#) or do not (.) currently contain a plant, the initial state. (No
// other pots currently contain plants.) For example, an initial state of #..##.... indicates that
// pots 0, 3, and 4 currently contain plants.
//
// Your puzzle input also contains some notes you find on a nearby table: someone has been trying
// to figure out how these plants spread to nearby pots. Based on the notes, for each generation
// of plants, a given pot has or does not have a plant based on whether that pot (and the two pots
// on either side of it) had a plant in the last generation. These are written as LLCRR => N, where
// L are pots to the left, C is the current pot being considered, R are the pots to the right, and
// N is whether the current pot will have a plant in the next generation. For example:
//
// - A note like ..#.. => . means that a pot that contains a plant but with no plants within two
// pots of it will not have a plant in it during the next generation.
// - A note like ##.## => . means that an empty pot with two plants on each side of it will remain
// empty in the next generation.
// - A note like .##.# => # means that a pot has a plant in a given generation if, in the previous
// generation, there were plants in that pot, the one immediately to the left, and the one two pots
// to the right, but not in the ones immediately to the right and two to the left.
//
// It's not clear what these plants are for, but you're sure it's important, so you'd like to make
// sure the current configuration of plants is sustainable by determining what will happen after
// 20 generations.
//
// For example, given the following input:
//
// initial state: #..#.#..##......###...###
//
//  ...## => #
//  ..#.. => #
//  .#... => #
//  .#.#. => #
//  .#.## => #
//  .##.. => #
//  .#### => #
//  #.#.# => #
//  #.### => #
//  ##.#. => #
//  ##.## => #
//  ###.. => #
//  ###.# => #
//  ####. => #
// For brevity, in this example, only the combinations which do produce a plant are listed. (Your
// input includes all possible combinations.) Then, the next 20 generations will look like this:
//
//                  1         2         3
//        0         0         0         0
//  0: ...#..#.#..##......###...###...........
//  1: ...#...#....#.....#..#..#..#...........
//  2: ...##..##...##....#..#..#..##..........
//  3: ..#.#...#..#.#....#..#..#...#..........
//  4: ...#.#..#...#.#...#..#..##..##.........
//  5: ....#...##...#.#..#..#...#...#.........
//  6: ....##.#.#....#...#..##..##..##........
//  7: ...#..###.#...##..#...#...#...#........
//  8: ...#....##.#.#.#..##..##..##..##.......
//  9: ...##..#..#####....#...#...#...#.......
// 10: ..#.#..#...#.##....##..##..##..##......
// 11: ...#...##...#.#...#.#...#...#...#......
// 12: ...##.#.#....#.#...#.#..##..##..##.....
// 13: ..#..###.#....#.#...#....#...#...#.....
// 14: ..#....##.#....#.#..##...##..##..##....
// 15: ..##..#..#.#....#....#..#.#...#...#....
// 16: .#.#..#...#.#...##...#...#.#..##..##...
// 17: ..#...##...#.#.#.#...##...#....#...#...
// 18: ..##.#.#....#####.#.#.#...##...##..##..
// 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
// 20: .#....##....#####...#######....#.#..##.
// The generation is shown along the left, where 0 is the initial state. The pot numbers are shown
// along the top, where 0 labels the center pot, negative-numbered pots extend to the left, and
// positive pots extend toward the right. Remember, the initial state begins at pot 0, which is
// not the leftmost pot used in this example.
//
// After one generation, only seven plants remain. The one in pot 0 matched the rule looking for
// ..#.., the one in pot 4 matched the rule looking for .#.#., pot 9 matched .##.., and so on.
//
// In this example, after 20 generations, the pots shown as # contain plants, the furthest left of
// which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-
// containing pots after the 20th generation produces 325.
//
// After 20 generations, what is the sum of the numbers of all pots which contain a plant?

pub fn part1(init_state: &String, plant_notes: &HashMap<String, String>) -> i32 {
    let mut curr_state = init_state.clone();
    // The plant that we're initially looking at
    let init_plant_idx = 30;
    for _ in 0..30 {
        curr_state.insert(0, '.');
        curr_state.push('.');
    }

    let mut new_state = String::new();
    for _ in 1..=20 {
        let mut curr_state_chars = curr_state.bytes().collect::<Vec<_>>();
        for _ in 0..2 {
            curr_state_chars.insert(0, b'.');
            curr_state_chars.push(b'.');
        }

        curr_state_chars.windows(5).for_each(|window| {
            let window_str = String::from_utf8_lossy(window);
            match plant_notes.get(&*window_str) {
                Some(s) => new_state.push_str(s),
                None => new_state.push('.')
            }
        });

        assert_eq!(new_state.len(), curr_state_chars.len() - 4);
        curr_state = new_state.clone();
        new_state = String::new();
    }

    let mut sum = 0;
    let bytes = curr_state.bytes().collect::<Vec<_>>();
    for i in 0..bytes.len() {
        sum += if bytes[i] == b'#' { (i as i32) - init_plant_idx } else { 0 }
    }

    return sum;
}

pub fn part2() -> String {
    return "".to_string();
}