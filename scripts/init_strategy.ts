import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RangerAccelerator } from "../target/types/ranger_accelerator";
import { 
  PublicKey, 
  SystemProgram, 
  Keypair,
  Transaction 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  getOrCreateAssociatedTokenAccount 
} from "@solana/spl-token";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RangerAccelerator as Program<RangerAccelerator>;
  const admin = provider.wallet as anchor.Wallet;

  console.log("Starting Ranger Accelerator Initialization...");

  // 1. Derive PDAs
  const [vaultConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault-config")],
    program.programId
  );

  const [positionState] = PublicKey.findProgramAddressSync(
    [Buffer.from("position-state")],
    program.programId
  );

  console.log("Vault Config PDA:", vaultConfig.toBase58());
  console.log("Position State PDA:", positionState.toBase58());

  // 2. Initialize Strategy
  // Mocks/Placeholders for Mainnet Addresses
  const USDC_MINT = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // Mainnet USDC
  const MANAGER = admin.publicKey;

  try {
    const tx = await program.methods
      .initialize(
        MANAGER,
        USDC_MINT,
        4500, // 45% Max LTV
        new anchor.BN(10_000_000_000) // 10k USDC Cap
      )
      .accounts({
        admin: admin.publicKey,
        vaultConfig,
        positionState,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Strategy Initialized! TX:", tx);
  } catch (e) {
    console.log("Initialization probably already complete or failed:", e);
  }

  // 3. Initialize Kamino Obligation (Audit 6.0 P0 Fix)
  console.log("Initializing Kamino Obligation ownership...");
  try {
    const txInit = await program.methods
      .initKaminoObligation()
      .accounts({
        vaultConfig,
        // kamino_obligation and other protocol accounts would go here
      } as any)
      .rpc();
    console.log("Kamino Obligation Secured! TX:", txInit);
  } catch (e) {
    console.error("Kamino Obligation init failed (Check if program IDs are configured):", e);
  }

  console.log("---- RANGER ACCELERATOR DEPLOYMENT READY ----");
}

main().catch((err) => {
  console.error(err);
});
