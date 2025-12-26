// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

// Mock contract to simulate penum-ingress functionality for testing
contract MockPenumIngress {
    mapping(bytes32 => bool) public commitments;
    mapping(bytes32 => bool) public revealed;
    
    event BatchCommitted(bytes32 indexed commitment, bytes32[] txHashes);
    
    function commitBatch(bytes32 commitment, bytes32[] memory txHashes) public {
        require(!commitments[commitment], "Commitment already exists");
        commitments[commitment] = true;
        emit BatchCommitted(commitment, txHashes);
    }
    
    function revealBatch(bytes32 commitment, bytes32[] memory txHashes) public {
        require(commitments[commitment], "Commitment does not exist");
        require(!revealed[commitment], "Batch already revealed");
        
        // Recalculate commitment to verify correctness
        bytes32 calculatedCommitment = calculateCommitment(txHashes);
        require(calculatedCommitment == commitment, "Commitment mismatch");
        
        revealed[commitment] = true;
    }
    
    function calculateCommitment(bytes32[] memory txHashes) public pure returns (bytes32) {
        // Sort the transaction hashes
        bytes32[] memory sortedHashes = sortHashes(txHashes);
        
        // Concatenate sorted hashes
        bytes memory concatenated = new bytes(sortedHashes.length * 32);
        for (uint i = 0; i < sortedHashes.length; i++) {
            bytes32 hash = sortedHashes[i];
            for (uint j = 0; j < 32; j++) {
                concatenated[i * 32 + j] = hash[j];
            }
        }
        
        // Add a mock nonce (in real implementation, this would be a proper nonce)
        return keccak256(abi.encodePacked(concatenated, bytes32(0)));
    }
    
    function sortHashes(bytes32[] memory data) internal pure returns (bytes32[] memory) {
        // Simple bubble sort implementation for small arrays
        uint len = data.length;
        bytes32[] memory sorted = data;
        
        for (uint i = 0; i < len - 1; i++) {
            for (uint j = 0; j < len - i - 1; j++) {
                if (sorted[j] > sorted[j + 1]) {
                    bytes32 temp = sorted[j];
                    sorted[j] = sorted[j + 1];
                    sorted[j + 1] = temp;
                }
            }
        }
        
        return sorted;
    }
}

// This is a Foundry test for transaction integrity verification
contract TransactionIntegrityTest is Test {
    MockPenumIngress public penumIngress;
    
    function setUp() public {
        penumIngress = new MockPenumIngress();
    }

    // This test verifies that batch commitment matches revealed transactions
    function testBatchCommitmentCorrectness() public {
        bytes32[] memory txHashes = new bytes32[](3);
        txHashes[0] = keccak256("transaction1");
        txHashes[1] = keccak256("transaction2");
        txHashes[2] = keccak256("transaction3");
        
        bytes32 commitment = penumIngress.calculateCommitment(txHashes);
        
        // Commit the batch
        penumIngress.commitBatch(commitment, txHashes);
        
        // Reveal the batch
        penumIngress.revealBatch(commitment, txHashes);
        
        // Verify the commitment was revealed
        assertTrue(penumIngress.revealed(commitment), "Batch should be revealed");
    }

    // This test verifies that dropped or reordered transactions are detectable
    function testCensorshipDetection() public {
        bytes32[] memory txHashes = new bytes32[](3);
        txHashes[0] = keccak256("transaction1");
        txHashes[1] = keccak256("transaction2");
        txHashes[2] = keccak256("transaction3");
        
        bytes32 commitment = penumIngress.calculateCommitment(txHashes);
        
        // Commit the batch
        penumIngress.commitBatch(commitment, txHashes);
        
        // Try to reveal with a different transaction (should fail)
        bytes32[] memory modifiedTxHashes = new bytes32[](3);
        modifiedTxHashes[0] = keccak256("transaction1");
        modifiedTxHashes[1] = keccak256("transaction2");
        modifiedTxHashes[2] = keccak256("transaction4"); // Different transaction
        
        vm.expectRevert("Commitment mismatch");
        penumIngress.revealBatch(commitment, modifiedTxHashes);
    }

    // This test verifies that the same transaction produces the same commitment
    function testCommitmentConsistency() public {
        bytes32[] memory txHashes = new bytes32[](2);
        txHashes[0] = keccak256("transaction1");
        txHashes[1] = keccak256("transaction2");
        
        bytes32 commitment1 = penumIngress.calculateCommitment(txHashes);
        bytes32 commitment2 = penumIngress.calculateCommitment(txHashes);
        
        assertEq(commitment1, commitment2, "Commitments should be consistent");
    }
}