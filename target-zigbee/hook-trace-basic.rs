pub fn main(api) {
    //  ------------ Hook evey interrupt and basic blocks ------------
    api.on_interrupt(None, None, |pc, isr| print_symbol_name(pc, true, isr));
    api.on_basic_block(None, |pc| print_symbol_name(pc, false, None));
    api.on_ram_read(None, None, |pc, address, size, value| log::info!("RAM read from {:08x}: value {:08x}", address, value));
    api.on_ram_write(None, None, |pc, address, size, value| log::info!("RAM write to {:08x} with value {:08x}", address, value));
    api.on_mmio_read(None, None, |pc, address, size, value| log::info!("PC: {:08x}, MMIO read from {:08x}: value {:08x}", pc, address, value));
    api.on_mmio_write(None, None, |pc, address, size, value| log::info!("PC: {:08x}, MMIO write to {:08x} with value {:08x}", pc, address, value));
  }

  fn print_symbol_name(pc, is_interrupt, isr_number) {
    match symbolizer::lookup(pc)
    {
      Ok(symbol_name) => {
        if (is_interrupt) {
          log::info!("0x{:08x}: [ISR:{}] {}", pc, isr_number, symbol_name);
        }
        else if (symbol_name != "memset" && symbol_name != "memcpy") {
          log::info!("0x{:08x}: {}", pc, symbol_name);
        }
      }
    }
  }