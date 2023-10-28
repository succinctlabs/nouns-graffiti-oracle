// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/NounsRaffle.sol";

contract RequestUpdateScript is Script {
    function run() public {
        vm.startBroadcast();
        address gateway = 0x6e4f1e9eA315EBFd69d18C2DB974EEf6105FB803;
        bytes32 functionId = 0xef77b52fb0f0aa51fe587c7476df0e0417d4b4e1160f1fd7f4cfac206dd6f75a;
        address lightclient = address(0x53Fd3133bEf76DdA3ca18FB24769ebe59E28BB24);
        address owner = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;
        address prover = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;
        uint256 payoutAmount = 0;
        NounsRaffle raffle = new NounsRaffle(
            gateway,
            functionId,
            lightclient,
            owner,
            prover,
            payoutAmount
        );
        vm.stopBroadcast();
    }
}
