// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/NounsRaffle.sol";

contract RequestUpdateScript is Script {
    function run() public {
        vm.startBroadcast();
        address gateway = 0x6e4f1e9eA315EBFd69d18C2DB974EEf6105FB803;
        bytes32 functionId = 0x64699cf367ef7ad72dad20aea6bb68a7624e8aef7e8e3c96e675020a398275f0;
        address lightclient = address(0x53Fd3133bEf76DdA3ca18FB24769ebe59E28BB24);
        address owner = 0xC5D0fF59Ff452668080b1F0385Bd90b86195EfDd;
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
        // uint64 raffleIdx = 0;
        // uint64 targetSlot = 6339600;
        // vm.startBroadcast();
        // raffle.startRaffle(raffleIdx, targetSlot);
        // vm.stopBroadcast();
    }
}
