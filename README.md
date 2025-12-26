# penum-ingress

penum-ingress is a privacy-preserving Ethereum TRANSACTION INGRESS LAYER designed to reduce transaction-level metadata correlation and make MEV-driven surveillance and soft censorship harder and more observable.

## Grant Readiness

This project is fully prepared for Ethereum Foundation grant applications with:

- Complete technical specification ([TECHNICAL-SPEC.md](TECHNICAL-SPEC.md))
- Detailed grant proposal ([GRANT-PROPOSAL.md](GRANT-PROPOSAL.md))
- Comprehensive development roadmap ([ROADMAP.md](ROADMAP.md))
- Production-ready implementation
- Extensive documentation and testing

## Scope

penum-ingress accepts only fully signed Ethereum transactions and forwards them to existing MEV relays/builders in a way that reduces transaction-level metadata correlation. It operates strictly BEFORE transactions reach public mempools, MEV relays, builders, or proposers.

### What penum-ingress DOES
- Accept only fully signed Ethereum transactions
- Operate only on transaction ingress
- Forward transactions only to existing MEV relays/builders
- Reduce transaction-to-sender timing correlation
- Normalize observable submission behavior
- Preserve compatibility with existing MEV relays
- Make censorship and manipulation detectable

### What penum-ingress DOES NOT
- Implement RPC logic
- Act as a wallet
- Act as a proxy or VPN
- Perform transaction simulation or execution
- Implement MEV strategies
- Optimize ordering for profit
- Introduce economic incentives, tokens, or fees
- Require on-chain protocol changes
- Track users, identities, or addresses
- Persist per-user state

## Security Guarantees

penum-ingress aims to REDUCE correlation probability between transactions and their senders, particularly:
- Transaction timing correlation
- Ordering pattern correlation
- Inclusion behavior correlation

**Note**: penum-ingress does NOT claim to eliminate all correlation possibilities, but rather reduces them measurably.

## Architecture

The system consists of five mandatory components:

1. **Transaction Envelope**: Minimal wrapper for signed transactions
2. **Deterministic Batching Engine**: Time/window-based batching with secure shuffling
3. **Commit-Reveal Pipeline**: Immutable commitment before content revelation
4. **Relay Forwarding Layer**: Relay-agnostic forwarding to MEV infrastructure
5. **Privacy-Safe Observability**: Aggregate metrics only

## Cryptographic Primitives

- Hashing: SHA-256 for batch commitments
- Randomness: Cryptographically secure PRNG (Rust: `rand::rngs::OsRng` or `ChaCha20Rng`)
- Signatures: Ethereum transaction signatures (no modifications)

## Non-Goals

- Encryption or zero-knowledge systems
- MEV strategy implementation
- Economic incentive design
- On-chain protocol changes
- User tracking or identification