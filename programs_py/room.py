# rizz
# Built with Seahorse v0.2.7

from seahorse.prelude import *
# from seahorse.pyth import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Guest:
    keysOwned: u8
    address: Pubkey


# Define a data structure for the Room
class Room(Account):
    id: u64
    owner: Pubkey
    price: u64
    # guest: Pubkey | None 
    is_locked: bool
    guests: Array[Guest]
    

    @property
    def guest(self):
      guests = sorted(self.guests, key=lambda x: x.keysOwned)
      self.guests = guests
      return guests[-1]          


# @instruction
# def use_sol_usd_price(price_account: PriceAccount):
#   price_feed = price_account.validate_price_feed('SOL/USD')

# Initialize the Room
@instruction
def init_room(owner: Pubkey, room_account: Empty[Room], room_id: u64, price: u64):
    room_account.init(
        owner=owner.key,
        price=5980500,#3$
    )
    room_account.id = room_id
    room_account.owner = owner
    room_account.price = price
    room_account.is_locked = False
    room_account.save()

# Buy a key to access the Room
@instruction
def buy_key(room_account: Room):
    assert room_account.is_locked, "Room is not locked"
    assert room_account.price > 0, "Key is not available for sale"
    assert room_account.price <= 1000, "Insufficient funds"  # Adjust as per requirement
    # Transfer funds to the room owner (or escrow account) and grant access to the buyer
    # Implement this logic based on your use case

# Sell a key to access the Room
@instruction
def sell_key(room_account: Room, new_price: u64):
    assert not room_account.is_locked, "Room is already locked"
    room_account.price = new_price
    room_account.save()

