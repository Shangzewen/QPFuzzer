// ----------- UDP Socket Info -----------
// Python server: 127.0.0.1:7777
// Listen packet client: 127.0.0.1:9999
// Sending packet client: 127.0.0.1:8888 
struct State {
  sequence,
  data_connection,
  log_details,
  adv_ind_flag,
  initial_pdu_flag,
  pkt_length,
}
pub fn main(api) {
    //  ------------ Apply Firmware Patches ------------
    // we can only patch the certain memorries at apply_patch() function
    // since this function was called later after the qemu was initialised
    // if we do api.on_instru, it will be exucuted before qemu initialised, then we 
    // might encounter the memory region not found issue
    api.on_init(apply_patches);
    //  ------------ Hook Link Layer Packets ------------
    let cfg = State {
      sequence: 0,
      data_connection: false,
      log_details: false,
      adv_ind_flag: false,
      initial_pdu_flag:true,
      pkt_length: 0
    };

    hook_link_layer(api, cfg);

    //  ------------ Print Logs ------------
    // Exit Hoo1ks
    // api.on_instruction(Some(symbolizer::resolve("arch_system_halt")?), |_| log::info!("===========exit_hook reached arch_system_halt==========="));
    // api.on_instruction(Some(symbolizer::resolve("z_do_kernel_oops")?), |_| log::info!("===========exit_hook reached z_do_kernel_oops==========="));
    // api.on_instruction(Some(symbolizer::resolve("z_fatal_error")?), |_| log::info!("===========exit_hook reached z_fatal_error==========="));
    // api.on_instruction(Some(symbolizer::resolve("z_arm_fault")?), |_| log::info!("===========exit_hook reached z_arm_fault==========="));

    //ZBE Setup
    // api.on_instruction(Some(symbolizer::resolve("nrf5_init")?), |_| log::info!("===========nrf5_init=========="));
    // api.on_instruction(Some(symbolizer::resolve("net_init")?), |_| log::info!("===========net_init=========="));
    
    // api.on_instruction(Some(symbolizer::resolve("main")?), |_| log::info!("===========main=========="));
    // api.on_instruction(Some(symbolizer::resolve("zigbee_enable")?), |_| log::info!("===========zigbee_enable=========="));
    // api.on_instruction(Some(symbolizer::resolve("process_channel")?), |_| log::info!("===========process_channel=========="));
    // api.on_instruction(Some(symbolizer::resolve("timer_handler")?), |_| log::info!("===========timer_handler=========="));
    api.on_instruction(Some(symbolizer::resolve("frame_transmit")?), |_| log::info!("===========frame_transmit=========="));
    api.on_instruction(Some(symbolizer::resolve("nrf_802154_core_transmit")?), |_| log::info!("===========nrf_802154_core_transmit=========="));
    api.on_instruction(Some(symbolizer::resolve("tx_init")?), |_| log::info!("===========tx_init=========="));
    api.on_instruction(Some(symbolizer::resolve("nrf_802154_trx_transmit_frame")?), |_| log::info!("===========nrf_802154_trx_transmit_frame=========="));
    api.on_instruction(Some(symbolizer::resolve("rx_init")?), |_| log::info!("===========rx_init=========="));
    api.on_instruction(Some(symbolizer::resolve("nrf_raal_init")?), |_| log::info!("===========nrf_raal_init=========="));
    
    //  return 1 for timeslot_is_granted only when tx_init was called
    // api.on_instruction(Some(symbolizer::resolve("tx_init")?), |_| memory::write_u8(0x20008ae7, 1));
    api.on_instruction(Some(0x0001caf0), |_| log::info!("===========Am I reaching here??=========="));
    api.on_instruction(Some(symbolizer::resolve("irq_handler_sync")?), |_| log::info!("===========Am I reaching here??=========="));
    // api.on_instruction(Some(0x1d982), |_| register::write("r3",0x0)?);
    // api.on_instruction(Some(0x1d982), |_| register::read("r3")?);
    
    // api.on_instruction(Some(0x26106), |_| log::info!("===========2bdb_network_steering_start_scan=========="));
    // api.on_instruction(Some(0x2628a), |_| log::info!("===========bdb_network_steering_not_on_network=========="));
    // api.on_instruction(Some(0x26226), |_| log::info!("===========bdb_network_steering_start_scan=========="));


    // api.on_instruction(Some(symbolizer::resolve("zb_mlme_scan_step")?), |_| log::info!("===========zb_mlme_scan_step=========="));
    // api.on_instruction(Some(symbolizer::resolve("zb_mac_send_beacon_request_command")?), |_| log::info!("===========zb_mac_send_beacon_request_command=========="));

    // api.on_instruction(Some(0x1baa4), |_| log::info!("===========nrf_egu_task_trigger==========="));
    // api.on_instruction(Some(symbolizer::resolve("nrf_egu_task_trigger")?), |_| register::read("pc")?);
   
    // force to start steering at the begining
    // api.on_instruction(Some(0x000266a2), |_| register::write("r3",0x01)?);
    // api.on_instruction(Some(0x000266a2), |_| register::read("r3")?);
    // api.on_instruction(Some(0x000266a2), |_| log::info!("2"));
    // common::patch_address(0x000266a2, [0x01, 0x2b]);
    //  patched the timeslot_is_granted() function to return 1
    // api.on_instruction(Some(0x0001911c), |_| register::write("r3",0x01)?);
    // api.on_instruction(Some(0x0001911c), |_| register::read("r3")?);
    //  patched the nrf_raal_time_slot_request() function to return 1
    // api.on_instruction(Some(0x00003ce0), |_| register::write("r0",0x00)?);
    // api.on_instruction(Some(0x00003ce0), |_| register::read("r0")?);
    // api.on_instruction(Some(0x0000233e), |_| register::write("r0",0x0)?);
    // api.on_instruction(Some(0x0001911c), |_| register::read("r0")?);
    // patch prority function mpsl
    api.on_instruction(Some(0x000008c0), |_| memory::write_u16(0x20000f68,0x0569)?);
    // test irq number 1
    // api.on_instruction(Some(0xa30e), |_| register::write("r3",0x01)?);
    // api.on_instruction(Some(0x0001911c), |_| register::read("r3")?);
  }
  fn print_symbol_name(pc, is_interrupt, isr_number) {
    match symbolizer::lookup(pc)
    {
      Ok(symbol_name) => {
        if (is_interrupt) {
          log::info!("0x{:08x}: [ISR:0x{:02X}] {}", pc, isr_number, symbol_name);
        }
        else if (symbol_name != "memset" && symbol_name != "memcpy") {
          log::info!("0x{:08x}: {}", pc, symbol_name);
        }
      }
    }
  }

  fn hook_link_layer(api, cfg){
      // Reset state before every run
      api.on_prepare_run(||{
        cfg.sequence = 0;
        // cfg.adv_ind_flag = false;
        // cfg.initial_pdu_flag = true;
        // common::clear_tx_data();
        // Normal case need to sart the transmission at here
        // common::send_socket_data("hello");
        // enable the data_connection_flag for fuzzing from scrach
        // cfg.data_connection = false;
      });
    
      // TX
      // need to +1 for the address in r0 to shift right for one byte to get the real data
      api.on_instruction(Some(symbolizer::resolve("tx_init")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 1));
      // RX
      // api.on_instruction(Some(symbolizer::resolve("radio_pkt_rx_set")?), 
      //           |_| handle_link_layer_packet(cfg, register::read("r0")?, 0));
      
  }
  fn handle_link_layer_packet(cfg, pkt_buf_addr, direction) {
      // let pkt_hex = "";

    if (direction == 1) {
      if cfg.log_details {
        log::info!("<============> TX PKT <============>");
      }
      let pkt_hdr = memory::read_u8(pkt_buf_addr)?;
      // need to find the correct pkt_length
      let pdu_length = memory::read_u8(pkt_buf_addr)?;
      if (pdu_length == 0 && pkt_hdr == 0) {return;}
      if (pdu_length == 0)
      {
        cfg.pkt_length = pdu_length + 3;
      }
      else{
        cfg.pkt_length = pdu_length + 1;
      }
      // println!("This is pkt_length {}", cfg.pkt_length);
      // if (cfg.pkt_length > 20){
      //   cfg.data_connection = false;
      //   log::info!("Received adv packet!");
      // }else{
      //   cfg.data_connection = true;
      //   log::info!("Received data packet!");

      // }
      let pkt_data = memory_read_buffer(pkt_buf_addr, cfg.pkt_length);
      let pkt_hex = common::encode_hex(pkt_data);
      log::info!(" TX Pkt. Bytes: {}", pkt_hex);
    }
  }


  // fn handle_link_layer_packet(cfg, pkt_buf_addr, direction) {
  //   // let pkt_hex = "";

  //   if (direction == 1) {
  //     if cfg.log_details {
  //       log::info!("<============> TX PKT <============>");
  //     }
  //     let pkt_hdr = memory::read_u8(pkt_buf_addr)?;
  //     let pdu_length = memory::read_u8(pkt_buf_addr+1)?;
  //     if (pdu_length == 0 && pkt_hdr == 0) {return;}
  //     if (pdu_length == 0)
  //     {
  //       cfg.pkt_length = pdu_length + 3;
  //     }
  //     else{
  //       cfg.pkt_length = pdu_length + 1;
  //     }
  //     // println!("This is pkt_length {}", cfg.pkt_length);
  //     // if (cfg.pkt_length > 20){
  //     //   cfg.data_connection = false;
  //     //   log::info!("Received adv packet!");
  //     // }else{
  //     //   cfg.data_connection = true;
  //     //   log::info!("Received data packet!");

  //     // }
  //     let pkt_data = memory_read_buffer(pkt_buf_addr, cfg.pkt_length);
  //     let pkt_hex = common::encode_hex(pkt_data);
  //     log::info!(" TX Pkt. Bytes: {}", pkt_hex);

  //     let pkt_summary = common::parse_packet("ble", pkt_hex, direction, !cfg.initial_pdu_flag, cfg.log_details);
  //     if pkt_summary.contains("BTLE_ADV_IND") {
  //       // log::info!("BTLE_ADV_IND !!!! {}", pkt_summary);
  //       if cfg.adv_ind_flag == false {
  //         cfg.adv_ind_flag = true;
  //         common::update_tx_data(pkt_hex);
  //       }
  //       else{
  //         return
  //       }
  //     }
  //     else{
  //       common::update_tx_data(pkt_hex);
  //     }
  //     if cfg.log_details {
  //       log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
  //       log::info!("Pkt. Length: {}", cfg.pkt_length);
  //       log::info!("Pkt. Bytes: {}", pkt_hex);
  //       log::info!("Pkt. Bytes: {}", pkt_hex);

  //     }

  //     log::info!("TX ---> {}", pkt_summary);
  //   }
  //   else {
  //     if cfg.log_details {
  //       log::info!("<============> RX PKT <============>");
  //     }
  //     let rx_pdu = "";
  //     let test_str = "";

  //     // ADV Channel
  //     if cfg.data_connection == false {
  //       if cfg.sequence == 0 {
  //         // Scan Request
  //         // rx_pdu = common::get_adv_rpl_data();
  //         rx_pdu = common:: get_rx_data();
  //         log::info!("RX adv: {}", rx_pdu);
  //       }
  //       else if cfg.sequence >= 1 {
  //         log::info!("RX adv: {}", rx_pdu);

  //         // rx_pdu = common::get_adv_rpl_data();
  //         rx_pdu = common:: get_rx_data();

  //         cfg.data_connection = true; // Switch to data channel
  //       }
  //       else {
  //         cfg.sequence = cfg.sequence + 1;
  //         return;
  //       }
  //     }
  //     else {
  //       // set initial flag
  //       if cfg.initial_pdu_flag == true{
  //         cfg.initial_pdu_flag = false;
  //         // rx_pdu = common::get_empty_pdu_data();
  //         rx_pdu = common:: get_rx_data();

  //         // log::info!("<============> initial_empty_pdu received <============>");
  //       }
  //       else{
  //         // rx_pdu = common::get_data_rpl_data();
  //         rx_pdu = common:: get_rx_data();

  //         log::info!("RX data pdu: {}", rx_pdu);
  //         //  log::info!("<============> rx_pdu received <============>");
  //       }
  //     }

  //     cfg.sequence = cfg.sequence + 1;
      
  //     let data = common::decode_hex(rx_pdu)?;
  //     for (i, v) in data.iter().enumerate() {
  //       memory::write_u8(pkt_buf_addr + i, v);
  //     }

  //     let pkt_summary = common::parse_packet("ble", rx_pdu, direction, !cfg.initial_pdu_flag, cfg.log_details);
  //     log::info!("RX <--- {}", pkt_summary);

  //     if cfg.log_details {
  //         log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
  //         log::info!("Pkt. Length: {}", data[1]);
  //         log::info!("Pkt. Bytes: {}", rx_pdu);
  //       }
  //   }
  // }

  // // fn print_symbol_name(pc) {
  // //   if let Ok(symbol_name) = symbolizer::lookup(pc) {
  // //     log::info!("0x{:08x}: {}", pc, symbol_name);
  // //   }
  // // }

  fn memory_read_buffer(addr, length) {
    let pkt_data = [];
      for idx in 0..length {
        pkt_data.push(memory::read_u8(addr+idx)?);
      }
      return pkt_data;
  }

  // fn parse_ble_packet(pdu_hex_string, direction) {
  //   return common::system(format!("python3 scripts/parse-ble-pdu.py {} {}",
  //                                 pdu_hex_string, direction));
  // }

  // fn get_data(ddata){

  //   return ddata;
  // }

  fn apply_patches() {
    common::patch_function("z_tick_sleep", arm::RETURN);
    // common::patch_function("mpsl_init", arm::RETURN_0);
    // common::patch_function("mpsl_fem_init", arm::RETURN_0);
    // common::patch_function("mpsl_lib_init_internal", arm::RETURN_0);
    common::patch_function("CC_PalMutexCreate", arm::RETURN_0);
    common::patch_function("CC_PalPowerSaveModeSelect", arm::RETURN_0);
    common::patch_function("cc_mbedtls_platform_zeroize", arm::RETURN);
    common::patch_function("mbedtls_zeroize_internal", arm::RETURN);
    common::patch_function("nrf_cc3xx_platform_init", arm::RETURN_0);
    common::patch_function("nrf_gpio_pin_present_check", arm::RETURN_1);
    common::patch_function("z_impl_device_is_ready", arm::RETURN_1);
    common::patch_function("z_impl_entropy_get_entropy", arm::RETURN_0);
    common::patch_function("delay_machine_code.0", arm::RETURN_1);
    common::patch_function("arch_cpu_atomic_idle", arm::RETURN);
    common::patch_function("nrfy_pwm_int_init", arm::RETURN);
    common::patch_function("pinctrl_configure_pins", arm::RETURN_0);
    common::patch_function("nrf_clock_is_running", arm::RETURN_1);
    common::patch_function("z_nrf_clock_control_lf_on", arm::RETURN);
    common::patch_function("pinctrl_apply_state", arm::RETURN_0);
    common::patch_function("pm_device_driver_init", arm::RETURN_0);
    common::patch_function("is_tx_ready", arm::RETURN_1);
    // common::patch_function("set_cc", arm::RETURN_0);
    common::patch_function("log_output_process", arm::RETURN);
    common::patch_function("nrf_event_readback", arm::RETURN);
    common::patch_function("zb_trace_msg_port_vl", arm::RETURN_0);
    common::patch_function("zb_schedule_alarm", arm::RETURN_0);
    common::patch_function("nrf_uarte_event_check", arm::RETURN_1);
    common::patch_function("temp_nrf5_mpsl_sample_fetch", arm::RETURN_0);
    common::patch_function("light_bulb_set_brightness", arm::RETURN);
    // -----------------------------------------------------------
    common::patch_function("nrf_egu_event_check", arm::RETURN_1);
    common::patch_function("nrf_egu_int_enable_check", arm::RETURN_1);
    common::patch_function("mpsl_clock_hfclk_request", arm::RETURN_0);
    common::patch_function("rand_get", arm::RETURN_0);
    common::patch_function("zb_random_seed", arm::RETURN_2);
    common::patch_function("nrf_timer_event_check", arm::RETURN_1);
    common::patch_function("timeslot_is_granted", arm::RETURN_1);
    common::patch_function("remaining_timeslot_time_is_enough_for_crit_sect", arm::RETURN_1);
    // common::patch_function("sym_NFDFVOR5BUFND4TNTGYIYR4ARXJRXWSQ4PVFUKY", arm::RETURN_1);
    
    // common::patch_function("sym_FYHKZOVAJN6VDDHY43FT7PF4YLRDHWRTEWHFG6I", arm::RETURN_0);
    // common::patch_function("receive_frame_abort", arm::RETURN);
    // common::patch_function("zb_osif_disable_all_inter", arm::RETURN);
    // common::patch_function("sym_DCDRLVBT43ANSLX3KNDZ4TST3Z3CVWXAQQUSXQQ", arm::RETURN);
    // common::patch_function("sym_S2UAPMFVIQXDUOA6CV7GJMB33TYHEUH5D6LHO5Q", arm::RETURN);


    // force sym_4PX37LW4KIUYQZ73JWLPH5GAGIRWAKTV3E6F62Q to return 0
    // common::patch_address(0x0000233e, [0x01, 0x28]);
    
    
    // force the loop to enter steering 
    common::patch_address(0x000266a2, [0x01, 0x2b]);
    // force to branch to radio_irq_handler
    // common::patch_address(0x0000237c, [0x1b,0xf0,0xf8,0xfa]);
    //force the nef_802154_radio_irq_handler() first condition to return false for nre)egu_int_able_check
    // common::patch_address(0x0001d982, [0x01, 0x2b]);
    // patch the timer check to froce the meulation think the timeslot time left 
    // is enough for the transmiation
    common::patch_function("nrf_raal_timeslot_request", arm::RETURN_1);
    // force to make sure the current prority is high enough
    common::patch_function("is_state_allowed_for_prio", arm::RETURN_1);
    // -----------------------------------------------------------


    // --------------------------TODO
    common::patch_function("zb_nvram_load", arm::RETURN);
    common::patch_function("zb_nvram_write_dataset", arm::RETURN);
    common::patch_function("zb_nvram_dataset_is_supported", arm::RETURN_0);
    common::patch_function("nrf_802154_random_init", arm::RETURN_0);
    common::patch_function("sym_76IVKPQMZOZ7IXNJTGMWOSIWIVCASAX3TTNHN7I", arm::RETURN_0);
    common::patch_function("rng_pool_get", arm::RETURN_1);
    // common::patch_function("nrf_802154_queue_is_empty", arm::RETURN_0);



    // common::patch_function("active_vector_priority_is_high", arm::RETURN_1);

    common::patch_address(0x0000b732, arm::NOP);
    common::patch_address(0x0000c612, arm::NOP);
    common::patch_address(0x0000ca4a, arm::NOP);
    common::patch_address(0x00016d6a, arm::NOP);
    common::patch_address(0x0001756a, arm::NOP);
    common::patch_address(0x00017b3a, arm::NOP);
    
    common::patch_function("qspi_nor_init", arm::RETURN_0);
    common::patch_function("settings_subsys_init", arm::RETURN_0);
    common::patch_function("nrfx_gpiote_0_irq_handler", arm::RETURN);

    // common::patch_function("lll_preempt_calc", arm::RETURN_0); // **

    // common::patch_function("isr_race", arm::RETURN);
  
    // Fix memcmp on scan_req addr check
    // brach to _swi_irq_handler 0x00092f7a
    common::patch_address(0x0001bac0, [0x77, 0xf0, 0x5b, 0xfa]);
    common::patch_address(0x0001bac8, [0x00, 0xbf]);
    
    // Branch to radio handler
    // Force zb_zdo_joined return 0
    // common::patch_address(0x24efe, arm::NOP);

    // common::patch_address(0x0001bac4, [0x48,0x47]);
    // // Fix memcmp on adv_ind addr check
    // common::patch_address(0x0001b760, [0x4f, 0xf0, 0x20, 0x00]);
    
  }