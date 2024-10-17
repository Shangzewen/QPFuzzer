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
}

pub fn main(api) {
    //  ------------ Apply Firmware Patches ------------
    api.on_init(apply_patches);
    let received_pkt="";

    //  ------------ Test udp Socket ---------------
    api.on_instruction(Some(symbolizer::resolve("bt_enable")?), |_| common::running_socket_background(received_pkt));
    // api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_rx")?), |_| log::info!("===========lll_conn_isr_rx==========="));

    //  ------------ Hook Link Layer Packets ------------
    let cfg = State {
              sequence: 0,
              data_connection: false,
              log_details: false,
              adv_ind_flag: false,
              initial_pdu_flag:true
            };

    hook_link_layer(api, cfg);

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
    
    // api.on_instruction(Some(symbolizer::resolve("rx_demux_rx")?), |_| log::info!("===========rx_demux_rx==========="));


    api.on_instruction(Some(symbolizer::resolve("ull_cp_rx")?), |_| log::info!("===========ull_cp_rx==========="));

    api.on_instruction(Some(symbolizer::resolve("ull_conn_rx")?), |_| log::info!("===========ull_conn_rx==========="));

    api.on_instruction(Some(symbolizer::resolve("pdu_validate_version_ind")?), |_| log::info!("===========pdu_validate_version_ind==========="));

    
    api.on_instruction(Some(symbolizer::resolve("llcp_rr_rx")?), |_| log::info!("===========llcp_rr_rx==========="));
    
    api.on_instruction(Some(symbolizer::resolve("llcp_lr_rx")?), |_| log::info!("===========llcp_lr_rxllcp_lr_rx==========="));


    
    
    api.on_instruction(Some(symbolizer::resolve("llcp_pdu_decode_version_ind")?), |_| log::info!("===========llcp_pdu_decode_version_ind==========="));
    api.on_instruction(Some(symbolizer::resolve("llcp_pdu_encode_version_ind")?), |_| log::info!("===========llcp_pdu_encode_version_ind==========="));

    api.on_instruction(Some(symbolizer::resolve("ull_rx_sched")?), |_| log::info!("===========ull_rx_sched==========="));
    // api.on_instruction(Some(0x00016632), |_| log::info!("--> enter condition!!!"));
    // api.on_instruction(Some(0x00016624), |_| memory::write_u8(0x20002410,1)?);
    // api.on_instruction(Some(0x0001662e), |_| memory::read_u8(0x20002410)?);
    api.on_instruction(Some(symbolizer::resolve("rp_comm_tx_proxy")?), |_| log::info!("===========rp_comm_tx_proxy==========="));
    api.on_instruction(Some(symbolizer::resolve("llcp_tx_enqueue")?), |_| log::info!("===========llcp_tx_enqueue==========="));
    api.on_instruction(Some(symbolizer::resolve("ull_tx_q_enqueue_ctrl")?), |_| log::info!("===========ull_tx_q_enqueue_ctrl==========="));
    // api.on_instruction(Some(), |_| log::info!("===========rp_comm_send_rsp==========="));
    
    // api.on_instruction(Some(symbolizer::resolve("isr_tx")?), |_| log::info!("===========isr_tx==========="));
    api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_rx")?), |_| log::info!("===========lll_conn_isr_rx==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_conn_isr_tx")?), |_| log::info!("===========lll_conn_isr_tx==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_done")?), |_| log::info!("===========isr_done==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_race")?), |_| log::info!("===========isr_race==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_abort")?), |_| log::info!("===========isr_abort==========="));
    // api.on_instruction(Some(symbolizer::resolve("isr_abort_all")?), |_| log::info!("===========isr_abort_all==========="));
    // api.on_instruction(Some(symbolizer::resolve("lll_isr_early_abort")?), |_| log::info!("===========lll_isr_early_abort==========="));
    
    // BLE Link Layer
    // api.on_instruction(Some(symbolizer::resolve("radio_pkt_tx_set")?), |_| log::info!("===========radio_pkt_tx_set==========="));
    // api.on_instruction(Some(symbolizer::resolve("radio_pkt_rx_set")?), |_| log::info!("===========radio_pkt_rx_set==========="));
  }

  fn hook_link_layer(api, cfg){
      // Reset state before every run
      api.on_prepare_run(||{
        cfg.sequence = 0;
        cfg.adv_ind_flag = false;
        // common::send_socket_data("RESET")
      });
    
      // TX
      api.on_instruction(Some(symbolizer::resolve("radio_pkt_tx_set")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 1));
      // RX
      api.on_instruction(Some(symbolizer::resolve("radio_pkt_rx_set")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 0));
      
  }


  fn handle_link_layer_packet(cfg, pkt_buf_addr, direction) {
    // let pkt_hex = "";

    if (direction == 1) {
      if cfg.log_details {
        log::info!("<============> TX PKT <============>");
      }
      let pkt_hdr = memory::read_u8(pkt_buf_addr)?;
      let pdu_length = memory::read_u8(pkt_buf_addr+1)?;
      if (pdu_length == 0 && pkt_hdr == 0) {return;}
      let pkt_length = pdu_length + 2;
      
      let pkt_data = memory_read_buffer(pkt_buf_addr, pkt_length);
      // log::info!("Pkt data: {}", pkt_data);

      let pkt_hex = common::encode_hex(pkt_data);
      // send packet to python socket
      // println(pkt_hex);
      // common::send_socket_data(pkt_hex);
      // Update the tx data to a global variable which will be passed to the gen_adv_rpl to get the rpl pkt
      // common::update_tx_data(pkt_hex);
      let pkt_summary = parse_ble_packet(pkt_hex, direction);
      if pkt_summary.contains("BTLE_ADV_IND") {
        // log::info!("I enterted this loop !!!!");
        if cfg.adv_ind_flag == false {
          cfg.adv_ind_flag = true;
          common::update_tx_data(pkt_hex);
        }
        else{
          return
        }
      }
      else{
        common::update_tx_data(pkt_hex);
      }
      if cfg.log_details {
        log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
        log::info!("Pkt. Length: {}", pkt_length);
        log::info!("Pkt. Bytes: {}", pkt_hex);
        log::info!("Pkt. Bytes: {}", pkt_hex);

      }

      // log::info!("TX ---> {}", pkt_summary);
    }
    else {
      if cfg.log_details {
        log::info!("<============> RX PKT <============>");
      }
      let rx_pdu = "";
      let test_str = "";

      // ADV Channel
      if cfg.data_connection == false {
        if cfg.sequence == 0 {
          // Scan Request
          rx_pdu = common::get_adv_rpl_data();
          // log::info!("rx_pdu is here {}", rx_pdu);

          log::info!("<============> rx_pdu received <============>");

          // log::info!("<============> rx_pdu received <============>");
          // log::info!("{}",rx_pdu);
          // rx_pdu = "830cf37a7d65de2800000000000c2aba95";
        }
        else if cfg.sequence >= 1 {
          // Connection Request (where the show begins)
          // rx_pdu = common::get_socket_data();
          // log::info!("Testtt!!!!! Pkt. Bytes: {}", pkt_hex);

          log::info!("<============> rx_pdu received <============>");
          log::info!("{}",rx_pdu);
          rx_pdu = common::get_adv_rpl_data();
          // rx_pdu = "8522a942f80f51c300000000000c7083329a9c9a17020100100000006400ffffffff1f05002939";
          cfg.data_connection = true; // Switch to data channel
          // common::send_socket_data("Connected Update Flag");
          // log::info!("Connected Update flag")

        }
        else {
          cfg.sequence = cfg.sequence + 1;
          return;
        }
      }
      else {
        // set initial flag
        if cfg.initial_pdu_flag == true{
          cfg.initial_pdu_flag = false;
          rx_pdu = common::get_empty_pdu_data();
          log::info!("<============> initial_empty_pdu received <============>");
          // log::info!("{}",rx_pdu);
        }
        else{
          rx_pdu = common::get_data_rpl_data();
           log::info!("<============> rx_pdu received <============>");
          //  log::info!("{}",rx_pdu);
        }
        // TODO: data channel, time to implement 3rd party link layer stack (zephyr via BubbleSim)
        // log::warn!("-------------- TODO -------------");
        // common::send_socket_data("Update Flag");
        // common::gen_reply("empty_pdu", "000000000000000000");
        // rx_pdu = "0800";
        // rx_pdu = common::get_socket_data();
        // log::info!("<============> rx_pdu received <============>");
        // log::info!("{}",rx_pdu);
        // return;
      }

      cfg.sequence = cfg.sequence + 1;
      
      let data = common::decode_hex(rx_pdu)?;
      for (i, v) in data.iter().enumerate() {
        memory::write_u8(pkt_buf_addr + i, v);
      }

      let pkt_summary = parse_ble_packet(rx_pdu, direction);

      if cfg.log_details {
          log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
          log::info!("Pkt. Length: {}", data[1]);
          log::info!("Pkt. Bytes: {}", rx_pdu);
        }
      // log::info!("RX <--- {}", pkt_summary);
    }
  }

  fn print_symbol_name(pc) {
    if let Ok(symbol_name) = symbolizer::lookup(pc) {
      log::info!("0x{:08x}: {}", pc, symbol_name);
    }
  }

  fn memory_read_buffer(addr, length) {
    let pkt_data = [];
      for idx in 0..length {
        pkt_data.push(memory::read_u8(addr+idx)?);
      }
      return pkt_data;
  }

  fn parse_ble_packet(pdu_hex_string, direction) {
    return common::system(format!("python3 scripts/parse-ble-pdu.py {} {}",
                                  pdu_hex_string, direction));
  }

  fn get_data(ddata){

    return ddata;
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
  
    // Force ticker_trigger within rtc0_nrf5_isr
    // common::patch_address(0x1a408, arm::MOVS_NOP(2,1)); // does not work
  
    // Fix isr_radio - Check if isr_cb is not NULL
    common::patch_address(0x1c380, [0x04, 0x34, 0x00, 0x20]);
  
    // Force RX to be successfull (Optional it seems)
    common::patch_function("radio_has_disabled", arm::RETURN_1);
    common::patch_function("radio_is_done", arm::RETURN_1);
    common::patch_function("radio_crc_is_valid", arm::RETURN_1);
    common::patch_function("radio_rssi_is_ready", arm::RETURN_1);
    common::patch_function("radio_filter_has_match", arm::RETURN_0);
    common::patch_function("radio_filter_match_get", arm::RETURN_0);
    common::patch_function("radio_ar_has_match", arm::RETURN_0);
    common::patch_function("radio_ar_match_get", arm::RETURN_0);
  
    // Fix memcmp on scan_req addr check
    common::patch_address(0x0001b684, [0x4f, 0xf0, 0x20, 0x00]);
    // Fix memcmp on adv_ind addr check
    common::patch_address(0x0001b760, [0x4f, 0xf0, 0x20, 0x00]);
    // Fix upper buffer boundary
    common::patch_address(0x0001c016, [0x01, 0x20]);
  }