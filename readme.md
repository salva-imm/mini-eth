# Mini Ethereum – Implementation Roadmap

This roadmap guides you step-by-step to implement a mini Ethereum blockchain in Rust. It focuses on concepts, function responsibilities, and incremental development so you can learn while building.

---

## Phase 0 – Setup
**Goal:** Prepare dependencies and data structures.

**Tasks:**
- Define structs: `Transaction`, `Receipt`, `Block`, `BlockHeader`, `Account`.
- Add dependencies in `Cargo.toml`:
    - `secp256k1` – transaction signing
    - `keccak-hash` – hashing
    - `rlp` – transaction encoding
    - `serde`, `serde_json` – serialization
- Prepare a simple `HashMap<String, Account>` for global state.

---

## Phase 1 – Transaction Handling
**Functions:** `execute_transaction`, `update_state`

**Steps:**
1. Serialize transactions using RLP.
2. Sign transactions with `secp256k1`.
3. Validate transactions:
    - Correct nonce
    - Sufficient balance
    - Valid signature
    - Enough gas
4. Deduct gas and value from sender.
5. Add value to recipient.
6. Increment sender nonce.
7. Generate a `Receipt` recording:
    - Status (success/failure)
    - Gas used
    - Logs (optional)

> Focus on deterministic state updates.

---

## Phase 2 – Block Production
**Functions:** `produce_block`

**Steps:**
1. Collect valid transactions from a pool.
2. Execute each transaction.
3. Track cumulative gas usage.
4. Compute transaction root (`tx_root`) — hash of all transactions.
5. Compute state root (`state_root`) — hash of updated accounts.
6. Build `BlockHeader` with:
    - `parent_hash`
    - `number` (block height)
    - `state_root`
    - `tx_root`
    - `timestamp`
    - `gas_used`, `gas_limit`
    - `beneficiary` (miner/reward)
7. Construct `Block` with header, transactions, and receipts.

> Start with simple Keccak hashes; Merkle trees can come later.

---

## Phase 3 – Block Validation
**Functions:** `validate_block`

**Steps:**
1. Check parent hash matches previous block.
2. Validate all transactions in the block.
3. Ensure `tx_root` matches hash of included transactions.
4. Ensure `state_root` matches resulting state.
5. Verify `gas_used ≤ gas_limit`.

> Ensures blockchain consistency and immutability.

---

## Phase 4 – Update & Store State
**Functions:** `update_state`, `store_block`

**Steps:**
1. Update global state (`HashMap<String, Account>`) after transactions.
2. Keep track of state per block.
3. Store blocks in memory or a simple database.

> State updates should be atomic.

---

## Phase 5 – Blockchain Queries
**Functions:** `query_blockchain`

**Steps:**
1. Retrieve block by number or hash.
2. Get transaction by hash.
3. Retrieve account balance or contract storage at a block.
4. Optional: query receipts for logs/events.

> Build read-only APIs that do not modify state.

---

## Phase 6 – Smart Contracts
**Functions:** `deploy_smart_contract`, extend `execute_transaction`

**Steps:**
1. Deploy contract: store bytecode in a contract account.
2. Execute contract calls: read/write storage and update gas.
3. Track execution logs in `Receipt`.
4. Optionally, implement simplified opcodes.

> Keep contract execution separate from simple transfers.

---

## Phase 7 – Consensus & Networking
**Functions:** `consensus_mechanism`

**Steps:**
1. Implement simplified Proof-of-Authority (single node produces blocks).
2. Optionally, implement Proof-of-Work with difficulty target.
3. Broadcast blocks to peers (network simulation optional).
4. Validate incoming blocks with `validate_block`.

> Ensures all nodes agree on the blockchain.

---

## Phase 8 – Testing & Iteration
- Write unit tests for:
    - Transaction validation and execution
    - State updates and account balances
    - Block production and validation
    - Smart contract calls
- Simulate multiple blocks and transactions.
- Check determinism: same transactions → same state root.

---

## Function Mapping

| Function | Phase | Focus |
|----------|-------|-------|
| `execute_transaction` | 1 | Tx validation, gas, update state, receipts |
| `update_state` | 1/4 | Modify account balances, nonce, storage |
| `produce_block` | 2 | Build block from tx pool, compute roots |
| `validate_block` | 3 | Check tx_root, state_root, gas, parent hash |
| `store_block` | 4 | Persist block & state |
| `query_blockchain` | 5 | Retrieve blocks, transactions, accounts, receipts |
| `deploy_smart_contract` | 6 | Save code & initialize storage |
| `consensus_mechanism` | 7 | PoA / PoW / block agreement |

---

## Mini Ethereum Network Flow

```text
           +------------------+
           |  User / Wallet   |
           +--------+---------+
                    |
                    v
           +------------------+
           |  Create Transaction |
           +--------+---------+
                    |
                    v
           +------------------+
           |  Sign Transaction |
           +--------+---------+
                    |
                    v
           +------------------+
           |  Validate Tx      |
           | (nonce, balance, |
           |  signature, gas) |
           +--------+---------+
                    |
                    v
           +------------------+
           | Execute Tx       |
           | (update state,   |
           | gas accounting)  |
           +--------+---------+
                    |
                    v
           +------------------+
           | Generate Receipt  |
           +--------+---------+
                    |
                    v
           +------------------+
           | Include in Block  |
           +--------+---------+
                    |
                    v
           +------------------+
           | Compute Roots     |
           | (tx_root, state_root) |
           +--------+---------+
                    |
                    v
           +------------------+
           | Validate & Store  |
           |     Block         |
           +--------+---------+
                    |
                    v
           +------------------+
           | Network / Consensus|
           | (PoA / PoW)       |
           +--------+---------+
                    |
                    v
           +------------------+
           | Update Blockchain |
           +------------------+