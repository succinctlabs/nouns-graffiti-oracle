// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/NounsRaffle.sol";

contract RequestUpdateScript is Script {
    function run() public {
        vm.startBroadcast();
        address gateway = 0xE304f6B116bE5e43424cEC36a5eFd0B642E0dC95;
        bytes32 functionId = 0xad63f1e4433832495bc28d88b01a2d30cf969374eda7cb4c46807ca6448d3f7f;
        address lightclient = address(0);
        address owner = 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e;
        address prover = owner;
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
        uint64 raffleIdx = 2;
        uint64 targetSlot = 6778799;
        vm.startBroadcast();
        raffle.startRaffle(raffleIdx, targetSlot);
        vm.stopBroadcast();
    }
}
