// mod cpu;
// mod memory;

use rust_by_example::lib::cpu::CPU;
use rust_by_example::lib::memory::Memory;

// TODO: Define a public Computer struct with two public fields:
// - cpu: CPU
// - memory: Memory
pub struct Computer {
    pub cpu: CPU,
    pub memory: Memory
}

// TODO: Implement a system_info method on Computer that:
// - Delegates to cpu.specs() and memory.specs()
// - Prints: System: {cpu_specs} | {memory_specs}
impl Computer {
    fn system_info(&self) {
        println!("System: {} | {}",self.cpu.specs(), self.memory.specs());
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let cores: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let speed_ghz: f32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let size_gb: u32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create CPU and Memory instances using the parsed inputs
    let cpu: CPU = CPU {cores, speed_ghz};
    let memory: Memory = Memory {size_gb};
    // TODO: Create a Computer instance with the CPU and Memory
    let computer: Computer = Computer {cpu, memory};
    // TODO: Call system_info() on the computer to print the result
    computer.system_info();
}
