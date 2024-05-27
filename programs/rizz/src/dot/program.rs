#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};
use anchor_lang::prelude::borsh::{BorshSerialize, BorshDeserialize};

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum BuySell {
    BUY,
    SELL,
}

impl Default for BuySell {
    fn default() -> Self {
        BuySell::BUY
    }
}

#[derive(Clone, Debug, Default, Copy, BorshSerialize, BorshDeserialize)]
pub struct Guest {
    pub keys_owned: u8,
    pub address: Pubkey,
}

impl Guest {
    pub fn __new__(keys_owned: u8, address: Pubkey) -> Mutable<Self> {
        let obj = Guest {
            keys_owned,
            address,
        };

        return Mutable::new(obj);
    }
}

#[account]
#[derive(Debug)]
pub struct Room {
    pub id: u64,
    pub owner: Pubkey,
    pub cur_price: u64,
    pub is_locked: bool,
    pub guests: Vec<Mutable<Guest>>, // Use Vec directly
}

impl<'info, 'entrypoint> Room {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedRoom<'info, 'entrypoint>> {
        let id = account.id;
        let owner = account.owner.clone();
        let cur_price = account.cur_price;
        let is_locked = account.is_locked;
        let guests = account.guests.clone(); // No need for Mutable here

        Mutable::new(LoadedRoom {
            __account__: account,
            __programs__: programs_map,
            id,
            owner,
            cur_price,
            is_locked,
            guests,
        })
    }

    pub fn store(loaded: Mutable<LoadedRoom>) {
        let mut loaded = loaded.borrow_mut();
        let id = loaded.id;
        loaded.__account__.id = id;

        let owner = loaded.owner.clone();
        loaded.__account__.owner = owner;

        let cur_price = loaded.cur_price;
        loaded.__account__.cur_price = cur_price;

        let is_locked = loaded.is_locked;
        loaded.__account__.is_locked = is_locked;

        let guests = loaded.guests.clone();
        loaded.__account__.guests = guests; // No need for Mutable here
    }
}

#[derive(Debug)]
pub struct LoadedRoom<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Room>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub id: u64,
    pub owner: Pubkey,
    pub cur_price: u64,
    pub is_locked: bool,
    pub guests: Vec<Mutable<Guest>>, // Use Vec directly
}

pub fn buy_key_handler<'info>(
    mut room: Mutable<LoadedRoom<'info, '_>>,
    mut buyer: SeahorseSigner<'info, '_>,
) -> () {
    if !room.borrow().is_locked {
        panic!("Room is not locked");
    }

    if !(room.borrow().cur_price < 0) {
        panic!("Key is not available for sale");
    }

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &buyer.key(),
            &room.borrow().__account__.key(),
            room.borrow().cur_price.clone(),
        ),
        &[
            buyer.to_account_info(),
            room.borrow().__account__.to_account_info(),
            buyer.programs.get("system_program").clone(),
        ],
    )
    .unwrap();

    assign!(room.borrow_mut().cur_price, (room.borrow().cur_price as f64 * 1.1).round() as u64);
}

pub fn get_user_handler<'info>(mut room: Mutable<LoadedRoom<'info, '_>>, mut user: Pubkey) -> () {}

pub fn init_room_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut room: Empty<Mutable<LoadedRoom<'info, '_>>>,
) -> () {
    let mut room = room.account.clone();

    assign!(room.borrow_mut().id, 0);
    assign!(room.borrow_mut().owner, owner.key());
    assign!(room.borrow_mut().cur_price, 17963000);
    assign!(room.borrow_mut().is_locked, false);
    assign!(room.borrow_mut().guests, Vec::new()); // Initialize guests vector
}

pub fn sell_key_handler<'info>(
    mut room: Mutable<LoadedRoom<'info, '_>>,
    mut seller: SeahorseSigner<'info, '_>,
) -> () {
    if !(!room.borrow().is_locked) {
        panic!("Room is already locked");
    }

    {
        let amount = room.borrow().cur_price.clone();

        **room
            .borrow()
            .__account__
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() -= amount;

        **seller
            .clone()
            .to_account_info()
            .try_borrow_mut_lamports()
            .unwrap() += amount;
    };

    assign!(room.borrow_mut().cur_price, (room.borrow().cur_price as f64 / 1.1).round() as u64);
}

// Implement BorshSerialize and BorshDeserialize for Mutable<T>
impl<T> BorshSerialize for Mutable<T>
where
    T: BorshSerialize,
{
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.borrow().serialize(writer)
    }
}

impl<T> BorshDeserialize for Mutable<T>
where
    T: BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let value = T::deserialize(buf)?;
        Ok(Mutable::new(value))
    }
}
