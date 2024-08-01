/// <reference lib="dom" />yar

import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import bs58 from "bs58";
import wallet from "../wba-wallet.json"

// Decode the base58 secret key
const secretKey = bs58.decode(wallet.skey);

// Create a Keypair from the secret key
const keypair = Keypair.fromSecretKey(secretKey);


//Create a Solana devnet connection
const commitment: Commitment = "confirmed"; //66% confirmed in the network
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        const mint = createMint(connection, keypair, keypair.publicKey, null, 6);
        console.log(mint);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
