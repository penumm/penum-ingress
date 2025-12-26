# penum-ingress Design Decisions

## Cryptographic Decisions

### 1. SHA-256 for Batch Commitments
**Decision**: Use SHA-256 for batch commitments instead of Keccak-256 (Ethereum's standard hash).

**Rationale**: 
- SHA-256 is widely standardized and well-understood
- Provides strong collision resistance for commitment schemes
- Separates internal commitment logic from Ethereum transaction hashing
- Reduces potential confusion between different hashing contexts

**Alternative Considered**: Keccak-256 (Ethereum standard)
**Why Not**: Would create confusion between internal commitments and transaction-level hashing

### 2. Cryptographically Secure PRNG for Shuffling
**Decision**: Use `rand::rngs::OsRng` or `ChaCha20Rng` (seeded) for shuffling.

**Rationale**:
- Provides cryptographically secure randomness for shuffling
- Ensures deterministic behavior when seeded
- Prevents predictable ordering that could be exploited
- Meets security requirements for breaking correlation

**Alternative Considered**: Standard `Math.random` or simple shuffling algorithms
**Why Not**: Insufficient security, predictable patterns, non-deterministic

### 3. Deterministic Shuffling with Seed
**Decision**: Shuffling must be deterministic given a seed.

**Rationale**:
- Enables auditability and reproducibility
- Allows verification of shuffle correctness
- Prevents manipulation through non-deterministic behavior
- Supports testing and validation

## Architecture Decisions

### 4. Transaction Envelope Design
**Decision**: Minimal envelope containing only tx_bytes, batch_id, and envelope_version.

**Rationale**:
- Maintains transaction integrity without modification
- Provides necessary metadata for batching
- Minimizes complexity and potential attack surface
- Preserves all original transaction fields

**Constraints**:
- Must NOT modify nonce, gas fields, signature, or calldata
- Must preserve cryptographic validity

### 5. Fixed Window vs. Fixed Size Batching
**Decision**: Support either fixed time window OR fixed batch size (configurable).

**Rationale**:
- Fixed time windows: Better for timing correlation reduction
- Fixed batch size: Better for predictable resource usage
- Configurability allows optimization for different use cases
- Both approaches achieve correlation reduction goals

### 6. Commit-Reveal Pipeline
**Decision**: Implement strict commit-before-reveal for batch contents.

**Rationale**:
- Makes censorship detectable through commitment verification
- Prevents relay operators from silently dropping transactions
- Creates immutable record of batch contents
- Enables third-party verification of processing

**Implementation**:
- Calculate SHA-256 commitment before revealing contents
- Persist commitment immutably
- Only reveal after commitment is recorded

### 7. Relay-Agnostic Forwarding
**Decision**: Design forwarding layer to be relay-agnostic and stateless.

**Rationale**:
- Maintains compatibility with existing MEV infrastructure
- Prevents lock-in to specific relay providers
- Simplifies implementation and maintenance
- Supports multiple relay strategies

**Constraints**:
- No relay-specific state persistence
- No prioritization based on relay responses
- Failover must not introduce timing fingerprints

## Privacy and Observability Decisions

### 8. Privacy-Safe Metrics
**Decision**: Collect only aggregate metrics, never per-user data.

**Rationale**:
- Prevents metadata correlation attacks
- Maintains user privacy
- Supports system monitoring without privacy leaks
- Enables auditability without tracking

**Allowed Metrics**:
- Batch sizes
- Forwarding latency
- Relay acceptance rates

**Prohibited Logging**:
- Wallet addresses
- Per-user timestamps
- Per-transaction identifiers tied to users

### 9. Stateless Design Priority
**Decision**: Prioritize stateless operation where possible.

**Rationale**:
- Reduces complexity and potential failure modes
- Improves scalability
- Minimizes data persistence risks
- Simplifies deployment and maintenance

**Exception**: Minimal ephemeral state only when strictly necessary (e.g., commitment logs)

## Technology Decisions

### 10. Rust Implementation Preference
**Decision**: Prefer Rust for implementation due to safety and determinism.

**Rationale**:
- Memory safety prevents many classes of vulnerabilities
- Deterministic behavior important for cryptographic operations
- Strong type system reduces runtime errors
- Good cryptographic library ecosystem

**Alternative**: TypeScript
**Condition**: Only if it reduces LOC and complexity significantly

### 11. Minimal Framework Usage
**Decision**: Avoid heavy frameworks and abstractions.

**Rationale**:
- Reduces attack surface
- Improves auditability
- Maintains focus on core functionality
- Reduces dependency complexity

## Testing Decisions

### 12. Foundry Testing Scope
**Decision**: Limit Foundry testing to transaction integrity and commit-reveal correctness.

**Rationale**:
- Foundry is appropriate for execution equivalence testing
- Transaction integrity verification requires EVM execution
- Privacy and correlation testing is off-chain
- Maintains clear testing boundaries

**Limited Scope**:
- Transaction forwarding integrity
- Commit-reveal correctness
- Execution equivalence

**Excluded from Foundry**:
- Metadata correlation testing
- Timing entropy analysis
- Privacy guarantees

### 13. Off-chain Analysis Testing
**Decision**: Use Rust/TypeScript for privacy and correlation analysis.

**Rationale**:
- Privacy analysis requires different methodologies than on-chain testing
- Statistical correlation testing is off-chain by nature
- Allows synthetic adversary modeling
- Provides measurable statistics rather than claims

## Security Decisions

### 14. No Encryption or ZK Systems
**Decision**: Explicitly exclude encryption, SNARKs, STARKs, or ZK systems.

**Rationale**:
- Out of scope for transaction ingress layer
- Adds unnecessary complexity
- May introduce new attack vectors
- Focus on correlation reduction rather than encryption

### 15. No Economic Incentives
**Decision**: Exclude economic incentives, tokens, or fees.

**Rationale**:
- Prevents drift into MEV strategy implementation
- Maintains focus on privacy-preserving ingress
- Reduces complexity and potential vulnerabilities
- Aligns with system scope boundaries