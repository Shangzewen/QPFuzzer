// #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
use std::collections::HashMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;

pub type USize = u32;
pub type ISize = i32;
pub type Address = USize;
pub type MmioAddress = Address;

pub struct AccessContext {
    pc: Address,
    // #[serde(flatten)]
    mmio: MmioContext,
}

impl AccessContext {
    pub fn new(pc: Address, mmio: MmioAddress) -> Self {
        Self {
            pc,
            mmio: MmioContext::new(mmio),
        }
    }

    pub fn pc(&self) -> Address {
        self.pc
    }

    pub fn mmio(&self) -> &MmioContext {
        &self.mmio
    }
}

// impl fmt::Display for AccessContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:x}_{}", self.pc, self.mmio)
//     }
// }

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MmioContext {
    // #[serde(rename = "mmio")]
    addr: MmioAddress,
}

impl MmioContext {
    pub fn new(addr: MmioAddress) -> Self {
        Self { addr }
    }

    pub fn addr(&self) -> MmioAddress {
        self.addr
    }

}

// impl fmt::Display for MmioContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:x}", self.addr as u8)
//     }
// }

lazy_static! {
    static ref global_dic: Mutex<HashMap<MmioContext, i32>> = Mutex::new(HashMap::new());
}
fn main() {
    // println!("Hello, world!");
    // global_dic.lock().insert(MmioContext::new(0x12345678), 1);
    // global_dic.lock().contains_key(k);
    let test_mmio = MmioContext::new(0x12345678);
    let mut binding = global_dic.lock();
    binding.insert(test_mmio.clone(), 1);

    // let mut test_dic = HashMap::new();
    // test_dic.insert(&test_mmio, 1);
    // let clone_test_mmio = &test_mmio.clone();
    let value = binding.get_key_value(&test_mmio).unwrap().1;
    println!("{}", value);

    if binding.contains_key(&test_mmio) {
        binding.get_mut(&test_mmio).map(|v| *v =5);
    }
    let val_2 = binding.get_key_value(&test_mmio).unwrap().1;
    println!("{}", val_2);
    // let real_val = &value.1;
    // binding.insert(test_mmio.clone(), 2);
    // let clone_test_mmio = &test_mmio.clone();
    // let Some(heihei) = binding.get(&clone_test_mmio) else{return};
}
