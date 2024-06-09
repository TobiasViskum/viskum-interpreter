#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Halt,
    LoadBool,
    LoadInt32,
    LoadString,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Not,
    Jmp,
    JmpPop,
    CmpEq,
    CmpNeq,
    CmpGt,
    CmpGeq,
    CmpLt,
    CmpLeq,
    CmpEqJmp,
    CmpNeqJmp,
    CmpGtJmp,
    CmpGeqJmp,
    CmpLtJmp,
    CmpLeqJmp,
    Return,
    ReturnPop,
    Pop,
    PushStack,
    LoadStack,
    PushFunction,
    CallFunction,
}

impl PartialEq for Opcode {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }

    fn ne(&self, other: &Self) -> bool {
        (*self as u8) != (*other as u8)
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Halt,
            1 => Opcode::LoadBool,
            2 => Opcode::LoadInt32,
            3 => Opcode::LoadString,
            4 => Opcode::Add,
            5 => Opcode::Sub,
            6 => Opcode::Mul,
            7 => Opcode::Div,
            8 => Opcode::Neg,
            9 => Opcode::Not,
            10 => Opcode::Jmp,
            11 => Opcode::JmpPop,
            12 => Opcode::CmpEq,
            13 => Opcode::CmpNeq,
            14 => Opcode::CmpGt,
            15 => Opcode::CmpGeq,
            16 => Opcode::CmpLt,
            17 => Opcode::CmpLeq,
            18 => Opcode::CmpEqJmp,
            19 => Opcode::CmpNeqJmp,
            20 => Opcode::CmpGtJmp,
            21 => Opcode::CmpGeqJmp,
            22 => Opcode::CmpLtJmp,
            23 => Opcode::CmpLeqJmp,
            24 => Opcode::Return,
            25 => Opcode::ReturnPop,
            26 => Opcode::Pop,
            27 => Opcode::PushStack,
            28 => Opcode::LoadStack,
            29 => Opcode::PushFunction,
            30 => Opcode::CallFunction,
            _ => panic!("Invalid opcode value: {}", value),
        }
    }
}

const INSTR_BITS: usize = 64;
const U48_MAX: u64 = 281_474_976_710_656;
pub const U40_MAX: u64 = 1_099_511_627_776;
type InstrSize = u64;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FastInstr(InstrSize);

impl FastInstr {
    // Instruction CREATE:
    pub fn new_halt() -> Self {
        Self(Opcode::Halt as u8 as InstrSize)
    }

    pub fn new_binary(opcode: Opcode, dst: u8, reg1: u8, reg2: u8) -> Self {
        Self(
            Self::place_b1(opcode as u8) |
                Self::place_b2(dst) |
                Self::place_b3(reg1) |
                Self::place_b4(reg2)
        )
    }

    pub fn new_unary(opcode: Opcode, dst: u8, reg: u8) -> Self {
        Self(Self::place_b1(opcode as u8) | Self::place_b2(dst) | Self::place_b3(reg))
    }

    pub fn new_load_bool(dst: u8, bool: bool) -> FastInstr {
        Self(
            Self::place_b1(Opcode::LoadBool as u8) |
                Self::place_b2(dst) |
                Self::place_b3(bool as u8)
        )
    }

    pub fn new_load_i32(dst: u8, int32: i32) -> FastInstr {
        let load_instr = Self::new_load(Opcode::LoadInt32, dst, 1).0 | ((int32 as InstrSize) << 32);

        Self(load_instr)
    }

    pub fn new_load_i32_v2(dst: u8, const_table_pos: u64) -> FastInstr {
        if const_table_pos > U40_MAX {
            panic!("Greater than u40 max in load_i32");
        }

        Self(
            Self::place_b1(Opcode::LoadInt32 as u8) | Self::place_b2(dst) | (const_table_pos << 24)
        )
    }

