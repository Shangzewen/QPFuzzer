use std::{borrow::Cow, fmt::Debug, io::Write, mem, ops::Shr};

use anyhow::{Context, Result};
use common::FxHashMap;
use qemu_rs::{Address, Exception, MmioAddress, USize};

use crate::{
    input::{
        value::{InputValue, InputValueType},
        InputContext,
    },
    mmio::{aligned, AccessContext},
    mmio_model::{MmioModel, ReadSize},
    modeling::Modeling,
};

pub type Interrupt = Exception;

pub trait Input: WriteTo {
    fn id(&self) -> usize;
    fn reset(&mut self);
    fn read(&mut self, context: &InputContext) -> Option<Cow<InputValue>>;
}

pub trait WriteTo {
    fn write_to<W: Write>(&self, writer: W) -> anyhow::Result<()>;
    fn write_size(&self) -> Result<u64>;
    fn filename(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct HardwareSnapshot {
    memory: Memory,
}

#[derive(Debug)]
pub struct Hardware<I: Input + Debug> {
    modeling: Modeling,
    memory: Memory,
    input: Option<I>,
    access_log: Vec<InputContext>,
    ticker_rtc0: USize,
    ticker_rtc1: USize,
    ticker_rtc2: USize,
    event_en_nrf0:USize,
    event_en_nrf1:USize,
    event_en_nrf2:USize,
    event_en_nrf3:USize,
    event_en_nrf4:USize,
    egu_event_check0:USize,
    egu_event_check1:USize,
    egu_event_check2:USize,
    egu_event_check3:USize,
    pub tx_flag: USize,

    // 0x40011504
}

#[derive(Debug, Clone)]
pub struct Memory {
    data: FxHashMap<MmioAddress, USize>,
}
#[derive(Debug)]
pub struct HardwareResult<I: Input + Debug> {
    pub input: I,
    pub access_log: Vec<InputContext>,
}

impl<I: Input + Debug> Hardware<I> {
    pub fn new(modeling: Modeling) -> Self {
        Self {
            modeling,
            memory: Memory::new(),
            input: None,
            access_log: vec![],
            ticker_rtc0: 0,
            ticker_rtc1: 0,
            ticker_rtc2: 0,
            event_en_nrf0:0,
            event_en_nrf1:0,
            event_en_nrf2:0,
            event_en_nrf3:0,
            event_en_nrf4:0,
            egu_event_check0:0,
            egu_event_check1:0,
            egu_event_check2:0,
            egu_event_check3:0,
            tx_flag:0,
        }
    }

    pub fn prepare_run(&mut self, input: I) {
        debug_assert!(self.input.is_none());
        debug_assert!(self.access_log.is_empty());

        self.input = Some(input);
        self.ticker_rtc0 = 0 ;
        self.ticker_rtc1 = 0;
        self.ticker_rtc2 = 0;
        self.event_en_nrf0=0;
        self.event_en_nrf1=0;
        self.event_en_nrf2=0;
        self.event_en_nrf3=0;
        self.event_en_nrf4=0;
        self.egu_event_check0=0;
        self.egu_event_check1=0;
        self.egu_event_check2=0;
        self.egu_event_check3=0;
        self.tx_flag = 0;

    }

    pub fn modeling(&self) -> &Modeling {
        &self.modeling
    }

    pub fn take_result(&mut self) -> Result<HardwareResult<I>> {
        // MMIO access log
        let mut access_log = Vec::with_capacity(self.access_log.capacity());
        mem::swap(&mut self.access_log, &mut access_log);

        Ok(HardwareResult {
            input: self.input.take().context("input file missing")?,
            access_log,
        })
    }

    pub fn input_read(&mut self, context: InputContext) -> Option<Cow<InputValue>> {
        let value = self
            .input
            .as_mut()
            .expect("input file missing")
            .read(&context);

        if value.is_some() {
            self.access_log.push(context);
        }

        value
    }
    // Probabily need to update this function
    pub fn mmio_read(
        &mut self,
        context: &AccessContext,
        size: ReadSize,
    ) -> Result<Option<(USize, bool)>> {

        // Handle manually specified timers
        if context.mmio().addr() == 0x4000b104 {
            return Ok(Some((0, true)));
        }
        else if context.mmio().addr() == 0x4000b504 {
            // log::info!("Ticker Access: {}", self.ticker);
            let ticker_pre_update = self.ticker_rtc0.clone();
            self.ticker_rtc0 += 2;
            return Ok(Some((ticker_pre_update, true)));
        }else if context.mmio().addr() == 0x40011504{
            // log::info!("Ticker Access: {}", self.ticker);
            let ticker_pre_update = self.ticker_rtc1.clone();
            self.ticker_rtc1 += 2;
            return Ok(Some((ticker_pre_update, true)));
        } 
        else if context.mmio().addr() == 0x40024000{
            // log::info!("Ticker Access: {}", self.ticker);
            let ticker_pre_update = self.ticker_rtc2.clone();
            self.ticker_rtc2 += 2;
            return Ok(Some((ticker_pre_update, true)));   
        }
        // Hadle manually timer irq enent enable
        else if context.mmio().addr() == 0x40008304{
            let event_read_en0 = self.event_en_nrf0.clone();
            return Ok(Some((event_read_en0, true))); 
        }
        else if context.mmio().addr() == 0x40009304{
            let event_read_en1 = self.event_en_nrf1.clone();
            return Ok(Some((event_read_en1, true))); 
        }
        else if context.mmio().addr() == 0x4000A304{
            let event_read_en2 = self.event_en_nrf2.clone();
            return Ok(Some((event_read_en2, true))); 
        }
        else if context.mmio().addr() == 0x4001A304{
            let event_read_en3 = self.event_en_nrf3.clone();
            return Ok(Some((event_read_en3, true))); 
        }
        else if context.mmio().addr() == 0x4001B304{
            let event_read_en4 = self.event_en_nrf4.clone();
            return Ok(Some((event_read_en4, true))); 
        }
        // Hadle egu event check
        else if context.mmio().addr() == 0x4001413C{
            let event_check0 = self.egu_event_check0.clone();
            return Ok(Some((event_check0, true))); 
        }
        else if context.mmio().addr() == 0x4001410C{
            let event_check1 = self.egu_event_check1.clone();
            return Ok(Some((event_check1, true))); 
        }
        else if context.mmio().addr() == 0x40014100{
            let event_check2 = self.egu_event_check2.clone();
            return Ok(Some((event_check2, true))); 
        }
        else if context.mmio().addr() == 0x40014108{
            let event_check3 = self.egu_event_check3.clone();
            return Ok(Some((event_check3, true))); 
        }
        // enable or diable the radio irq handler based on tx_flag
        else if context.mmio().addr() == 0x40001168{
            // make sure the branch of irq_handler never reaches
            let irq_handler_sync = 0x0;
            // println!("Force the irq_handler_sync varible: {}", irq_handler_sync);
            return Ok(Some((irq_handler_sync, true)));
        }
        else if context.mmio().addr() == 0x40001128{
            // make sure the branch of irq_handler never reaches
            let irq_handler_bcmatch = 0;
            // println!("Force the irq_handler_bcmatch varible: {}", irq_handler_bcmatch);
            return Ok(Some((irq_handler_bcmatch, true)));
        }
        else if context.mmio().addr() == 0x40001134{
            // make sure the branch of irq_handler never reaches
            let irq_handler_crcerror = 0;
            // println!("Force the irq_handler_crcerror varible: {}", irq_handler_crcerror);
            return Ok(Some((irq_handler_crcerror, true)));
        }
        else if context.mmio().addr() == 0x40001100{
            // make sure the branch of irq_handler never reaches
            let irq_handler_ready = 0;
            // println!("Force the irq_handler_ready varible: {}", irq_handler_ready);
            return Ok(Some((irq_handler_ready, true)));
        }
        else if context.mmio().addr() == 0x40001110{
            // make sure the branch of irq_handler never reaches
            let irq_handler_disabled = 0;
            // println!("Force the irq_handler_disabled varible: {}", irq_handler_disabled);
            return Ok(Some((irq_handler_disabled, true)));
        }
        else if context.mmio().addr() == 0x40001104{
            // make sure the branch of irq_handler never reaches
            let irq_handler_address = 0;
            // println!("Force the irq_handler_address varible: {}", irq_handler_address);
            return Ok(Some((irq_handler_address, true)));
        }
        else if context.mmio().addr() == 0x40001130{
            // make sure the branch of irq_handler never reaches
            if self.tx_flag == 1{
                // let irq_handler_crcok: u32 = 0x1;
                // println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
                // return Ok(Some((irq_handler_crcok, true)));
                let irq_handler_crcok: u32 = 0x0;
                println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
                return Ok(Some((irq_handler_crcok, true)));
            }
        }
        else if context.mmio().addr() == 0x4000116c{
            // make sure the branch of irq_handler never reaches
            if self.tx_flag == 1{
                // let irq_handler_crcok: u32 = 0x1;
                // println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
                // return Ok(Some((irq_handler_crcok, true)));
                let irq_handler_phyend: u32 = 0x03534de5;
                println!("Force the irq_handler_phyend varible: {}", irq_handler_phyend);
                return Ok(Some((irq_handler_phyend, true)));
            }else{
                let irq_handler_phyend: u32 = 0x0;
                println!("Force the irq_handler_phyend varible: {}", irq_handler_phyend);
                return Ok(Some((irq_handler_phyend, true)));
            }
        }
        else if context.mmio().addr() == 0x40001148{
            // make sure the branch of irq_handler never reaches
            // if self.tx_flag == 1{
                // let irq_handler_crcok: u32 = 0x1;
                // println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
                // return Ok(Some((irq_handler_crcok, true)));
            let irq_handler_ccabusy: u32 = 0x0;
            println!("Force the irq_handler_ccabusy varible: {}", irq_handler_ccabusy);
            return Ok(Some((irq_handler_ccabusy, true)));
            // }
        }
        // else if context.mmio().addr() == 0x40001148{
        //     // make sure the branch of irq_handler never reaches
        //     let irq_handler_ccabusy = 0;
        //     // println!("Force the irq_handler_address varible: {}", irq_handler_address);
        //     return Ok(Some((irq_handler_ccabusy, true)));
        // }
        // else if context.mmio().addr() == 0x40001144{
        //     // make sure the branch of irq_handler never reaches
        //     let irq_handler_ccaidle = 0;
        //     // println!("Force the irq_handler_address varible: {}", irq_handler_address);
        //     return Ok(Some((irq_handler_ccaidle, true)));
        // }
        // INNSET register to control the radio event
        // else if context.mmio().addr() == 0x40001304{
        //     // make sure the branch of irq_handler never reaches
        //     if self.tx_flag == 1{
        //         // let irq_handler_crcok: u32 = 0x1;
        //         // println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
        //         // return Ok(Some((irq_handler_crcok, true)));
        //         let intenset: u32 = 0x8000000;
        //         // println!("Force the irq_handler_crcok varible: {}", intenset);
        //         return Ok(Some((intenset, true)));
        //     }
        //     else{
        //         let intenset: u32 = 0x1000;
        //         // println!("Force the irq_handler_crcok varible: {}", intenset);
        //         return Ok(Some((intenset, true)));
        //     }
        // }
        // else if context.mmio().addr() == 0x4000116c{
        //     // make sure the branch of irq_handler never reaches
        //     if self.tx_flag == 0{
        //         // let irq_handler_crcok: u32 = 0x1;
        //         // println!("Force the irq_handler_crcok varible: {}", irq_handler_crcok);
        //         // return Ok(Some((irq_handler_crcok, true)));
        //         let irq_handler_phyend: u32 = 0x0;
        //         println!("Force the irq_handler_crcok varible: {}", irq_handler_phyend);
        //         return Ok(Some((irq_handler_phyend, true)));
        //     }
        // }
        // else if context.mmio().addr() == 0x40001134{
        //     // make sure the branch of irq_handler never reaches
        //     if self.tx_flag == 1{
        //     //     let irq_handler_crerror: u32 = 0x1;
        //     //     println!("Force the irq_handler_crcerror varible: {}", irq_handler_crerror);
        //     //     return Ok(Some((irq_handler_crerror, true)));
        //     // }else {
        //         let irq_handler_crerror: u32 = 0x0;
        //         println!("Force the irq_handler_crcerror varible: {}", irq_handler_crerror);
        //         return Ok(Some((irq_handler_crerror, true)));
        //     }
        // }
        // else if context.mmio().addr() == 0x40001128{
        //     // make sure the branch of irq_handler never reaches
        //     if self.tx_flag == 1{
        //     //     let irq_handler_bcmatch: u32 = 0x1;
        //     //     println!("Force the irq_handler_crcerror varible: {}", irq_handler_bcmatch);
        //     //     return Ok(Some((irq_handler_bcmatch, true)));
        //     // }else {
        //         let irq_handler_bcmatch: u32 = 0x0;
        //         println!("Force the irq_handler_crcerror varible: {}", irq_handler_bcmatch);
        //         return Ok(Some((irq_handler_bcmatch, true)));
        //     }
        // }
        // unwrap input file
        let input = self.input.as_mut().expect("input file missing");

        // apply the MMIO model
        let model = self
            .modeling
            .get_or_create(context)
            .context("get/create MMIO model failed")?;
        log::trace!("model = {:x?}", model);


        // get input value (either from model or input file)
        let mut input_context = None;
        let value = match model {
            Some(MmioModel::Passthrough { initial_value }) => {
                let mmio = context.mmio();
                Some(
                    self.memory
                        .read(mmio.addr(), size)
                        .unwrap_or(*initial_value),
                )
            }
            Some(MmioModel::Constant { value }) => Some(*value),
            Some(MmioModel::Set { values }) => {
                let context =
                    InputContext::from_access(context, InputValueType::Choice(values.len() as u8));
                let value = input
                    .read(&context)
                    .map(|input_value| match input_value.as_ref() {
                        InputValue::Choice { index, .. } => values[*index as usize],
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
            Some(MmioModel::BitExtract(be)) => {
                let context = InputContext::from_access(context, InputValueType::Bits(be.bits()));
                let value = input
                    .read(&context)
                    .map(|input_value| match input_value.as_ref() {
                        InputValue::Bits { value, .. } => be.apply(*value),
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
            None => {
                let context = InputContext::from_access(context, size.into());
                let value = input
                    .read(&context)
                    .map(|input_value| match input_value.as_ref() {
                        InputValue::Byte(value) => *value as u32,
                        InputValue::Word(value) => *value as u32,
                        InputValue::DWord(value) => *value,
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
        };
        log::trace!("[READ] {:x?} => {:x?}", context, value);

        Ok(value.map(|value| {
            // track mmio accesses
            let input_value = if let Some(context) = input_context {
                self.access_log.push(context);
                true
            } else {
                false
            };
            // println!("This is value: {}", value);

            (value, input_value)
        }))
    }
    // Probabily need to update this function
    pub fn mmio_read_update(
        &mut self,
        context: &AccessContext,
        size: ReadSize,
    ) -> Result<Option<(USize, bool)>> {
        // unwrap input file
        let input = self.input.as_mut().expect("input file missing");

        // apply the MMIO model
        let model = self
            .modeling
            .get_or_create(context)
            .context("get/create MMIO model failed")?;
        log::trace!("model = {:x?}", model);

        // model = option::Option<&modeling::mmio_model::MmioModel>

        // get input value (either from model or input file)
        let mut input_context = None;
        let value = match model {
            Some(MmioModel::Passthrough { initial_value }) => {
                let mmio = context.mmio();
                Some(
                    self.memory
                        .read(mmio.addr(), size)
                        .unwrap_or(*initial_value),
                )
            }
            Some(MmioModel::Constant { value }) => Some(*value),
            Some(MmioModel::Set { values }) => {
                let context =
                    InputContext::from_access(context, InputValueType::Choice(values.len() as u8));
                let value = input
                    .read(&context)
                    .map(|input_value| match input_value.as_ref() {
                        InputValue::Choice { index, .. } => values[*index as usize],
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
            Some(MmioModel::BitExtract(be)) => {
                let context = InputContext::from_access(context, InputValueType::Bits(be.bits()));
                let value = input
                    .read(&context)
                    .map(|input_value| match input_value.as_ref() {
                        InputValue::Bits { value, .. } => be.apply(*value),
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
            None => {
                let context = InputContext::from_access(context, size.into());
                let value = input
                    .read(&context)
                    .map_or(Some(0),|input_value| match input_value.as_ref() {
                        InputValue::Byte(value) => Some(*value as u32),
                        InputValue::Word(value) => Some(*value as u32),
                        InputValue::DWord(value) => Some(*value),
                        _ => unreachable!("invalid InputValue type"),
                    });

                input_context = Some(context);
                value
            }
        };
        log::trace!("[READ] {:x?} => {:x?}", context, value);

        Ok(value.map(|value| {
            // track mmio accesses
            let input_value = if let Some(context) = input_context {
                self.access_log.push(context);
                true
            } else {
                false
            };
            // println!("This is value: {}", value);

            (value, input_value)
        }))
    }
    pub fn mmio_write(&mut self, context: &AccessContext, data: USize, size: ReadSize) {
        let mmio = context.mmio();
        if self.modeling.is_passthrough(mmio.addr()) {
            // TODO: overlapping initial_values with values != 0 can cause issues
            // this should never happen with fuzzware models, but a warning/error would be nice
            self.memory.write(mmio.addr(), data, size);
        }
        // handle timer event enable
        if context.mmio().addr() == 0x40008304 {
            self.event_en_nrf0 = data;
        }else if context.mmio().addr() == 0x40009304 {
            self.event_en_nrf1 = data;
        }else if context.mmio().addr() == 0x4000A304 {
            self.event_en_nrf2 = data;
        }else if context.mmio().addr() == 0x4001A304 {
            self.event_en_nrf3 = data;
        }else if context.mmio().addr() == 0x4001B304 {
            self.event_en_nrf4 = data;
        }
        // handle egu event check
        else if context.mmio().addr() == 0x4001413C {
            self.egu_event_check0 = data;
        }else if context.mmio().addr() == 0x4001410C {
            self.egu_event_check2 = data;
        }else if context.mmio().addr() == 0x40014100 {
            self.egu_event_check3 = data;
        }else if context.mmio().addr() == 0x40014108 {
            self.egu_event_check3 = data;
        }
        log::trace!("[WRITE] {:x?} data: {:x}", context, data);
    }

    pub fn snapshot_create(&self) -> HardwareSnapshot {
        HardwareSnapshot {
            memory: self.memory.clone(),
        }
    }

    pub fn snapshot_restore(&mut self, snapshot: &HardwareSnapshot) {
        let HardwareSnapshot { memory } = snapshot;

        self.memory = memory.clone();
    }
}

impl Memory {
    fn new() -> Self {
        Self {
            data: FxHashMap::default(),
        }
    }

    fn read(&self, address: Address, size: ReadSize) -> Option<USize> {
        let value = self
            .data
            .get(&aligned(address))
            .map(|raw_data| raw_data.shr(memory_shift(address)) & size.mask());
        log::trace!(
            "read: address = {:08x?}, size = {:?}, value = {:08x?}",
            address,
            size,
            value,
        );

        value
    }

    fn write(&mut self, address: Address, value: USize, size: ReadSize) {
        let shift = memory_shift(address);
        let mask = size.mask() << shift;
        let old_data = self.data.get(&aligned(address)).copied().unwrap_or(0);
        let other_data = old_data & !mask;
        let new_data = ((value << shift) & mask) | other_data;
        log::trace!(
            "write: address = {:08x?}, size = {:?}, value = {:08x?}, old_data = {:08x?}, new_data = {:08x?}",
            address,
            size,
            value,
            old_data,
            new_data,
        );

        self.data.insert(aligned(address), new_data);
    }
}

fn memory_shift(address: Address) -> u32 {
    let byte_offset = address - aligned(address);
    byte_offset * u8::BITS
}
