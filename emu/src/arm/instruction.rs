use std::fmt::{Display, Formatter};

use logger::log;

use crate::bitwise::Bits;

#[derive(Debug, PartialEq, Eq)]
pub enum ArmModeInstruction {
    DataProcessing,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    BranchAndExchange,
    HalfwordDataTransferRegisterOffset,
    HalfwordDataTransferImmediateOffset,
    SingleDataTransfer,
    Undefined,
    BlockDataTransfer,
    Branch,
    CoprocessorDataTransfer,
    CoprocessorDataOperation,
    CoprocessorRegisterTrasfer,
    SoftwareInterrupt,
}

impl From<u32> for ArmModeInstruction {
    fn from(op_code: u32) -> Self {
        use ArmModeInstruction::*;

        // NOTE: The order is based on how many bits are already know at decoding time.
        // It can happen `op_code` coalesced into one/two or more than two possible solution, that's because
        // we tried to order with this priority.
        if op_code.get_bits(4..=27) == 0b0001_0010_1111_1111_1111_0001 {
            BranchAndExchange
        } else if op_code.get_bits(23..=27) == 0b00010
            && op_code.get_bits(20..=21) == 0b00
            && op_code.get_bits(4..=11) == 0b0000_1001
        {
            SingleDataSwap
        } else if op_code.get_bits(22..=27) == 0b000000 && op_code.get_bits(4..=7) == 0b1001 {
            Multiply
        } else if op_code.get_bits(23..=27) == 0b00001 && op_code.get_bits(4..=7) == 0b1001 {
            MultiplyLong
        } else if op_code.get_bits(25..=27) == 0b000
            && !op_code.get_bit(22)
            && op_code.get_bits(7..=11) == 0b00001
            && op_code.get_bit(4)
        {
            HalfwordDataTransferRegisterOffset
        } else if op_code.get_bits(25..=27) == 0b000
            && op_code.get_bit(22)
            && op_code.get_bit(7)
            && op_code.get_bit(4)
        {
            HalfwordDataTransferImmediateOffset
        } else if op_code.get_bits(25..=27) == 0b011 && op_code.get_bit(4) {
            log("undefined instruction decode...");
            Undefined
        } else if op_code.get_bits(24..=27) == 0b1111 {
            SoftwareInterrupt
        } else if op_code.get_bits(24..=27) == 0b1110 && op_code.get_bit(4) {
            CoprocessorRegisterTrasfer
        } else if op_code.get_bits(24..=27) == 0b1110 && !op_code.get_bit(4) {
            CoprocessorDataOperation
        } else if op_code.get_bits(25..=27) == 0b110 {
            CoprocessorDataTransfer
        } else if op_code.get_bits(25..=27) == 0b100 {
            BlockDataTransfer
        } else if op_code.get_bits(25..=27) == 0b101 {
            Branch
        } else if op_code.get_bits(26..=27) == 0b01 {
            SingleDataTransfer
        } else if op_code.get_bits(26..=27) == 0b00 {
            DataProcessing
        } else {
            log("not identified instruction");
            unimplemented!()
        }
    }
}

impl Display for ArmModeInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{arm::instruction::ArmModeInstruction, arm::opcode::ArmModeOpcode};

    #[test]
    fn decode_half_word_data_transfer_immediate_offset() {
        let output: ArmModeInstruction = 0b1110_0001_1100_0001_0000_0000_1011_0000.into();
        assert_eq!(
            output,
            ArmModeInstruction::HalfwordDataTransferImmediateOffset
        );
    }

    // FIXME: Not sure about this, just because `BranchAndExchange` if is first.
    #[test]
    fn decode_branch_and_exchange() {
        let output: ArmModeOpcode = 0b1110_0001_0010_1111_1111_1111_0001_0001
            .try_into()
            .unwrap();
        assert_eq!(output.instruction, ArmModeInstruction::BranchAndExchange);
    }

    #[test]
    fn decode_branch_link() {
        let output: ArmModeOpcode = 0b1110_1011_0000_0000_0000_0000_0111_1111
            .try_into()
            .unwrap();
        assert_eq!(output.instruction, ArmModeInstruction::Branch);
    }
}
