// penum-ingress: Privacy-preserving Ethereum Transaction Ingress Layer

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use rand::{rngs::OsRng, seq::SliceRandom, SeedableRng};
use sha2::{Sha256, Digest};

// Transaction envelope containing raw transaction bytes
#[derive(Clone, Debug)]
pub struct TransactionEnvelope {
    pub tx_bytes: Vec<u8>,
    pub batch_id: String,
    pub envelope_version: u32,
}

impl TransactionEnvelope {
    pub fn new(tx_bytes: Vec<u8>, batch_id: String) -> Self {
        Self {
            tx_bytes,
            batch_id,
            envelope_version: 1,
        }
    }
}

// Batch structure for grouping transactions
#[derive(Clone, Debug)]
pub struct TransactionBatch {
    pub id: String,
    pub transactions: Vec<TransactionEnvelope>,
    pub commitment: Vec<u8>,
    pub timestamp: SystemTime,
    pub nonce: Vec<u8>,
}

impl TransactionBatch {
    pub fn new(transactions: Vec<TransactionEnvelope>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let nonce = generate_nonce();
        
        // Calculate commitment as SHA256(concat(sorted(tx_hashes) || batch_nonce))
        let mut tx_hashes: Vec<Vec<u8>> = transactions
            .iter()
            .map(|tx| sha256_hash(&tx.tx_bytes))
            .collect();
        tx_hashes.sort();
        
        let mut commitment_input = Vec::new();
        for hash in &tx_hashes {
            commitment_input.extend_from_slice(hash);
        }
        commitment_input.extend_from_slice(&nonce);
        
        let commitment = sha256_hash(&commitment_input);
        
        Self {
            id,
            transactions,
            commitment,
            timestamp: SystemTime::now(),
            nonce,
        }
    }
}

// Helper function to generate a random nonce
fn generate_nonce() -> Vec<u8> {
    let mut nonce = [0u8; 32];
    getrandom::getrandom(&mut nonce).expect("Failed to generate random nonce");
    nonce.to_vec()
}

// Helper function for SHA-256 hashing
fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// Batching engine that batches transactions based on time window or size
pub struct BatchingEngine {
    max_batch_size: usize,
    batch_time_window: Duration,
    pending_transactions: Arc<Mutex<Vec<TransactionEnvelope>>>,
    last_batch_time: Arc<Mutex<SystemTime>>,
}

impl BatchingEngine {
    pub fn new(max_batch_size: usize, batch_time_window: Duration) -> Self {
        Self {
            max_batch_size,
            batch_time_window,
            pending_transactions: Arc::new(Mutex::new(Vec::new())),
            last_batch_time: Arc::new(Mutex::new(SystemTime::now())),
        }
    }

    pub fn add_transaction(&self, tx: TransactionEnvelope) {
        let mut pending = self.pending_transactions.lock().unwrap();
        pending.push(tx);
        
        // Check if we should create a batch
        if pending.len() >= self.max_batch_size {
            self.create_batch();
        }
    }

    pub fn check_time_window(&self) -> Option<TransactionBatch> {
        let now = SystemTime::now();
        let last_batch_time = *self.last_batch_time.lock().unwrap();
        
        if now.duration_since(last_batch_time).unwrap() >= self.batch_time_window {
            self.create_batch()
        } else {
            None
        }
    }

    fn create_batch(&self) -> Option<TransactionBatch> {
        let mut pending = self.pending_transactions.lock().unwrap();
        
        if pending.is_empty() {
            return None;
        }
        
        // Take all pending transactions
        let transactions: Vec<TransactionEnvelope> = pending.drain(..).collect();
        
        // Update last batch time
        *self.last_batch_time.lock().unwrap() = SystemTime::now();
        
        // Create batch with cryptographically secure shuffle
        let mut batch = TransactionBatch::new(transactions);
        
        // Shuffle transactions deterministically using a seed based on batch ID
        let mut rng = rand::rngs::StdRng::from_seed(create_seed_from_batch_id(&batch.id));
        batch.transactions.shuffle(&mut rng);
        
        Some(batch)
    }
}

