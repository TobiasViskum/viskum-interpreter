#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::tests::{ pack_bools, unpack_bools };

    const LOOP_ITERATIONS: usize = 100_000_000;

    #[test]
    fn approach_1_1() {
        type Bytecode = (u8, usize, usize, usize);

        let instructions: &[Bytecode] = &vec![
            (1, 23, 32, 22),
            (2, 343, 23, 32),
            (3, 23, 32, 22),
            (4, 343, 23, 32),
            (5, 23, 32, 22),
            (6, 343, 23, 32),
            (7, 23, 32, 22),
            (8, 343, 23, 32)
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op, dst, reg1, reg2) = instructions[ip];

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_2() {
        type Bytecode = [u8; 8];

        let instructions: &[Bytecode] = &vec![
            [1, 9, 4, 135, 99, 43, 43, 2],
            [2, 33, 4, 3, 5, 7, 55, 3],
            [3, 34, 4, 36, 5, 73, 54, 22],
            [4, 7, 4, 44, 5, 7, 54, 12],
            [5, 84, 4, 45, 34, 7, 54, 02],
            [6, 5, 4, 77, 5, 7, 54, 01],
            [7, 33, 34, 55, 88, 89, 54, 33],
            [8, 65, 34, 55, 5, 54, 54, 98]
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = instructions[ip];
            let op = instruction[0];
            let dst = ((instruction[2] as usize) >> 8) | (instruction[1] as usize);
            let reg1 = ((instruction[4] as usize) >> 8) | (instruction[3] as usize);
            let reg2 = ((instruction[6] as usize) >> 8) | (instruction[5] as usize);

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_3() {
        type Bytecode = u64;

        let instructions: &[Bytecode] = &vec![
            (1u64 << 56) | (23u64 << 40) | (32u64 << 24) | (22u64 << 8),
            (2u64 << 56) | (343u64 << 40) | (23u64 << 24) | (32u64 << 8),
            (3u64 << 56) | (23u64 << 40) | (32u64 << 24) | (22u64 << 8),
            (4u64 << 56) | (343u64 << 40) | (23u64 << 24) | (32u64 << 8),
            (5u64 << 56) | (23u64 << 40) | (32u64 << 24) | (22u64 << 8),
            (6u64 << 56) | (343u64 << 40) | (23u64 << 24) | (32u64 << 8),
            (7u64 << 56) | (23u64 << 40) | (32u64 << 24) | (22u64 << 8),
            (8u64 << 56) | (343u64 << 40) | (23u64 << 24) | (32u64 << 8)
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = instructions[ip];
            let op = (instruction >> 56) as u8;
            let dst = ((instruction >> 40) & 0xffff) as usize;
            let reg1 = ((instruction >> 24) & 0xffff) as usize;
            let reg2 = ((instruction >> 8) & 0xffff) as usize;

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_4() {
        type Bytecode = (u16, u16, u16, u16);

        let instructions: &[Bytecode] = &vec![
            (1, 23, 32, 22),
            (2, 343, 23, 32),
            (3, 23, 32, 22),
            (4, 343, 23, 32),
            (5, 23, 32, 22),
            (6, 343, 23, 32),
            (7, 23, 32, 22),
            (8, 343, 23, 32)
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op, dst, reg1, reg2) = instructions[ip];

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_5() {
        type Bytecode = [usize; 4];

        let instructions: &[Bytecode] = &vec![
            [1, 23, 32, 22],
            [2, 343, 23, 32],
            [3, 23, 32, 22],
            [4, 343, 23, 32],
            [5, 23, 32, 22],
            [6, 343, 23, 32],
            [7, 23, 32, 22],
            [8, 343, 23, 32]
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = instructions[ip];
            let op = instruction[0];
            let dst = instruction[1];
            let reg1 = instruction[2];
            let reg2 = instruction[3];

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_6() {
        type Bytecode = Vec<u32>;

        let instructions: &[Bytecode] = &vec![
            vec![1, 23, 32, 22],
            vec![2, 343, 23, 32],
            vec![3, 23, 32, 22],
            vec![4, 343, 23, 32],
            vec![5, 23, 32, 22],
            vec![6, 343, 23, 32],
            vec![7, 23, 32, 22],
            vec![8, 343, 23, 32]
        ];

        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = &instructions[ip];
            let op = instruction[0];
            let dst = instruction[1];
            let reg1 = instruction[2];
            let reg2 = instruction[3];

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_7() {
        type Bytecode = (u32, u32);

        let instructions: &[Bytecode] = &vec![
            ((1u32 << 24) | 23, (32u32 << 24) | 22),
            ((2u32 << 24) | 343, (23u32 << 24) | 32),
            ((3u32 << 24) | 23, (32u32 << 24) | 22),
            ((4u32 << 24) | 343, (23u32 << 24) | 32),
            ((5u32 << 24) | 23, (32u32 << 24) | 22),
            ((6u32 << 24) | 343, (23u32 << 24) | 32),
            ((7u32 << 24) | 23, (32u32 << 24) | 22),
            ((8u32 << 24) | 343, (23u32 << 24) | 32)
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op_dst, reg1_reg2) = instructions[ip];
            let op = (op_dst >> 24) as u8;
            let dst = (op_dst & 0xffffff) as usize;
            let reg1 = (reg1_reg2 >> 24) as usize;
            let reg2 = (reg1_reg2 & 0xffffff) as usize;

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_8() {
        type Bytecode = [u16; 4];

        let instructions: &[Bytecode] = &vec![
            [1, 23, 32, 22],
            [2, 343, 23, 32],
            [3, 23, 32, 22],
            [4, 343, 23, 32],
            [5, 23, 32, 22],
            [6, 343, 23, 32],
            [7, 23, 32, 22],
            [8, 343, 23, 32]
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = instructions[ip];
            let op = instruction[0];
            let dst = instruction[1] as usize;
            let reg1 = instruction[2] as usize;
            let reg2 = instruction[3] as usize;

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_9() {
        type Bytecode = (u32, u32, u32, u32);

        let instructions: &[Bytecode] = &vec![
            (1, 23, 32, 22),
            (2, 343, 23, 32),
            (3, 23, 32, 22),
            (4, 343, 23, 32),
            (5, 23, 32, 22),
            (6, 343, 23, 32),
            (7, 23, 32, 22),
            (8, 343, 23, 32)
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op, dst, reg1, reg2) = instructions[ip];

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_10() {
        type Bytecode = [u32; 4];

        let instructions: &[Bytecode] = &vec![
            [1, 23, 32, 22],
            [2, 343, 23, 32],
            [3, 23, 32, 22],
            [4, 343, 23, 32],
            [5, 23, 32, 22],
            [6, 343, 23, 32],
            [7, 23, 32, 22],
            [8, 343, 23, 32]
        ];
        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let instruction = instructions[ip];
            let op = instruction[0];
            let dst = instruction[1] as usize;
            let reg1 = instruction[2] as usize;
            let reg2 = instruction[3] as usize;

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_11() {
        type Bytecode = (u8, u64);

        let instructions: &[Bytecode] = &[
            (1, (1u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64),
            (2, (2u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64),
            (3, (3u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64),
            (4, (4u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64),
            (5, (5u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64),
            (6, (6u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64),
            (7, (7u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64),
            (8, (8u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64),
        ];

        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = std::time::Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op, instruction) = instructions[ip];
            let dst = ((instruction >> 48) & 0xffff) as usize;
            let reg1 = ((instruction >> 32) & 0xffff) as usize;
            let reg2 = ((instruction >> 16) & 0xffff) as usize;

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn approach_3_half_instructions() {
        type Bytecode = (u8, u64, u8, u64);

        let instructions: &[Bytecode] = &[
            (
                1,
                (1u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64,
                2,
                (2u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64,
            ),
            (
                3,
                (3u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64,
                4,
                (4u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64,
            ),
            (
                5,
                (5u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64,
                6,
                (6u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64,
            ),
            (
                7,
                (7u64 << 48) | (23u64 << 32) | (32u64 << 16) | 22u64,
                8,
                (8u64 << 48) | (343u64 << 32) | (23u64 << 16) | 32u64,
            ),
        ];

        assert_eq!(instructions.len(), 4);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = std::time::Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op1, instruction1, op2, instruction2) = instructions[ip];
            let dst1 = ((instruction1 >> 48) & 0xffff) as usize;
            let reg1_1 = ((instruction1 >> 32) & 0xffff) as usize;
            let reg2_1 = ((instruction1 >> 16) & 0xffff) as usize;

            let dst2 = ((instruction2 >> 48) & 0xffff) as usize;
            let reg1_2 = ((instruction2 >> 32) & 0xffff) as usize;
            let reg2_2 = ((instruction2 >> 16) & 0xffff) as usize;

            match op1 {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            match op2 {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1;
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn custom_bytecode_test_2() {
        type Bytecode = (Instruction, Instruction);
        type Instruction = (u8, u16, u16, u16, u8);

        let instructions: &[Bytecode] = &[
            (
                (1, 23, 32, 22, pack_bools(true, false, true)),
                (2, 343, 23, 32, pack_bools(true, false, true)),
            ),
            (
                (3, 23, 32, 22, pack_bools(true, true, false)),
                (4, 343, 23, 32, pack_bools(true, false, true)),
            ),
            (
                (5, 23, 32, 22, pack_bools(true, false, true)),
                (6, 343, 23, 32, pack_bools(true, false, true)),
            ),
            (
                (7, 23, 32, 22, pack_bools(true, false, true)),
                (8, 343, 23, 32, pack_bools(true, false, true)),
            ),
        ];

        assert_eq!(instructions.len(), 4);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = std::time::Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let ((op1, dst1, reg1_1, reg2_1, bool1), (op2, dst2, reg1_2, reg2_2, bool2)) =
                instructions[ip];
            let dst1 = dst1 as usize;
            let reg1_1 = reg1_1 as usize;
            let reg2_1 = reg2_1 as usize;
            let (dst_rel_1, reg_rel_1_1, reg_rel_2_1) = unpack_bools(bool1);

            let dst2 = dst2 as usize;
            let reg1_2 = reg1_2 as usize;
            let reg2_2 = reg2_2 as usize;
            let (dst_rel_2, reg_rel_1_2, reg_rel_2_2) = unpack_bools(bool2);

            match op1 {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            match op2 {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1; // Resetting index if it reaches the end
        }
        println!("{:?}", now.elapsed());
    }

    #[test]
    fn custom_bytecode_test_single_instruction() {
        type Bytecode = (u8, u16, u16, u16, u8);

        let instructions: &[Bytecode] = &[
            (1, 23, 32, 22, pack_bools(true, false, true)),
            (2, 23, 32, 22, pack_bools(true, false, true)),
            (3, 23, 32, 22, pack_bools(true, false, true)),
            (4, 23, 32, 22, pack_bools(true, false, true)),
            (5, 23, 32, 22, pack_bools(true, false, true)),
            (6, 23, 32, 22, pack_bools(true, false, true)),
            (7, 23, 32, 22, pack_bools(true, false, true)),
            (8, 23, 32, 22, pack_bools(true, false, true)),
        ];

        assert_eq!(instructions.len(), 8);

        let mut reached_7_amount = 0;
        let mut ip = 0;
        let now = std::time::Instant::now();
        while reached_7_amount < LOOP_ITERATIONS {
            let (op, dst, reg1, reg2, bools) = instructions[ip];
            let dst = dst as usize;
            let reg1 = reg1 as usize;
            let reg2 = reg2 as usize;
            let (dst_rel, reg_rel_1, reg_rel_2) = unpack_bools(bools);

            match op {
                7 => {
                    reached_7_amount += 1;
                    ip = 0;
                }
                5 => {}
                3 => {}
                9 => {}
                2 => {}
                _ => {}
            }

            ip += 1; // Resetting index if it reaches the end
        }
        println!("{:?}", now.elapsed());
    }
}

fn pack_bools(bool1: bool, bool2: bool, bool3: bool) -> u8 {
    let mut packed = 0;
    if bool1 {
        packed |= 0b0000_0001;
    }
    if bool2 {
        packed |= 0b0000_0010;
    }
    if bool3 {
        packed |= 0b0000_0100;
    }
    packed
}

// Function to unpack three boolean values from a single u8
fn unpack_bools(byte: u8) -> (bool, bool, bool) {
    let bool1 = (byte & 0b0000_0001) != 0;
    let bool2 = (byte & 0b0000_0010) != 0;
    let bool3 = (byte & 0b0000_0100) != 0;
    (bool1, bool2, bool3)
}
