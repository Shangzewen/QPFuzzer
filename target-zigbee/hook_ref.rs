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
  counter,
}
pub fn main(api) {
    //  ------------ Apply Firmware Patches ------------
    // we can only patch the certain memorries at apply_patch() function
    // since this function was called later after the qemu was initialised
    // if we do api.on_instru, it will be exucuted before qemu initialised, then we 
    // might encounter the memory region not found issue
    api.on_init(apply_patches);
    //  ------------ Hook Link Layer Packets ------------
    // This just initialise the cfg structure one time but every fuzzing session wont reset the value
    let cfg = State {
      sequence: 0,
      data_connection: false,
      log_details: false,
      adv_ind_flag: false,
      initial_pdu_flag:true,
      pkt_length: 0,
      counter: 0
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
    // api.on_basic_block(Some(symbolizer::resolve("mpsl_init")?), |_| log::info!("===========mpsl_init=========="));
    
    // CLEAN
    // api.on_basic_block(Some(symbolizer::resolve("frame_transmit")?), |_| log::info!("===========frame_transmit=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_core_transmit")?), |_| log::info!("===========nrf_802154_core_transmit=========="));
    // api.on_basic_block(Some(symbolizer::resolve("tx_init")?), |_| log::info!("===========tx_init=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_trx_transmit_frame")?), |_| log::info!("===========nrf_802154_trx_transmit_frame=========="));
    // api.on_basic_block(Some(symbolizer::resolve("rx_init")?), |_| log::info!("===========rx_init=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_raal_init")?), |_| log::info!("===========nrf_raal_init=========="));
    // api.on_basic_block(Some(symbolizer::resolve("rx_timeslot_started_callback")?), |_| log::info!("===========rx_timeslot_started_callback=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_core_receive")?), |_| log::info!("===========nrf_802154_core_receive=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_radio_irq_handler")?), |_| log::info!("===========nrf_802154_radio_irq_handler=========="));
    
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_trx_receive_buffer_set")?), |_| log::info!("===========nrf_802154_trx_receive_buffer_set=========="));
    // api.on_basic_block(Some(symbolizer::resolve("radio_pkt_rx_set")?), |_| log::info!("===========radio_pkt_rx_set=========="));

    // CLEAN
    
    
    //  return 1 for timeslot_is_granted only when tx_init was called
    // api.on_instruction(Some(symbolizer::resolve("tx_init")?), |_| memory::write_u8(0x20008ae7, 1));
    // api.on_basic_block(Some(0x0001caf0), |_| log::info!("===========Am I reaching here??=========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_sync")?), |_| log::info!("===========irq_handler_sync=========="));
    // api.on_instruction(Some(0x0001c94a), |_| register::write("r3",0x2)?);
    // api.on_instruction(Some(0x0001c94a), |_| register::read("r3")?);
    //Beacon Rsp
    api.on_basic_block(Some(0x6a6f2), |_| log::info!("===========Reached!!!!!!!!!=========="));
    // api.on_instruction(Some(0x2628a), |_| log::info!("===========bdb_network_steering_not_on_network=========="));
    // api.on_instruction(Some(0x26226), |_| log::info!("===========bdb_network_steering_start_scan=========="));
    // api.on_instruction(Some(symbolizer::resolve("zb_mlme_scan_step")?), |_| log::info!("===========zb_mlme_scan_step=========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mac_send_beacon_request_command")?), |_| log::info!("===========zb_mac_send_beacon_request_command=========="));
    api.on_basic_block(Some(symbolizer::resolve("nrf5_rx_thread")?), |_| log::info!("===========nrf5_rx_thread=========="));
    api.on_basic_block(Some(symbolizer::resolve("zdo_handle_nlme_network_discovery_confirm")?), |_| log::info!("===========zdo_handle_nlme_network_discovery_confirm=========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_scan_confirm")?), |_| log::info!("===========zb_mlme_scan_confirm=========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_scan_step")?), |_| log::info!("===========zb_mlme_scan_step=========="));
    // trigger the scan_confirm confition after second zb_mlme_scan_step
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_scan_step")?), |_| pach_2nd_scan_step(cfg));
    api.on_basic_block(Some(symbolizer::resolve("zb_nlme_network_discovery_confirm")?), |_| log::info!("===========zb_nlme_network_discovery_confirm=========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_nlme_network_discovery_request")?), |_| log::info!("===========zb_nlme_network_discovery_request=========="));
    api.on_basic_block(Some(symbolizer::resolve("net_recv_data")?), |_| log::info!("===========net_recv_data=========="));
    api.on_basic_block(Some(symbolizer::resolve("net_pkt_set_timestamp_ns")?), |_| log::info!("===========net_pkt_set_timestamp_ns=========="));
    api.on_basic_block(Some(symbolizer::resolve("zigbee_l2_recv")?), |_| log::info!("===========zigbee_l2_recv==========="));
    api.on_basic_block(Some(symbolizer::resolve("nrf_802154_co_received_raw")?), |_| log::info!("===========nrf_802154_co_received_raw==========="));
    
    // api.on_ram_read(Some(pc), Some(memory_address), |_, _, _, _| memory::write_u8(20009ddf, 0)?);
    // api.on_instruction(Some(0x9501e), |_| register::read("r0")?);
    // api.on_instruction(Some(0x00004070), |_| register::write("r3",0x10)?);
    // --------------------------bypass the ns time less than 0 assert()new--------------------------------
    api.on_basic_block(Some(0x6777c), |_| register::write("r0",0x1)?);

    // api.on_basic_block(Some(0x16c3a), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c3c), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c3e), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c40), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c42), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c48), |_| register::write("r3",0x0)?);
    // api.on_basic_block(Some(0x16c4a), |_| register::write("r3",0x0)?);

    // --------------------------------------------------------------------------
    // api.on_basic_block(Some(symbolizer::resolve("nrf_egu_task_trigger")?), |_| log::info!("===========nrf_egu_task_trigger==========="));
    // api.on_basic_block(Some(symbolizer::resolve("submit_to_queue")?), |_| log::info!("===========submit_to_queue=========="));
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_frame_parser_src_addr_type_get")?), |_| register::read("r0")?);
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_frame_parser_src_addr_type_get")?), |_| memory::read_u8(0x2000B02C));
    // Assoiate
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_send_association_req_cmd")?), |_| log::info!("===========zb_mlme_send_association_req_cmd==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_associate_request_do")?), |_| log::info!("===========zb_mlme_associate_request_do==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_associate_request")?), |_| log::info!("===========zb_mlme_associate_request==========="));
    api.on_basic_block(Some(symbolizer::resolve("nwk_association_join")?), |_| log::info!("===========nwk_association_join==========="));
    api.on_basic_block(Some(symbolizer::resolve("zdo_join_to_nwk_descr")?), |_| log::info!("===========zdo_join_to_nwk_descr==========="));
    api.on_basic_block(Some(symbolizer::resolve("zdo_commissioning_join_via_scanlist")?), |_| log::info!("===========zdo_commissioning_join_via_scanlist=========="));
    // After assoication request
    api.on_basic_block(Some(symbolizer::resolve("mac_association_req_sent")?), |_| log::info!("===========mac_association_req_sent==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mac_assoc_send_data_req_alarm")?), |_| log::info!("===========zb_mac_assoc_send_data_req_alarm==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mac_assoc_send_data_req")?), |_| log::info!("===========zb_mac_assoc_send_data_req==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mac_get_indirect_data")?), |_| log::info!("===========zb_mac_get_indirect_data==========="));
    api.on_basic_block(Some(symbolizer::resolve("zb_mlme_send_data_req_done")?), |_| log::info!("===========zb_mlme_send_data_req_done==========="));


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
      // This Reset state before every run
      api.on_prepare_run(||{
        cfg.sequence = 0;
        cfg.counter = 0;
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
      // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_trx_transmit_frame")?), 
      api.on_basic_block(Some(symbolizer::resolve("nrf_802154_trx_transmit_frame")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 1));
      // RX
      api.on_basic_block(Some(symbolizer::resolve("nrf_802154_trx_receive_buffer_set")?), 
                |_| handle_link_layer_packet(cfg, register::read("r0")?, 0));
      
  }
  fn test_pkt(){
    let pkt_data2 = memory_read_buffer(0x20005704, 4);
    let pkt_hex2 = common::encode_hex(pkt_data2);
    log::info!(" TX Pkt2. Bytes: {}", pkt_hex2);
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
        cfg.pkt_length = pdu_length;
      }
      // println!("This is pkt_length {}", cfg.pkt_length);
      // if (cfg.pkt_length > 20){
      //   cfg.data_connection = false;
      //   log::info!("Received adv packet!");
      // }else{
      //   cfg.data_connection = true;
      //   log::info!("Received data packet!");

      // }
      let pkt_data = memory_read_buffer(pkt_buf_addr+1, cfg.pkt_length);

      let pkt_hex = common::encode_hex(pkt_data);
      // cfg.sequence == 1 beacon req received 
      // cfg.sequence == 2 Assoation req received 
      if pkt_hex == "030800ffffffff070000"{
        log::info!("Beacon Req");
        cfg.sequence = 1
      }
      // else if pkt_hex == "23c80182b50000ffff007098905a36cef4018e0000"{
      else if pkt_hex.contains("8e"){
        log::info!("Assoation Req");
        cfg.sequence = 2
      }
      else{
        cfg.sequence = 0
      }
      log::info!(" TX Pkt. Bytes: {}", pkt_hex);
      // log::info!(" TX PK. Bytes: {}", pkt_hex);
    }
    else {
      if cfg.log_details {
        log::info!("<============> RX PKT <============>");
      }
      // let rx_pdu = "1c00806582b50000ffcf0000002286b28a1020a436cef4ffffff00e179";
      let rx_pdu = "";
      match cfg.sequence {
        // 0 => rx_pdu = "830cf37a7d65de2800000000000c2aba95",
        1 => rx_pdu = "1c00806582b50000ffcf0000002286b28a1020a436cef4ffffff00e179",
        // 2 => {
          // log::info!("-------------- ANCHOR POINT -------------");
          // cfg.data_connection = true;
          // rx_pdu = "0900";
          // },
        2 => rx_pdu = "050200010000", // ACK with seq number of 01
        // 6 => rx_pdu = "0306000c0800000000", // LL_VERSION_IND
        // 6 => rx_pdu = "03090014fb004808fb004808", // LL_LENGTH_REQ
        _ => rx_pdu = "",
      }
      // let test_str = "";
      log::info!("Pkt Raw: {}",rx_pdu);


  //     // // ADV Channel
  //     // if cfg.data_connection == false {
  //     //   if cfg.sequence == 0 {
  //     //     // Scan Request
  //     //     // rx_pdu = common::get_adv_rpl_data();
  //     //     rx_pdu = common:: get_rx_data();
  //     //     log::info!("RX adv: {}", rx_pdu);
  //     //   }
  //     //   else if cfg.sequence >= 1 {
  //     //     log::info!("RX adv: {}", rx_pdu);

  //     //     // rx_pdu = common::get_adv_rpl_data();
  //     //     rx_pdu = common:: get_rx_data();

  //     //     cfg.data_connection = true; // Switch to data channel
  //     //   }
  //     //   else {
  //     //     cfg.sequence = cfg.sequence + 1;
  //     //     return;
  //     //   }
  //     // }
  //     // else {
  //     //   // set initial flag
  //     //   if cfg.initial_pdu_flag == true{
  //     //     cfg.initial_pdu_flag = false;
  //     //     // rx_pdu = common::get_empty_pdu_data();
  //     //     rx_pdu = common:: get_rx_data();

  //     //     // log::info!("<============> initial_empty_pdu received <============>");
  //     //   }
  //     //   else{
  //     //     // rx_pdu = common::get_data_rpl_data();
  //     //     rx_pdu = common:: get_rx_data();

  //     //     log::info!("RX data pdu: {}", rx_pdu);
  //     //     //  log::info!("<============> rx_pdu received <============>");
  //     //   }
  //     // }

      // cfg.sequence = cfg.sequence + 1;

      let data = common::decode_hex(rx_pdu)?;
      // log::info!("Pkt Raw: {}",data);
      for (i, v) in data.iter().enumerate() {
        memory::write_u8(pkt_buf_addr + i, v);
      }

      // let pkt_summary = common::parse_packet("ble", rx_pdu, direction, !cfg.initial_pdu_flag, cfg.log_details);
      // log::info!("RX <--- {}", pkt_summary);
      // let pkt_len 
      let pdu_length = memory::read_u8(pkt_buf_addr)?;
      log::info!("This is pdu length {}",pdu_length);
      // need to -1 for the pdu_length since the first bytes is the length itself which is a extra bytes
      let pkt_data = memory_read_buffer(pkt_buf_addr+1, pdu_length);
      let pkt_hex = common::encode_hex(pkt_data);
      log::info!(" RX Pkt. Bytes: {}", pkt_hex);
      if cfg.log_details {
          log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
          log::info!("Pkt. Length: {}", data[1]);
          log::info!("Pkt. Bytes: {}", rx_pdu);
        }
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
        // log::info!("Test pkt", memory::read_u8(addr+idx));
        // println!("This is pkt_data ",memory::read_u8(addr+idx));
      }
      // let pkt_hex = common::encode_hex(pkt_data);
      // log::info!(" RX Pkt. Bytes: {}", pkt_hex);
      return pkt_data;
  }
  fn pach_2nd_scan_step(cfg){
    if cfg.counter == 1{
      // Second scan_step_encountered
      common::patch_address(0x0007a1b2, arm::NOP);
      log::info!("Executed");

      cfg.counter = 0
    }
    else{
      cfg.counter = 1
    }
    // println!("This is the counter: ",counter);
    log::info!(" This is the counter : {}", cfg.counter);
  }

  // fn parse_ble_packet(pdu_hex_string, direction) {
  //   return common::system(format!("python3 scripts/parse-ble-pdu.py {} {}",
  //                                 pdu_hex_string, direction));
  // }

  // fn get_data(ddata){

  //   return ddata;
  // }

  fn apply_patches() {
    // common::patch_function("z_tick_sleep", arm::RETURN);
    common::patch_function("CC_PalMutexCreate", arm::RETURN_0);
    common::patch_function("CC_PalPowerSaveModeSelect", arm::RETURN_0);
    common::patch_function("cc_mbedtls_platform_zeroize", arm::RETURN);
    common::patch_function("mbedtls_zeroize_internal", arm::RETURN);
    common::patch_function("nrf_cc3xx_platform_init", arm::RETURN_0);
    common::patch_function("nrf_gpio_pin_present_check", arm::RETURN_1);
    common::patch_function("z_impl_device_is_ready", arm::RETURN_1);
    common::patch_function("z_impl_entropy_get_entropy", arm::RETURN_0);
    common::patch_function("delay_machine_code.0", arm::RETURN_1);
    common::patch_function("nrfy_pwm_int_init", arm::RETURN);
    common::patch_function("pinctrl_configure_pins", arm::RETURN_0);
    common::patch_function("nrf_clock_is_running", arm::RETURN_1);
    common::patch_function("z_nrf_clock_control_lf_on", arm::RETURN);
    common::patch_function("pinctrl_apply_state", arm::RETURN_0);
    common::patch_function("pm_device_driver_init", arm::RETURN_0);
    // common::patch_function("is_tx_ready", arm::RETURN_1);
    common::patch_function("log_output_process", arm::RETURN);
    common::patch_function("nrf_event_readback", arm::RETURN);
    common::patch_function("zb_trace_msg_port_vl", arm::RETURN_0);
    // need to be removed for new
    // common::patch_function("zb_schedule_alarm", arm::RETURN_0);
    common::patch_function("nrf_uarte_event_check", arm::RETURN_1);
    common::patch_function("temp_nrf5_mpsl_sample_fetch", arm::RETURN_0);
    common::patch_function("light_bulb_set_brightness", arm::RETURN);
    // -----------------------------------------------------------
    common::patch_function("nrf_egu_event_check", arm::RETURN_1);
    common::patch_function("nrf_egu_int_enable_check", arm::RETURN_1);
    common::patch_function("mpsl_temperature_get", arm::RETURN_(112));
    common::patch_function("rand_get", arm::RETURN_0);
    common::patch_function("zb_random_seed", arm::RETURN_2);
    common::patch_function("nrf_timer_event_check", arm::RETURN_1);
    common::patch_function("timeslot_is_granted", arm::RETURN_1);
    common::patch_function("remaining_timeslot_time_is_enough_for_crit_sect", arm::RETURN_1);

    
    // force the loop to enter steering 
    common::patch_address(0x000266a2, [0x01, 0x2b]);

    // patch the timer check to froce the meulation think the timeslot time left 
    common::patch_function("nrf_raal_timeslot_request", arm::RETURN_1);
    // force to make sure the current prority is high enough
    // common::patch_function("is_state_allowed_for_prio", arm::RETURN_1);
    // -----------------------------------------------------------

    common::patch_function("zb_nvram_load", arm::RETURN);
    common::patch_function("zb_nvram_write_dataset", arm::RETURN);
    common::patch_function("zb_nvram_dataset_is_supported", arm::RETURN_0);
    common::patch_function("nrf_802154_random_init", arm::RETURN_0);
    common::patch_function("sym_76IVKPQMZOZ7IXNJTGMWOSIWIVCASAX3TTNHN7I", arm::RETURN_1); // Read LFCLKSTAT 0x40000418
    common::patch_function("rng_pool_get", arm::RETURN_1);
    // common::patch_function("nrf_802154_queue_is_empty", arm::RETURN_0);
    // common::patch_function("active_vector_priority_is_high", arm::RETURN_1);

    
    common::patch_function("qspi_nor_init", arm::RETURN_0);
    common::patch_function("settings_subsys_init", arm::RETURN_0);
    common::patch_function("nrfx_gpiote_0_irq_handler", arm::RETURN);
    //-------------------------------------------- New------------------------------------------------------------------------------
    common::patch_function("are_preconditions_met", arm::RETURN_1);
    common::patch_function("nrf_802154_pib_promiscuous_get", arm::RETURN_1);
    common::patch_function("ns_to_net_ptp_time", arm::RETURN_1);
    // force the state_transition for collision avodiance state to be always successful
    common::patch_function("csma_ca_state_set", arm::RETURN_1);
    // ignore radio_state_check for receive_buffer_missing_buffer_set 
    common::patch_address(0x0001c94c, arm::NOP);

    // -------------------------------------------------------------------------------
    //  Test, eventually the functon zigbee_l2_recv need to append the rx packet to the queue then process it but now the function was not called
    // common::patch_function("zb_trans_rx_pending", arm::RETURN_1);
    
    // ---------
    // nrf_egu_task_trigger - Force branch to _swi_irq_handler (TODO IRQ Handling via peripheral)
    common::patch_address(0x0001bac0, [0x77, 0xf0, 0x5b, 0xfa]);
    common::patch_address(0x0001bac8, [0x00, 0xbf]);

    // make sure the zb_trans_transmit alawys think zigbee_event is transmited successfully
    // the var_2c_1 is returnning error, need to check up why??
    common::patch_address(0x00016c42, [0x13, 0xf1, 0x01, 0x0f]);
    common::patch_address(0x00016c4c, arm::NOP);

    // -----
    // for the scan_step think all channls are scaned:
    // common::patch_address(0x0007a1b2, arm::NOP);
    // for to make sure the 7th bit of the 0x20007889 to be set so :
    // common::patch_address(0x0007a1b2, arm::NOP);
    // Ignore the time < 0 error for ns_to_net_pip_time
    // common::patch_address(0x00004080, arm::NOP);
    
    // Branch to radio handler
    // Force zb_zdo_joined return 0
    // common::patch_address(0x24efe, arm::NOP);

    // common::patch_address(0x0001bac4, [0x48,0x47]);
    // // Fix memcmp on adv_ind addr check
    // common::patch_address(0x0001b760, [0x4f, 0xf0, 0x20, 0x00]);
    
  }