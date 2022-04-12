
import {Connection, Keypair, PublicKey, 
    sendAndConfirmTransaction, Transaction,
     TransactionInstruction , SystemProgram} from '@solana/web3.js';
let main = async () => {

    console.log("creating a solana transaction");

    let key = Keypair.fromSecretKey(
        Uint8Array.from(
            [/* your private key */] 
        ));

    let connection = new Connection("https://api.devnet.solana.com ");

    let programId = new PublicKey("dyn8Jm4Q5nnStdJujrXxPa9WjLird7v4B6umjMmcjj7");
    
    let datacount = Keypair.generate();
    let createAccountTransaction = new Transaction();
    let size = 5;
    let topay = await connection.getMinimumBalanceForRentExemption( size);
    createAccountTransaction.add(
        SystemProgram.createAccount(
            {
                fromPubkey: key.publicKey,
                newAccountPubkey: datacount.publicKey,
                lamports: topay,
                space: size,
                programId
            }
        )
    );

    console.log("creating new account " + datacount.publicKey.toBase58());
    let ctx = await sendAndConfirmTransaction(connection ,
        createAccountTransaction,
        [ key,  datacount  ], {commitment: 'max'}
        );
    
    console.log("dataaccount created " , ctx);
    
    let transaction = new Transaction();

    transaction.add(
        new TransactionInstruction(
            {
                programId,
                keys: [{pubkey: datacount.publicKey , 
                       isSigner: false, 
                      isWritable : true}],
                data: Buffer.from([ 4 , 3 , 8 , 101 , 200 ])
            }
        )
    );

    let tx = await sendAndConfirmTransaction(connection , transaction , [key]);
    
    console.log(tx);
}


main();