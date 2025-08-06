import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { IcoLaunchpad } from "../target/types/ico_launchpad";
import idl from "../target/idl/ico_launchpad.json";
import {
  bn,
  CompressedAccountWithMerkleContext,
  createRpc,
  defaultStaticAccountsStruct,
  defaultTestStateTreeAccounts,
  deriveAddress,
  deriveAddressSeed,
  LightSystemProgram,
  Rpc,
  sleep,
} from "@lightprotocol/stateless.js";
import { assert } from "chai";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, 
  getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID,
  mintTo  
} from "@solana/spl-token";
import BN from "bn.js";

describe("ICO Launchpad Test", () => {
  const program = anchor.workspace.IcoLaunchpad as Program<IcoLaunchpad>;
  const rpc = createRpc(
    "http://127.0.0.1:8899",
    "http://127.0.0.1:8784",
    "http://127.0.0.1:3001",
    { commitment: "confirmed" },
  );

  let owner: Keypair;


    const outputMerkleTree = defaultTestStateTreeAccounts().merkleTree;
    const addressTree = defaultTestStateTreeAccounts().addressTree;
    const addressQueue = defaultTestStateTreeAccounts().addressQueue;

  before(async () => {
    console.log("Starting test...");
    owner = anchor.web3.Keypair.generate();
    const airdropSignature = await rpc.requestAirdrop(
      owner.publicKey,
      LAMPORTS_PER_SOL
    );
    await rpc.confirmTransaction({
    signature: airdropSignature,
    blockhash: (await rpc.getLatestBlockhash()).blockhash,
    lastValidBlockHeight: (await rpc.getLatestBlockhash()).lastValidBlockHeight,
  });

    console.log("Airdropped SOL to ", owner.publicKey.toBase58());

    const balance = await rpc.getBalance(owner.publicKey);
  console.log("Account balance:", balance/ LAMPORTS_PER_SOL,"SOL");
  });

  it("Initialize ICO", async () => {
  const tokenMint = await createMint(
    rpc,
    owner,           // payer
    owner.publicKey, // mint authority
    owner.publicKey, // freeze authority (optional)
    9                 // decimals
  );

  console.log("Mint created successfully:", tokenMint.toString());

  // You don't need to create the ATA manually - Anchor will do it
  // Just calculate what the address will be for your assertions later
const ownerAta = await createAssociatedTokenAccount(
  rpc,        // Connection
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
  const mintInfo = await rpc.getAccountInfo(tokenMint);
  console.log('Mint account exists:', !!mintInfo);
  console.log('Mint account owner:', mintInfo?.owner.toString());
  
  const ataInfo = await rpc.getAccountInfo(ownerAta);
  console.log('ATA account exists:', !!ataInfo);
  console.log('ATA account owner:', ataInfo?.owner.toString());
  
} catch (error) {
  console.error('Error checking accounts:', error);
}

      await mintTo(
        rpc,
        owner,
        tokenMint,
        ownerAta,
        owner,
        mintAmount
      );

      console.log(`Minted ${mintAmount} tokens to ICO account`);

  const now = Math.floor(Date.now() / 1000); // Convert ms to seconds
const startTime = new BN(now + 1); // 1 second after deployment
const endTime = new BN(now + 3600);
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

    // console.log("Transaction signature:", tx);
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

it("Create Investment", async () => {
  await sleep(3000); 
  const amount = new BN(100);
  const IcoSeed = new TextEncoder().encode("ico");
   const seed = deriveAddressSeed(
      [IcoSeed, owner.publicKey.toBuffer()],
      new web3.PublicKey(program.idl.address)
    );
    console.log("seed", seed);
    const address = deriveAddress(seed, addressTree);
    console.log("address", bn(address.toBytes()));
try {
      let IcoAccount = await rpc.getCompressedAccount(bn(address.toBytes()));
      console.log("IcoAccount", IcoAccount);
} catch (error) {
  console.log("Error getting compressed account", error);
}
 let IcoAccount = await rpc.getCompressedAccount(bn(address.toBytes()));
    const proofRpcResult = await rpc.getValidityProofV0(
      [
        {
          hash: IcoAccount.hash,
          tree: IcoAccount.treeInfo.tree,
          queue: IcoAccount.treeInfo.queue,
        },
      ],
      []
    );

    const systemAccountConfig = SystemAccountMetaConfig.new(program.programId);
    let remainingAccounts =
      PackedAccounts.newWithSystemAccounts(systemAccountConfig);

    const addressMerkleTreePubkeyIndex =
      remainingAccounts.insertOrGet(addressTree);
    const addressQueuePubkeyIndex = remainingAccounts.insertOrGet(addressQueue);
    const packedAddreesMerkleContext = {
      rootIndex: proofRpcResult.rootIndices[0],
      addressMerkleTreePubkeyIndex,
      addressQueuePubkeyIndex,
    };
    const outputMerkleTreeIndex =
      remainingAccounts.insertOrGet(outputMerkleTree);

    let proof = {
      0: proofRpcResult.compressedProof,
    };
    const computeBudgetIx = web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 1000000,
    });

  const [icoPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ico"), owner.publicKey.toBuffer()],
    program.programId
  );

  const [solVaultPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), icoPDA.toBuffer()],
    program.programId
  );

  const ico = await program.account.ico.fetch(icoPDA);
  console.log("ICO:", ico);
  console.log("Current time", Date.now());
  console.log("Start time", ico.startTime);

  try {
    console.log("About to call invest...");
    
    const tx = await program.methods
      .createInvestment(
        proof,
        packedAddreesMerkleContext,
        outputMerkleTreeIndex,
        amount,
      )
      .accounts({
        ico: icoPDA,
        investor: owner.publicKey,
        mint: ico.tokenMint,
        
      })
      .preInstructions([computeBudgetIx])
      .remainingAccounts(remainingAccounts.toAccountMetas().remainingAccounts)
      .signers([owner])
      .transaction();
    tx.recentBlockhash = (await rpc.getRecentBlockhash()).blockhash;
    tx.sign(owner);

    const sig = await rpc.sendTransaction(tx, [owner]);
    await rpc.confirmTransaction(sig);
    // console.log("Created counter compressed account ", sig);

      // console.log("Transaction signature:", tx);
      console.log("Investment transaction successful");
      assert.equal(ico.totalRaised.toNumber(), new BN(100).toNumber());
      
  } catch (error) {
     console.error("Error investing:", error);
  }
  });
});


