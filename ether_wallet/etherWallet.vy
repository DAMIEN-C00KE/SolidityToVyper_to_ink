# @version ^0.3.9

owner: public(address)

event Deposit:
    sender: indexed(address)
    amount: uint256

@external
def __init__():
    self.owner = msg.sender

@external
@payable
def deposit():
    log Deposit(msg.sender, msg.value)

@external
@payable
def withdraw(_amount: uint256):
    assert msg.sender == self.owner, "caller is not owner"
    send(self.owner, _amount)

@external
@view
def getBalance() -> uint256:
    return self.balance
