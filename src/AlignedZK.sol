// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.24;

import "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";

// This is how the Aligned Layer Service Manager is defined.
// So I imagine we are just writing a Rollup that calls the 3
// functions in the interface.
interface IAlignedLayerServiceManager {
    function createNewTask(bytes32 batchMerkleRoot, string calldata batchDataPointer) external payable;
    function respondToTask(bytes32 batchMerkleRoot, bytes calldata nonSignerStakesAndSignature) external;
    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes calldata merkleProof,
        uint256 verificationDataBatchIndex
    ) external view returns (bool);
}

contract ZKRollup is Ownable {
    IAlignedLayerServiceManager public alignedManager;

    // params for the Aligned kini (library?).
    bytes32 public currentBatchMerkleRoot;
    string public currentBatchDataPointer;

    // events that we're going to emit.
    event BatchSubmitted(bytes32 indexed batchMerkleRoot, string batchDataPointer);
    event BatchVerified(bytes32 indexed batchMerkleRoot, bool verified);

    // build the manager constructor.
    constructor(address _alignedManager) {
        alignedManager = IAlignedLayerServiceManager(_alignedManager);
    }

    // two major functions we need.

    function submitBatch() {}

    function verifyBatch() {}
}