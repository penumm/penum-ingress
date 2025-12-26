# penum-ingress Threat Model

## Adversary Capabilities (What Adversaries CAN Do)

### 1. Transaction Submission Timing Observation
- Monitor when transactions are submitted to MEV relays/builders
- Correlate submission timing with transaction contents
- Identify patterns in submission behavior

### 2. Ordering Pattern Analysis
- Observe transaction ordering across different relays/builders
- Analyze sequence patterns to infer original submission order
- Perform statistical correlation analysis on ordering

### 3. Inclusion Behavior Monitoring
- Track which transactions are included in blocks
- Analyze inclusion timing and patterns
- Correlate inclusion with submission timing

### 4. Infrastructure Collaboration
- Adversaries across multiple infrastructure layers can collaborate
- Share timing and correlation data
- Combine observations from different vantage points

### 5. Statistical Correlation Analysis
- Apply statistical methods to identify patterns
- Use machine learning techniques to find correlations
- Perform cross-referencing between different data sources

### 6. Honest-But-Curious Model
- Infrastructure providers may be honest in protocol compliance
- But curious about transaction sources and timing
- May retain logs for analysis

## Adversary Limitations (What Adversaries CANNOT Do)

### 1. Cryptographic Primitive Break
- Cannot break SHA-256 hashing algorithm
- Cannot compromise cryptographically secure PRNGs
- Cannot forge Ethereum transaction signatures

### 2. Internal System Access
- Cannot access internal penum-ingress memory
- Cannot observe internal batching decisions before commitment
- Cannot modify system state directly

### 3. Transaction Content Modification
- Cannot alter signed transaction contents
- Cannot change nonces, gas fields, signatures, or calldata
- Cannot modify cryptographic signatures

### 4. User Key Control
- Cannot access user signing keys
- Cannot sign transactions on behalf of users
- Cannot impersonate users at the signing level

## Attack Vectors

### 1. Timing Analysis Attacks
- **Risk**: Correlating submission timing with user activity
- **Mitigation**: Fixed batching windows and secure shuffling
- **Residual Risk**: Statistical correlation may still be possible

### 2. Order Analysis Attacks
- **Risk**: Inferring original transaction order through pattern analysis
- **Mitigation**: Cryptographically secure shuffling with deterministic seeds
- **Residual Risk**: Sophisticated statistical analysis may reveal patterns

### 3. Fingerprinting Attacks
- **Risk**: Identifying penum-ingress traffic patterns
- **Mitigation**: Normalized submission behavior and relay-agnostic forwarding
- **Residual Risk**: Traffic analysis may still identify batching patterns

### 4. Censorship Attacks
- **Risk**: Suppressing specific transactions without detection
- **Mitigation**: Commit-reveal pipeline with immutable commitments
- **Residual Risk**: Relay operators may still attempt censorship

### 5. Statistical Inference Attacks
- **Risk**: Using aggregate statistics to infer individual behavior
- **Mitigation**: Privacy-safe observability with no per-user logging
- **Residual Risk**: Cross-correlation with external data sources

## Security Properties

### 1. Timing Correlation Reduction
- **Property**: Reduces correlation between transaction submission and user activity
- **Method**: Fixed batching windows break direct timing correlation
- **Verification**: Off-chain analysis tests measure correlation probability

### 2. Order Randomization
- **Property**: Randomizes transaction order within batches
- **Method**: Cryptographically secure deterministic shuffle
- **Verification**: Batch entropy measurement tests

### 3. Censorship Detectability
- **Property**: Makes transaction suppression detectable
- **Method**: Commit-reveal pipeline with immutable commitments
- **Verification**: Commitment matching tests detect dropped transactions

### 4. Privacy Preservation
- **Property**: Prevents user-specific metadata collection
- **Method**: No per-user state or logging
- **Verification**: Audit of observability components

## Risk Assessment

### High Risk Areas
- Timing correlation through statistical analysis
- Identification of penum-ingress traffic patterns
- Potential relay-level censorship

### Medium Risk Areas
- Sophisticated statistical inference
- Cross-infrastructure correlation
- Fingerprinting of batching behavior

### Low Risk Areas
- Transaction content modification (protected by signatures)
- Direct system compromise (external to penum-ingress)
- Cryptographic primitive attacks

## Countermeasures Effectiveness

The system design implements multiple countermeasures:
- **Batching**: Reduces timing correlation
- **Shuffling**: Reduces ordering correlation
- **Commit-Reveal**: Enables censorship detection
- **Privacy-Safe Observability**: Prevents metadata leaks
- **Relay Agnosticism**: Maintains compatibility and distribution

**Effectiveness**: These measures reduce correlation probability but do not eliminate it entirely. The system provides measurable improvement over direct relay submission while maintaining compatibility with existing infrastructure.