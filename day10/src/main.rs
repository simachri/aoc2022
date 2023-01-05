use std::fmt;
use std::fmt::Write;

struct Cpu {
    register_value: i32,
    cycle: u32,
    processing_duration: u32,
    processing_instruction: Option<CpuInstruction>,
}

enum CpuState {
    Processing,
    Halted,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            register_value: 1,
            cycle: 1,
            processing_duration: 0,
            processing_instruction: None,
        }
    }

    fn tik(&mut self) -> CpuState {
        self.cycle += 1;
        self.processing_duration -= 1;

        if self.processing_duration == 0 {
            match self.processing_instruction.as_ref().unwrap() {
                CpuInstruction::Noop => (),
                CpuInstruction::Addx(value) => {
                    self.register_value += value;
                }
            }

            self.processing_instruction = None;
            CpuState::Halted
        } else {
            CpuState::Processing
        }
    }

    fn add_instruction(&mut self, instruction: CpuInstruction) {
        match instruction {
            CpuInstruction::Noop => self.processing_duration = 1,
            CpuInstruction::Addx(_) => self.processing_duration = 2,
        }

        self.processing_instruction = Some(instruction);
    }
}

enum CpuInstruction {
    Noop,
    Addx(i32),
}

#[derive(Clone)]
enum CrtPixel {
    Lit,
    Dark,
}

impl fmt::Display for CrtPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrtPixel::Lit => write!(f, "#"),
            CrtPixel::Dark => write!(f, "."),
        }
    }
}

struct Crt {
    width_px: u32,
    current_pixel_pos_x: u32,
    current_pixel_pos_y: u32,
    display: Vec<Vec<CrtPixel>>,
}

impl Crt {
    fn new(width_px: u32, height_px: u32) -> Crt {
        let display = vec![vec![CrtPixel::Dark; width_px as usize]; height_px as usize];
        Crt {
            width_px,
            current_pixel_pos_x: 0,
            current_pixel_pos_y: 0,
            display,
        }
    }

    fn draw(&mut self, cpu: &Cpu) {
        let draw_lit_range = cpu.register_value - 1..=cpu.register_value + 1;

        let pixel: CrtPixel;
        if draw_lit_range.contains(&(self.current_pixel_pos_x as i32)) {
            pixel = CrtPixel::Lit;
        } else {
            pixel = CrtPixel::Dark;
        }

        println!("Cycle {}, register value {}", cpu.cycle, cpu.register_value);
        println!(
            "Drawing pixel {} at {},{}",
            &pixel, self.current_pixel_pos_x, self.current_pixel_pos_y
        );

        self.display[self.current_pixel_pos_y as usize][self.current_pixel_pos_x as usize] = pixel;

        self.advance_pixel_pos();
    }

    fn advance_pixel_pos(&mut self) {
        if self.current_pixel_pos_x == self.width_px - 1 {
            self.current_pixel_pos_x = 0;
            self.current_pixel_pos_y += 1;
        } else {
            self.current_pixel_pos_x += 1;
        }
    }

    fn render(&self) -> String {
        let mut rendering_result: String = String::new();

        for (idx, row) in self.display.iter().enumerate() {
            for pixel in row {
                rendering_result.push_str(&pixel.to_string());
            }
            if idx != self.display.len() - 1 {
                write!(rendering_result, "\n").unwrap();
            }
        }

        return rendering_result;
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

    let result_part_1 = calculate_sum_of_signal_strengths(input, SignalStrengthMeasure::new());
    let result_part_2 = produce_crt_image(input);

    println!("Result of part 1: {}", result_part_1);
    println!("Result of part 2:\n{}", result_part_2);
}

fn calculate_sum_of_signal_strengths(input: &str, mut measure: SignalStrengthMeasure) -> i32 {
    let mut total_signal_strength = 0;

    let mut cpu = Cpu::new();

    let mut next_cycle_measure = measure.next();
    let mut cycle_signal_strength: i32;
    for instr in input.lines().map(parse_instruction) {
        cpu.add_instruction(instr);

        loop {
            let processing;

            match cpu.tik() {
                CpuState::Processing => processing = true,
                CpuState::Halted => processing = false,
            }

            if cpu.cycle == next_cycle_measure {
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

            if !processing {
                break;
            }
        }
    }

    return total_signal_strength;
}

fn produce_crt_image(input: &str) -> String {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new(40, 6);

    for instr in input.lines().map(parse_instruction) {
        cpu.add_instruction(instr);

        loop {
            let processing;

            crt.draw(&cpu);

            match cpu.tik() {
                CpuState::Processing => processing = true,
                CpuState::Halted => processing = false,
            }

            if !processing {
                break;
            }
        }
    }

    return crt.render();
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
            calculate_sum_of_signal_strengths(input, SignalStrengthMeasure::new()),
            13140
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");

        assert_eq!(
            produce_crt_image(input),
            r########"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."########
        );
    }
}
