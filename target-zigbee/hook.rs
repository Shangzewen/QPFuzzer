// ----------- UDP Socket Info -----------
// Python server: 127.0.0.1:7777
// Listen packet client: 127.0.0.1:9999
// Sending packet client: 127.0.0.1:8888 
struct State {
  sequence,
  data_connection,
  log_details,
  beacon_req_flag,
  initial_pdu_flag,
  data_req_flag,
  pkt_length,
  counter,
  counter_data_req_ack,
  counter_rx_pkt,
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
      beacon_req_flag: false,
      initial_pdu_flag:true,
      data_req_flag: false,
      pkt_length: 0,
      counter: 0,
      counter_data_req_ack: 0,
      counter_rx_pkt: 0,
    };

    hook_link_layer(api, cfg);

    //  ------------ Print Logs ------------
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
    // api.on_basic_block(Some(0x16c58), |_| log::info!("===========Mac Notif Send Successfully!!!!!!!!=========="));
    // api.on_basic_block(Some(0x16c48), |_| log::info!("===========Checking wait_type !!!!!!!!=========="));
    // api.on_basic_block(Some(0x7685a), |_| log::info!("===========Checking mac callback !!!!!!!!=========="));
    // api.on_ram_read(Some(pc), Some(memory_address), |_, _, _, _| memory::write_u8(20009ddf, 0)?);
    // api.on_instruction(Some(0x9501e), |_| register::read("r0")?);
    // api.on_instruction(Some(0x00004070), |_| register::write("r3",0x10)?);
    // --------------------------bypass the ns time less than 0 assert()new--------------------------------
    api.on_basic_block(Some(0x6777c), |_| register::write("r0",0x1)?);

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
    // Radio Irq handler
    // api.on_basic_block(Some(symbolizer::resolve("nrf_802154_radio_irq_handler")?), |_| log::info!("===========nrf_802154_radio_irq_handler==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_sync")?), |_| log::info!("===========irq_handler_sync==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_ready")?), |_| log::info!("===========irq_handler_ready==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_address")?), |_| log::info!("===========irq_handler_address==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_bcmatch")?), |_| log::info!("===========irq_handler_bcmatch==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_crcerror")?), |_| log::info!("===========irq_handler_crcerror==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_crcok")?), |_| log::info!("===========irq_handler_crcok==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_phyend")?), |_| log::info!("===========irq_handler_phyend==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_disabled")?), |_| log::info!("===========irq_handler_disabled==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_ccaidle")?), |_| log::info!("===========irq_handler_ccaidle==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_ccabusy")?), |_| log::info!("===========irq_handler_ccabusy==========="));
    // api.on_basic_block(Some(symbolizer::resolve("irq_handler_edend")?), |_| log::info!("===========irq_handler_edend==========="));
    // Ack
    api.on_basic_block(Some(symbolizer::resolve("ack_match_check")?), |_| log::info!("===========ack_match_check==========="));
    api.on_basic_block(Some(symbolizer::resolve("ieee802154_handle_ack")?), |_| log::info!("===========ieee802154_handle_ack==========="));
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
        cfg.counter_data_req_ack = 0;
        cfg.data_req_flag = false;
        cfg.beacon_req_flag = false;
        cfg.counter_rx_pkt = 0;
        // cfg.initial_pdu_flag = true;
        // Need to make sure the data_buffer is empty every new itteration
        common::clear_transmission_data();
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
      let pkt_data = memory_read_buffer(pkt_buf_addr+1, cfg.pkt_length);

      let pkt_hex = common::encode_hex(pkt_data);
      log::info!(" TX Pkt. Bytes: {}", pkt_hex);

      let pkt_summary = common::parse_packet("zigbee", pkt_hex, direction, !cfg.initial_pdu_flag, cfg.log_details);
      // zmq tx transmition
      common::zmq_transmit_receive(pkt_hex,"00",0);

      // println!("This is the pkt summary {}", pkt_summary);
      if pkt_summary.contains("BeaconReq") {
        // log::info!("Beacon req !!!! {}", pkt_summary);
        if cfg.beacon_req_flag == false {
          cfg.beacon_req_flag = true;
          common::update_tx_data(pkt_hex);
        }
        else{
          return
        }
      }
      else if pkt_summary.contains("DataReq"){
        if cfg.data_req_flag == false {
          cfg.data_req_flag = true;
          cfg.counter_data_req_ack = 1;
          common::update_tx_data(pkt_hex);
        }
      }
      else{
        common::update_tx_data(pkt_hex);
      }
    }
    else {
      if cfg.log_details {
        log::info!("<============> RX PKT <============>");
      }
      // let rx_pdu = "1c00806582b50000ffcf0000002286b28a1020a436cef4ffffff00e179";
      // cfg.counter_rx_pkt +=1;
      // cfg.counter_rx_pkt = (cfg.counter_rx_pkt + 1) % 256;
      // let hex_counter = format!("{:02x}", cfg.counter_rx_pkt & 0xFF);
      let rx_pdu = "";
      let fuzzed_msg = "";
      let rx_pdu_test = "";
      // just receive the data requet, send ack
      if cfg.data_req_flag == true && cfg.counter_data_req_ack == 1{
        cfg.counter_data_req_ack = 2;
        rx_pdu = common::get_zigbee_rpl_data(0);
      // after ack the data request, send asso_rsp then only wait until acknowledged then reset the flag and oucnter
      // other than that keep sending Association Response
      }else if cfg.data_req_flag == true && cfg.counter_data_req_ack == 2{
        // reset the counter to 0 and the data_req to to make sure the data_req re_transmit was handled the same way
        // cfg.counter_data_req_ack = 0;
        // cfg.data_req_flag = false;
        rx_pdu = common::get_zigbee_rpl_data(1);
        if rx_pdu == ""{
          // memory::write_u8(pkt_buf_addr, 0x0);
          // pass;
          // let data = 0x0;
        }else{
          // rx_pdu.replace_range(rx_pdu.len() - 2.., &hex_counter);
          // rx_pdu = common::update_counter_last_byte(rx_pdu, cfg.counter_rx_pkt);
          fuzzed_msg = common::zmq_transmit_receive(rx_pdu[2..],"01",cfg.counter_rx_pkt);
        }
      }else{
        rx_pdu = common::get_zigbee_rpl_data(0);
        if rx_pdu == ""{
          // memory::write_u8(pkt_buf_addr, 0x0);
          // let data = 0x0;
          // log::info!("rx pdu     {}",rx_pdu);
        }else{
          // Update the last byte to the counter to make sure the duplicated pkt can be distinguish by the poc generater 
          // PoC Generator need a clear filter to filtter out the pkt then it can mutate
          // rx_pdu = common::update_counter_last_byte(rx_pdu, cfg.counter_rx_pkt);
          fuzzed_msg = common::zmq_transmit_receive(rx_pdu[2..],"01",cfg.counter_rx_pkt);
        }
      }
      // let test_zigbee_stack = common::get_zigbee_rpl_data(); 
      // log::info!("Zigbee Pkt Raw: {}",rx_pdu_test);
      // zigbee_rx_pdu = 
      // let test_str = "";
      // log::info!("Pkt Raw: {}",rx_pdu);
      log::info!("fuzzed msg {}",fuzzed_msg);
      log::info!("rx pdu     {}",rx_pdu);
      // Need to clear the stack manually if the rx_pdu is empty
      if rx_pdu == ""{
        memory::write_u8(pkt_buf_addr, 0x0);
        // let data = 0x0;
      }else{
        let data = common::decode_hex(fuzzed_msg)?;
        // log::info!("Pkt Raw data: {:?}",data);
        for (i, v) in data.iter().enumerate() {
          memory::write_u8(pkt_buf_addr + i, v);
          // log::info!("This is the memory write result {:?}", result);
        }
        let pdu_length = memory::read_u8(pkt_buf_addr)?;
        log::info!("This is pdu length {}",pdu_length);
        // need to -1 for the pdu_length since the first bytes is the length itself which is a extra bytes
        let pkt_data = memory_read_buffer(pkt_buf_addr+1, pdu_length);
        let pkt_hex = common::encode_hex(pkt_data);
        log::info!(" RX Pkt. Bytes: {}", pkt_hex);
      }
      // let pkt_summary = common::parse_packet("ble", rx_pdu, direction, !cfg.initial_pdu_flag, cfg.log_details);
      // log::info!("RX <--- {}", pkt_summary);
      // let pkt_len 
      // let pdu_length = memory::read_u8(pkt_buf_addr)?;
      // log::info!("This is pdu length {}",pdu_length);
      // // need to -1 for the pdu_length since the first bytes is the length itself which is a extra bytes
      // let pkt_data = memory_read_buffer(pkt_buf_addr+1, pdu_length);
      // let pkt_hex = common::encode_hex(pkt_data);
      // log::info!(" RX Pkt. Bytes: {}", pkt_hex);
      // if cfg.log_details {
      //     log::info!("Pkt. Addr: 0x{:08x}", pkt_buf_addr);
      //     log::info!("Pkt. Length: {}", data[1]);
      //     log::info!("Pkt. Bytes: {}", rx_pdu);
      //   }
    }
  }

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

    // focre the handle_ack to ignore the timestamp check
    common::patch_address(0x000101ce, [0xff,0x2b]);
    common::patch_address(0x000101d0, [0x0b,0xd9]);
    // -----
    
  }