# @version ^0.3.9

# Constants to represent shipping status
PENDING: public(uint256) = 0
SHIPPED: public(uint256) = 1
ACCEPTED: public(uint256) = 2
REJECTED: public(uint256) = 3
CANCELED: public(uint256) = 4

# Default value is the first element, in this case "PENDING"
status: public(uint256) = self.PENDING

@public
def get() -> uint256:
    return self.status

@public
def set(_status: uint256):
    assert _status >= self.PENDING and _status <= self.CANCELED, "Invalid value"
    self.status = _status

@public
def cancel():
    self.status = self.CANCELED

@public
def reset():
    self.status = self.PENDING
