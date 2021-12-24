/* This one boiled down to reversing the input.  My rules were:
 *
 * w5=w4
 * w7=w6-4
 * w8=w3-1
 * w10=w9+7
 * w11=w2-6
 * w12=w1+6
 * w13=w0+4
 *
 * For most significant digit w0, and least significant digit w13, ie
 * input = [w0 w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13]
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum InstructionType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Register {
    X,
    Y,
    Z,
    W,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum B {
    Reg(Register),
    Literal(i64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Instruction {
    itype: InstructionType,
    a: Register,
    b: Option<B>,
}

impl Instruction {
    fn new(itype: InstructionType, a_input: char, b_input: &str) -> Instruction {
        let a = match a_input {
            'x' => Register::X,
            'y' => Register::Y,
            'z' => Register::Z,
            'w' => Register::W,
            _ => panic!("Invalid 'a' char!"),
        };

        let mut b = None;
        if !b_input.is_empty() {
            if let Ok(bint) = b_input.parse::<i64>() {
                // Literal
                b = Some(B::Literal(bint));
            } else {
                // Register
                b = match b_input {
                    "x" => Some(B::Reg(Register::X)),
                    "y" => Some(B::Reg(Register::Y)),
                    "z" => Some(B::Reg(Register::Z)),
                    "w" => Some(B::Reg(Register::W)),
                    _ => panic!("Invalid 'b' char!"),
                }
            }
        }

        Instruction { itype, a, b }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    counter: usize,
    _input: Vec<i64>,
}

impl ALU {
    fn new() -> ALU {
        ALU {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            counter: 0,
            _input: vec![],
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        //println!("Instruction: {}", self.counter);
        //println!("  {:?}", inst);
        //println!("  {:?}", self);
        match inst.itype {
            InstructionType::Inp => {
                let input = self._input.pop();
                if let Some(input_value) = input {
                    match inst.a {
                        Register::X => self.x = input_value,
                        Register::Y => self.y = input_value,
                        Register::Z => self.z = input_value,
                        Register::W => self.w = input_value,
                    };
                } else {
                    panic!("No input passed!");
                }
            }
            InstructionType::Add => {
                if let Some(b) = inst.b {
                    let bnum;
                    match b {
                        B::Reg(reg) => {
                            match reg {
                                Register::X => bnum = self.x,
                                Register::Y => bnum = self.y,
                                Register::Z => bnum = self.z,
                                Register::W => bnum = self.w,
                            };
                        }
                        B::Literal(value) => {
                            bnum = value;
                        }
                    };

                    match inst.a {
                        Register::X => self.x += bnum,
                        Register::Y => self.y += bnum,
                        Register::Z => self.z += bnum,
                        Register::W => self.w += bnum,
                    };
                } else {
                    panic!("No b passed!");
                }
            }
            InstructionType::Mul => {
                if let Some(b) = inst.b {
                    let bnum;
                    match b {
                        B::Reg(reg) => {
                            match reg {
                                Register::X => bnum = self.x,
                                Register::Y => bnum = self.y,
                                Register::Z => bnum = self.z,
                                Register::W => bnum = self.w,
                            };
                        }
                        B::Literal(value) => {
                            bnum = value;
                        }
                    };

                    match inst.a {
                        Register::X => self.x *= bnum,
                        Register::Y => self.y *= bnum,
                        Register::Z => self.z *= bnum,
                        Register::W => self.w *= bnum,
                    };
                } else {
                    panic!("No b passed!");
                }
            }
            InstructionType::Div => {
                if let Some(b) = inst.b {
                    let bnum;
                    match b {
                        B::Reg(reg) => {
                            match reg {
                                Register::X => bnum = self.x,
                                Register::Y => bnum = self.y,
                                Register::Z => bnum = self.z,
                                Register::W => bnum = self.w,
                            };
                        }
                        B::Literal(value) => {
                            bnum = value;
                        }
                    };

                    if bnum == 0 {
                        println!("PC: {}", self.counter);
                        panic!("Cannot divide by 0!");
                    }

                    match inst.a {
                        Register::X => self.x /= bnum,
                        Register::Y => self.y /= bnum,
                        Register::Z => self.z /= bnum,
                        Register::W => self.w /= bnum,
                    };
                } else {
                    panic!("No b passed!");
                }
            }
            InstructionType::Mod => {
                if let Some(b) = inst.b {
                    let bnum;
                    match b {
                        B::Reg(reg) => {
                            match reg {
                                Register::X => bnum = self.x,
                                Register::Y => bnum = self.y,
                                Register::Z => bnum = self.z,
                                Register::W => bnum = self.w,
                            };
                        }
                        B::Literal(value) => {
                            bnum = value;
                        }
                    };

                    if bnum <= 0 {
                        println!("PC: {}", self.counter);
                        panic!("Cannot mod by b <= 0!");
                    }

                    match inst.a {
                        Register::X => self.x %= bnum,
                        Register::Y => self.y %= bnum,
                        Register::Z => self.z %= bnum,
                        Register::W => self.w %= bnum,
                    };
                } else {
                    panic!("No b passed!");
                }
            }
            InstructionType::Eql => {
                if let Some(b) = inst.b {
                    let bnum;
                    match b {
                        B::Reg(reg) => {
                            match reg {
                                Register::X => bnum = self.x,
                                Register::Y => bnum = self.y,
                                Register::Z => bnum = self.z,
                                Register::W => bnum = self.w,
                            };
                        }
                        B::Literal(value) => {
                            bnum = value;
                        }
                    };

                    match inst.a {
                        Register::X => {
                            if self.x == bnum {
                                self.x = 1;
                            } else {
                                self.x = 0;
                            }
                        }
                        Register::Y => {
                            if self.y == bnum {
                                self.y = 1;
                            } else {
                                self.y = 0;
                            }
                        }
                        Register::Z => {
                            if self.z == bnum {
                                self.z = 1;
                            } else {
                                self.z = 0;
                            }
                        }
                        Register::W => {
                            if self.w == bnum {
                                self.w = 1;
                            } else {
                                self.w = 0;
                            }
                        }
                    };
                } else {
                    panic!("No b passed!");
                }
            }
        }
        self.counter += 1;
    }

    fn execute_program(&mut self, program: &[Instruction], input: String) {
        // We parse the input a digit at a time and reverse the order for our
        // stack based input reader.
        let input_vec: Vec<_> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .rev()
            .collect();
        for v in input_vec {
            self._input.push(v);
        }
        for inst in program {
            self.execute(inst);
        }
    }

    fn clear_input(&mut self) {
        self.counter = 0;
        self._input = vec![];
    }
}

#[aoc_generator(day24)]
fn load_input(input: &str) -> Vec<Instruction> {
    let mut output = vec![];
    for line in input.lines() {
        let mut liter = line.split(' ');
        let itypestr = liter.next().unwrap();
        let astr = liter.next().unwrap();
        let bstr = liter.next();

        let b;
        if let Some(value) = bstr {
            b = value;
        } else {
            b = "";
        }

        let itype = match itypestr {
            "inp" => InstructionType::Inp,
            "add" => InstructionType::Add,
            "mul" => InstructionType::Mul,
            "div" => InstructionType::Div,
            "mod" => InstructionType::Mod,
            "eql" => InstructionType::Eql,
            _ => panic!("What even is this"),
        };

        let inst = Instruction::new(itype, astr.chars().next().unwrap(), b);
        output.push(inst);
    }
    output
}

#[aoc(day24, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut alu = ALU::new();
    let input_num = 53999995829399;
    let input_str = input_num.to_string();
    if !input_str.contains('0') {
        alu.clear_input();
        alu.execute_program(input, input_str);
        if alu.z == 0 {
            println!("FOUND IT");
            return input_num;
        }
    }
    panic!("Shouldn't get here");
}

#[aoc(day24, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut alu = ALU::new();
    let input_num = 11721151118175;
    let input_str = input_num.to_string();
    if !input_str.contains('0') {
        alu.clear_input();
        alu.execute_program(input, input_str);
        if alu.z == 0 {
            println!("FOUND IT");
            return input_num;
        }
    }
    panic!("Shouldn't get here");
}
