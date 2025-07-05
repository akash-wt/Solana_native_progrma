import { expect, test } from "bun:test";
import { COUNTER_SIZE } from "./types";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";


const PROGRAM_ID = new PublicKey("3bvFxhFbvQYNqEKjPgpqhE68R4B3XEy5gE3hddgbctx8");
const connection = new Connection("http://127.0.0.1:8899", "confirmed");
let counterAccountKeypair: Keypair;
let adminKeypair: Keypair;

test("counter does increase", async () => {
    adminKeypair = Keypair.generate();
    counterAccountKeypair = Keypair.generate();

    const res = await connection.requestAirdrop(adminKeypair.publicKey, 2 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(res);

    let adminBalance = await connection.getBalance(adminKeypair.publicKey);
    expect(adminBalance).toBe(2 * LAMPORTS_PER_SOL);
    console.log(adminBalance);

})