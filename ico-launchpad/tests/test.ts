import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IcoLaunchpad } from "../target/types/ico_launchpad";
import idl from "../target/idl/ico_launchpad.json";
// import {
//   bn,
//   createRpc,
//   sleep,
//   defaultTestStateTreeAccounts,
//   deriveAddressSeed,
//   deriveAddress,
// } from "@lightprotocol/stateless.js";
import { assert } from "chai";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, 
  getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID,
  mintTo  
} from "@solana/spl-token";
import BN from "bn.js";

describe("ICO Launchpad Test", () => {
  const program = anchor.workspace.IcoLaunchpad as Program<IcoLaunchpad>;
  // const rpc = createRpc(
  //   "http://127.0.0.1:8899",
  //   "http://127.0.0.1:8784",
  //   "http://127.0.0.1:3001",
  //   { commitment: "confirmed" },
  // );

  const connection = anchor.getProvider().connection;

  let owner: Keypair;

  before(async () => {
    console.log("Starting test...");
    owner = anchor.web3.Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(
      owner.publicKey,
      LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction({
    signature: airdropSignature,
    blockhash: (await connection.getLatestBlockhash()).blockhash,
    lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
  });

    console.log("Airdropped SOL to ", owner.publicKey.toBase58());

    const balance = await connection.getBalance(owner.publicKey);
  console.log("Account balance:", balance/ LAMPORTS_PER_SOL,"SOL");
  });

  it("Initialize ICO", async () => {
  const tokenMint = await createMint(
    connection,
    owner,           // payer
    owner.publicKey, // mint authority
    owner.publicKey, // freeze authority (optional)
    9                 // decimals
  );



  // You don't need to create the ATA manually - Anchor will do it
  // Just calculate what the address will be for your assertions later
const ownerAta = await createAssociatedTokenAccount(
  connection,        // Connection
  owner,            // Keypair (payer)
  tokenMint,        // PublicKey (mint)
  owner.publicKey   // PublicKey (owner)
);

console.log("Owner ATA:", ownerAta.toBase58());

    const mintAmount = 1000000 * Math.pow(10, 9); // 1M tokens
      
    console.log('=== DEBUG INFO ===');
console.log('Token Mint:', tokenMint.toString());
console.log('Owner ATA:', ownerAta.toString());
console.log('Owner PublicKey:', owner.publicKey.toString());

try {
  const mintInfo = await connection.getAccountInfo(tokenMint);
  console.log('Mint account exists:', !!mintInfo);
  console.log('Mint account owner:', mintInfo?.owner.toString());
  
  const ataInfo = await connection.getAccountInfo(ownerAta);
  console.log('ATA account exists:', !!ataInfo);
  console.log('ATA account owner:', ataInfo?.owner.toString());
  
} catch (error) {
  console.error('Error checking accounts:', error);
}

      await mintTo(
        connection,
        owner,
        tokenMint,
        ownerAta,
        owner,
        mintAmount
      );

      console.log(`Minted ${mintAmount} tokens to ICO account`);

  const startTime = new BN(Math.floor(Date.now() / 1000));
  const endTime = startTime.add(new BN(1000));
  const totalTokens = new BN(100);
  const priceLamports = new BN(10);

  // Derive PDAs for assertions later
  const [icoPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ico"), owner.publicKey.toBuffer()],
    program.programId
  );

  const [vaultPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), icoPDA.toBuffer()],
    program.programId
  );

  console.log("ICO PDA:", icoPDA.toBase58());
  console.log("Vault PDA:", vaultPDA.toBase58());
  console.log("Token Mint:", tokenMint.toBase58());
  console.log("Owner ATA:", ownerAta.toBase58());

  try {
    console.log("About to call initializeIco...");
    
    const tx = await program.methods
      .initializeIco(
        tokenMint,
        ownerAta,        // This is the tokenAccount parameter
        startTime,
        endTime,
        totalTokens,
        priceLamports,
      )
      .accounts({
        owner: owner.publicKey,
        tokenMint: tokenMint,
        // Don't specify PDAs - Anchor auto-derives them:
        // - ico (auto-derived from seeds)
        // - icoVault (auto-derived from seeds) 
        // - ownerAta (auto-derived from seeds)
        // - All program accounts are auto-populated from IDL
      })
      .signers([owner])
      .rpc();

    console.log("Transaction signature:", tx);
    console.log("ICO initialization transaction successful");

    // Now your assertions should work!
    const icoAccount = await program.account.ico.fetch(icoPDA);
    
    assert.equal(icoAccount.owner.toBase58(), owner.publicKey.toBase58());
    assert.equal(icoAccount.tokenMint.toBase58(), tokenMint.toBase58());
    assert.equal(icoAccount.tokenAccount.toBase58(), ownerAta.toBase58());
    assert.equal(icoAccount.startTime.toNumber(), startTime.toNumber());
    assert.equal(icoAccount.endTime.toNumber(), endTime.toNumber());
    assert.equal(icoAccount.totalTokens.toNumber(), totalTokens.toNumber());
    assert.equal(icoAccount.totalInvested.toNumber(), 0);
    assert.equal(icoAccount.priceLamports.toNumber(), priceLamports.toNumber());
    assert.isTrue(icoAccount.isActive);
    assert.equal(icoAccount.totalRaised.toNumber(), 0);
    
    console.log("ICO created successfully:", icoAccount);
    console.log("All assertions passed!");
    
  } catch (error) {
    console.error("Error initializing ICO:", error);
    throw error;
  }
});
});