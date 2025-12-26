# Foundry Tests for penum-ingress

This directory contains Foundry tests for the penum-ingress system, focused on transaction integrity verification, commit-reveal correctness, and execution equivalence.

## Test Coverage

The tests specifically verify:

1. **Batch Commitment Correctness**: Verifies that the SHA256 commitment calculation works correctly and matches revealed transactions.

2. **Censorship Detection**: Tests that dropped or reordered transactions are detectable through commitment verification.

3. **Commitment Consistency**: Ensures that the same transactions produce the same commitment values.

## Test Design

The tests use a mock implementation of the penum-ingress system to validate the core cryptographic properties:

- Commit-reveal functionality
- Batch commitment calculation using SHA256(concat(sorted(tx_hashes) || batch_nonce))
- Detection of transaction modifications

## Installation on Windows

Foundry does not natively support Windows PowerShell. You must use one of the following methods:

### Option 1: Using WSL (Windows Subsystem for Linux)
1. Install WSL: `wsl --install`
2. Restart your computer
3. Launch Ubuntu from the Start menu
4. In the Ubuntu terminal, run:
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

### Option 2: Using Git BASH
1. Download and install Git BASH from: https://gitforwindows.org/
2. Open Git BASH terminal
3. Run:
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

## Running the Tests

### Execute Tests

```bash
# Navigate to the foundry-test directory
cd foundry-test

# Run all tests
forge test

# Run tests with logs
forge test -vvv
```

## Expected Results

All tests should pass:
- `testBatchCommitmentCorrectness`: Verifies batch commitments match revealed transactions
- `testCensorshipDetection`: Ensures modified transactions are detected and rejected
- `testCommitmentConsistency`: Confirms deterministic commitment calculation

## Security Properties Tested

- Commitments are calculated correctly using sorted transaction hashes
- Revealed transactions match the original commitment
- Modifications to transaction sets are detectable
- Commitment calculation is deterministic and consistent