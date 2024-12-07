struct State {
  tick,
  sequence,
  data_connection,
  log_details,
  log_packet,
  sn,
  nesn,
}

pub fn main(api) {
    //  ------------ Apply Firmware Patches ------------
    api.on_init(apply_patches);

    //  ------------ Hook Link Layer Packets ------------
    hook_link_layer(api);

    //  ------------ Print Logs ------------
    // BLE Setup
    // api.on_instruction(Some(symbolizer::resolve("bt_enable")?), |_| log::info!("===========bt_enable=========="));
    // api.on_instruction(Some(symbolizer::resolve("cts_init")?), |_| log::info!("===========cts_init=========="));
    // api.on_instruction(Some(symbolizer::resolve("settings_load")?), |_| log::info!("===========settings_load=========="));
    // api.on_instruction(Some(symbolizer::resolve("bt_le_adv_start")?), |_| log::info!("===========bt_le_adv_start=========="));

    // BLE HCI
    // api.on_instruction(Some(0x6b94), |_| log::info!("--> bt_hci_cmd_send_sync"));
    // api.on_instruction(Some(0x7498), |_| log::info!("--> hci_tx_thread->bt_send"));
    // api.on_instruction(Some(0x74a6), |_| log::info!("--> hci_tx_thread->bt_send FAIL"));
    
    // BLE Periodic Timers
    // api.on_instruction(Some(symbolizer::resolve("ticker_start_ext")?), |_| log::info!("===========ticker_start_ext==========="));
    // api.on_instruction(Some(symbolizer::resolve("ticker_trigger")?), |_| log::info!("===========ticker_trigger==========="));
    // api.on_instruction(Some(symbolizer::resolve("rtc0_nrf5_isr")?), |_| log::info!("===========rtc0_nrf5_isr==========="));
    // api.on_instruction(Some(symbolizer::resolve("ticker_cb")?), |_| log::info!("===========ticker_cb==========="));

    // BLE Advertisement
    // api.on_instruction(Some(symbolizer::resolve("lll_adv_prepare")?), |_| log::info!("===========lll_adv_prepare==========="));
    
    // BLE Interrupts
    // api.on_instruction(Some(symbolizer::resolve("radio_isr_set")?), |_| log::info!("===========radio_isr_set==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_radio")?), |_| log::info!("===========isr_radio==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_rx")?), |_| log::info!("===========isr_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_tx")?), |_| log::info!("===========isr_tx==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_rx")?), |_| log::info!("===========lll_conn_isr_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_tx")?), |_| log::info!("===========lll_conn_isr_tx==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_done")?), |_| log::info!("===========isr_done==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_race")?), |_| log::info!("===========isr_race==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_abort")?), |_| log::info!("===========isr_abort==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_abort_all")?), |_| log::info!("===========isr_abort_all==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_isr_early_abort")?), |_| log::info!("===========lll_isr_early_abort==========="));
    
    // BLE Link Layer
    // api.on_instruction(Some(symbolizer::resolve("radio_pkt_tx_set")?), |_| log::info!("===========radio_pkt_tx_set==========="));
    // api.on_instruction(Some(symbolizer::resolve("radio_pkt_rx_set")?), |_| log::info!("===========radio_pkt_rx_set==========="));
  
   // ACK
    // api.on_instruction(Some(symbolizer::resolve("ull_cp_tx_ack")?), |_| log::info!("===========ull_cp_tx_ack==========="));
    // api.on_instruction(Some(symbolizer::resolve("ull_cp_rx")?), |_| log::info!("===========ull_cp_rx==========="));
    


    // api.on_instruction(Some(symbolizer::resolve("isr_done")?), |_| log::info!("===========ull_cp_rx==========="));

    
    
    // api.on_instruction(Some(symbolizer::resolve("ull_periph_ticker_cb")?), |_| log::info!("===========ull_periph_ticker_cb==========="));
    // api.on_instruction(Some(symbolizer::resolve("ull_conn_llcp")?), |_| log::info!("===========ull_conn_llcp==========="));

    

    // api.on_instruction(Some(symbolizer::resolve("ull_conn_rx")?), |_| log::info!("===========ull_conn_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("llcp_rr_rx")?), |_| log::info!("===========llcp_rr_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("llcp_lr_rx")?), |_| log::info!("===========llcp_lr_rxllcp_lr_rx==========="));
    // // api.on_instruction(Some(symbolizer::resolve("pdu_validate_version_ind")?), |_| log::info!("===========pdu_validate_version_ind==========="));
    // // api.on_instruction(Some(symbolizer::resolve("pdu_validate_feature_req")?), |_| log::info!("===========pdu_validate_feature_req==========="));
    // api.on_instruction(Some(symbolizer::resolve("ll_rx_get")?), |_| log::info!("===========ll_rx_get==========="));
    // api.on_instruction(Some(symbolizer::resolve("ll_rx_dequeue")?), |_| log::info!("===========ll_rx_get==========="));    
    // api.on_instruction(Some(symbolizer::resolve("ull_pdu_rx_alloc")?), |_| log::info!("===========ull_pdu_rx_alloc==========="));
    // api.on_instruction(Some(symbolizer::resolve("rx_demux")?), |_| log::info!("===========rx_demux==========="));
    // api.on_instruction(Some(symbolizer::resolve("ll_rx_put_sched")?), |_| log::info!("===========ll_rx_put_sched==========="));
    
    
    // api.on_instruction(Some(symbolizer::resolve("ll_rx_release")?), |_| log::info!("===========ll_rx_release==========="));
    // api.on_instruction(Some(symbolizer::resolve("ll_rx_mem_release")?), |_| log::info!("===========ll_rx_mem_release==========="));
    // api.on_instruction(Some(symbolizer::resolve("bt_buf_get_evt")?), |_| log::info!("===========bt_buf_get_evt==========="));
    // api.on_instruction(Some(symbolizer::resolve("hci_num_cmplt_encode")?), |_| log::info!("===========hci_num_cmplt_encode==========="));
    // api.on_instruction(Some(symbolizer::resolve("bt_recv_prio")?), |_| log::info!("===========bt_recv_prio==========="));
    
    // api.on_instruction(Some(symbolizer::resolve("radio_nrf5_isr")?), |_| log::info!("===========radio_nrf5_isr==========="));
    // api.on_instruction(Some(symbolizer::resolve("z_irq_spurious")?), |_| log::info!("===========z_irq_spurious==========="));
    // api.on_instruction(Some(symbolizer::resolve("_isr_wrapper")?), |_| log::info!("===========_isr_wrapper==========="));

    // api.on_instruction(Some(symbolizer::resolve("rtc0_nrf5_isr")?), |_| log::info!("===========rtc0_nrf5_isr==========="));
    // api.on_instruction(Some(symbolizer::resolve("ticker_trigger")?), |_| log::info!("===========ticker_trigger==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_rx")?), |_| log::info!("===========lll_conn_isr_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("ull_rx_put")?), |_| log::info!("===========ull_rx_put==========="));
    // api.on_instruction(Some(symbolizer::resolve("ull_rx_sched")?), |_| log::info!("===========ull_rx_sched==========="));

   

    // api.on_instruction(Some(symbolizer::resolve("cntr_cnt_get")?), |_| common::patch_function("cntr_cnt_get", arm::RETURN_(1000))?);

    
    
    
    // Debug
    // api.on_interrupt(None, None, |pc, isr| print_symbol_name(pc, true, isr));
  }

  fn apply_patches() {
    common::patch_function("arch_system_halt", arm::RETURN);
    common::patch_function("log_0", arm::RETURN);
    common::patch_function("log_1", arm::RETURN);
    common::patch_function("log_2", arm::RETURN);
    common::patch_function("log_3", arm::RETURN);
    common::patch_function("log_n", arm::RETURN);
    common::patch_function("printk", arm::RETURN);
    common::patch_function("vfprintf", arm::RETURN);
    common::patch_function("print_formatted", arm::RETURN);    
    common::patch_function("z_vprintk", arm::RETURN);
    common::patch_function("z_log_vprintk", arm::RETURN);
    common::patch_function("z_impl_k_busy_wait", arm::RETURN);
    common::patch_function("z_impl_k_sleep", arm::RETURN);
    common::patch_function("z_arm_exc_exit", arm::RETURN);
    common::patch_function("qspi_nor_init", arm::RETURN);
    common::patch_function("settings_subsys_init", arm::RETURN);
    common::patch_function("nrfx_gpiote_0_irq_handler", arm::RETURN);
  
  
    common::patch_function("rng_pool_get", arm::RETURN_1);
    common::patch_function("settings_save_one", arm::RETURN_0);
    common::patch_function("bt_read_static_addr", arm::RETURN_0);
    common::patch_function("radio_is_ready", arm::RETURN_0);
  
    // Force ticker_trigger within rtc0_nrf5_isr
    // common::patch_address(0x1a408, arm::MOV(2,1));
  
    // Fix isr_radio - Check if isr_cb is not NULL
    common::patch_address(0x1c380, [0x04, 0x34, 0x00, 0x20]);
  
    // Force RX to be successfull (Optional it seems)
    common::patch_function("radio_has_disabled", arm::RETURN_1);
    common::patch_function("radio_is_done", arm::RETURN_1);
    common::patch_function("radio_crc_is_valid", arm::RETURN_1);
    common::patch_function("radio_rssi_is_ready", arm::RETURN_1);
    common::patch_function("radio_df_cte_ready", arm::RETURN_0);
    common::patch_function("radio_filter_has_match", arm::RETURN_0);
    common::patch_function("radio_filter_match_get", arm::RETURN_0);
    common::patch_function("radio_ar_has_match", arm::RETURN_0);
    common::patch_function("radio_ar_match_get", arm::RETURN_0);
    common::patch_function("radio_tmr_aa_restore", arm::RETURN_0);
    

    common::patch_function("lll_preempt_calc", arm::RETURN_0); // **

    common::patch_function("isr_race", arm::RETURN);
  
    // Fix memcmp on scan_req addr check
    common::patch_address(0x0001b684, [0x4f, 0xf0, 0x20, 0x00]);
    // Fix memcmp on adv_ind addr check
    common::patch_address(0x0001b760, [0x4f, 0xf0, 0x20, 0x00]);

    // Fix upper buffer boundary
    // common::patch_address(0x0001c016, [0x00, 0x20]);
    // common::patch_address(0x0001c016, [0x01, 0x20]); // *
    // common::patch_address(0x1c01e, arm::NOP); // *
    // common::patch_address(0x0001c032, arm::NOP); 

    // Force pdu_data_rx->nesn != lll->sn
    // common::patch_address(0x0001be64, arm::NOP);
    // Force pdu_data_rx->sn == lll->nesn
    // common::patch_address(0x0001bfa0, arm::NOP);
    
    // Fix upper buffer boundary 2
    // common::patch_address(0x1bda8, [0x00, 0x20]);
    
    
    // common::patch_address(0x0001c016, [0x02, 0x20]);
    // common::patch_address(0x0001c016, [0x03, 0x20]);

    // common::patch_address(0x0001bf12, [0x01, 0x26]);
    // common::patch_address(0x0001bdd4, [0x01, 0x26]);

    
    // common::patch_address(0x11a08, [0x4f,0xf0,0x01,0x01]); // R1

    // common::patch_address(0x11a0e, [0x00, 0x46, 0x00, 0x46]);
  }

  fn hook_link_layer(api){
      let cfg = State {
        tick:0,
        sequence: 0,
        data_connection: false,
        log_details: false,
        log_packet: true,
        sn: 0,
        nesn: 0,
      };

      // Reset state before every run
      api.on_prepare_run(||{
        cfg.sequence = 0;
        cfg.data_connection = false;
        cfg.sn = 0;
        cfg.nesn = 0;
      });
    
      // TX
      api.on_instruction(Some(symbolizer::resolve("radio_pkt_tx_set")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 1));
      // RX
      api.on_instruction(Some(symbolizer::resolve("radio_pkt_rx_set")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 0));
      
      // Clock control (32.768Khz)
      // api.on_instruction(Some(symbolizer::resolve("cntr_cnt_get")?), |_| log::info!("===========cntr_cnt_get==========="));
      // api.on_instruction(Some(0x1c2be), |_| {
      //   cfg.tick += 33333; // 1ms
      //   register::write("R0", cfg.tick)?;
      // });
  }


  fn handle_link_layer_packet(cfg, pkt_buf_addr, direction) {

    if (direction == 1) {
      if cfg.log_details {
        log::info!("<============> TX PKT <============>");
      }
      let pkt_hdr = memory::read_u8(pkt_buf_addr)?;
      let pdu_length = memory::read_u8(pkt_buf_addr+1)?;
      if (pdu_length == 0 && pkt_hdr == 0) {return;}
      let pkt_length = pdu_length + 3;
      
      let pkt_data = memory_read_buffer(pkt_buf_addr, pkt_length);
      // Extract ACK bits
      let header = pkt_data[0];
      let nesn = (header & 0b100) >> 2;
      let sn = (header & 0b1000) >> 3;

      // Update stored ACK bits
      if cfg.data_connection {
        if sn == cfg.nesn {
          cfg.nesn = (cfg.nesn + 1) % 2;
        }
  
        if nesn != cfg.sn {
          cfg.sn = (cfg.sn + 1) % 2;
        }
      }

      

      let pkt_hex = common::encode_hex(pkt_data);

      if cfg.log_details {
        log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
        log::info!("Pkt. Length: {}", pkt_length);
        log::info!("Pkt. Bytes: {}", pkt_hex);
      }

      if cfg.log_packet {
        let pkt_summary = parse_ble_packet(pkt_hex, direction, cfg.data_connection);
        log::info!("TX [SN:{}, NESN:{}] ---> {}", sn, nesn, pkt_summary);
        // log::info!("cfg.sn={}, cfg.nesn={}, header={:02X}", cfg.sn, cfg.nesn, header);
      }
      
    }
    else {
      if cfg.log_details {
        log::info!("<============> RX PKT <============>");
      }

      let rx_pdu = "";
      match cfg.sequence {
        0 => rx_pdu = "830cf37a7d65de2800000000000c2aba95",
        1 => rx_pdu = "8522a942f80f51c300000000000c7083329a9c9a17020100100000006400ffffffff1f05002939",
        2 => {
          log::info!("-------------- ANCHOR POINT -------------");
          cfg.data_connection = true;
          rx_pdu = "0900";
          },
        3 => rx_pdu = "030900082100000000000000", // LL_FEATURE_REQ
        // 6 => rx_pdu = "0306000c0800000000", // LL_VERSION_IND
        6 => rx_pdu = "03090014fb004808fb004808", // LL_LENGTH_REQ
        _ => rx_pdu = "0900",
      }

      cfg.sequence = cfg.sequence + 1;
      
      let pkt_data = common::decode_hex(rx_pdu)?;

      
      if cfg.data_connection {
        // Update ACK bits
        let header = pkt_data[0];
        header = (header & 0b11111011) | (cfg.nesn << 2);
        header = (header & 0b11110111) | (cfg.sn << 3);
        pkt_data[0] = header;
        
      }
    
      // Write Link Layer Payload into firmware memory
      for (i, v) in pkt_data.iter().enumerate() {
        memory::write_u8(pkt_buf_addr + i, v);
      }

      if cfg.log_details {
          log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
          log::info!("Pkt. Length: {}", pkt_data[1]);
          log::info!("Pkt. Bytes: {}", rx_pdu);
      }

      // Print Packet Summary  
      if cfg.log_packet {
        let pkt_summary = parse_ble_packet(rx_pdu, direction, cfg.data_connection);
        let nesn = (pkt_data[0] & 0b100) >> 2;
        let sn = (pkt_data[0] & 0b1000) >> 3;
        log::info!("RX [SN:{}, NESN:{}] <--- {}", sn, nesn, pkt_summary);
      }
    }
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

  fn memory_read_buffer(addr, length) {
    let pkt_data = [];
      for idx in 0..length {
        pkt_data.push(memory::read_u8(addr+idx)?);
      }
      return pkt_data;
  }

  fn parse_ble_packet(pdu_hex_string, direction, data_channel) {
    return common::system(format!("python3 scripts/parse-ble-pdu.py {} {} {}",
                                  pdu_hex_string, direction, data_channel));
  }
 
