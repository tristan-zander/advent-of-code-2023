use std::{cell::RefCell, collections::{HashMap, VecDeque}, ops::Not, rc::Rc, str::FromStr};

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_twenty.txt");

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Pulse {
    Low = 0,
    High = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum FlipFlop {
    Off = 0,
    On = 1,
}

impl Not for FlipFlop {
    type Output = FlipFlop;

    fn not(self) -> Self::Output {
        match self {
            FlipFlop::Off => FlipFlop::On,
            FlipFlop::On => FlipFlop::Off,
        }
    }
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        name: String,
        state: FlipFlop,
        outputs: Box<[String]>,
    },
    Conjunction {
        name: String,
        previous_inputs: HashMap<String, Pulse>,
        outputs: Box<[String]>,
    },
    Broadcaster {
        outputs: Box<[String]>,
    },
}

impl Module {
    pub fn name(&self) -> String {
        match self {
            Module::FlipFlop {
                name,
                state: _,
                outputs: _,
            } => name.to_owned(),
            Module::Conjunction {
                name,
                previous_inputs: _,
                outputs: _,
            } => name.to_owned(),
            Module::Broadcaster { outputs: _ } => "broadcaster".to_owned(),
        }
    }

    pub fn outputs(&self) -> &[String] {
        match self {
            Module::FlipFlop {
                name: _,
                state: _,
                outputs,
            } => outputs.as_ref(),
            Module::Conjunction {
                name: _,
                previous_inputs: _,
                outputs,
            } => outputs.as_ref(),
            Module::Broadcaster { outputs } => outputs.as_ref(),
        }
    }

    pub fn pulse(&mut self, name: String, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop {
                name: _,
                state,
                outputs: _,
            } => {
                if !matches!(pulse, Pulse::Low) {
                    return None;
                }

                *state = !*state;
                let pulse = *state as u8;
                // SAFETY: I'm ensuring that FlipFlop's On/Off variants are the same as Pulse's High/Low variants
                return Some(unsafe { std::mem::transmute(pulse) });
            }
            Module::Conjunction {
                name: _,
                previous_inputs,
                outputs: _,
            } => {
                previous_inputs.insert(name, pulse);

                if previous_inputs.values().all(|v| matches!(v, Pulse::High)) {
                    return Some(Pulse::Low);
                }
                return Some(Pulse::High);
            }
            Module::Broadcaster { outputs: _ } => Some(pulse),
        }
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (module, targets) = s.split(" -> ").collect_tuple().unwrap();
        let outputs = targets.split(", ").map(|s| s.to_owned()).collect();

        if module == "broadcaster" {
            return Ok(Module::Broadcaster { outputs });
        }

        let (typ, name) = (module.chars().next().unwrap(), module[1..].to_owned());

        match typ {
            '%' => Ok(Module::FlipFlop {
                name,
                state: FlipFlop::Off,
                outputs,
            }),
            '&' => Ok(Module::Conjunction {
                name,
                previous_inputs: HashMap::new(),
                outputs,
            }),
            _ => unreachable!(),
        }
    }
}

fn input() -> HashMap<String, Rc<RefCell<Module>>> {
    let modules = FILE_CONTENTS
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|m: Module| (m.name(), Rc::new(RefCell::new(m))))
        .collect::<HashMap<_, _>>();

    for (name, module) in &modules {
        let module = module.borrow_mut();
        for output in module.outputs() {
            let output_module = modules.get(output);
            if output_module.is_none() {
                continue;
            }
            let output_module = output_module.unwrap();
            let output_module = &mut *output_module.borrow_mut();
            match output_module {
                Module::Conjunction {
                    name: _,
                    previous_inputs,
                    outputs: _,
                } => {
                    previous_inputs.insert(name.to_owned(), Pulse::Low);
                }
                _ => {}
            }
        }
    }

    modules
}

fn press_button(input: &mut HashMap<String, Rc<RefCell<Module>>>) -> (usize, usize, bool) {
    let mut modules = VecDeque::new();
    modules.push_front((Pulse::Low, input.get("broadcaster").unwrap().clone()));

    let mut high_pulse_counter = 0;
    // Low starts at one because the button push counts as a low
    let mut low_pulse_counter = 1;

    let mut hit_rx = false;

    while let Some((output_pulse, module)) = modules.pop_front() {
        let name = module.borrow().name();
        for output in module.borrow().outputs() {
            match output_pulse {
                Pulse::Low => low_pulse_counter += 1,
                Pulse::High => high_pulse_counter += 1,
            }
            let output_module_rc = input.get_mut(output);
            if output_module_rc.is_none() {
                if output.as_str() == "rx" && matches!(output_pulse, Pulse::Low) {
                    hit_rx = true;
                }
                continue;
            }
            let output_module_rc = output_module_rc.unwrap().clone();
            let mut output_module = output_module_rc.borrow_mut();
            if let Some(new_pulse) = output_module.pulse(name.to_owned(), output_pulse) {
                drop(output_module);
                modules.push_back((new_pulse, output_module_rc));
            }
        }
    }

    (high_pulse_counter, low_pulse_counter, hit_rx)
}

pub fn part_one(_args: Args) {
    let mut input = input();
    let mut high_counter = 0;
    let mut low_counter = 0;
    for _ in 0..1000 {
        let (high, low, _) = press_button(&mut input);
        high_counter += high;
        low_counter += low;
    }
    println!("High: {}, Low: {}", high_counter, low_counter);
    println!("Answer: {}", high_counter * low_counter);
}

pub fn part_two(_args: Args) {
    let mut input = input();
    let mut presses = 0;
    loop {
        presses += 1;
        if presses % 1000 == 0 {
            println!("Pressing: {}", presses);
        }
        let (_, _, hit_rx) = press_button(&mut input);
        if hit_rx {
            break;
        }
    }
    println!("Answer: {}", presses);
}