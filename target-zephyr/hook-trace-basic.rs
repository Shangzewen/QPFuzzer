pub fn main(api) {
    //  ------------ Hook evey interrupt and basic blocks ------------
    api.on_interrupt(None, None, |pc, isr| print_symbol_name(pc, true, isr));
    api.on_basic_block(None, |pc| print_symbol_name(pc, false, None));
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