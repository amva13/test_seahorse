#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::ADD
    }
}

#[account]
#[derive(Debug)]
pub struct TestSeahorse {
    pub owner: Pubkey,
    pub display: i64,
}

impl<'info, 'entrypoint> TestSeahorse {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedTestSeahorse<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let display = account.display;

        Mutable::new(LoadedTestSeahorse {
            __account__: account,
            __programs__: programs_map,
            owner,
            display,
        })
    }

    pub fn store(loaded: Mutable<LoadedTestSeahorse>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let display = loaded.display;

        loaded.__account__.display = display;
    }
}

#[derive(Debug)]
pub struct LoadedTestSeahorse<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, TestSeahorse>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub display: i64,
}

pub fn do_operation_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Mutable<LoadedTestSeahorse<'info, '_>>,
    mut op: Operation,
    mut num: i64,
) -> () {
    solana_program::msg!("{}", "do_operation running".to_string());

    if !(owner.key() == calculator.borrow().owner) {
        panic!("This is not your calculator!");
    }

    if op == Operation::ADD {
        assign!(
            calculator.borrow_mut().display,
            calculator.borrow().display + num
        );
    } else {
        if op == Operation::SUB {
            assign!(
                calculator.borrow_mut().display,
                calculator.borrow().display - num
            );
        } else {
            if op == Operation::MUL {
                assign!(
                    calculator.borrow_mut().display,
                    calculator.borrow().display * num
                );
            } else {
                if op == Operation::DIV {
                    assign!(
                        calculator.borrow_mut().display,
                        calculator.borrow().display / num
                    );
                }
            }
        }
    }
}

pub fn init_calculator_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Empty<Mutable<LoadedTestSeahorse<'info, '_>>>,
) -> () {
    solana_program::msg!("{}", "init calculator running".to_string());

    let mut calculator = calculator.account.clone();

    assign!(calculator.borrow_mut().owner, owner.key());
}

pub fn reset_calculator_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Mutable<LoadedTestSeahorse<'info, '_>>,
) -> () {
    if !(owner.key() == calculator.borrow().owner) {
        panic!("This is not your calculator");
    }

    solana_program::msg!(
        "{:?} {} {:?}",
        owner.key(),
        "is resetting".to_string(),
        calculator.borrow().__account__.key()
    );

    assign!(calculator.borrow_mut().display, 0);
}
