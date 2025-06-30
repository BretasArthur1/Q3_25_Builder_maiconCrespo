import { Keypair } from "@solana/web3.js";

//generate a new Keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet:${kp.publicKey.toBase58()}`);
console.log(`[${kp.secretKey}]`);
//4QNLtxb5XnHY25qGVc7DFdKy74KyGgv8GMyYxTavULDd
