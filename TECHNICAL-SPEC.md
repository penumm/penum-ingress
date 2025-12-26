# Penum Ingress - Technical Specification

## Overview
Penum Ingress is a privacy-preserving Ethereum transaction ingress layer that accepts only fully signed Ethereum transactions and forwards them to MEV relays/builders in a way that reduces transaction-level metadata correlation and makes MEV-driven surveillance and soft censorship harder and more observable.

## Architecture

### Components
1. **Transaction Envelope**: Standardized format for transaction encapsulation
2. **Deterministic Batching Engine**: Secure batching with cryptographically secure shuffling
3. **Commit-Reveal Pipeline**: SHA-256 commitments before content revelation
4. **Relay Forwarding Layer**: Relay-agnostic forwarding to existing MEV infrastructure
5. **Privacy-Safe Observability**: Aggregate metrics without leaking sensitive data

### Data Flow
```
Ethereum Wallet → Penum Ingress → MEV Relays/Builders → Block Producers
(signs transaction)  (batches, shuffles, commits)    (existing infrastructure)
```

## Cryptographic Protocol

### Randomization
- Algorithm: CSPRNG via `rand::rngs::OsRng` (ChaCha20-based)
- Purpose: Cryptographically secure transaction shuffling
- Implementation: `rand` crate with OS entropy source

### Commitments
- Algorithm: SHA-256
- Purpose: Batch commitments before content revelation
- Format: SHA256(concat(sorted(tx_hashes) || batch_nonce))
- Security: Preimage and collision resistant

### Batch Formation
- Method: Fixed time windows or fixed batch sizes
- Shuffling: Cryptographically secure random permutation
- Nonce: Cryptographically random batch identifier

## Security Properties

### Privacy Guarantees
- Timing Privacy: Batching breaks direct timing correlation
- Ordering Privacy: Cryptographic shuffling obscures original order
- Behavioral Privacy: Groups transactions to mask individual patterns
- Censorship Detection: Commit-reveal makes suppression observable

### Threat Model
- Trusted: Transaction signers (wallets/users)
- Untrusted: MEV relays, builders, block proposers
- Assumptions: Adversaries cannot break cryptographic primitives

### Attack Resistance
- Timing Analysis: Mitigated through batching
- Ordering Analysis: Mitigated through shuffling
- Correlation Attacks: Mitigated through aggregation
- Censorship: Detectable through commit-reveal scheme

## Implementation Details

### Transaction Envelope
```rust
struct TransactionEnvelope {
    tx_bytes: Vec<u8>,        // Serialized transaction
    batch_id: String,         // Batch identifier
    envelope_version: u8,     // Version for future compatibility
}
```

### Batching Configuration
```json
{
  "batch_size": 100,
  "batch_interval_ms": 1000,
  "shuffle_algorithm": "cryptographically_secure"
}
```

### Supported Transaction Types
- Legacy transactions (EIP-155)
- EIP-2930 (access list transactions)
- EIP-1559 (fee market transactions)
- Future transaction types (upgradeable)

## Performance Characteristics

### Latency
- Batching Delay: Configurable (100ms to 5s)
- Processing: <10ms per transaction
- Total: Batch interval + processing time

### Throughput
- Theoretical Max: Limited by relay capacity
- Practical: Thousands of transactions per batch
- Scalability: Linear with number of ingress nodes

### Resource Usage
- Memory: Proportional to batch size
- CPU: Minimal (batching and shuffling)
- Bandwidth: Efficient due to aggregation

## Security Features

### Cryptographically Secure Shuffling
- Uses `rand::rngs::OsRng` for true randomness
- Implements Fisher-Yates shuffle algorithm
- Ensures uniform distribution of transaction ordering

### Commit-Reveal Scheme
- SHA-256 commitments published before content revelation
- Enables censorship detection and verification
- Maintains transaction integrity

### Deterministic Behavior
- Reproducible batching for verification
- Consistent behavior across restarts
- Predictable performance characteristics

### Privacy-Safe Metrics
- Aggregate statistics only
- No individual transaction data
- Observable without privacy leakage

## Integration Points

### Relay Compatibility
- Flashbots Relay API
- Eden Network API
- Bloxroute API
- Custom relay implementations

### Monitoring Endpoints
- Batch statistics
- Performance metrics
- Privacy effectiveness measurements
- Health checks

## Future Enhancements

### Advanced Batching
- Adaptive batch sizing based on network conditions
- Machine learning for optimal batch timing
- Cross-block batch coordination

### Enhanced Privacy
- Differential privacy techniques
- Mix networks for additional obfuscation
- Zero-knowledge proofs for verification

### Protocol Improvements
- Multi-party batching for enhanced privacy
- Decentralized ingress coordination
- Incentive mechanisms for ingress operators