    pub fn new_load_string(dst: u8, string: String) -> (FastInstr, Vec<FastInstr>) {
        let bytes = string.as_bytes().to_vec();
        let load_instr_amount = ((string.len() + (INSTR_BITS / 8 - 1)) / (INSTR_BITS / 8)) as u16;

        let load_instr = Self::new_load(Opcode::LoadString, dst, load_instr_amount);
        let mut instructions = Vec::with_capacity(load_instr_amount as usize);

        let mut count = 0;
        let mut current_chunk: InstrSize = 0;
        for byte in bytes {
            if count == INSTR_BITS / 8 {
                count = 0;
                instructions.push(FastInstr::new_value_holder(current_chunk));
                current_chunk = 0;
            }

            current_chunk = current_chunk | ((byte as InstrSize) << (8 * count));

            count += 1;
        }

        if current_chunk != 0 {
            instructions.push(FastInstr::new_value_holder(current_chunk));
        }

        (load_instr, instructions)
    }

    fn new_load(opcode: Opcode, dst: u8, load_instr_amount: u16) -> Self {
        Self(
            Self::place_b1(opcode as u8) | Self::place_b2(dst) | Self::place_b34(load_instr_amount)
        )
    }

    fn new_value_holder(bytes: InstrSize) -> Self {
        Self(bytes as InstrSize)
    }

    pub fn new_cmp(opcode: Opcode, dst: u8, reg1: u8, reg2: u8) -> Self {
        Self(
            Self::place_b1(opcode as u8) |
                Self::place_b2(dst) |
                Self::place_b3(reg1) |
                Self::place_b4(reg2)
        )
    }

    pub fn new_cmp_jmp(opcode: Opcode, reg1: u8, reg2: u8, true_pos: u16, false_pos: u16) -> Self {
        Self(
            Self::place_b1(opcode as u8) |
                Self::place_b2(reg1) |
                Self::place_b3(reg2) |
                Self::place_b45(true_pos) |
                Self::place_b67(false_pos)
        )
    }

    pub fn new_jmp(jmp_pos: u16) -> Self {
        Self(Self::place_b1(Opcode::Jmp as u8) | Self::place_b23(jmp_pos))
    }

    pub fn new_jmp_pop(jmp_pos: u16, amount: u16) -> Self {
        Self(Self::place_b1(Opcode::Jmp as u8) | Self::place_b23(jmp_pos) | Self::place_b45(amount))
    }

    pub fn new_pop(amount: u16) -> Self {
        Self(Self::place_b1(Opcode::Pop as u8) | Self::place_b23(amount))
    }

    pub fn new_return(reg: u8) -> Self {
        Self(Self::place_b1(Opcode::Return as u8) | Self::place_b2(reg))
    }

    pub fn new_return_pop(reg: u8, amount: u16) -> Self {
        Self(
            Self::place_b1(Opcode::ReturnPop as u8) | Self::place_b2(reg) | Self::place_b34(amount)
        )
    }

    pub fn new_load_stack(dst: u8, is_relative: bool, stack_pos: u64) -> Self {
        if stack_pos > U40_MAX {
            panic!("LOAD_STACK: Stack position is greater than u40: {}", U40_MAX);
        }
        Self(
            Self::place_b1(Opcode::LoadStack as u8) |
                Self::place_b2(dst) |
                Self::place_b3(is_relative as u8) |
                (stack_pos << 24)
        )
    }

    pub fn new_push_stack(reg: u8) -> Self {
        Self(Self::place_b1(Opcode::PushStack as u8) | Self::place_b2(reg))
    }

    pub fn new_push_function(instr_amount: u32) -> Self {
        Self(Self::place_b1(Opcode::PushFunction as u8) | Self::place_b3456(instr_amount))
    }

    pub fn new_call_function(stack_pos: u64) -> Self {
        if stack_pos > U40_MAX {
            panic!("CALL_FUNCTION: Stack position is greater than u40: {}", U40_MAX);
        }
        Self(Self::place_b1(Opcode::CallFunction as u8) | (stack_pos << 24))
    }

