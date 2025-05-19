import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaosLinkedlist } from "../target/types/daos_linkedlist";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("daos-linkedlist", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.daosLinkedlist as Program<DaosLinkedlist>;
  const provider = anchor.getProvider();

  // We'll use these to store our account keypairs
  let linkedListAccount: Keypair;
  let node1Account: Keypair;
  let node2Account: Keypair;
  let node3Account: Keypair;

  before(async () => {
    // Generate keypairs for our accounts
    linkedListAccount = anchor.web3.Keypair.generate();
    node1Account = anchor.web3.Keypair.generate();
    node2Account = anchor.web3.Keypair.generate();
    node3Account = anchor.web3.Keypair.generate();
  });

  it("Initializes a linked list", async () => {
    // Initialize the linked list
    await program.methods
      .initialize()
      .accounts({
        linkedList: linkedListAccount.publicKey,
        payer: provider.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([linkedListAccount])
      .rpc();

    // Fetch the account and verify its state
    const linkedList = await program.account.linkedList.fetch(
      linkedListAccount.publicKey
    );
    expect(linkedList.head).to.equal(null);
  });

  it("Inserts a first node (at the head)", async () => {
    const dataValue = 42;

    await program.methods
      .insert(dataValue)
      .accounts({
        linkedList: linkedListAccount.publicKey,
        node: node1Account.publicKey,
        afterNode: null, // null means insert at head
        payer: provider.publicKey,
      })
      .signers([node1Account])
      .rpc();

    // Verify the node was inserted correctly
    const linkedList = await program.account.linkedList.fetch(
      linkedListAccount.publicKey
    );
    const node1 = await program.account.node.fetch(node1Account.publicKey);

    expect(linkedList.head).to.eql(node1Account.publicKey);
    expect(node1.data).to.equal(dataValue);
    expect(node1.next).to.equal(null);
  });

  it("Inserts a second node after the first", async () => {
    const dataValue = 99;

    await program.methods
      .insert(dataValue)
      .accounts({
        linkedList: linkedListAccount.publicKey,
        node: node2Account.publicKey,
        afterNode: node1Account.publicKey,
        payer: provider.publicKey,
      })
      .signers([node2Account])
      .rpc();

    // Verify the second node was inserted correctly
    const node1 = await program.account.node.fetch(node1Account.publicKey);
    const node2 = await program.account.node.fetch(node2Account.publicKey);

    expect(node1.next).to.eql(node2Account.publicKey);
    expect(node2.data).to.equal(dataValue);
    expect(node2.next).to.equal(null);
  });

  it("Inserts a third node after the second", async () => {
    const dataValue = 101;

    await program.methods
      .insert(dataValue)
      .accounts({
        linkedList: linkedListAccount.publicKey,
        node: node3Account.publicKey,
        afterNode: node2Account.publicKey,
        payer: provider.publicKey,
      })
      .signers([node3Account])
      .rpc();

    // Verify the third node was inserted correctly
    const node2 = await program.account.node.fetch(node2Account.publicKey);
    const node3 = await program.account.node.fetch(node3Account.publicKey);

    expect(node2.next).to.eql(node3Account.publicKey);
    expect(node3.data).to.equal(dataValue);
    expect(node3.next).to.equal(null);
  });

  it("Deletes the middle node", async () => {
    await program.methods
      .delete()
      .accounts({
        linkedList: linkedListAccount.publicKey,
        nodeToDelete: node2Account.publicKey,
        prevNode: node1Account.publicKey,
        payer: provider.publicKey,
      })
      .rpc();

    // Verify the node1 now points to node3
    const node1 = await program.account.node.fetch(node1Account.publicKey);
    expect(node1.next).to.eql(node3Account.publicKey);
  });

  it("Deletes the head node", async () => {
    await program.methods
      .delete()
      .accounts({
        linkedList: linkedListAccount.publicKey,
        nodeToDelete: node1Account.publicKey,
        prevNode: null,
        payer: provider.publicKey,
      })
      .rpc();

    // Verify node3 is now the head
    const linkedList = await program.account.linkedList.fetch(
      linkedListAccount.publicKey
    );
    expect(linkedList.head).to.eql(node3Account.publicKey);
  });

  it("Traverses the entire list to verify final structure", async () => {
    // Fetch the linked list to find the head
    const linkedList = await program.account.linkedList.fetch(
      linkedListAccount.publicKey
    );

    // Start at the head
    let currentNodeKey = linkedList.head;
    let nodesVisited = 0;

    // Keep track of what we've seen
    const visitedNodes = [];

    // Traverse the list
    while (currentNodeKey) {
      const currentNode = await program.account.node.fetch(currentNodeKey);
      visitedNodes.push({
        publicKey: currentNodeKey.toString(),
        data: currentNode.data,
      });

      currentNodeKey = currentNode.next;
      nodesVisited++;

      // Safety check to prevent infinite loops
      if (nodesVisited > 10) break;
    }

    // We should only have node3 left in the list
    expect(visitedNodes.length).to.equal(1);
    expect(visitedNodes[0].publicKey).to.equal(
      node3Account.publicKey.toString()
    );
  });
});
