import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Psol } from "../target/types/psol";
import { assert } from "chai";

describe("pSOL", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Psol as Program<Psol>;
  
  let pool: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let userPrivacyAccount: anchor.web3.PublicKey;

  before(async () => {
    [pool] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("privacy_pool")],
      program.programId
    );

    [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), pool.toBuffer()],
      program.programId
    );

    [userPrivacyAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("privacy_account"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes privacy pool", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        pool,
        vault,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize tx:", tx);

    const poolAccount = await program.account.privacyPool.fetch(pool);
    assert.equal(poolAccount.authority.toString(), provider.wallet.publicKey.toString());
    assert.equal(poolAccount.totalLocked.toNumber(), 0);
    assert.equal(poolAccount.depositFeeBps, 10);
    assert.equal(poolAccount.withdrawFeeBps, 10);
    assert.equal(poolAccount.paused, false);
  });

  it("Deposits SOL into privacy pool", async () => {
    const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL

    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        pool,
        privacyAccount: userPrivacyAccount,
        vault,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit tx:", tx);

    const poolAccount = await program.account.privacyPool.fetch(pool);
    const privacyAccount = await program.account.privacyAccount.fetch(userPrivacyAccount);

    // Fee is 0.1% = 1_000_000 lamports
    const expectedNetAmount = 999_000_000;
    
    assert.equal(poolAccount.totalLocked.toNumber(), expectedNetAmount);
    assert.equal(poolAccount.totalAccounts.toNumber(), 1);
    assert.equal(privacyAccount.totalDeposits.toNumber(), expectedNetAmount);
  });

  it("Transfers privately within pool", async () => {
    const recipient = anchor.web3.Keypair.generate();
    
    const [recipientPrivacyAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("privacy_account"), recipient.publicKey.toBuffer()],
      program.programId
    );

    // Initialize recipient account first
    await program.methods
      .deposit(new anchor.BN(100_000_000)) // 0.1 SOL
      .accounts({
        pool,
        privacyAccount: recipientPrivacyAccount,
        vault,
        user: recipient.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([recipient])
      .rpc();

    // Perform private transfer
    const encryptedAmount = Buffer.alloc(64);
    const proof = Buffer.alloc(32);

    const tx = await program.methods
      .transfer(Array.from(encryptedAmount), Array.from(proof))
      .accounts({
        pool,
        senderAccount: userPrivacyAccount,
        recipientAccount: recipientPrivacyAccount,
        sender: provider.wallet.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    console.log("Transfer tx:", tx);
  });

  it("Withdraws SOL from privacy pool", async () => {
    const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL
    const nullifier = Array.from(Buffer.alloc(32, 1));
    const proof = Array.from(Buffer.alloc(32));

    const recipient = anchor.web3.Keypair.generate();
    
    const [nullifierAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("nullifier"), Buffer.from(nullifier)],
      program.programId
    );

    const initialBalance = await provider.connection.getBalance(recipient.publicKey);

    const tx = await program.methods
      .withdraw(withdrawAmount, nullifier, proof)
      .accounts({
        pool,
        privacyAccount: userPrivacyAccount,
        nullifierAccount,
        vault,
        recipient: recipient.publicKey,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw tx:", tx);

    const finalBalance = await provider.connection.getBalance(recipient.publicKey);
    
    // Fee is 0.1% = 500_000 lamports
    const expectedNetAmount = 499_500_000;
    assert.approximately(
      finalBalance - initialBalance,
      expectedNetAmount,
      1000
    );
  });
});