    // Instruction READ (used in vm so therefore most are inlined)
    #[inline(always)]
    pub fn get_binary(&self) -> (u8, u8, u8) {
        (self.get_b2(), self.get_b3(), self.get_b4())
    }

    #[inline(always)]
    pub fn get_unary(&self) -> (u8, u8) {
        (self.get_b2(), self.get_b3())
    }

    #[inline(always)]
    pub fn get_load_i32(&self) -> (u8, i32) {
        (self.get_b2(), self.get_b5678() as i32)
    }

    #[inline(always)]
    pub fn get_load_i32_v2(&self) -> (u8, u64) {
        (self.get_b2(), self.get_b45678())
    }

    pub fn get_load_string(&self, others: Vec<&Self>) -> (u8, String) {
        let mut all_bytes: Vec<u8> = Vec::with_capacity(others.len() * 8);

        for instr in &others {
            let bytes = instr.0.to_le_bytes();
            for u8 in bytes {
                all_bytes.push(u8);
            }
        }

        (self.get_b2(), String::from_utf8_lossy(&all_bytes).replace("\0", ""))
    }

    #[inline(always)]
    pub fn get_string_instr_amount(&self) -> u16 {
        self.get_b34()
    }

    #[inline(always)]
    pub fn get_load_bool(&self) -> (u8, bool) {
        (self.get_b2(), self.get_b3() != 0)
    }

    #[inline(always)]
    pub fn get_cmp(&self) -> (u8, u8, u8) {
        (self.get_b2(), self.get_b3(), self.get_b4())
    }

    #[inline(always)]
    pub fn get_cmp_jmp(&self) -> (u8, u8, u16, u16) {
        (self.get_b2(), self.get_b3(), self.get_b45(), self.get_b67())
    }

    #[inline(always)]
    pub fn get_jmp(&self) -> u16 {
        self.get_b23()
    }

    #[inline(always)]
    pub fn get_jmp_pop(&self) -> (u16, u16) {
        (self.get_b23(), self.get_b45())
    }

    #[inline(always)]
    pub fn get_pop(&self) -> u16 {
        self.get_b23()
    }

    #[inline(always)]
    pub fn get_return(&self) -> u8 {
        self.get_b2()
    }

    #[inline(always)]
    pub fn get_return_pop(&self) -> (u8, u16) {
        (self.get_b2(), self.get_b34())
    }

    #[inline(always)]
    pub fn get_load_stack(&self) -> (u8, bool, u64) {
        (self.get_b2(), self.get_b3() != 0, self.get_b45678())
    }

    #[inline(always)]
    pub fn get_push_stack(&self) -> u8 {
        self.get_b2()
    }

    #[inline(always)]
    pub fn get_push_function(&self) -> u32 {
        self.get_b3456()
    }

    #[inline(always)]
    pub fn get_call_function(&self) -> u64 {
        self.get_b45678()
    }

    // To get the opcode

    #[inline(always)]
    pub fn get_op(&self) -> Opcode {
        Opcode::from(self.get_b1())
    }

    // Private read methods
    #[inline(always)]
    fn get_b1(&self) -> u8 {
        (self.0 & 0xff) as u8
    }

