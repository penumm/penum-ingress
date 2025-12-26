# penum-ingress Architecture

## Position in Ethereum Transaction Pipeline

This document describes where penum-ingress sits in the Ethereum transaction lifecycle and how it interacts with various components.

## Transaction Flow

```
[Wallet/EOA] → [penum-ingress] → [MEV Relays] → [Builders] → [Proposers] → [Blocks]
```

### Traditional Flow (without penum-ingress)
```
[Wallet/EOA] → [Public Mempool] → [Miners/Builders] → [Blocks]
```

### With penum-ingress
```
[Wallet/EOA] → [Sign Transaction] → [penum-ingress] → [MEV Relays/Builders] → [Blocks]
```

## System Components

### 1. Transaction Envelope
- **Purpose**: Accept only fully signed Ethereum transactions
- **Interface**: Raw transaction bytes wrapped in minimal envelope
- **Responsibility**: Validation and encapsulation of signed transactions
- **Input**: Raw signed transaction bytes
- **Output**: Transaction envelope with batch_id and version

### 2. Deterministic Batching Engine
- **Purpose**: Break timing and single-transaction correlation
- **Mechanism**: Fixed time window or fixed batch size batching
- **Algorithm**: Cryptographically secure shuffle (deterministic with seed)
- **Input**: Transaction envelopes
- **Output**: Batched transaction sets

### 3. Commit-Reveal Pipeline
- **Purpose**: Make censorship and manipulation detectable
- **Mechanism**: Immutable commitment before content revelation
- **Algorithm**: SHA-256 commitment = SHA256(concat(sorted(tx_hashes) || batch_nonce))
- **Input**: Batched transactions
- **Output**: Commitment log + revealed batch contents

### 4. Relay Forwarding Layer
- **Purpose**: Forward transactions to MEV relays/builders
- **Mechanism**: Relay-agnostic, stateless forwarding
- **Responsibility**: Maintain compatibility with existing infrastructure
- **Input**: Revealed batch contents
- **Output**: Forwarded transactions to multiple relays

### 5. Privacy-Safe Observability
- **Purpose**: Provide auditability without privacy leaks
- **Mechanism**: Aggregate metrics only
- **Output**: Batch sizes, latency, relay acceptance rates (no per-user data)

## Data Flow

1. **Ingress**: Signed transaction bytes arrive at the transaction envelope component
2. **Validation**: Transaction envelope validates raw transaction format
3. **Batching**: Transactions enter the batching engine based on time/size triggers
4. **Commitment**: Batch commitment is calculated and logged before content revelation
5. **Reveal**: Full batch contents are revealed to relays
6. **Forwarding**: Transactions are forwarded to MEV relays/builders
7. **Observability**: Aggregate metrics are collected (no user-identifiable data)

## Security Boundaries

- **Before penum-ingress**: User has control of signed transaction
- **At penum-ingress**: Transaction correlation reduction occurs
- **After penum-ingress**: Traditional MEV infrastructure processes transactions

## Integration Points

- **Upstream**: Any wallet or application that can sign Ethereum transactions
- **Downstream**: Existing MEV relay infrastructure (Flashbots, Bloxroute, Eden, etc.)
- **No changes required**: Existing MEV relay interfaces remain unchanged