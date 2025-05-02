use std::collections::HashMap;
// use crate::{
//     modeling::hardware::*;
// };
// radio peripheral base address 0x40001000
const TASKS_TXEN: u32 = 0x000;     // Enable radio in TX mode
const TASKS_RXEN: u32 = 0x004;     // Enable radio in RX mode
const TASKS_START: u32 = 0x008;    // Start radio operation
const TASKS_STOP: u32 = 0x00C;     // Stop radio operation
const TASKS_DISABLE: u32 = 0x010;  // Disable radio
const EVENTS_READY: u32 = 0x100;   // Radio is ready (after ramp-up)
const EVENTS_ADDRESS: u32 = 0x104;   // Address sent or received
const EVENTS_PAYLOAD: u32 = 0x108;   // Payload sent or received
const EVENTS_END: u32 = 0x10C;     // Operation completed
const EVENTS_DISABLED: u32 = 0x110;// Radio disabled
const EVENTS_PHYEND: u32 = 0x16C;   // Event for PHY layer end (assumed offset)
const SHORTS: u32 = 0x200;
const INTENSET: u32 = 0x304;       // Interrupt set
const INTENCLR: u32 = 0x308;       // Interrupt clear

// SHORTS register bit definitions
const SHORTS_READY_START: u32 = 1 << 0;    // READY -> START (covers TXREADY_START, RXREADY_START)
const SHORTS_END_DISABLE: u32 = 1 << 1;    // END -> DISABLE
const SHORTS_DISABLED_TXEN: u32 = 1 << 2;  // DISABLED -> TXEN
const SHORTS_DISABLED_RXEN: u32 = 1 << 3;  // DISABLED -> RXEN
const SHORTS_END_START: u32 = 1 << 4;      // END -> START
const SHORTS_PHYEND_DISABLE: u32 = 1 << 5; // PHYEND -> DISABLE
const SHORTS_PHYEND_START: u32 = 1 << 6;   // PHYEND -> START
pub struct Radio {
    state: RadioState,
    registers: HashMap<u32, u32>,
    interrupts: u32,
}
#[derive(Debug, PartialEq, Eq)]
pub enum RadioState {
    Disabled,
    RxIdle,
    Rx,
    TxIdle,
    Tx,
}


