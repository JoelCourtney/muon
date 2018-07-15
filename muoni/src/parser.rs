use super::ast::*;
use super::values::*;

pub fn parse(lexemes: Vec<Lexeme>) -> Box<Scope> {
    let mut statements = Vec::new();
    let l = lexemes.len();
    let mut i = 0;
    while i < l {
        let (control,length) = parse_control(&lexemes[i..]);
        statements.push(control);
        i += length;
    }
    Box::new(Scope {
        statements
    })
}

fn parse_scope(lexemes: &[Lexeme]) -> (Box<Scope>,usize) {
    unimplemented!();
}

fn parse_control(lexemes: &[Lexeme]) -> (Box<Control>,usize) {
    let mut i = 0;
    let mut start = lexemes.get(i);
    while start == Some(&Lexeme::NewLine) {
        i += 1;
        start = lexemes.get(i);
    }
    match start {
        Some(Lexeme::Handle(s)) => {
            let name = s.clone();
            let next = lexemes.get(i+1);
            match next {
                Some(Lexeme::Assign) => {
                    i += 2;
                    let (e1,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::AssignVariable {
                            name,
                            e1,
                        }),
                        i,
                    )
                }
                Some(Lexeme::AssignOp(op)) => {
                    i += 2;
                    let (e1,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::AssignVariableBOP {
                            name,
                            op: *op,
                            e1,
                        }),
                        i,
                    )
                }
                Some(Lexeme::Let) => {
                    i += 2;
                    let (e1,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::AssignConstant {
                            name,
                            e1,
                        }),
                        i,
                    )
                }
                Some(Lexeme::OArgList) => {
                    let mut definition = false;
                    let mut cursor = i + 2;
                    let mut args = Vec::new();
                    loop {
                        let lexeme = lexemes.get(cursor);
                        match lexeme {
                            Some(Lexeme::Handle(s)) => {
                                args.push(s.clone());
                                let comma = lexemes.get(cursor+1);
                                match comma {
                                    Some(Lexeme::Comma) => {
                                        cursor += 2;
                                    }
                                    Some(Lexeme::CParen) => {
                                        let arrow = lexemes.get(cursor+2);
                                        match arrow {
                                            Some(Lexeme::RightArrow) => {
                                                definition = true;
                                                i = cursor + 3;
                                            }
                                            _ => {}
                                        }
                                        break;
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                            }
                            Some(Lexeme::CParen) => {
                                let arrow = lexemes.get(cursor+1);
                                match arrow {
                                    Some(Lexeme::RightArrow) => {
                                        definition = true;
                                        i = cursor + 2;
                                    }
                                    _ => {}
                                }
                                break;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    if definition {
                        let (body,length) = parse_arith(&lexemes[i..]);
                        i += length;
                        (
                            Box::new(Control::AssignFunction {
                                name,
                                args,
                                body,
                            }),
                            i,
                        )
                    } else {
                        let (e1,length) = parse_arith(&lexemes[i..]);
                        i += length;
                        (
                            Box::new(Control::StateValue {
                                e1,
                            }),
                            i,
                        )
                    }
                }
                Some(Lexeme::RightArrow) => {
                    i += 2;
                    let (body,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::AssignFunction {
                            name,
                            args: Vec::new(),
                            body,
                        }),
                        i,
                    )
                }
                _ => {
                    let (e1,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::StateValue {
                            e1,
                        }),
                        i,
                    )
                }
            }
        }
        Some(Lexeme::For) => {
            i += 1;
            let (range,length) = parse_arith(&lexemes[i..]);
            i += length;
            let key1 = lexemes.get(i);
            match key1 {
                Some(Lexeme::As) => {
                    i += 1;
                    let (target,length) = parse_decomposition(&lexemes[i..]);
                    i += length;
                    let key2 = lexemes.get(i);
                    match key2 {
                        Some(Lexeme::At) => {
                            i += 1;
                            let (index,length) = parse_decomposition(&lexemes[i..]);
                            i += length;
                            let (body,length) = parse_arith(&lexemes[i..]);
                            i += length;
                            (
                                Box::new(Control::ForAsAt {
                                    range,
                                    target,
                                    index,
                                    body,
                                }),
                                i,
                            )
                        }
                        Some(_) => {
                            let (body,length) = parse_arith(&lexemes[i..]);
                            i += length;
                            (
                                Box::new(Control::ForAs {
                                    range,
                                    target,
                                    body,
                                }),
                                i,
                            )
                        }
                        None => panic!("ya dun fuqd up"),
                    }
                }
                Some(Lexeme::At) => {
                    i += 1;
                    let (index,length) = parse_decomposition(&lexemes[i..]);
                    i += length;
                    let key2 = lexemes.get(i);
                    match key2 {
                        Some(Lexeme::As) => {
                            i += 1;
                            let (target,length) = parse_decomposition(&lexemes[i..]);
                            i += length;
                            let (body,length) = parse_arith(&lexemes[i..]);
                            i += length;
                            (
                                Box::new(Control::ForAsAt {
                                    range,
                                    target,
                                    index,
                                    body,
                                }),
                                i,
                            )
                        }
                        Some(_) => {
                            let (body,length) = parse_arith(&lexemes[i..]);
                            i += length;
                            (
                                Box::new(Control::ForAt {
                                    range,
                                    index,
                                    body,
                                }),
                                i,
                            )
                        }
                        None => panic!("ya dun fuqd up"),
                    }
                }
                Some(_) => {
                    let (body,length) = parse_arith(&lexemes[i..]);
                    i += length;
                    (
                        Box::new(Control::For {
                            range,
                            body,
                        }),
                        i,
                    )
                }
                None => {
                    panic!("ya dun fuqd up")
                }
            }
        }
        Some(Lexeme::If) => {
            unimplemented!();
        }
        Some(Lexeme::While) => {
            unimplemented!();
        }
        Some(_) => {
            let (e1,length) = parse_arith(&lexemes[i..]);
            i += length;
            (
                Box::new(Control::StateValue {
                    e1,
                }),
                i,
            )
        }
        None => (Box::new(Control::Empty),0)
    }
}

