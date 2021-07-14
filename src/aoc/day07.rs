use std::collections::HashSet;

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, i32) {
    let steps: Vec<_> = input.iter()
        .map(|x| {
            // Step C must be finished before step A can begin.
            // 0    1 2    3  4        5      6    7 8   9
            let res = x.split(" ").collect::<Vec<_>>();
            return Step { requirement: res[1].to_string(), for_part: res[7].to_string() };
        })
        .collect();

    return (part1(&steps), part2());
}

// https://adventofcode.com/2018/day/7
//
// --- Day 7: The Sum of Its Parts ---
// You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.
//
// "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
//
// "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."
//
// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.
//
// The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:
//
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.
// Visually, these requirements look like this:
//
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----
// Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:
//
// Only C is available, and so it is done first.
// Next, both A and F are available. A is first alphabetically, so it is done next.
// Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
// After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
// F is the only choice, so it is done next.
// Finally, E is completed.
// So, in this example, the correct order is CABDFE.
//
// In what order should the steps in your instructions be completed?

#[allow(dead_code)]
pub fn part1(ins: &Vec<Step>) -> String {
    let mut instructions: Vec<&Step> = ins.iter().collect();
    let mut finished = String::new();
    let mut poss_avail: HashSet<&String> = HashSet::new();

    // Step 1: We're looking for a (req, for) pair such that we don't see req anywhere in the "for"
    // member of any Step struct in the vector. That is, we're essentially doing:
    //      all_for = {map instructions to its for_part member.}
    //      all_req = {map instructions to its requirements member.}
    //  =>  result = all_req \ all_for
    // Then, we can add both the requirement and for_part members to the possible available set.
    //
    // The important thing to keep in mind is that, unlike the example, there are *multiple*
    // pairs that can be the "first" character in the string.
    let mut indices_to_remove: Vec<usize> = vec![];
    for (idx, instruction) in instructions.iter().enumerate() {
        if instructions.iter().any(|y| y.for_part == instruction.requirement) {
            continue;
        }

        indices_to_remove.push(idx);
        poss_avail.insert(&instruction.requirement);
        poss_avail.insert(&instruction.for_part);
    }

    // Go through the instructions...
    while !poss_avail.is_empty() {
        // Step 2: Filter all possibly available elements so that we only have elements that are
        // available.
        let mut all_avail: Vec<&String> = vec![];
        for &elem in &poss_avail {
            if instructions.iter().any(|x| x.for_part == *elem) {
                continue;
            }

            all_avail.push(elem);
        }

        // 2.1: Sort the available elements.
        if all_avail.is_empty() {
            panic!("Something went terribly wrong.");
        }

        all_avail.sort();

        // Step 3: Now that we have all elements that are available, take the first element and
        // then find any instructions that has this element has a requirement.
        let target_elem = all_avail[0];
        instructions.retain(|&item| {
            if *target_elem == item.requirement {
                poss_avail.insert(&item.for_part);
                return false;
            }

            return true;
        });

        poss_avail.retain(|&item| item != target_elem);

        // Step 4: Add this element to the finished vector.
        finished += target_elem.as_str();
    }

    return finished;
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    return -1;
}

#[derive(Clone, Debug)]
pub struct Step {
    requirement: String,
    for_part: String
}