import {
    Transaction,
    SystemProgram,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
    PublicKey,
  } from "@solana/web3.js";
  import wallet from "./dev-wallet.json";
  const from = Keypair.fromSecretKey(new Uint8Array(wallet.skey));
  const to = new PublicKey("DeAW7SyArr7Stk2U6NL8EtoXaLNPyG3wfkVfRzvcPG4S");
  const connection = new Connection("https://api.devnet.solana.com");
  (async () => {
    try {
      const transaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: LAMPORTS_PER_SOL / 100,
        })
      );
      transaction.recentBlockhash = (
        await connection.getLatestBlockhash("confirmed")
      ).blockhash;
  
      transaction.feePayer = from.publicKey;
      // Sign transaction, broadcast, and confirm
      const signature = await sendAndConfirmTransaction(connection, transaction, [
        from,
      ]);
      console.log(`Success! Check out your TX here:
      https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  })();