class SystemAccountMetaConfig {
  selfProgram: web3.PublicKey;
  cpiContext?: web3.PublicKey;
  solCompressionRecipient?: web3.PublicKey;
  solPoolPda?: web3.PublicKey;

  private constructor(
    selfProgram: web3.PublicKey,
    cpiContext?: web3.PublicKey,
    solCompressionRecipient?: web3.PublicKey,
    solPoolPda?: web3.PublicKey
  ) {
    this.selfProgram = selfProgram;
    this.cpiContext = cpiContext;
    this.solCompressionRecipient = solCompressionRecipient;
    this.solPoolPda = solPoolPda;
  }

  static new(selfProgram: web3.PublicKey): SystemAccountMetaConfig {
    return new SystemAccountMetaConfig(selfProgram);
  }

  static newWithCpiContext(
    selfProgram: web3.PublicKey,
    cpiContext: web3.PublicKey
  ): SystemAccountMetaConfig {
    return new SystemAccountMetaConfig(selfProgram, cpiContext);
  }
}

class PackedAccounts {
  private preAccounts: web3.AccountMeta[] = [];
  private systemAccounts: web3.AccountMeta[] = [];
  private nextIndex: number = 0;
  private map: Map<web3.PublicKey, [number, web3.AccountMeta]> = new Map();

  static newWithSystemAccounts(
    config: SystemAccountMetaConfig
  ): PackedAccounts {
    const instance = new PackedAccounts();
    instance.addSystemAccounts(config);
    return instance;
  }

  addPreAccountsSigner(pubkey: web3.PublicKey): void {
    this.preAccounts.push({ pubkey, isSigner: true, isWritable: false });
  }

  addPreAccountsSignerMut(pubkey: web3.PublicKey): void {
    this.preAccounts.push({ pubkey, isSigner: true, isWritable: true });
  }

  addPreAccountsMeta(accountMeta: web3.AccountMeta): void {
    this.preAccounts.push(accountMeta);
  }

  addSystemAccounts(config: SystemAccountMetaConfig): void {
    this.systemAccounts.push(...getLightSystemAccountMetas(config));
  }

  insertOrGet(pubkey: web3.PublicKey): number {
    return this.insertOrGetConfig(pubkey, false, true);
  }

