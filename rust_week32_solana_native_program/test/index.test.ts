import  *as borsh from "borsh";
import {expect,test} from "bun:test";
import { Keypair,Connection,LAMPORTS_PER_SOL, Transaction, SystemProgram, PublicKey} from "@solana/web3.js";
import {COUNTER_SIZE,CounterAccount,schema} from "./types.ts"

let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();

let connection = new Connection("http://127.0.0.1:8899","confirmed");

test("initialize accounts",async () =>{
    

    //airdrop sol to admin account
    const airdropSignature = await connection.requestAirdrop(adminAccount.publicKey,25*LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature);
    
    const bal =await connection.getBalance(adminAccount.publicKey);
    console.log("adminAccount balance",bal/LAMPORTS_PER_SOL,"sol");
    
    const info = await connection.getAccountInfo(adminAccount.publicKey);
    console.log("account info",info);
    
    const b = await connection.getBalance(dataAccount.publicKey);
    console.log("dataAccount balance",b/LAMPORTS_PER_SOL,"sol");
    
    //owner of the account is the program id, so we need to create an account and assign it to the program id
    let programId = new PublicKey("4sdpWPnSCJ7YnPnEbkJbADW88ZsCgec3GzFrqqokYsck");

    //lamports required for rent exemption, so that the account will not be deleted by the network
    let lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const txn = new Transaction();
    
    // const counterAccountInstruction = SystemProgram.createAccount({
        //     fromPubkey: adminAccount.publicKey,
        //     newAccountPubkey: dataAccount.publicKey,
        //     programId,
        //     lamports,
        //     space: COUNTER_SIZE,
        // })
        // txn.add(counterAccountInstruction);
       //OR
        
        txn.add(SystemProgram.createAccount({
            fromPubkey: adminAccount.publicKey,
            newAccountPubkey: dataAccount.publicKey,
            programId,
            lamports,
            space: COUNTER_SIZE,
        }));

        const txnSignature = await connection.sendTransaction(txn,[adminAccount,dataAccount]);
        await connection.confirmTransaction(txnSignature);

        const info2 = await connection.getAccountInfo(dataAccount.publicKey);
        console.log("data account info2",info2);
        console.log("counterDataAccount public key",dataAccount.publicKey.toBase58())

        const counterAccountInfo = await connection.getAccountInfo(dataAccount.publicKey);
        console.log("counter Account Info",counterAccountInfo?.data);


        const counter = borsh.deserialize(schema,counterAccountInfo?.data as Uint8Array) as CounterAccount;

        console.log("count is ",counter.count);
        expect(counter.count).toBe(0);
});

//test to check if the sum function is working correctly
test("is equal",() =>{
    expect(sum(1,2)).toBe(3);
    expect(10).toBe(10);
})
function sum(a:number,b:number):number{
    return a+b;
}

//test to check if the equation is working correctly
test("is equal",() =>{
    expect(10).toBe(10);
})