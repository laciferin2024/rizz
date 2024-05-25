# Built with Seahorse v0.2.7

from seahorse.prelude import *
# from seahorse.pyth import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Guest:
    keysOwned: u8
    address: Pubkey

class BuySell(Enum):
    BUY=1
    SELL=-1

class Room(Account):
    id: u64
    owner: Pubkey
    curPrice: u64 
    # guest: Pubkey | None 
    is_locked: bool
    guests: Dict[Pubkey,Guest]
    

    @property
    def guest(self)->Guest:
      gtGuest:Guest = None
      
      for k,v in self.guests.items():
          if gtGuest is None or gtGuest.keysOwned<v.keysOwned:
              gtGuest = v.keysOwned


      return gtGuest        
  
    def incK(self,incVal:BuySell, _guest:Pubkey)->Guest:
        g = self.guests[_guest]
        g.keysOwned+=incVal
        return g
        


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
def buy_key(room: Room,buyer:Signer):
    assert room.is_locked, "Room is not locked"
    assert room.curPrice < 0, "Key is not available for sale"
    
    buyer.transfer_lamports(room,room.curPrice)
    buyer.save()

    room.curPrice*=1.1

    room.incK(BuySell.BUY, buyer.key())
    
    room.save()

    # assert room_account.curPrice <= 1000, "Insufficient funds"  
    # Transfer funds to the room owner (or escrow account) and grant access to the buyer
    # Implement this logic based on your use case

# Sell a key
@instruction
def sell_key(room: Room, seller: Signer):
    assert not room.is_locked, "Room is already locked"
    
    room.transfer_lamports(room,seller.curPrice)
    room.curPrice/=1.1
    
    room.save()
    seller.save()
    

@instruction
def leading_guest(room: Room)->Guest:
    guest = room.guest
    assert guest, "Guest not found"
    return guest
    