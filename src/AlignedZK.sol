// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.24;

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

}