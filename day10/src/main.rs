struct Cpu {
    register_value: i32,
    cycle: u32,
}

enum CpuInstruction {
    Noop,
    Addx(i32),
}

trait CpuInstructionExecutor {
    fn execute(&self, cpu: &mut Cpu);
}

impl CpuInstructionExecutor for CpuInstruction {
    fn execute(&self, cpu: &mut Cpu) {
        match self {
            CpuInstruction::Noop => {
                cpu.cycle += 1;
                println!("Processed instruction 'noop'. New cycle: {}", cpu.cycle);
            }
            CpuInstruction::Addx(value) => {
                cpu.cycle += 2;
                cpu.register_value += value;
                println!(
                    "Processed instruction 'addx {}'. New cycle: {}. New register value: {}",
                    value, cpu.cycle, cpu.register_value
                );
            }
        }
    }
}

struct SignalStrengthMeasure {
    next_cycle: u32,
    cycle_step_size: u32,
}

impl SignalStrengthMeasure {
    fn new() -> SignalStrengthMeasure {
        SignalStrengthMeasure {
            next_cycle: 20,
            cycle_step_size: 40,
        }
    }

    fn next(&mut self) -> u32 {
        let next = self.next_cycle;
        self.next_cycle += self.cycle_step_size;
        return next;
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let result_part_1 = calculate_sum_of_signal_strenghts(input, SignalStrengthMeasure::new());

    println!("Result of part 1: {}", result_part_1);
}

fn calculate_sum_of_signal_strenghts(input: &str, mut measure: SignalStrengthMeasure) -> i32 {
    let mut total_signal_strength = 0;

    let mut cpu = Cpu {
        register_value: 1,
        cycle: 1,
    };

    let mut next_cycle_measure = measure.next();
    let mut cycle_signal_strength: i32;
    for instr in input.lines().map(parse_instruction) {
        instr.execute(&mut cpu);

        // The next instruction might be a 'Addx' that increments the cpu cycle by 2.
        // If the next measure is at cycle i + 1 and not at i + 2, the measure result has to be the
        // current register value.
        // This condition does not interfere with instruction 'noop'.
        let measure_now = (next_cycle_measure - 1..=next_cycle_measure).contains(&cpu.cycle);
        if measure_now {
            cycle_signal_strength = cpu.register_value * next_cycle_measure as i32;

            println!(
                "Measuring register value at cycle {}: {}",
                next_cycle_measure, cpu.register_value
            );
            println!(
                "Signal strength at cycle {}: {}",
                next_cycle_measure, cycle_signal_strength
            );

            total_signal_strength += cycle_signal_strength;
            next_cycle_measure = measure.next();
        }
    }

    return total_signal_strength;
}

fn parse_instruction(line: &str) -> CpuInstruction {
    let mut parts = line.split_whitespace();
    let _instr = parts.next().unwrap();
    match parts.next() {
        Some(value) => CpuInstruction::Addx(value.parse().unwrap()),
        None => CpuInstruction::Noop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");

        assert_eq!(
            calculate_sum_of_signal_strenghts(input, SignalStrengthMeasure::new()),
            13140
        );
    }
}
