import { Keypair,Connection,LAMPORTS_PER_SOL,SystemProgram,Transaction } from "@solana/web3.js";

//creation of data account and airdrop to payer account using SystemProgram 
async function main(){
    const payer = new Keypair();
    // console.log("Payer Public Key:", payer.publicKey.toBase58());
    
    const dataAccount = new Keypair();
    // console.log("DataAccount pubkey",dataAccount.publicKey.toBase58());
    
    const conn = new Connection("http://127.0.0.1:8899", "confirmed");

    //airdrop  to payer
    const sig = await conn.requestAirdrop(payer.publicKey,3*LAMPORTS_PER_SOL);
    console.log("signature",sig)
    const { blockhash, lastValidBlockHeight } = await conn.getLatestBlockhash('confirmed');
    console.log(blockhash,lastValidBlockHeight);

    await conn.confirmTransaction(sig);
        

    // await conn.confirmTransaction({
    //     signature: sig,
    //     blockhash,
    //     lastValidBlockHeight
    // },'confirmed');


    const bal = await conn.getBalance(payer.publicKey);
    console.log("Payer Balance:", bal);

    const instruction = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: dataAccount.publicKey,
        lamports: 2*LAMPORTS_PER_SOL,
        space: 8,
        programId: SystemProgram.programId,
    });

    console.log("transaction start")
    
        const transaction = new Transaction().add(instruction);
    
        // transaction.feePayer = payer.publicKey;
        transaction.recentBlockhash = (await conn.getLatestBlockhash()).blockhash;
        transaction.sign(payer);
    
        conn.sendTransaction(transaction,[payer,dataAccount]); 

        console.log("trasaction successfully done")
        console.log("data public key",dataAccount.publicKey.toBase58());

}
main();
