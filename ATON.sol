/**
 * This file was automatically generated by Stylus and represents a Rust program.
 * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
 */

// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface IATON  {
    function name() external pure returns (string memory);

    function symbol() external pure returns (string memory);

    function decimals() external pure returns (uint8);

    function totalSupply() external view returns (uint256);

    function balanceOf(address owner) external view returns (uint256);

    function transferFrom(address from, address to, uint256 value) external returns (bool);

    function approve(address spender, uint256 value) external returns (bool);

    function allowance(address owner, address spender) external view returns (uint256);

    function owner() external view returns (address);

    function transferOwnership(address new_owner) external;

    function initialize() external returns (bool);

    function transfer(address to, uint256 amount) external returns (bool);

    function mintAton() external payable returns (bool);

    function swap(uint256 amount) external returns (bool);

    function updateEngine(address account, bool status) external;

    error InsufficientBalance(address, uint256, uint256);

    error Zero(address);

    error InsufficientAllowance(address, address, uint256, uint256);

    error UnauthorizedAccount(address);
}