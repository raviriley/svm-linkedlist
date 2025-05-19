# Solana Singly Linked List

This project implements a simple singly linked list using the Anchor framework.

## Details

- Insert nodes with u8 data values
- Supports proper list reconnection after deletion
- Solana account public keys as pointers
- Each node is stored in its own account

## Account Structure

1. **LinkedList**: Account that tracks the head of the list
2. **Node**: Represents a node in the list with:
   - `data`: u8 value stored in the node
   - `next`: Optional reference to the next node in the list

## How to Use

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy (not needed for local testing tho)

Ensure the local test validator is running:

```bash
solana-test-validator
```

Then deploy:

```bash
anchor deploy
```

## Example Usage in `tests/daos-linkedlist.ts`

The test suite demonstrates the following:

1. Create a linked list
2. Insert nodes at the head or after an existing node
3. Traverses the list to verify structure
4. Delete nodes from different positions
5. Traverse the list to read its contents
