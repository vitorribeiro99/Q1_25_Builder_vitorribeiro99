import { config } from "dotenv";
import {
  Keypair,
  PublicKey,
  Connection,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "../wallet/Turbin3-wallet.json";

// Load environment variables
config();

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection(
  process.env.RPC_URL || "https://api.devnet.solana.com",
  commitment
);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("D3meZbenDsK7GF1ZV9th3x5u1oSA8rFRc2FvQCDNoHeb");

(async () => {
  try {
    // Create an ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);

    // Mint to ATA
    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      1n * token_decimals
    );
    console.log(`Your mint txid: ${mintTx}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