  insertOrGetReadOnly(pubkey: web3.PublicKey): number {
    return this.insertOrGetConfig(pubkey, false, false);
  }

  insertOrGetConfig(
    pubkey: web3.PublicKey,
    isSigner: boolean,
    isWritable: boolean
  ): number {
    const entry = this.map.get(pubkey);
    if (entry) {
      return entry[0];
    }
    const index = this.nextIndex++;
    const meta: web3.AccountMeta = { pubkey, isSigner, isWritable };
    this.map.set(pubkey, [index, meta]);
    return index;
  }

  private hashSetAccountsToMetas(): web3.AccountMeta[] {
    const entries = Array.from(this.map.entries());
    entries.sort((a, b) => a[1][0] - b[1][0]);
    return entries.map(([, [, meta]]) => meta);
  }

  private getOffsets(): [number, number] {
    const systemStart = this.preAccounts.length;
    const packedStart = systemStart + this.systemAccounts.length;
    return [systemStart, packedStart];
  }

  toAccountMetas(): {
    remainingAccounts: web3.AccountMeta[];
    systemStart: number;
    packedStart: number;
  } {
    const packed = this.hashSetAccountsToMetas();
    const [systemStart, packedStart] = this.getOffsets();
    return {
      remainingAccounts: [
        ...this.preAccounts,
        ...this.systemAccounts,
        ...packed,
      ],
      systemStart,
      packedStart,
    };
  }
}

function getLightSystemAccountMetas(
  config: SystemAccountMetaConfig
): web3.AccountMeta[] {
  let signerSeed = new TextEncoder().encode("cpi_authority");
  const cpiSigner = web3.PublicKey.findProgramAddressSync(
    [signerSeed],
    config.selfProgram
  )[0];
  const defaults = SystemAccountPubkeys.default();
  const metas: web3.AccountMeta[] = [
    { pubkey: defaults.lightSystemProgram, isSigner: false, isWritable: false },
    { pubkey: cpiSigner, isSigner: false, isWritable: false },
    {
      pubkey: defaults.registeredProgramPda,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: defaults.noopProgram, isSigner: false, isWritable: false },
    {
      pubkey: defaults.accountCompressionAuthority,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: defaults.accountCompressionProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: config.selfProgram, isSigner: false, isWritable: false },
  ];
  if (config.solPoolPda) {
    metas.push({
      pubkey: config.solPoolPda,
      isSigner: false,
      isWritable: true,
    });
  }
  if (config.solCompressionRecipient) {
    metas.push({
      pubkey: config.solCompressionRecipient,
      isSigner: false,
      isWritable: true,
    });
  }
  metas.push({
    pubkey: defaults.systemProgram,
    isSigner: false,
    isWritable: false,
  });
  if (config.cpiContext) {
    metas.push({
      pubkey: config.cpiContext,
      isSigner: false,
      isWritable: true,
    });
  }
  return metas;
}

class SystemAccountPubkeys {
  lightSystemProgram: web3.PublicKey;
  systemProgram: web3.PublicKey;
  accountCompressionProgram: web3.PublicKey;
  accountCompressionAuthority: web3.PublicKey;
  registeredProgramPda: web3.PublicKey;
  noopProgram: web3.PublicKey;
  solPoolPda: web3.PublicKey;

  private constructor(
    lightSystemProgram: web3.PublicKey,
    systemProgram: web3.PublicKey,
    accountCompressionProgram: web3.PublicKey,
    accountCompressionAuthority: web3.PublicKey,
    registeredProgramPda: web3.PublicKey,
    noopProgram: web3.PublicKey,
    solPoolPda: web3.PublicKey
  ) {
    this.lightSystemProgram = lightSystemProgram;
    this.systemProgram = systemProgram;
    this.accountCompressionProgram = accountCompressionProgram;
    this.accountCompressionAuthority = accountCompressionAuthority;
    this.registeredProgramPda = registeredProgramPda;
    this.noopProgram = noopProgram;
    this.solPoolPda = solPoolPda;
  }

  static default(): SystemAccountPubkeys {
    return new SystemAccountPubkeys(
      LightSystemProgram.programId,
      web3.PublicKey.default,
      defaultStaticAccountsStruct().accountCompressionProgram,
      defaultStaticAccountsStruct().accountCompressionAuthority,
      defaultStaticAccountsStruct().registeredProgramPda,
      defaultStaticAccountsStruct().noopProgram,
      web3.PublicKey.default
    );
    }
  }