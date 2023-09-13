// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract BasicContract {
    bool private _theBool;
    event UpdatedBool(bool indexed _theBool);

    constructor(bool theBool_) {
        require(theBool_ == true, "theBool_ must start as true");

        _theBool = theBool_;
    }

    function setBool(bool newBool) public returns (bool boolChanged) {
        if (_theBool == newBool) {
               boolChanged = false;
        } else {
            boolChanged = true;
        }

        _theBool = newBool;
        // emit event
        emit UpdatedBool(newBool);
    }
}
