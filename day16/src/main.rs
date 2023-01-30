// inspiration taken from https://github.com/WinterCore/aoc2022/blob/main/day16/main.rs
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

type ValveId = String;
type ValveDistance = u32;
type BitmaskOfOpenValves = u64;

#[derive(Debug, PartialEq, Clone)]
struct RawValve {
    id: ValveId,
    flow_rate: u32,
    neighbours: Vec<ValveId>,
}

impl FromStr for RawValve {
    type Err = ParseValveError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"([A-Z]{2}).+?(\d{1,2}).+?([A-Z]{2})(?:, ([A-Z]{2}))?(?:, ([A-Z]{2}))?(?:, ([A-Z]{2}))?(?:, ([A-Z]{2}))*"
            )
            .unwrap();
        }
        let captures = RE.captures(input).ok_or(ParseValveError)?;

        let mut neighbours = Vec::new();
        for neighbour_id in captures.iter().skip(3) {
            if neighbour_id.is_none() {
                break;
            }
            neighbours.push(neighbour_id.unwrap().as_str().to_string());
        }

        Ok(RawValve {
            id: captures[1].to_string(),
            flow_rate: captures[2].parse().unwrap(),
            neighbours,
        })
    }
}



#[derive(Debug, PartialEq, Clone)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    neighbours_idx: Vec<usize>,
}

#[derive(Debug)]
struct ParseValveError;

fn main() {
    let input = include_str!("../input.txt");

    println!("Solution of part 1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    const STARTING_VALVE_ID: &str = "AA";
    const TIME_REMAINING_MIN: u32 = 30;

    let valves = parse_valves_from_input(input);
    let distance_graph = floyd_warshall(&valves);

    calculate_max_pressure_release(
        valves,
        distance_graph,
        STARTING_VALVE_ID,
        TIME_REMAINING_MIN,
    )
}

fn calculate_max_pressure_release(
    valves: Vec<Valve>,
    distance_graph: Vec<Vec<ValveDistance>>,
    starting_valve_id: &str,
    time_window: u32,
) -> u32 {
    let valves_idx_non_zero_flow_rate: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(idx, _)| idx)
        .collect();

    let mut pressure_release_for_opened_valves_state: HashMap<BitmaskOfOpenValves, u32> =
        HashMap::new();

    let opened_valves_mask: BitmaskOfOpenValves = 0;

    let starting_valve_idx = valves
        .iter()
        .position(|valve| valve.id == starting_valve_id)
        .unwrap();

    travelling_salesman_bruteforce(
        &valves,
        &distance_graph,
        &valves_idx_non_zero_flow_rate,
        opened_valves_mask,
        &mut pressure_release_for_opened_valves_state,
        starting_valve_idx,
        0,
        time_window,
    )
}

fn travelling_salesman_bruteforce(
    all_valves: &Vec<Valve>,
    distance_graph: &Vec<Vec<ValveDistance>>,
    non_zero_flow_rate_valves_idx: &Vec<usize>,
    opened_valves: BitmaskOfOpenValves,
    max_pressrelease_for_opened_valves_state: &mut HashMap<BitmaskOfOpenValves, u32>,
    source_valve_idx: usize,
    curr_pressure_release: u32,
    minutes_left: u32,
) -> u32 {
    const TIME_TO_OPEN_VALVE: u32 = 1;

    let mut max_pressure_release = curr_pressure_release;

    max_pressrelease_for_opened_valves_state.insert(
        opened_valves,
        *max_pressrelease_for_opened_valves_state
            .get(&opened_valves)
            .unwrap_or(&0)
            .max(&curr_pressure_release),
    );

    for dest_valve_idx in non_zero_flow_rate_valves_idx.iter() {
        let new_minutes_left = minutes_left
            .checked_sub(distance_graph[source_valve_idx][*dest_valve_idx])
            .and_then(|minutes_left| minutes_left.checked_sub(TIME_TO_OPEN_VALVE))
            .unwrap_or(0);

        if valve_already_opened(opened_valves, *dest_valve_idx) || new_minutes_left == 0 {
            continue;
        }

        let new_pressure_release =
            curr_pressure_release + all_valves[*dest_valve_idx].flow_rate * new_minutes_left;

        let updated_opened_valves = opened_valves | 1 << dest_valve_idx;

        max_pressure_release = max_pressure_release.max(travelling_salesman_bruteforce(
            all_valves,
            distance_graph,
            non_zero_flow_rate_valves_idx,
            updated_opened_valves,
            max_pressrelease_for_opened_valves_state,
            *dest_valve_idx,
            new_pressure_release,
            new_minutes_left,
        ));
    }

    max_pressure_release
}

fn valve_already_opened(opened_valves_bitmask: BitmaskOfOpenValves, valve_idx: usize) -> bool {
    // 0b0001 & 0b0010 = 0b0000 (not opened) whereas 0b0001 & 0b0001 = 0b0001 (opened)
    return 1 << valve_idx & opened_valves_bitmask != 0;
}

fn floyd_warshall(valves: &Vec<Valve>) -> Vec<Vec<ValveDistance>> {
    let mut distance_graph = init_distance_graph(valves);
    let len = valves.len();

    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                if distance_graph[i][j] > distance_graph[i][k] + distance_graph[k][j] {
                    distance_graph[i][j] = distance_graph[i][k] + distance_graph[k][j];
                }
            }
        }
    }

    distance_graph
}

fn init_distance_graph(valves: &Vec<Valve>) -> Vec<Vec<ValveDistance>> {
    // Start with a long distance between all nodes.
    let mut graph_dist = vec![vec![10_0000; valves.len()]; valves.len()];

    // Direct neighbours have distance 1.
    valves.iter().enumerate().for_each(|(i, valve)| {
        valve.neighbours_idx.iter().for_each(|neighbour_valve_idx| {
            graph_dist[i][*neighbour_valve_idx as usize] = 1;
        });
    });

    graph_dist
}

fn parse_valves_from_input(input: &str) -> Vec<Valve> {
    let raw_valves: Vec<RawValve> = input.lines().map(|line| line.parse().unwrap()).collect();

    let idx_map: HashMap<ValveId, usize> = raw_valves
        .iter()
        .enumerate()
        .map(|(idx, valve)| (valve.id.to_string(), idx))
        .collect();

    let valves = raw_valves
        .iter()
        .map(|raw_valve| Valve {
            id: raw_valve.id.to_string(),
            flow_rate: raw_valve.flow_rate,
            neighbours_idx: raw_valve
                .neighbours
                .iter()
                .map(|valve_id| idx_map[valve_id])
                .collect(),
        })
        .collect();

    return valves;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(part1(input), 1651);
    }
}
