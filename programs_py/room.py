# Built with Seahorse v0.2.7

from seahorse.prelude import *
# from seahorse.pyth import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Guest:
    keysOwned: u8
    address: Pubkey


class Room(Account):
    id: u64
    owner: Pubkey
    curPrice: u64
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
def init_room(owner: Pubkey, room: Empty[Room], room_id: u64, price: u64):
    room.init(
        owner=owner.key,
        price=5980500,#3$#TODO: use pyth oracle
    )
    room.id = room_id
    room.owner = owner
    room.price = price
    room.is_locked = False
    room.save()

#Buy a key
@instruction
def buy_key(room: Room):
    assert room.is_locked, "Room is not locked"
    assert room.curPrice < 0, "Key is not available for sale"
    room.curPrice*=1.1
    room.save()

    # assert room_account.curPrice <= 1000, "Insufficient funds"  
    # Transfer funds to the room owner (or escrow account) and grant access to the buyer
    # Implement this logic based on your use case

# Sell a key
@instruction
def sell_key(room: Room, new_price: u64):
    assert not room.is_locked, "Room is already locked"
    room.curPrice/=1.1
    room.save()

