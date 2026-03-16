import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { RangerAccelerator } from "../target/types/ranger_accelerator";
import { assert } from "chai";
import * as fs from "fs";
import * as path from "path";

process.env.ANCHOR_PROVIDER_URL = "http://127.0.0.1:8899";
process.env.ANCHOR_WALLET = "/home/craig/.config/solana/id.json";

describe("ranger-accelerator", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.rangerAccelerator as Program;

  // Helper to load keypair from fixtures
  const loadKeypair = (name: string): anchor.web3.Keypair => {
    const p = path.join(__dirname, "fixtures", `${name}.json`);
    const secret = JSON.parse(fs.readFileSync(p, "utf-8"));
    return anchor.web3.Keypair.fromSecretKey(new Uint8Array(secret));
  };

  // Constants / Static Accounts
  const managerKeypair = loadKeypair("manager");
  const manager = managerKeypair.publicKey;

  const jitosolMintKeypair = loadKeypair("jitosol_mint");
  const jitosolMint = jitosolMintKeypair.publicKey;

  const usdgMintKeypair = loadKeypair("usdg_mint");
  const usdgMint = usdgMintKeypair.publicKey;

  const solMintKeypair = loadKeypair("sol_mint");
  const solMint = solMintKeypair.publicKey;

  // CPI placeholders
  const kaminoProgram = new anchor.web3.PublicKey("KLend2g3M67enU7fB6uMj9795m1ebSy78pS7CzW9XDP");
  const meteoraProgram = new anchor.web3.PublicKey("LbM7pZu499K7f6N8NVpA4SST7S5DDA9hkDsS4NVcKEX");
  const driftProgram = new anchor.web3.PublicKey("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(manager, jitosolMint, usdgMint, solMint)
      .accounts({
        admin: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Initialize tx:", tx);
  });

  it("Fails deposit with SlippageExceeded", async () => {
    const userJitosol = anchor.web3.Keypair.generate().publicKey; // Dummy
    const vaultJitosol = anchor.web3.Keypair.generate().publicKey;
    const dummyOracle = anchor.web3.Keypair.generate().publicKey; // Will fail magic test

    const amount = new anchor.BN(1000);
    // Ask for 1001 shares on first deposit (which returns 1000) -> Should fail
    const minSharesOut = new anchor.BN(1001); 

    try {
      await program.methods
        .deposit(amount, minSharesOut)
        .accounts({
          user: provider.wallet.publicKey,
          userJitosol: userJitosol,
          vaultJitosol: vaultJitosol,
          kaminoProgram: kaminoProgram,
          meteoraProgram: meteoraProgram,
          driftProgram: driftProgram,
          oracleSolPrice: dummyOracle, // Fake oracle
        } as any) // Avoid Strict ts errors on AccountInfo placeholders
        .rpc();
      
      assert.fail("Should have failed with SlippageExceeded");
    } catch (err: any) {
      assert.include(err.message, "SlippageExceeded", "Unexpected error message");
    }
  });

  it("Fails deposit with InvalidOracle", async () => {
    const userJitosol = anchor.web3.Keypair.generate().publicKey;
    const vaultJitosol = anchor.web3.Keypair.generate().publicKey;
    
    // Pass User wallet address as oracle (data will not match magic number structure)
    const dummyOracle = provider.wallet.publicKey; 

    const amount = new anchor.BN(1000);
    const minSharesOut = new anchor.BN(0); // Allow any share

    try {
      await program.methods
        .deposit(amount, minSharesOut)
        .accounts({
          user: provider.wallet.publicKey,
          userJitosol: userJitosol,
          vaultJitosol: vaultJitosol,
          kaminoProgram: kaminoProgram,
          meteoraProgram: meteoraProgram,
          driftProgram: driftProgram,
          oracleSolPrice: dummyOracle,
        } as any)
        .rpc();
      
      assert.fail("Should have failed with InvalidOracle");
    } catch (err: any) {
      assert.include(err.message, "InvalidOracle", "Unexpected error message for oracle");
    }
  });
});
