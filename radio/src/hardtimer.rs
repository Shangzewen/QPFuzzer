use std::collections::HashMap;
// timer2 peripheral base address 0x4000B000
// full implementation needed
// const TASKS_START_RTC0: u32 = 0x000;    // Start RTC0_COUNTER
// const TASKS_STOP_RTC0: u32 = 0x004;    // Stop RTC0_COUNTER
// const TASKS_CLEAR_RTC0: u32 = 0x008;    // Clear RTC0_COUNTER
// const EVENTS_OVRFLW_RTC0: u32 = 0x104;    // EVENT on Counter overflow
// const EVTEN_RTC0: u32 = 0x340;    // Enable or diable event routing
// const EVTENSET_RTC0: u32 = 0x344;    // Enable event routing
// const EVTENCLR_RTC0: u32 = 0x348;    // Disable event routing
// const PRESCALER_RTC0: u32 = 0x508;    // 12 bit prescaler for counter 
// const CC0_RTC0: u32 = 0x540;    // CC0
// const CC1_RTC0: u32 = 0x544;    // CC1
// const CC2_RTC0: u32 = 0x548;    // CC2
// short version
const EVENTS_COMPARE0_TIMER2: u32 = 0x140;    // Compare event on CC0 match
const EVENTS_COMPARE1_TIMER2: u32 = 0x144;    // Compare event on CC1 match
const EVENTS_COMPARE2_TIMER2: u32 = 0x148;    // Compare event on CC2 match
const EVENTS_COMPARE3_TIMER2: u32 = 0x14c;    // Compare event on CC0 match
const EVENTS_COMPARE4_TIMER2: u32 = 0x150;    // Compare event on CC1 match
const EVENTS_COMPARE5_TIMER2: u32 = 0x154;    // Compare event on CC2 match
const INTENSET_TIMER2: u32 = 0x304;       // Interrupt set
const INTENCLR_TIMER2: u32 = 0x308;       // Interrupt clear

#[derive(Debug)]
pub struct Timer2 {
    registers: HashMap<u32, u32>,
    interrupts: u32,
}

impl Timer2 {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(EVENTS_COMPARE0_TIMER2, 0);
        registers.insert(EVENTS_COMPARE1_TIMER2, 0);
        registers.insert(EVENTS_COMPARE2_TIMER2, 0);
        Timer2 {
            registers,
            interrupts: 0x0,
        }
    }
    pub fn read_register(&self, address: u32) -> u32 {
        if address == INTENSET_TIMER2 || address == INTENCLR_TIMER2{
        // println!("Read register from 0x{:0x}, return value 0x{:0x}",address,self.interrupts);
            return self.interrupts
        }else{
            // println!("Read register from 0x{:0x}, return value 0x{:0x}",address,*self.registers.get(&address).unwrap_or(&0));
            return *self.registers.get(&address).unwrap_or(&0)

        }
    }
    pub fn write_register(&mut self, address: u32, value: u32) {
        // Update or insert the register value
        self.registers.insert(address, value);
        // if address >= 0x100 {
        //     self.registers.insert(address, value);
        // }
        // Handle TASKS registers (trigger on write of 1)
        // println!("write register at 0x{:0x} with value {}",address, value);
        match (address, value) {
            // INTENSET and INTENCLR is handle the interrupt not the event
            // the event clear will handled by event_clear function but interrupt need to
            // be handled by us
            (INTENSET_TIMER2, 0x00010000) => {
                println!("Set EVENTS_COMPARE0_TIMER2");
                self.set_event(EVENTS_COMPARE0_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENSET_TIMER2, 0x00020000) => {
                println!("Set EVENTS_COMPARE1_TIMER2");
                self.set_event(EVENTS_COMPARE1_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENSET_TIMER2, 0x00040000) => {
                println!("Set EVENTS_COMPARE2_TIMER2");
                self.set_event(EVENTS_COMPARE2_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENSET_TIMER2, 0x00080000) => {
                println!("Set EVENTS_COMPARE3_TIMER2");
                self.set_event(EVENTS_COMPARE3_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENSET_TIMER2, 0x00100000) => {
                println!("Set EVENTS_COMPARE4_TIMER2");
                self.set_event(EVENTS_COMPARE4_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENSET_TIMER2, 0x00200000) => {
                println!("Set EVENTS_COMPARE5_TIMER2");
                self.set_event(EVENTS_COMPARE5_TIMER2, 1);
                self.interrupts |= value;
            }
            (INTENCLR_TIMER2, 0x00010000) => {
                println!("Clear EVENTS_COMPARE0_TIMER2");
                self.set_event(EVENTS_COMPARE0_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x00020000) => {
                println!("Clear EVENTS_COMPARE1_TIMER2");
                self.set_event(EVENTS_COMPARE1_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x00040000) => {
                println!("Clear EVENTS_COMPARE2_TIMER2");
                self.set_event(EVENTS_COMPARE2_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x00080000) => {
                println!("Clear EVENTS_COMPARE3_TIMER2");
                self.set_event(EVENTS_COMPARE3_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x00100000) => {
                println!("Clear EVENTS_COMPARE4_TIMER2");
                self.set_event(EVENTS_COMPARE4_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x00200000) => {
                println!("Clear EVENTS_COMPARE5_TIMER2");
                self.set_event(EVENTS_COMPARE5_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
            }
            (INTENCLR_TIMER2, 0x003F0000) => {
                println!("Clear ALL !!! EVENTS_COMPARE_TIMER2");
                self.set_event(EVENTS_COMPARE0_TIMER2, 0);
                self.set_event(EVENTS_COMPARE1_TIMER2, 0);
                self.set_event(EVENTS_COMPARE2_TIMER2, 0);
                self.set_event(EVENTS_COMPARE3_TIMER2, 0);
                self.set_event(EVENTS_COMPARE4_TIMER2, 0);
                self.set_event(EVENTS_COMPARE5_TIMER2, 0);
                let reversed_bit = !value;
                self.interrupts &= reversed_bit;
                // println!("interrupts after {:b}",self.interrupts);
            }
            _ => {}
        }
    
        // Clear EVENTS registers if writing 0 (assuming EVENTS range 0x100-0x1FF)
        // if address >= 0x100 && address < 0x200 && value == 0 {
        //     self.set_event(address, 0);
        // }
    }
    /// Sets an event and triggers shortcuts if enabled
    pub fn set_event(&mut self, event_address: u32, value: u32) {
        self.registers.insert(event_address, value);
        // if value == 1 {
        //     self.handle_event(event_address);
        // }
    }
}