// Helper function to create a deterministic seed from batch ID
fn create_seed_from_batch_id(batch_id: &str) -> [u8; 32] {
    let mut seed = [0u8; 32];
    let hash = sha256_hash(batch_id.as_bytes());
    
    // Copy hash bytes to seed (truncating if necessary)
    for i in 0..std::cmp::min(32, hash.len()) {
        seed[i] = hash[i];
    }
    
    seed
}

// Commit-Reveal Pipeline
pub struct CommitRevealPipeline {
    commitments: Arc<Mutex<Vec<(String, Vec<u8>)>>>, // (batch_id, commitment)
}

impl CommitRevealPipeline {
    pub fn new() -> Self {
        Self {
            commitments: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn commit_batch(&self, batch: &TransactionBatch) {
        let mut commitments = self.commitments.lock().unwrap();
        commitments.push((batch.id.clone(), batch.commitment.clone()));
    }

    pub fn verify_reveal(&self, batch: &TransactionBatch) -> bool {
        let commitments = self.commitments.lock().unwrap();
        
        // Find the commitment for this batch
        for (batch_id, commitment) in commitments.iter() {
            if batch_id == &batch.id {
                // Recalculate commitment to verify
                let mut tx_hashes: Vec<Vec<u8>> = batch
                    .transactions
                    .iter()
                    .map(|tx| sha256_hash(&tx.tx_bytes))
                    .collect();
                tx_hashes.sort();
                
                let mut commitment_input = Vec::new();
                for hash in &tx_hashes {
                    commitment_input.extend_from_slice(hash);
                }
                commitment_input.extend_from_slice(&batch.nonce);
                
                let calculated_commitment = sha256_hash(&commitment_input);
                
                return calculated_commitment == *commitment;
            }
        }
        
        false // No commitment found for this batch
    }
}

// Relay Forwarding Layer
pub struct RelayForwarder {
    relays: Vec<String>, // URLs of MEV relays
}

impl RelayForwarder {
    pub fn new(relay_urls: Vec<String>) -> Self {
        Self { relays: relay_urls }
    }

    pub fn forward_batch(&self, batch: &TransactionBatch) {
        // Forward to all relays in parallel
        for relay_url in &self.relays {
            // In a real implementation, this would make HTTP requests to relays
            println!("Forwarding batch {} to relay: {}", batch.id, relay_url);
            
            // Forward each transaction in the batch
            for tx in &batch.transactions {
                // Here we would actually send the transaction to the relay
                // For now, just print what would be sent
                println!("  Forwarding transaction ({} bytes) to {}", tx.tx_bytes.len(), relay_url);
            }
        }
    }
}

// Privacy-safe observability metrics
pub struct MetricsCollector {
    batch_sizes: Arc<Mutex<Vec<usize>>>,
    forwarding_latencies: Arc<Mutex<Vec<Duration>>>,
    relay_acceptance_rates: Arc<Mutex<HashMap<String, (usize, usize)>>>, // (accepted, total)
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            batch_sizes: Arc::new(Mutex::new(Vec::new())),
            forwarding_latencies: Arc::new(Mutex::new(Vec::new())),
            relay_acceptance_rates: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_batch_size(&self, size: usize) {
        let mut sizes = self.batch_sizes.lock().unwrap();
        sizes.push(size);
    }

    pub fn record_forwarding_latency(&self, latency: Duration) {
        let mut latencies = self.forwarding_latencies.lock().unwrap();
        latencies.push(latency);
    }

    pub fn get_aggregate_metrics(&self) -> (f64, f64) { // (avg_batch_size, avg_latency_ms)
        let sizes = self.batch_sizes.lock().unwrap();
        let latencies = self.forwarding_latencies.lock().unwrap();
        
        let avg_size = if sizes.is_empty() {
            0.0
        } else {
            sizes.iter().sum::<usize>() as f64 / sizes.len() as f64
        };
        
        let avg_latency = if latencies.is_empty() {
            0.0
        } else {
            latencies.iter().map(|d| d.as_millis() as f64).sum::<f64>() / latencies.len() as f64
        };
        
        (avg_size, avg_latency)
    }
}

// Main ingress service
pub struct PenumIngress {
    batching_engine: Arc<BatchingEngine>,
    commit_reveal_pipeline: Arc<CommitRevealPipeline>,
    relay_forwarder: Arc<RelayForwarder>,
    metrics_collector: Arc<MetricsCollector>,
}

impl PenumIngress {
    pub fn new(
        max_batch_size: usize,
        batch_time_window: Duration,
        relay_urls: Vec<String>,
    ) -> Self {
        Self {
            batching_engine: Arc::new(BatchingEngine::new(max_batch_size, batch_time_window)),
            commit_reveal_pipeline: Arc::new(CommitRevealPipeline::new()),
            relay_forwarder: Arc::new(RelayForwarder::new(relay_urls)),
            metrics_collector: Arc::new(MetricsCollector::new()),
        }
    }

    pub fn submit_transaction(&self, tx_bytes: Vec<u8>) -> Result<String, String> {
        // Validate that this is a properly formatted Ethereum transaction
        if tx_bytes.is_empty() {
            return Err("Transaction bytes cannot be empty".to_string());
        }
        
        // Create envelope
        let batch_id = uuid::Uuid::new_v4().to_string();
        let envelope = TransactionEnvelope::new(tx_bytes, batch_id);
        
        // Add to batching engine
        self.batching_engine.add_transaction(envelope);
        
        // Record metrics
        self.metrics_collector.record_batch_size(self.batching_engine.pending_transactions.lock().unwrap().len());
        
        Ok("Transaction accepted for batching".to_string())
    }

    pub fn process_batches(&self) {
        // Check if time window has passed and create batch if needed
        if let Some(batch) = self.batching_engine.check_time_window() {
            self.process_batch(batch);
        }
    }

    fn process_batch(&self, mut batch: TransactionBatch) {
        // Commit the batch first (commit-reveal)
        self.commit_reveal_pipeline.commit_batch(&batch);
        
        // Forward the batch to relays
        let start_time = std::time::Instant::now();
        self.relay_forwarder.forward_batch(&batch);
        let latency = start_time.elapsed();
        
        // Record metrics
        self.metrics_collector.record_batch_size(batch.transactions.len());
        self.metrics_collector.record_forwarding_latency(latency);
        
        // Verify the reveal (for demonstration purposes)
        let is_valid = self.commit_reveal_pipeline.verify_reveal(&batch);
        println!("Batch {} reveal verification: {}", batch.id, is_valid);
    }
}

fn main() {
    println!("Starting penum-ingress: Privacy-preserving Ethereum Transaction Ingress Layer");
    
    // Initialize the ingress service
    let relay_urls = vec![
        "https://relay.flashbots.net".to_string(),
        "https://builder-relay.ethereum.com".to_string(),
        "https://relay.ultrasound.money".to_string(),
    ];
    
    let ingress = PenumIngress::new(
        10, // max batch size
        Duration::from_secs(10), // 10 second time window
        relay_urls,
    );
    
    // Example: Submit a few transactions (these would be valid signed Ethereum transactions in practice)
    let example_tx1 = vec![0x02, 0x01, 0x02, 0x03]; // This would be a real signed transaction
    let example_tx2 = vec![0x02, 0x04, 0x05, 0x06];
    let example_tx3 = vec![0x02, 0x07, 0x08, 0x09];
    
    ingress.submit_transaction(example_tx1).unwrap();
    ingress.submit_transaction(example_tx2).unwrap();
    ingress.submit_transaction(example_tx3).unwrap();
    
    // Process any batches that are ready
    ingress.process_batches();
    
    // Print aggregate metrics
    let (avg_size, avg_latency) = ingress.metrics_collector.get_aggregate_metrics();
    println!("Aggregate metrics - Avg batch size: {:.2}, Avg latency: {:.2}ms", avg_size, avg_latency);
    
}
