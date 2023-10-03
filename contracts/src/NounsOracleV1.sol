// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

/// @title IFunctionGateway
/// @dev The function gateway automatically verifies the proof and calls the callback function.
interface IFunctionGateway {
    function request(
        bytes32 functionId,
        bytes memory inputs,
        bytes4 callbackSelector,
        bytes memory context
    ) external payable;
}

/// @title NounsOracleV1
/// @notice A demo of how the Succinct SDK can be used to augment the security of the Lido Oracle.
contract NounsOracleV1 {
    /// @notice The address of the function gateway.
    address public constant FUNCTION_GATEWAY = 0x852a94F8309D445D27222eDb1E92A4E83DdDd2a8;

    /// @notice The function id of the oracle.
    bytes32 public constant FUNCTION_ID =
        0xbae1b06917c6c241263d1672e6fdac1eaa0ecae80c63396e05de2346b4469ff5;
  
}
