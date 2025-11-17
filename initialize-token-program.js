const anchor = require("@coral-xyz/anchor");
const { PublicKey, SystemProgram, Keypair } = require("@solana/web3.js");
const { TOKEN_PROGRAM_ID } = require("@solana/spl-token");
const fs = require("fs");

const TOKEN_PROGRAM_ID_CUSTOM = new PublicKey("CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy");
const PRIVACY_PROGRAM_ID = new PublicKey("2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv");
const RPC_URL = "https://api.devnet.solana.com";

async function initialize() {
  console.log("=".repeat(60));
  console.log("🚀 Initializing pSOL Token Program");
  console.log("=".repeat(60));
  console.log("");
  
  // Load wallet
  const walletPath = process.env.HOME + "/.config/solana/id.json";
  
  if (!fs.existsSync(walletPath)) {
    console.error("❌ Wallet not found. Run: solana-keygen new");
    return;
  }
  
  const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf-8"));
  const wallet = Keypair.fromSecretKey(Uint8Array.from(secretKey));
  
  console.log("Wallet:", wallet.publicKey.toString());
  
  // Setup connection
  const connection = new anchor.web3.Connection(RPC_URL, "confirmed");
  
  // Check balance
  const balance = await connection.getBalance(wallet.publicKey);
  console.log("Balance:", balance / 1e9, "SOL");
  
  if (balance < 0.1 * 1e9) {
    console.log("\n⚠️  Low balance! Get devnet SOL:");
    console.log("solana airdrop 2 --url devnet");
    return;
  }
  
  // Privacy Pool PDA
  const [privacyPool] = PublicKey.findProgramAddressSync(
    [Buffer.from("privacy_pool")],
    PRIVACY_PROGRAM_ID
  );
  
  // Token Program PDAs
  const [tokenVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault")],
    TOKEN_PROGRAM_ID_CUSTOM
  );
  
  const [psolMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("psol_mint")],
    TOKEN_PROGRAM_ID_CUSTOM
  );
  
  const [solVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("sol_vault")],
    TOKEN_PROGRAM_ID_CUSTOM
  );
  
  console.log("\n📍 Addresses:");
  console.log("  Privacy Pool:", privacyPool.toString());
  console.log("  Token Vault:", tokenVault.toString());
  console.log("  pSOL Mint:", psolMint.toString());
  console.log("  SOL Vault:", solVault.toString());
  console.log("");
  
  // Check if already initialized
  try {
    const vaultAccount = await connection.getAccountInfo(tokenVault);
    if (vaultAccount) {
      console.log("✅ Token already initialized!");
      console.log("\n💎 pSOL Mint Address:");
      console.log(psolMint.toString());
      console.log("\nAdd this to your wallet on devnet!");
      return;
    }
  } catch (e) {
    // Not initialized, continue
  }
  
  console.log("⚠️  Cannot initialize without program IDL");
  console.log("\nTo initialize:");
  console.log("1. Export IDL from Solana Playground");
  console.log("2. Place in target/idl/psol_token.json");
  console.log("3. Run this script again");
  console.log("\nOr use Playground when devnet RPC recovers!");
  
  console.log("\n" + "=".repeat(60));
}

initialize().catch(console.error);