impl Radio {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(EVENTS_READY, 0);
        registers.insert(EVENTS_END, 0);
        registers.insert(EVENTS_DISABLED, 0);
        registers.insert(EVENTS_PHYEND, 0); // Initialize PHYEND event
        registers.insert(SHORTS, 0);        // Initialize SHORTS register
        Radio {
            state: RadioState::Disabled,
            registers,
            interrupts: 0x0,
        }
    }
    pub fn read_register(&self, address: u32) -> u32 {
        if address == INTENSET || address == INTENCLR{
        println!("Read register from 0x{:0x}, return value 0x{:0x}",address,self.interrupts);
            return self.interrupts
        }else{
            println!("Read register from 0x{:0x}, return value 0x{:0x}",address,*self.registers.get(&address).unwrap_or(&0));
            return *self.registers.get(&address).unwrap_or(&0)

        }
        // do I need to handldr int_enable_check? to have a match case for INTENSET
        // I dont thik so

    }
    pub fn write_register(&mut self, address: u32, value: u32) {
        println!("write register at 0x{:0x} with value {}",address, value);

        // Update or insert the register value
        // self.registers.insert(address, value);
        // if address >= 0x100 {
        //     self.registers.insert(address, value);
        // }
        // Handle TASKS registers (trigger on write of 1)
        match (address, value) {
            // INTENSET and INTENCLR is handle the interrupt not the event
            // the event clear will handled by event_clear function but interrupt need to
            // be handled by us
            (INTENSET, _) => {
            //  Need to import from the hardware.rs
                self.interrupts |= value;
            }
            (INTENCLR, _) => {
                let reversed_bit = !value;
                // println!("value {:b}",value);
                // println!("reverse_bit {:b}",reversed_bit);
                // println!("interrupts before {:b}",self.interrupts);
                self.interrupts &= reversed_bit;
                // println!("interrupts after {:b}",self.interrupts);
            }
            (TASKS_TXEN, 1) if self.state == RadioState::Disabled => {
                self.state = RadioState::TxIdle;
                self.set_event(EVENTS_READY, 1);
            }
            (TASKS_RXEN, 1) if self.state == RadioState::Disabled => {
                self.state = RadioState::RxIdle;
                self.set_event(EVENTS_READY, 1);
            }
            (TASKS_START, 1) => match self.state {
                RadioState::TxIdle => {
                    self.state = RadioState::Tx;
                    self.set_event(EVENTS_ADDRESS, 1);
                    self.set_event(EVENTS_PAYLOAD, 1);
                    self.set_event(EVENTS_END, 1);
                    // Transmission starts; completion simulated separately
                }
                RadioState::RxIdle => {
                    self.state = RadioState::Rx;
                    self.set_event(EVENTS_ADDRESS, 1);
                    self.set_event(EVENTS_PAYLOAD, 1);
                    self.set_event(EVENTS_END, 1);
                    // Reception starts; completion simulated separately
                }
                _ => {}
            },
            (TASKS_STOP, 1) => match self.state {
                RadioState::Tx => self.state = RadioState::TxIdle,
                RadioState::Rx => self.state = RadioState::RxIdle,
                _ => {}
            },
            (TASKS_DISABLE, 1) => {
                self.state = RadioState::Disabled;
                // self.set_event(EVENTS_READY, 0);
                // self.set_event(EVENTS_END, 0);
                self.set_event(EVENTS_DISABLED, 1);
                // event_clear function will hadnler the set to 0
                // self.set_event(EVENTS_READY, 0);
                // self.set_event(EVENTS_END, 0);
            }
            _ => {}
        }
    
        // Clear EVENTS registers if writing 0 (assuming EVENTS range 0x100-0x1FF)
        if address >= 0x100 && address < 0x200 && value == 0 {
            self.set_event(address, 0);
        }
    }
    /// Sets an event and triggers shortcuts if enabled
    pub fn set_event(&mut self, event_address: u32, value: u32) {
        self.registers.insert(event_address, value);
        if value == 1 {
            self.handle_event(event_address);
        }
    }
    /// Checks the SHORTS register and triggers tasks for enabled shortcuts
    pub fn handle_event(&mut self, event_address: u32) {
        let shorts = self.read_register(SHORTS);
        if event_address == EVENTS_READY && (shorts & SHORTS_READY_START) != 0 {
            self.write_register(TASKS_START, 1); // Handles TXREADY_START, RXREADY_START
        }
        if event_address == EVENTS_END {
            if (shorts & SHORTS_END_DISABLE) != 0 {
                self.write_register(TASKS_DISABLE, 1);
            }
            if (shorts & SHORTS_END_START) != 0 {
                self.write_register(TASKS_START, 1);
            }
        }
        if event_address == EVENTS_DISABLED {
            if (shorts & SHORTS_DISABLED_TXEN) != 0 {
                self.write_register(TASKS_TXEN, 1);
            }
            if (shorts & SHORTS_DISABLED_RXEN) != 0 {
                self.write_register(TASKS_RXEN, 1);
            }
        }
        if event_address == EVENTS_PHYEND {
            if (shorts & SHORTS_PHYEND_DISABLE) != 0 {
                self.write_register(TASKS_DISABLE, 1);
            }
            if (shorts & SHORTS_PHYEND_START) != 0 {
                self.write_register(TASKS_START, 1);
            }
        }
    }
    pub fn simulate_tx_complete(&mut self) {
        if self.state == RadioState::Tx {
            self.state = RadioState::TxIdle;
            self.set_event(EVENTS_END, 1);
            self.set_event(EVENTS_PHYEND, 1); // Set PHYEND alongside END
        }
    }
    
    pub fn simulate_rx_complete(&mut self) {
        if self.state == RadioState::Rx {
            self.state = RadioState::RxIdle;
            self.set_event(EVENTS_END, 1);
            self.set_event(EVENTS_PHYEND, 1); // Set PHYEND alongside END
        }
    }
}


fn main() {
    let mut radio = Radio::new();
    // Enable Interrupt
    radio.write_register(INTENSET, 0x3402);
    radio.read_register(INTENSET);
    assert_eq!(radio.interrupts, 0x3402);
    // Disable Interrupt
    radio.write_register(INTENCLR, 0x2);
    // println!("interrupts {:b}",radio.interrupts);
    assert_eq!(radio.interrupts, 0x3400);


    // Enable TX mode
    radio.write_register(TASKS_TXEN, 1);
    assert_eq!(radio.read_register(EVENTS_READY), 1);
    assert_eq!(radio.state, RadioState::TxIdle);

    // Start transmission
    radio.write_register(TASKS_START, 1);
    assert_eq!(radio.state, RadioState::Tx);

    // Simulate transmission completion
    radio.simulate_tx_complete();
    assert_eq!(radio.read_register(EVENTS_END), 1);
    assert_eq!(radio.state, RadioState::TxIdle);

    // Clear the END event
    radio.write_register(EVENTS_END, 0);
    assert_eq!(radio.read_register(EVENTS_END), 0);

    // Disable the radio
    radio.write_register(TASKS_DISABLE, 1);
    assert_eq!(radio.state, RadioState::Disabled);
}
