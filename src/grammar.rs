use std::str::FromStr;
use ast::{Operator, Expr, Operation};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Operator, Expr, Operation};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtExpr(Expr),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtInteger(i64),
        NtOperation(Operation),
        NtOperator(Operator),
        NtProgram(Expr),
        Nt____Expr(Expr),
        Nt____Integer(i64),
        Nt____Operation(Operation),
        Nt____Operator(Operator),
        Nt____Program(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        4, // on "(", goto 3
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        5, // on r#"-?[0-9]+"#, goto 4
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        8, // on "*", goto 7
        9, // on "+", goto 8
        10, // on "-", goto 9
        11, // on "/", goto 10
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "(", error
        12, // on ")", goto 11
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 6
        16, // on "(", goto 15
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        17, // on r#"-?[0-9]+"#, goto 16
        // State 7
        -9, // on "(", reduce `Operator = "*" => ActionFn(8);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -9, // on r#"-?[0-9]+"#, reduce `Operator = "*" => ActionFn(8);`
        // State 8
        -7, // on "(", reduce `Operator = "+" => ActionFn(6);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -7, // on r#"-?[0-9]+"#, reduce `Operator = "+" => ActionFn(6);`
        // State 9
        -8, // on "(", reduce `Operator = "-" => ActionFn(7);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -8, // on r#"-?[0-9]+"#, reduce `Operator = "-" => ActionFn(7);`
        // State 10
        -10, // on "(", reduce `Operator = "/" => ActionFn(9);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -10, // on r#"-?[0-9]+"#, reduce `Operator = "/" => ActionFn(9);`
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 12
        -3, // on "(", reduce `Expr+ = Expr => ActionFn(14);`
        -3, // on ")", reduce `Expr+ = Expr => ActionFn(14);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -3, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr => ActionFn(14);`
        // State 13
        16, // on "(", goto 15
        -6, // on ")", reduce `Operation = Operator, Expr+ => ActionFn(11);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        17, // on r#"-?[0-9]+"#, goto 16
        // State 14
        -1, // on "(", reduce `Expr = Integer => ActionFn(12);`
        -1, // on ")", reduce `Expr = Integer => ActionFn(12);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -1, // on r#"-?[0-9]+"#, reduce `Expr = Integer => ActionFn(12);`
        // State 15
        0, // on "(", error
        0, // on ")", error
        8, // on "*", goto 7
        9, // on "+", goto 8
        10, // on "-", goto 9
        11, // on "/", goto 10
        0, // on r#"-?[0-9]+"#, error
        // State 16
        -5, // on "(", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        -5, // on ")", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"-?[0-9]+"#, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        // State 17
        -4, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        -4, // on ")", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        // State 18
        0, // on "(", error
        20, // on ")", goto 19
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 19
        -2, // on "(", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        -2, // on ")", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -2, // on r#"-?[0-9]+"#, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -12, // on EOF, reduce `__Expr = Expr => ActionFn(4);`
        -1, // on EOF, reduce `Expr = Integer => ActionFn(12);`
        0, // on EOF, error
        -5, // on EOF, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Expr, goto 1
        0, // on Expr+, error
        3, // on Integer, goto 2
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 1
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 2
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 3
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        6, // on Operation, goto 5
        7, // on Operator, goto 6
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 4
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 5
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 6
        13, // on Expr, goto 12
        14, // on Expr+, goto 13
        15, // on Integer, goto 14
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 7
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 8
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 9
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 10
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 11
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 12
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 13
        18, // on Expr, goto 17
        0, // on Expr+, error
        15, // on Integer, goto 14
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 14
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 15
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        19, // on Operation, goto 18
        7, // on Operator, goto 6
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 16
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 17
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 18
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 19
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
    ];
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expr, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Integer => ActionFn(12);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = "(", Operation, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtOperation(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr+ = Expr => ActionFn(14);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            4 => {
                // Expr+ = Expr+, Expr => ActionFn(15);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            5 => {
                // Integer = r#"-?[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            6 => {
                // Operation = Operator, Expr+ => ActionFn(11);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperation(__nt), __end));
                3
            }
            7 => {
                // Operator = "+" => ActionFn(6);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            8 => {
                // Operator = "-" => ActionFn(7);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            9 => {
                // Operator = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            10 => {
                // Operator = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            11 => {
                // Program = Operation => ActionFn(5);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                5
            }
            12 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                return Some(Ok(__nt));
            }
            13 => {
                // __Integer = Integer => ActionFn(2);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Integer(__nt), __end));
                7
            }
            14 => {
                // __Operation = Operation => ActionFn(3);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operation(__nt), __end));
                8
            }
            15 => {
                // __Operator = Operator => ActionFn(1);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operator(__nt), __end));
                9
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                10
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Integer<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Integer(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

mod __parse__Integer {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Operator, Expr, Operation};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtExpr(Expr),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtInteger(i64),
        NtOperation(Operation),
        NtOperator(Operator),
        NtProgram(Expr),
        Nt____Expr(Expr),
        Nt____Integer(i64),
        Nt____Operation(Operation),
        Nt____Operator(Operator),
        Nt____Program(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        3, // on r#"-?[0-9]+"#, goto 2
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -13, // on EOF, reduce `__Integer = Integer => ActionFn(2);`
        -5, // on EOF, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Expr, error
        0, // on Expr+, error
        2, // on Integer, goto 1
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 1
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 2
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
    ];
    pub fn parse_Integer<
        'input,
    >(
        input: &'input str,
    ) -> Result<i64, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<i64,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Integer => ActionFn(12);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = "(", Operation, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtOperation(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr+ = Expr => ActionFn(14);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            4 => {
                // Expr+ = Expr+, Expr => ActionFn(15);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            5 => {
                // Integer = r#"-?[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            6 => {
                // Operation = Operator, Expr+ => ActionFn(11);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperation(__nt), __end));
                3
            }
            7 => {
                // Operator = "+" => ActionFn(6);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            8 => {
                // Operator = "-" => ActionFn(7);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            9 => {
                // Operator = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            10 => {
                // Operator = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            11 => {
                // Program = Operation => ActionFn(5);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                5
            }
            12 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                6
            }
            13 => {
                // __Integer = Integer => ActionFn(2);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                return Some(Ok(__nt));
            }
            14 => {
                // __Operation = Operation => ActionFn(3);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operation(__nt), __end));
                8
            }
            15 => {
                // __Operator = Operator => ActionFn(1);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operator(__nt), __end));
                9
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                10
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Integer<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Integer(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Integer::parse_Integer;

mod __parse__Operation {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Operator, Expr, Operation};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtExpr(Expr),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtInteger(i64),
        NtOperation(Operation),
        NtOperator(Operator),
        NtProgram(Expr),
        Nt____Expr(Expr),
        Nt____Integer(i64),
        Nt____Operation(Operation),
        Nt____Operator(Operator),
        Nt____Program(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        4, // on "*", goto 3
        5, // on "+", goto 4
        6, // on "-", goto 5
        7, // on "/", goto 6
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        11, // on "(", goto 10
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        12, // on r#"-?[0-9]+"#, goto 11
        // State 3
        -9, // on "(", reduce `Operator = "*" => ActionFn(8);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -9, // on r#"-?[0-9]+"#, reduce `Operator = "*" => ActionFn(8);`
        // State 4
        -7, // on "(", reduce `Operator = "+" => ActionFn(6);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -7, // on r#"-?[0-9]+"#, reduce `Operator = "+" => ActionFn(6);`
        // State 5
        -8, // on "(", reduce `Operator = "-" => ActionFn(7);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -8, // on r#"-?[0-9]+"#, reduce `Operator = "-" => ActionFn(7);`
        // State 6
        -10, // on "(", reduce `Operator = "/" => ActionFn(9);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -10, // on r#"-?[0-9]+"#, reduce `Operator = "/" => ActionFn(9);`
        // State 7
        -3, // on "(", reduce `Expr+ = Expr => ActionFn(14);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -3, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr => ActionFn(14);`
        // State 8
        11, // on "(", goto 10
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        12, // on r#"-?[0-9]+"#, goto 11
        // State 9
        -1, // on "(", reduce `Expr = Integer => ActionFn(12);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -1, // on r#"-?[0-9]+"#, reduce `Expr = Integer => ActionFn(12);`
        // State 10
        0, // on "(", error
        0, // on ")", error
        4, // on "*", goto 3
        5, // on "+", goto 4
        6, // on "-", goto 5
        7, // on "/", goto 6
        0, // on r#"-?[0-9]+"#, error
        // State 11
        -5, // on "(", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"-?[0-9]+"#, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        // State 12
        -4, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        // State 13
        0, // on "(", error
        16, // on ")", goto 15
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 14
        20, // on "(", goto 19
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        21, // on r#"-?[0-9]+"#, goto 20
        // State 15
        -2, // on "(", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -2, // on r#"-?[0-9]+"#, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        // State 16
        -3, // on "(", reduce `Expr+ = Expr => ActionFn(14);`
        -3, // on ")", reduce `Expr+ = Expr => ActionFn(14);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -3, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr => ActionFn(14);`
        // State 17
        20, // on "(", goto 19
        -6, // on ")", reduce `Operation = Operator, Expr+ => ActionFn(11);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        21, // on r#"-?[0-9]+"#, goto 20
        // State 18
        -1, // on "(", reduce `Expr = Integer => ActionFn(12);`
        -1, // on ")", reduce `Expr = Integer => ActionFn(12);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -1, // on r#"-?[0-9]+"#, reduce `Expr = Integer => ActionFn(12);`
        // State 19
        0, // on "(", error
        0, // on ")", error
        4, // on "*", goto 3
        5, // on "+", goto 4
        6, // on "-", goto 5
        7, // on "/", goto 6
        0, // on r#"-?[0-9]+"#, error
        // State 20
        -5, // on "(", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        -5, // on ")", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"-?[0-9]+"#, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        // State 21
        -4, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        -4, // on ")", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        // State 22
        0, // on "(", error
        24, // on ")", goto 23
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 23
        -2, // on "(", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        -2, // on ")", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -2, // on r#"-?[0-9]+"#, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -14, // on EOF, reduce `__Operation = Operation => ActionFn(3);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr+ = Expr => ActionFn(14);`
        -6, // on EOF, reduce `Operation = Operator, Expr+ => ActionFn(11);`
        -1, // on EOF, reduce `Expr = Integer => ActionFn(12);`
        0, // on EOF, error
        -5, // on EOF, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        -4, // on EOF, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        2, // on Operation, goto 1
        3, // on Operator, goto 2
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 1
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 2
        8, // on Expr, goto 7
        9, // on Expr+, goto 8
        10, // on Integer, goto 9
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 3
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 4
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 5
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 6
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 7
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 8
        13, // on Expr, goto 12
        0, // on Expr+, error
        10, // on Integer, goto 9
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 9
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 10
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        14, // on Operation, goto 13
        15, // on Operator, goto 14
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 11
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 12
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 13
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 14
        17, // on Expr, goto 16
        18, // on Expr+, goto 17
        19, // on Integer, goto 18
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 15
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 16
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 17
        22, // on Expr, goto 21
        0, // on Expr+, error
        19, // on Integer, goto 18
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 18
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 19
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        23, // on Operation, goto 22
        15, // on Operator, goto 14
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 20
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 21
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 22
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 23
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
    ];
    pub fn parse_Operation<
        'input,
    >(
        input: &'input str,
    ) -> Result<Operation, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Operation,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Integer => ActionFn(12);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = "(", Operation, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtOperation(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr+ = Expr => ActionFn(14);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            4 => {
                // Expr+ = Expr+, Expr => ActionFn(15);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            5 => {
                // Integer = r#"-?[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            6 => {
                // Operation = Operator, Expr+ => ActionFn(11);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperation(__nt), __end));
                3
            }
            7 => {
                // Operator = "+" => ActionFn(6);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            8 => {
                // Operator = "-" => ActionFn(7);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            9 => {
                // Operator = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            10 => {
                // Operator = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            11 => {
                // Program = Operation => ActionFn(5);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                5
            }
            12 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                6
            }
            13 => {
                // __Integer = Integer => ActionFn(2);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Integer(__nt), __end));
                7
            }
            14 => {
                // __Operation = Operation => ActionFn(3);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                return Some(Ok(__nt));
            }
            15 => {
                // __Operator = Operator => ActionFn(1);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operator(__nt), __end));
                9
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                10
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Integer<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Integer(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Operation::parse_Operation;

mod __parse__Operator {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Operator, Expr, Operation};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtExpr(Expr),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtInteger(i64),
        NtOperation(Operation),
        NtOperator(Operator),
        NtProgram(Expr),
        Nt____Expr(Expr),
        Nt____Integer(i64),
        Nt____Operation(Operation),
        Nt____Operator(Operator),
        Nt____Program(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        3, // on "*", goto 2
        4, // on "+", goto 3
        5, // on "-", goto 4
        6, // on "/", goto 5
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -15, // on EOF, reduce `__Operator = Operator => ActionFn(1);`
        -9, // on EOF, reduce `Operator = "*" => ActionFn(8);`
        -7, // on EOF, reduce `Operator = "+" => ActionFn(6);`
        -8, // on EOF, reduce `Operator = "-" => ActionFn(7);`
        -10, // on EOF, reduce `Operator = "/" => ActionFn(9);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        2, // on Operator, goto 1
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 1
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 2
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 3
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 4
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 5
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
    ];
    pub fn parse_Operator<
        'input,
    >(
        input: &'input str,
    ) -> Result<Operator, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Operator,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Integer => ActionFn(12);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = "(", Operation, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtOperation(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr+ = Expr => ActionFn(14);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            4 => {
                // Expr+ = Expr+, Expr => ActionFn(15);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            5 => {
                // Integer = r#"-?[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            6 => {
                // Operation = Operator, Expr+ => ActionFn(11);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperation(__nt), __end));
                3
            }
            7 => {
                // Operator = "+" => ActionFn(6);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            8 => {
                // Operator = "-" => ActionFn(7);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            9 => {
                // Operator = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            10 => {
                // Operator = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            11 => {
                // Program = Operation => ActionFn(5);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                5
            }
            12 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                6
            }
            13 => {
                // __Integer = Integer => ActionFn(2);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Integer(__nt), __end));
                7
            }
            14 => {
                // __Operation = Operation => ActionFn(3);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operation(__nt), __end));
                8
            }
            15 => {
                // __Operator = Operator => ActionFn(1);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                return Some(Ok(__nt));
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                10
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Integer<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Integer(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Operator::parse_Operator;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Operator, Expr, Operation};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(&'input str),
        NtExpr(Expr),
        NtExpr_2b(::std::vec::Vec<Expr>),
        NtInteger(i64),
        NtOperation(Operation),
        NtOperator(Operator),
        NtProgram(Expr),
        Nt____Expr(Expr),
        Nt____Integer(i64),
        Nt____Operation(Operation),
        Nt____Operator(Operator),
        Nt____Program(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on r#"-?[0-9]+"#, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 2
        12, // on "(", goto 11
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        13, // on r#"-?[0-9]+"#, goto 12
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 4
        -9, // on "(", reduce `Operator = "*" => ActionFn(8);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -9, // on r#"-?[0-9]+"#, reduce `Operator = "*" => ActionFn(8);`
        // State 5
        -7, // on "(", reduce `Operator = "+" => ActionFn(6);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -7, // on r#"-?[0-9]+"#, reduce `Operator = "+" => ActionFn(6);`
        // State 6
        -8, // on "(", reduce `Operator = "-" => ActionFn(7);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -8, // on r#"-?[0-9]+"#, reduce `Operator = "-" => ActionFn(7);`
        // State 7
        -10, // on "(", reduce `Operator = "/" => ActionFn(9);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -10, // on r#"-?[0-9]+"#, reduce `Operator = "/" => ActionFn(9);`
        // State 8
        -3, // on "(", reduce `Expr+ = Expr => ActionFn(14);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -3, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr => ActionFn(14);`
        // State 9
        12, // on "(", goto 11
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        13, // on r#"-?[0-9]+"#, goto 12
        // State 10
        -1, // on "(", reduce `Expr = Integer => ActionFn(12);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -1, // on r#"-?[0-9]+"#, reduce `Expr = Integer => ActionFn(12);`
        // State 11
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on r#"-?[0-9]+"#, error
        // State 12
        -5, // on "(", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"-?[0-9]+"#, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        // State 13
        -4, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        // State 14
        0, // on "(", error
        17, // on ")", goto 16
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 15
        21, // on "(", goto 20
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        22, // on r#"-?[0-9]+"#, goto 21
        // State 16
        -2, // on "(", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -2, // on r#"-?[0-9]+"#, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        // State 17
        -3, // on "(", reduce `Expr+ = Expr => ActionFn(14);`
        -3, // on ")", reduce `Expr+ = Expr => ActionFn(14);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -3, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr => ActionFn(14);`
        // State 18
        21, // on "(", goto 20
        -6, // on ")", reduce `Operation = Operator, Expr+ => ActionFn(11);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        22, // on r#"-?[0-9]+"#, goto 21
        // State 19
        -1, // on "(", reduce `Expr = Integer => ActionFn(12);`
        -1, // on ")", reduce `Expr = Integer => ActionFn(12);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -1, // on r#"-?[0-9]+"#, reduce `Expr = Integer => ActionFn(12);`
        // State 20
        0, // on "(", error
        0, // on ")", error
        5, // on "*", goto 4
        6, // on "+", goto 5
        7, // on "-", goto 6
        8, // on "/", goto 7
        0, // on r#"-?[0-9]+"#, error
        // State 21
        -5, // on "(", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        -5, // on ")", reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"-?[0-9]+"#, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        // State 22
        -4, // on "(", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        -4, // on ")", reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"-?[0-9]+"#, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        // State 23
        0, // on "(", error
        25, // on ")", goto 24
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"-?[0-9]+"#, error
        // State 24
        -2, // on "(", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        -2, // on ")", reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -2, // on r#"-?[0-9]+"#, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -11, // on EOF, reduce `Program = Operation => ActionFn(5);`
        0, // on EOF, error
        -16, // on EOF, reduce `__Program = Program => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Expr+ = Expr => ActionFn(14);`
        -6, // on EOF, reduce `Operation = Operator, Expr+ => ActionFn(11);`
        -1, // on EOF, reduce `Expr = Integer => ActionFn(12);`
        0, // on EOF, error
        -5, // on EOF, reduce `Integer = r#"-?[0-9]+"# => ActionFn(10);`
        -4, // on EOF, reduce `Expr+ = Expr+, Expr => ActionFn(15);`
        0, // on EOF, error
        0, // on EOF, error
        -2, // on EOF, reduce `Expr = "(", Operation, ")" => ActionFn(13);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        2, // on Operation, goto 1
        3, // on Operator, goto 2
        4, // on Program, goto 3
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 1
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 2
        9, // on Expr, goto 8
        10, // on Expr+, goto 9
        11, // on Integer, goto 10
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 3
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 4
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 5
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 6
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 7
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 8
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 9
        14, // on Expr, goto 13
        0, // on Expr+, error
        11, // on Integer, goto 10
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 10
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 11
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        15, // on Operation, goto 14
        16, // on Operator, goto 15
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 12
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 13
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 14
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 15
        18, // on Expr, goto 17
        19, // on Expr+, goto 18
        20, // on Integer, goto 19
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 16
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 17
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 18
        23, // on Expr, goto 22
        0, // on Expr+, error
        20, // on Integer, goto 19
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 19
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 20
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        24, // on Operation, goto 23
        16, // on Operator, goto 15
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 21
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 22
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 23
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
        // State 24
        0, // on Expr, error
        0, // on Expr+, error
        0, // on Integer, error
        0, // on Operation, error
        0, // on Operator, error
        0, // on Program, error
        0, // on __Expr, error
        0, // on __Integer, error
        0, // on __Operation, error
        0, // on __Operator, error
        0, // on __Program, error
    ];
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expr, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expr = Integer => ActionFn(12);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            2 => {
                // Expr = "(", Operation, ")" => ActionFn(13);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtOperation(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                0
            }
            3 => {
                // Expr+ = Expr => ActionFn(14);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            4 => {
                // Expr+ = Expr+, Expr => ActionFn(15);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_NtExpr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr_2b(__nt), __end));
                1
            }
            5 => {
                // Integer = r#"-?[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                2
            }
            6 => {
                // Operation = Operator, Expr+ => ActionFn(11);
                let __sym1 = __pop_NtExpr_2b(__symbols);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOperation(__nt), __end));
                3
            }
            7 => {
                // Operator = "+" => ActionFn(6);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            8 => {
                // Operator = "-" => ActionFn(7);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            9 => {
                // Operator = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            10 => {
                // Operator = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOperator(__nt), __end));
                4
            }
            11 => {
                // Program = Operation => ActionFn(5);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                5
            }
            12 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                6
            }
            13 => {
                // __Integer = Integer => ActionFn(2);
                let __sym0 = __pop_NtInteger(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Integer(__nt), __end));
                7
            }
            14 => {
                // __Operation = Operation => ActionFn(3);
                let __sym0 = __pop_NtOperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operation(__nt), __end));
                8
            }
            15 => {
                // __Operator = Operator => ActionFn(1);
                let __sym0 = __pop_NtOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Operator(__nt), __end));
                9
            }
            16 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_2d_3f_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOperator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Integer<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Integer(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Operator<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Operator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Operator, usize),
) -> Operator
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> i64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Operation, usize),
) -> Operation
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, Operation, usize),
) -> Expr
{
    Expr::Operation(op)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Plus
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Minus
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Multiply
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Divide
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(s).expect("failed to parse integer")
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, operator, _): (usize, Operator, usize),
    (_, operands, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> Operation
{
    {
    Operation {
      operator: operator,
      operands: operands,
    }
  }
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, i64, usize),
) -> Expr
{
    Expr::Integer(i)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, op, _): (usize, Operation, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    {
    Expr::Operation(op)
  }
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
