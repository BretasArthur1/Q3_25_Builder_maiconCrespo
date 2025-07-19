import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount, getAssociatedTokenAddress } from "@solana/spl-token";
import { assert } from "chai";
import { AnchorEscrow } from "../target/types/anchor_escrow";


describe("anchor-vault-q3", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  const maker= Keypair.generate();
  let mintA:PublicKey, mintB:PublicKey,  makerAtaA:PublicKey,  makerAtaB:PublicKey;
  let escrowPda: PublicKey, vaultAta:PublicKey;
  const seed = new BN(42);
  const receive = new BN(1000);
  const depositAmount = new BN(500);

   before(async () => {
    // Airdrop SOL to maker
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(maker.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
  // Create mints
    mintA = await createMint(provider.connection, maker, maker.publicKey, null, 6);
    mintB = await createMint(provider.connection, maker, maker.publicKey, null, 6);
    // Create maker's ATAs
    makerAtaA = (await getOrCreateAssociatedTokenAccount(provider.connection, maker, mintA, maker.publicKey)).address;
    makerAtaB = (await getOrCreateAssociatedTokenAccount(provider.connection, maker, mintB, maker.publicKey)).address;
    // Mint tokens to maker
    await mintTo(provider.connection, maker, mintA, makerAtaA, maker, 2000);
    await mintTo(provider.connection, maker, mintB, makerAtaB, maker, 2000);
});

  });