// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/NounsRaffle.sol";

contract RequestUpdateScript is Script {
    function run() public {
        vm.startBroadcast();
        address gateway = 0xE304f6B116bE5e43424cEC36a5eFd0B642E0dC95;
        bytes32 functionId = 0xfbb1246ca21e36bdc216ab69cafc3a365a18c3c5e6e04a9b90840311bf8882e1;
        address lightclient = address(0);
        address owner = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;
        uint256 payoutAmount = 0;
        NounsRaffle raffle = new NounsRaffle(gateway, functionId, lightclient, owner, payoutAmount);
        vm.stopBroadcast();
        uint64 raffleIdx = 0;
        uint64 targetSlot = 6339600;
        vm.startBroadcast();
        raffle.startRaffle(raffleIdx, targetSlot);
        vm.stopBroadcast();
    }
}
