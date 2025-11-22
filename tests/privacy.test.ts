import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Psol } from "../target/types/psol";
import { BN } from "@coral-xyz/anchor";
import {
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { assert } from "chai";

describe("Privacy accounts", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Psol as Program<Psol>;

  const wallet = provider.wallet as anchor.Wallet;
  const mint = anchor.web3.Keypair.generate();

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  );

  const [privacyAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("privacy_account"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const userToken = getAssociatedTokenAddressSync(
    mint.publicKey,
    wallet.publicKey
  );

  it("creates vault and privacy account", async () => {
    await program.methods
      .createVault()
      .accounts({
        vault,
        mint: mint.publicKey,
        authority: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mint])
      .rpc();

    await program.methods
      .initPrivacyAccount()
      .accounts({
        privacyAccount,
        owner: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.privacyAccount.fetch(privacyAccount);
    assert.equal(account.owner.toBase58(), wallet.publicKey.toBase58());
    assert.equal(account.nonce.toNumber(), 0);
  });

  it("deposits privately and updates encrypted balance", async () => {
    const amount = new BN(1_000_000);
    await program.methods
      .depositPrivate(amount)
      .accounts({
        vault,
        mint: mint.publicKey,
        userToken,
        user: wallet.publicKey,
        privacyAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.privacyAccount.fetch(privacyAccount);
    const balance = new BN(
      Buffer.from(account.encryptedBalance).readBigUInt64LE()
    );
    assert.equal(balance.toNumber(), amount.toNumber());
    assert.equal(account.nonce.toNumber(), 1);

    const tokenAccount = await getAccount(provider.connection, userToken);
    assert.equal(Number(tokenAccount.amount), amount.toNumber());
  });

  it("withdraws privately and decreases encrypted balance", async () => {
    const amount = new BN(400_000);
    const beforeLamports = await provider.connection.getBalance(wallet.publicKey);

    await program.methods
      .withdrawPrivate(amount)
      .accounts({
        vault,
        mint: mint.publicKey,
        userToken,
        user: wallet.publicKey,
        privacyAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.privacyAccount.fetch(privacyAccount);
    const balance = new BN(
      Buffer.from(account.encryptedBalance).readBigUInt64LE()
    );
    assert.equal(balance.toNumber(), 600_000);
    assert.equal(account.nonce.toNumber(), 2);

    const tokenAccount = await getAccount(provider.connection, userToken);
    assert.equal(Number(tokenAccount.amount), 600_000);

    const afterLamports = await provider.connection.getBalance(wallet.publicKey);
    assert.isAbove(afterLamports, beforeLamports);
  });

  it("rejects unauthorized access and double initialization", async () => {
    const other = anchor.web3.Keypair.generate();
    const [otherPrivacy] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("privacy_account"), other.publicKey.toBuffer()],
      program.programId
    );

    // Double init should fail
    await assert.isRejected(
      program.methods
        .initPrivacyAccount()
        .accounts({
          privacyAccount,
          owner: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()
    );

    // Other user cannot touch existing privacy account
    await assert.isRejected(
      program.methods
        .depositPrivate(new BN(1))
        .accounts({
          vault,
          mint: mint.publicKey,
          userToken,
          user: other.publicKey,
          privacyAccount,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([other])
        .rpc()
    );

    // Other user can create their own privacy account
    await program.methods
      .initPrivacyAccount()
      .accounts({
        privacyAccount: otherPrivacy,
        owner: other.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([other])
      .rpc();
  });

  it("fails on underflow withdrawal", async () => {
    await assert.isRejected(
      program.methods
        .withdrawPrivate(new BN(10_000_000))
        .accounts({
          vault,
          mint: mint.publicKey,
          userToken,
          user: wallet.publicKey,
          privacyAccount,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()
    );
  });
});