    #[inline(always)]
    fn get_b2(&self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b3(&self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b4(&self) -> u8 {
        ((self.0 >> 24) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b5(&self) -> u8 {
        ((self.0 >> 32) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b6(&self) -> u8 {
        ((self.0 >> 40) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b7(&self) -> u8 {
        ((self.0 >> 48) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b8(&self) -> u8 {
        ((self.0 >> 56) & 0xff) as u8
    }

    #[inline(always)]
    fn get_b12(&self) -> u16 {
        (self.0 & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b23(&self) -> u16 {
        ((self.0 >> 8) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b34(&self) -> u16 {
        ((self.0 >> 16) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b45(&self) -> u16 {
        ((self.0 >> 24) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b56(&self) -> u16 {
        ((self.0 >> 32) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b67(&self) -> u16 {
        ((self.0 >> 40) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b78(&self) -> u16 {
        ((self.0 >> 48) & 0xffff) as u16
    }

    #[inline(always)]
    fn get_b3456(&self) -> u32 {
        ((self.0 >> 16) & 0xffffffff) as u32
    }

    #[inline(always)]
    fn get_b5678(&self) -> u32 {
        ((self.0 >> 32) & 0xffffffff) as u32
    }

    #[inline(always)]
    fn get_b345678(&self) -> u64 {
        (self.0 >> 16) & 0xffffffffffff
    }

    #[inline(always)]
    fn get_b45678(&self) -> u64 {
        (self.0 >> 24) & 0xffffffffff
    }

    // Public modify method
    pub fn modify_jmp(&mut self, jmp_pos: u16) {
        // Either JMP or JMP_POP
        self.modify_u16(jmp_pos, 2)
    }

    pub fn modify_cmp_jmp(&mut self, true_pos: u16, false_pos: u16) {
        self.modify_u16(true_pos, 4);
        self.modify_u16(false_pos, 6);
    }

    // Private modify method
    fn modify_u8(&mut self, byte: u8, byte_pos: usize) {
        let shift_amount = (byte_pos - 1) * 8;
        let new_byte = (byte as InstrSize) << shift_amount;
        let saved_bytes_before =
            (self.0 << (INSTR_BITS - shift_amount)) >> (INSTR_BITS - shift_amount);
        let saved_bytes_after = (self.0 >> (shift_amount + 8)) << (shift_amount + 8);

        self.0 = new_byte | saved_bytes_before | saved_bytes_after;
    }

    fn modify_u16(&mut self, byte: u16, byte_pos: usize) {
        let shift_amount = (byte_pos - 1) * 8;
        let new_byte = (byte as InstrSize) << shift_amount;
        let saved_bytes_before =
            (self.0 << (INSTR_BITS - shift_amount)) >> (INSTR_BITS - shift_amount);

        let saved_bytes_after = (self.0 >> (shift_amount + 16)) << (shift_amount + 16);

        self.0 = new_byte | saved_bytes_before | saved_bytes_after;
    }

    // Private init write methods
    #[inline(always)]
    fn place_b1(byte: u8) -> InstrSize {
        byte as InstrSize
    }

    #[inline(always)]
    fn place_b2(byte: u8) -> InstrSize {
        (byte as InstrSize) << 8
    }

    #[inline(always)]
    fn place_b3(byte: u8) -> InstrSize {
        (byte as InstrSize) << 16
    }

    #[inline(always)]
    fn place_b4(byte: u8) -> InstrSize {
        (byte as InstrSize) << 24
    }

    #[inline(always)]
    fn place_b12(bytes: u16) -> InstrSize {
        bytes as InstrSize
    }

    #[inline(always)]
    fn place_b23(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 8
    }

    #[inline(always)]
    fn place_b34(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 16
    }

    #[inline(always)]
    fn place_b45(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 24
    }

    #[inline(always)]
    fn place_b56(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 32
    }

    #[inline(always)]
    fn place_b67(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 40
    }

    #[inline(always)]
    fn place_b78(bytes: u16) -> InstrSize {
        (bytes as InstrSize) << 48
    }

    #[inline(always)]
    pub fn place_b3456(bytes: u32) -> InstrSize {
        (bytes as InstrSize) << 16
    }
}

#[cfg(test)]
mod test {
    use crate::vm::instructions::fast_instr::Opcode;

    use super::FastInstr;

    #[test]
    fn test_instruction() {
        let opcode = Opcode::Add;
        let dst = 2;
        let reg1 = 0;
        let reg2 = 1;
        let mut instr = FastInstr::new_binary(opcode, dst, reg1, reg2);
        assert_eq!((dst, reg1, reg2), instr.get_binary());
        let reg2 = 8;
        instr.modify_u8(reg2, 4);
        assert_eq!(
            (opcode, dst, reg1, reg2),
            (instr.get_op(), instr.get_binary().0, instr.get_binary().1, instr.get_binary().2)
        );

        let dst = 5;
        let string = "Hellß∂œƒ≠“o wo∆sdöß∂©ß∂ƒ©∆rld!".to_string();
        let (main_load, value_loads) = FastInstr::new_load_string(dst, string.clone());
        assert_eq!(
            (dst, string),
            main_load.get_load_string(value_loads.iter().collect::<Vec<_>>())
        );

        let dst = 8;
        let value = 87;
        let instr = FastInstr::new_load_i32(dst, value);
        assert_eq!((dst, value), instr.get_load_i32());

        let dst = 3;
        let bool = false;
        let instr = FastInstr::new_load_bool(dst, bool);
        assert_eq!((dst, bool), instr.get_load_bool());

        let opcode = Opcode::CmpEq;
        let dst = 1;
        let reg1 = 2;
        let reg2 = 8;
        let instr = FastInstr::new_cmp(opcode, dst, reg1, reg2);
        assert_eq!((dst, reg1, reg2), instr.get_cmp());

        let opcode = Opcode::CmpEqJmp;
        let reg1 = 2;
        let reg2 = 8;
        let true_pos = 8237;
        let false_pos = 2973;
        let mut instr = FastInstr::new_cmp_jmp(opcode, reg1, reg2, true_pos, false_pos);
        assert_eq!((reg1, reg2, true_pos, false_pos), instr.get_cmp_jmp());
        let true_pos = 1834;
        instr.modify_cmp_jmp(true_pos, false_pos);
        assert_eq!(
            (opcode, reg1, reg2, true_pos, false_pos),
            (
                instr.get_op(),
                instr.get_cmp_jmp().0,
                instr.get_cmp_jmp().1,
                instr.get_cmp_jmp().2,
                instr.get_cmp_jmp().3,
            )
        );

        let opcode = Opcode::Not;
        let dst = 87;
        let reg = 23;
        let instr = FastInstr::new_unary(opcode, dst, reg);
        assert_eq!((dst, reg), instr.get_unary());

        let amount = 2398;
        let mut instr = FastInstr::new_pop(amount);
        assert_eq!(amount, instr.get_pop());
        let amount = 8473;
        instr.modify_u16(amount, 2);
        assert_eq!((Opcode::Pop, amount), (instr.get_op(), instr.get_pop()));

        let reg = 18;
        let instr = FastInstr::new_return(reg);
        assert_eq!(reg, instr.get_return());

        let reg = 23;
        let amount = 2323;
        let instr = FastInstr::new_return_pop(reg, amount);
        assert_eq!((reg, amount), instr.get_return_pop());

        let jmp_pos = 34;
        let mut instr = FastInstr::new_jmp(jmp_pos);
        assert_eq!(jmp_pos, instr.get_jmp());
        let jmp_pos = 89;
        instr.modify_jmp(jmp_pos);
        assert_eq!((Opcode::Jmp, jmp_pos), (instr.get_op(), instr.get_jmp()));

        let jmp_pos = 834;
        let amount = 2934;
        let mut instr = FastInstr::new_jmp_pop(jmp_pos, amount);
        assert_eq!((jmp_pos, amount), instr.get_jmp_pop());
        let jmp_pos = 7649;
        instr.modify_jmp(jmp_pos);
        assert_eq!((Opcode::Jmp, jmp_pos), (instr.get_op(), instr.get_jmp()));

        let dst = 84;
        let is_relative = false;
        let stack_pos = 23849433343;
        let instr = FastInstr::new_load_stack(dst, is_relative, stack_pos);
        assert_eq!((dst, is_relative, stack_pos), instr.get_load_stack());

        let reg = 23;
        let instr = FastInstr::new_push_stack(reg);
        assert_eq!(reg, instr.get_push_stack());

        let instr_amount = 283832;
        let instr = FastInstr::new_push_function(instr_amount);
        assert_eq!(instr_amount, instr.get_push_function());

        let stack_pos = 10493948738;
        let instr = FastInstr::new_call_function(stack_pos);
        assert_eq!(stack_pos, instr.get_call_function())
    }
}
