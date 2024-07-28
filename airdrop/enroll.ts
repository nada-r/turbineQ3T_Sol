import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, WbaPrereq } from "./programs/wba_prereq";
import wallet from "./wba-wallet.json";
import bs58 from "bs58";

// Decode the base58 secret key
const secretKey = bs58.decode(wallet.skey);

// Create a Keypair from the secret key
const keypair = Keypair.fromSecretKey(secretKey);

const connection = new Connection("https://api.devnet.solana.com");
const github = Buffer.from("nada-r", "utf8");


const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed",
  });
  
  const program: Program<WbaPrereq> = new Program(IDL, provider);
  
  const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
  
  const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
    enrollment_seeds,
    program.programId
  );
  
  (async () => {
    try {
      const txhash = await program.methods
        .complete(github)
        .accounts({
          signer: keypair.publicKey,
        })
        .signers([keypair])
        .rpc();
      console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  })();