fn parse_decomposition(lexemes: &[Lexeme]) -> (Box<Decomposition>,usize) {
    unimplemented!();
}

fn parse_arith(lexemes: &[Lexeme]) -> (Box<Arith>,usize) {
    let mut complete = false;
    let mut level = 0;
    let mut i = 0;
    loop {
        let lexeme = lexemes.get(i);
        match lexeme {
            Some(lex) if OPENERS.contains(lex) => {
                level += 1;
                complete = false;
            }
            Some(lex) if CLOSERS.contains(lex) => {
                level -= 1;
                if level == 0 {
                    complete = true;
                }
            }
            Some(Lexeme::BinaryOp(_)) => {
                complete = false;
            }
            Some(Lexeme::UnaryOp(o)) if o != &UOP::Factorial => {
                complete = false;
            }
            Some(Lexeme::Handle(_)) | Some(Lexeme::Number(_)) | Some(Lexeme::StringLiteral(_)) => {
                if level == 0 {
                    complete = true;
                }
            }
            Some(Lexeme::Semicolon) | Some(Lexeme::NewLine) => {
                if complete {
                    break;
                }
            }
            Some(_) => {}               
            None => {
                if complete {
                    break;
                } else {
                    panic!("Unexpected end of input.");
                }
            }
        }
        i += 1;
    }
    let arith = parse_arith_contained(&lexemes[..i],0);
    (
        arith,
        i,
    )
}

lazy_static! {
    static ref PRECEDENCE: Vec<Vec<BOP>> = vec![
        vec![BOP::Or, BOP::NOr],
        vec![BOP::XOr],
        vec![BOP::And, BOP::NAnd],
        vec![BOP::Is, BOP::Isnt],
        vec![BOP::Less, BOP::LessOrEqual, BOP::Greater, BOP::GreaterOrEqual],
        vec![BOP::Plus, BOP::Minus],
        vec![BOP::Times, BOP::ElemTimes, BOP::Divide, BOP::ElemDivide, BOP::Modulus],
        vec![BOP::Power, BOP::ElemPower]
    ];
    static ref OPENERS: [Lexeme; 9] = [
        Lexeme::OArgList,
        Lexeme::OScope,
        Lexeme::OMatrix,
        Lexeme::ORangeIn,
        Lexeme::ORangeEx,
        Lexeme::OUnit,
        Lexeme::OList,
        Lexeme::ONorm,
        Lexeme::ODeterminant
    ];
    static ref CLOSERS: [Lexeme; 5] = [
        Lexeme::CParen,
        Lexeme::CBraket,
        Lexeme::CBrace,
        Lexeme::CNorm,
        Lexeme::CDeterminant,
    ];
}

fn parse_arith_contained(lexemes: &[Lexeme], level: usize) -> Box<Arith> {
    let l = lexemes.len();
    if level < 8 {
        let mut i = 0;
        while i < l {
            let lexeme = lexemes.get(i).unwrap();
            match lexeme {
                Lexeme::BinaryOp(bop) => {
                    if PRECEDENCE[level].contains(bop) {
                        let e1 = parse_arith_contained(&lexemes[..i], level + 1);
                        let e2 = parse_arith_contained(&lexemes[i+1..], level);
                        return Box::new( Arith::Binary {
                            op: *bop,
                            e1,
                            e2,
                        });
                    }
                    break;
                }
                _ if OPENERS.contains(lexeme) => {
                    i += traverse_atom(&lexemes[i..]);
                }
                _ => {
                    i += 1;
                }
            }
        }
    } else if level == 8 {
        let lexeme = lexemes.get(0).unwrap();
        match lexeme {
            Lexeme::UnaryOp(uop) => {
                let e1 = parse_arith_contained(&lexemes[1..], level);
                return Box::new( Arith::Unary {
                    op: *uop,
                    e1,
                });
            }
            _ => {
                return parse_arith_contained(&lexemes[..], level + 1);
            }
        }
    } else if level == 9 {
        let start = lexemes.get(0).unwrap();
        match start {
            Lexeme::Handle(s) => {
                if lexemes.len() == 1 {
                    unimplemented!();
                }
                unimplemented!();
            }
            Lexeme::Number(n) => {
                unimplemented!();
            }
            Lexeme::StringLiteral(s) => {
                unimplemented!();
            }
            Lexeme::OArgList => {
                unimplemented!();
            }
            Lexeme::OScope => {
                unimplemented!();
            }
            Lexeme::OMatrix => {
                unimplemented!();
            }
            Lexeme::ORangeIn => {
                unimplemented!();
            }
            Lexeme::ORangeEx => {
                unimplemented!();
            }
            Lexeme::OUnit => {
                unimplemented!();
            }
            Lexeme::OList => {
                unimplemented!();
            }
            Lexeme::ODeterminant => {
                unimplemented!();
            }
            _ => {}
        }
    }
    parse_arith_contained(lexemes, level + 1)
}

fn traverse_atom(lexemes: &[Lexeme]) -> usize {
    let mut i = 0;
    let mut level = 1;
    loop {
        i += 1;
        let lexeme = lexemes.get(i).unwrap();
        if OPENERS.contains(lexeme) {
            level += 1;
        } else if CLOSERS.contains(lexeme) {
            level -= 1;
            if level == 0 {
                return i + 1;
            }
        }
    }
}
