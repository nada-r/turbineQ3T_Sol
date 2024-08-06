import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import bs58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

// Decode the base58 secret key
const secretKey = bs58.decode(wallet.skey);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi,{
        mint,
        name: "nadarrugs gud",
        symbol: "rugud",
        uri: "https://arweave.net/cYt3MPr1fUX3YARmDzImfDJWV5CpHUw9Zjg7glMXf0c",
        sellerFeeBasisPoints: percentAmount(5, 2),
        creators: [
            {
                address: keypair.publicKey,
                verified: true,
                share: 100
            }
        ]
    })
    let result = await tx.sendAndConfirm(umi);
    const signature = bs58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();