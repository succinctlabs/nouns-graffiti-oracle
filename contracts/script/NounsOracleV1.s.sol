// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/NounsOracleV1.sol";

contract RequestUpdateScript is Script {
    function run() public {
        vm.broadcast();
        NounsOracleV1 s = NounsOracleV1(0x7A258aaa521e750dbe504117d660f6000C194404);
        bytes32 blockRoot = 0x49869d23ba93a746cc8ea649a48bb6c4b2159cf3a71aef492af63dac27522c9f;
        // s.requestUpdate{value: 30 gwei * 1_000_000}(blockRoot);
    }
}
