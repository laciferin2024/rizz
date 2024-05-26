#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod rizz {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    pub struct BuyKey<'info> {
        #[account(mut)]
        pub room: Box<Account<'info, dot::program::Room>>,
        #[account(mut)]
        pub buyer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    pub fn buy_key(ctx: Context<BuyKey>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let room = dot::program::Room::load(&mut ctx.accounts.room, &programs_map);
        let buyer = SeahorseSigner {
            account: &ctx.accounts.buyer,
            programs: &programs_map,
        };

        buy_key_handler(room.clone(), buyer.clone());

        dot::program::Room::store(room);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (user : Pubkey)]
    pub struct GetUser<'info> {
        #[account(mut)]
        pub room: Box<Account<'info, dot::program::Room>>,
    }

    pub fn get_user(ctx: Context<GetUser>, user: Pubkey) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let room = dot::program::Room::load(&mut ctx.accounts.room, &programs_map);

        get_user_handler(room.clone(), user);

        dot::program::Room::store(room);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitRoom<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Room > () + 8 , payer = owner , seeds = ["Rizz" . as_bytes () . as_ref () , owner . key () . as_ref () , "room" . as_bytes () . as_ref () , "rizz" . as_bytes () . as_ref ()] , bump)]
        pub room: Box<Account<'info, dot::program::Room>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn init_room(ctx: Context<InitRoom>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let room = Empty {
            account: dot::program::Room::load(&mut ctx.accounts.room, &programs_map),
            bump: ctx.bumps.get("room").map(|bump| *bump),
        };

        init_room_handler(owner.clone(), room.clone());

        dot::program::Room::store(room.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct SellKey<'info> {
        #[account(mut)]
        pub room: Box<Account<'info, dot::program::Room>>,
        #[account(mut)]
        pub seller: Signer<'info>,
    }

    pub fn sell_key(ctx: Context<SellKey>) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let room = dot::program::Room::load(&mut ctx.accounts.room, &programs_map);
        let seller = SeahorseSigner {
            account: &ctx.accounts.seller,
            programs: &programs_map,
        };

        sell_key_handler(room.clone(), seller.clone());

        dot::program::Room::store(room);

        return Ok(());
    }
}
