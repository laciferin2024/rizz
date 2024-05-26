from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

@dataclass
class Guest:
    keys_owned: u8
    address: Pubkey
    
# class Guests(NamedTuple):
#     id:Pubkey
    
    

class BuySell(Enum):
    BUY = 1
    SELL = -1

class Room(Account):
    id: u64
    owner: Pubkey
    cur_price: u64
    is_locked: bool
    guests: List[Guest]

    # def leading_guest(self) -> Guest:
    #     leading_guest:Guest
    #
    #     for guestI in self.guests:
    #         guest:Guest = guestI
    #         if leading_guest is None or guest.keys_owned> leading_guest.keys_owned:
    #             leading_guest = guest
    #             return leading_guest
    #
    #     return leading_guest

    # def inc_keys_owned(self, inc_val: BuySell, guest_pubkey: Pubkey) -> Guest:
    #     for idx, guest_tuple in enumerate(self.guests):
    #         if guest_tuple[0] == guest_pubkey:
    #             guest = guest_tuple[1]
    #             guest.keys_owned += inc_val
    #             self.guests[idx] = (guest_pubkey, guest)
    #             return guest
       

    # def get_guest(self, guest_pubkey: Pubkey) -> Guest:
    #     for guest_tuple in self.guests:
    #         if guest_tuple[0] == guest_pubkey:
    #             return guest_tuple[1]
        
@instruction
def init_room(owner: Signer, room: Empty[Room]):
    room = room.init(
        payer=owner,
        seeds=['Rizz',owner,'room', 'rizz'],
        # space=,
        # padding=,
    )
    room.id = 0
    room.owner = owner.key()
    room.cur_price =17963000
    room.is_locked = False

@instruction
def buy_key(room: Room, buyer: Signer):
    assert room.is_locked, "Room is not locked"
    assert room.cur_price < 0, "Key is not available for sale"

    buyer.transfer_lamports(room, room.cur_price)

    room.cur_price += 2 #u64(room.cur_price*.1)
    # room.inc_keys_owned(BuySell.BUY, buyer.key()) FIXME:

@instruction
def sell_key(room: Room, seller: Signer):
    assert not room.is_locked, "Room is already locked"

    room.transfer_lamports(seller, room.cur_price)
    room.cur_price-= 2 #u64(room.cur_price*.1)
    #TODO: inc

# @instruction
# def leading_guest(room: Room) -> Guest:
#     # guest = room.leading_guest()
#     assert guest, "No Guests so far"
#     return guest

@instruction
def get_user(room: Room, user: Pubkey):
    pass
    # get_guest(user)
