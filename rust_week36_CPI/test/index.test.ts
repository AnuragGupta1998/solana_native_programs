import path from "path";
import { test,expect } from "bun:test";
import { LiteSVM } from "litesvm";
import {
	PublicKey,
	Transaction,
	SystemProgram,
	Keypair,
	LAMPORTS_PER_SOL,
} from "@solana/web3.js";

test("one transfer", () => {
	const svm = new LiteSVM();
	const contract_publickey = PublicKey.unique();

	// loading our smart contract to local svm (solana virtual machine)
	
	// svm.addProgramFromFile(contract_publickey.publicKey,path.join(__dirname,"./double.so"));
	svm.addProgramFromFile(contract_publickey,"./double.so");

	const payer = new Keypair();

	svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));

	const data_account = new Keypair();
	const blockhash = svm.latestBlockhash();
	
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
            newAccountPubkey: data_account.publicKey,
            lamports:Number(svm.minimumBalanceForRentExemption(BigInt(4))),
            space: 4, //4 bytes for u32
            programId: contract_publickey,
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(payer, data_account);

	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(data_account.publicKey);
	console.log(balanceAfter,svm.getBalance(payer.publicKey));
	expect(balanceAfter).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));
});