import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import bs58 from "bs58"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

// Decode the base58 secret key
const secretKey = bs58.decode(wallet.skey);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = await readFile('./assets/generug.png');
        const generic = createGenericFile(image, 'rug', {
            contentType: "image/png"
        });

        const [myUri] = await umi.uploader.upload([generic])

        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
