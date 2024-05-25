from seahorse.prelude import *

# Define a data structure for the lock account
class LockAccount(Account):
    owner: Pubkey
    amount: u64
    unlock_time: u64
    is_locked: bool

# Initialize the lock account for the user
@instruction
def initialize_lock(owner: Pubkey, lock_account: Empty[LockAccount], amount: u64, unlock_time: u64):
    lock_account.init(
        owner=owner,
        amount=amount,
        unlock_time=unlock_time,
        is_locked=True
    ).save()

# Lock tokens into the user's lock account
@instruction
def lock(lock_account: LockAccount, amount: u64):
    assert not lock_account.is_locked, "Already locked"
    lock_account.amount = amount
    lock_account.is_locked = True
    lock_account.save()

# Unlock tokens from the user's lock account
@instruction
def unlock(lock_account: LockAccount):
    assert lock_account.is_locked, "Nothing is locked"
    assert lock_account.unlock_time <= Clock.now(), "Unlock time not reached"
    lock_account.is_locked = False
    lock_account.save()
    # Logic to transfer tokens back to the owner should be implemented here

# Optional: Reset lock with a new unlock time
@instruction
def reset_lock(lock_account: LockAccount, new_unlock_time: u64):
    assert lock_account.is_locked, "Lock is not active"
    lock_account.unlock_time = new_unlock_time
    lock_account.save()
