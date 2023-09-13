# @version ^0.3.9

theBool: public(bool)

event UpdatedBool:
    theBool: indexed(bool)

@public
def __init__(_theBool: bool):
    assert _theBool == True, "_theBool must start as true"
    self.theBool = _theBool

@public
def setBool(newBool: bool) -> bool:
    boolChanged: bool = False

    if self._theBool != newBool:
        boolChanged = True

    self._theBool = newBool

    # emit event
    log.UpdatedBool(newBool)

    return boolChanged 

