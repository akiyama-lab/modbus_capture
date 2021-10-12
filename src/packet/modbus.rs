use std::fmt;
use pnet_macros::packet;
use pnet_macros_support::types::*;
use pnet::packet::PrimitiveValues;

/// Documentation for ModbusFunctionCode
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct ModbusFunctionCode(pub u8);

impl ModbusFunctionCode {
    pub fn new(field_val: u8) -> ModbusFunctionCode {
        ModbusFunctionCode(field_val)
    }
}

impl PrimitiveValues for ModbusFunctionCode {
    type T = (u8,);
    fn to_primitive_values(&self) -> (u8,) {
        (self.0,)
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ModbusFunctionCodeValues {
    use super::ModbusFunctionCode;

    pub const ReadCoils: ModbusFunctionCode = ModbusFunctionCode(1);
    pub const ReadDiscreteInputs: ModbusFunctionCode = ModbusFunctionCode(2);
    pub const ReadHoldingRegisters: ModbusFunctionCode = ModbusFunctionCode(3);
    pub const ReadInputRegisters: ModbusFunctionCode = ModbusFunctionCode(4);
    pub const WriteSingleCoil: ModbusFunctionCode = ModbusFunctionCode(5);
    pub const WriteSingleRegister: ModbusFunctionCode = ModbusFunctionCode(6);
    pub const ReadExceptionStatus: ModbusFunctionCode = ModbusFunctionCode(7);
    pub const DiagnosticStatus: ModbusFunctionCode = ModbusFunctionCode(8);
    pub const GetCommEventCounter: ModbusFunctionCode = ModbusFunctionCode(11);
    pub const GetCommEventLog: ModbusFunctionCode = ModbusFunctionCode(12);
    pub const WriteMultipleCoils: ModbusFunctionCode = ModbusFunctionCode(15);
    pub const WriteMultipleRegisters: ModbusFunctionCode = ModbusFunctionCode(16);
    pub const ReportSlaveId: ModbusFunctionCode = ModbusFunctionCode(17);
    pub const ReadFileRecord: ModbusFunctionCode = ModbusFunctionCode(20);
    pub const WriteFileRecord: ModbusFunctionCode = ModbusFunctionCode(21);
    pub const MaskWriteRegister: ModbusFunctionCode = ModbusFunctionCode(22);
    pub const ReadWriteMultipleRegisters: ModbusFunctionCode = ModbusFunctionCode(23);
    pub const ReadFifoQueue: ModbusFunctionCode = ModbusFunctionCode(24);
    pub const ReadDeviceInformation: ModbusFunctionCode = ModbusFunctionCode(43);
}

impl fmt::Display for ModbusFunctionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &ModbusFunctionCodeValues::ReadCoils => "ReadCoils", // 1
                   &ModbusFunctionCodeValues::ReadDiscreteInputs => "ReadDiscreteInputs", // 2
                   &ModbusFunctionCodeValues::ReadHoldingRegisters => "ReadHoldingRegisters", // 3
                   &ModbusFunctionCodeValues::ReadInputRegisters => "ReadInputRegisters", // 4
                   &ModbusFunctionCodeValues::WriteSingleCoil => "WriteSingleCoil", // 5
                   &ModbusFunctionCodeValues::WriteSingleRegister => "WriteSingleRegister", // 6
                   &ModbusFunctionCodeValues::ReadExceptionStatus => "ReadExceptionStatus", // 7
                   &ModbusFunctionCodeValues::DiagnosticStatus => "DiagnosticStatus", // 8
                   &ModbusFunctionCodeValues::GetCommEventCounter => "GetCommEventCounter", // 11
                   &ModbusFunctionCodeValues::GetCommEventLog => "GetCommEventLog", // 12
                   &ModbusFunctionCodeValues::WriteMultipleCoils => "WriteMultipleCoils", // 15
                   &ModbusFunctionCodeValues::WriteMultipleRegisters => "WriteMultipleRegisters", // 16
                   &ModbusFunctionCodeValues::ReportSlaveId => "ReportSlaveId", // 17
                   &ModbusFunctionCodeValues::ReadFileRecord => "ReadFileRecord", // 20
                   &ModbusFunctionCodeValues::WriteFileRecord => "WriteFileRecord", // 21
                   &ModbusFunctionCodeValues::MaskWriteRegister => "MaskWriteRegister", // 22
                   &ModbusFunctionCodeValues::ReadWriteMultipleRegisters => "ReadWriteMultipleRegisters", // 23
                   &ModbusFunctionCodeValues::ReadFifoQueue => "ReadFifoQueue", // 24
                   &ModbusFunctionCodeValues::ReadDeviceInformation => "ReadDeviceInformation", // 43
                   _ => "unknown",
               })
    }
}

#[packet]
pub struct Modbus {
    tid: u16be,
    pid: u16be,
    length: u16be,
    uid: u8,
    #[construct_with(u8)]
    function_code: ModbusFunctionCode,
    #[payload]
    payload: Vec<u8>,
}
