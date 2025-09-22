# whisperrnote_icp (Minimal Production Variant)

This repository now contains a single focused Internet Computer canister that stores and shares encrypted notes. All non‑essential experimental modules were removed for a lean, auditable surface area.

## Current Scope
- Create empty encrypted notes
- Update encrypted contents
- List all notes owned or shared with caller
- Fetch a single note by ID (authorization enforced)
- Share and unshare notes with other principals
- Derive per‑note symmetric keys via VetKD (verification and encrypted key endpoints)

## Data Model
EncryptedNote fields:
- `id: nat` – monotonic identifier
- `encrypted_text: text` – opaque ciphertext (caller encrypts client‑side)
- `owner: text` – principal (string form) that created the note
- `users: vec text` – additional principals granted access
- `created_at: nat64` – IC time (nanoseconds)
- `updated_at: nat64` – IC time (nanoseconds)

All previous optional metadata (title, tags, attachments, etc.) was intentionally removed to keep storage compact and logic simple.

## Public Methods (all update calls)
- `create_note() -> nat`
- `update_note(id: nat, encrypted_text: text)`
- `get_notes() -> vec EncryptedNote`
- `get_note(id: nat) -> opt EncryptedNote`
- `add_user(id: nat, principal_text: text)`
- `remove_user(id: nat, principal_text: text)`
- `delete_note(id: nat)`
- `whoami() -> text`
- `symmetric_key_verification_key_for_note() -> text`
- `encrypted_symmetric_key_for_note(id: nat, transport_public_key: blob) -> text`

## Limits
Constants enforce basic quotas:
- Max notes per user: 500
- Max note characters: 1000 (counted as Unicode scalar values)
- Max shares per note: 50
- Max distinct owners: 1000

## Build & Deploy
```
dfx start --background
dfx deploy
```

## Rationale for Simplification
Reducing unused optional fields and modules:
- Lowers stable memory usage
- Eliminates accidental data clearing via broad update calls
- Minimizes security review surface
- Speeds up serialization/deserialization

## Future Extensions (Out of Scope Here)
Pagination, metadata, role distinctions, query methods with certification, and index optimization can be layered later without breaking this minimal API.
