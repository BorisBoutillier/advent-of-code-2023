use std::collections::{BTreeMap, HashMap, VecDeque};

use aoc_commons::*;

struct Broadcaster {
    dests: Vec<String>,
}
struct FlipFlop {
    state: bool,
    dests: Vec<String>,
}
struct Conjonction {
    state: BTreeMap<String, bool>,
    dests: Vec<String>,
}
pub fn solver(part: Part, input: &str) -> String {
    let mut broadcaster = Broadcaster { dests: vec![] };
    let mut flip_flops = BTreeMap::new();
    let mut conjonctions = BTreeMap::new();
    for line in input.lines() {
        let (module, dests) = line.split_once(" -> ").unwrap();
        let dests = dests
            .split(", ")
            .map(|dest| dest.to_string())
            .collect::<Vec<_>>();
        if module.starts_with("broadcaster") {
            broadcaster.dests = dests;
        } else if let Some(name) = module.strip_prefix('%') {
            flip_flops.insert(
                name.to_string(),
                FlipFlop {
                    state: false,
                    dests,
                },
            );
        } else if let Some(name) = module.strip_prefix('&') {
            conjonctions.insert(
                name.to_string(),
                Conjonction {
                    state: BTreeMap::new(),
                    dests,
                },
            );
        } else {
            panic!();
        }
    }
    for (name, flip_flop) in flip_flops.iter() {
        for dest in flip_flop.dests.iter() {
            if let Some(conjonction) = conjonctions.get_mut(dest) {
                conjonction.state.insert(name.clone(), false);
            }
        }
    }
    let mut to_do = vec![];
    for (name, conjonction) in conjonctions.iter() {
        for dest in conjonction.dests.iter() {
            if conjonctions.contains_key(dest) {
                to_do.push((dest.clone(), name.clone()));
            }
        }
    }
    for (name, input) in to_do {
        conjonctions
            .get_mut(&name)
            .unwrap()
            .state
            .insert(input.clone(), false);
    }
    println!("FlipFlops: {}", flip_flops.len());
    println!("Conjonctions: {}", conjonctions.len());
    let mut qls = HashMap::new();
    if part == Part::Part2 {
        // Hardcoded on my input where ql is the difficult conjonction
        // leading to 'rx'
        // Probably all user inputs have a last conjonction leading to 'rx'
        // Look into your input and replace 'ql' with your last conjonction name
        println!(
            "QLs inputs : {:?}",
            conjonctions["ql"].state.keys().collect::<Vec<_>>()
        );
        for k in conjonctions["ql"].state.keys() {
            qls.insert(k.clone(), vec![]);
        }
    }
    let mut iteration = 0u64;
    let mut cache = HashMap::new();
    let mut pulses = [0u64, 0u64];
    let n_iteration = match part {
        Part::Part1 => 1000,
        Part::Part2 => u64::MAX,
    };
    while iteration < n_iteration {
        {
            let cur_state = flip_flops
                .values()
                .map(|f| f.state)
                .chain(
                    conjonctions
                        .values()
                        .flat_map(|c| c.state.values().copied()),
                )
                .collect::<Vec<_>>();
            if let Some((it1, lows1, highs1)) = cache.get(&cur_state) {
                println!("FOUND {iteration} vs {it1}; {pulses:?} vs [{lows1},{highs1}]");
                let cycle = iteration - it1;
                let repeats = (n_iteration - 1 - iteration) / cycle;
                iteration += cycle * repeats;
                pulses[0] += (pulses[0] - lows1) * repeats;
                pulses[1] += (pulses[1] - highs1) * repeats;
            } else {
                cache.insert(cur_state, (iteration, pulses[0], pulses[1]));
            }
        }
        //println!("ITERATION {iteration}");
        pulses[0] += 1; // button
        let mut actions = broadcaster
            .dests
            .iter()
            .map(|dest| ("broadcaster".to_string(), dest.clone(), false))
            .collect::<VecDeque<_>>();
        let mut rx_low = false;
        while let Some((from, to, pulse)) = actions.pop_front() {
            //println!("  Actions: {from} {to} {pulse}");
            pulses[pulse as usize] += 1;
            if let Some(flip_flop) = flip_flops.get_mut(&to) {
                if !pulse {
                    flip_flop.state = !flip_flop.state;
                    actions.extend(
                        flip_flop
                            .dests
                            .iter()
                            .map(|dest| (to.clone(), dest.clone(), flip_flop.state)),
                    );
                }
            } else if let Some(conjonction) = conjonctions.get_mut(&to) {
                if part == Part::Part2 && to == "ql" && pulse {
                    if qls[&from].len() == 2 {
                        assert_eq!(
                            (iteration - qls[&from][1]) % (qls[&from][1] - qls[&from][0]),
                            0
                        );
                    } else {
                        qls.get_mut(&from).unwrap().push(iteration);
                        if qls.values().all(|v| v.len() == 2) {
                            let repeats = qls.values().map(|v| v[1] - v[0]).collect::<Vec<_>>();
                            let lcm = qls
                                .values()
                                .map(|v| v[1] - v[0])
                                .reduce(num::integer::lcm)
                                .unwrap();
                            println!("QLs: {qls:?}");
                            println!("Repeats: {repeats:?}");
                            println!("LCM: {lcm}");

                            return lcm.to_string();
                        }
                    }
                }
                conjonction.state.insert(from, pulse);
                let output = !conjonction
                    .state
                    .values()
                    .copied()
                    .reduce(|a, b| a & b)
                    .unwrap();
                actions.extend(
                    conjonction
                        .dests
                        .iter()
                        .map(|dest| (to.clone(), dest.clone(), output)),
                );
            } else if to == "rx" && !pulse {
                rx_low = true;
                break;
            }
        }
        iteration += 1;
        if rx_low {
            break;
        }
    }
    println!("Iteration {iteration}");
    println!("Lows: {} , Highs: {}", pulses[0], pulses[1]);
    match part {
        Part::Part1 => (pulses[0] * pulses[1]).to_string(),
        Part::Part2 => iteration.to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1_1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.1.txt")),
            "32000000"
        );
    }
    #[test]
    fn example_part1_2() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.2.txt")),
            "11687500"
        );
    }
    //#[test]
    //fn example_part2() {
    //    assert_eq!(
    //        solver(Part::Part2, include_str!("../example.txt")),
    //        "167409079868000"
    //    );
    //}
}
