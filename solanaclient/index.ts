
import {Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction, TransactionInstruction} from '@solana/web3.js';
let main = async () => {

    console.log("creating a solana transaction");

    let key = Keypair.fromSecretKey(
        Uint8Array.from(
            [/* your private key */] 
        ));

    let connection = new Connection("https://api.devnet.solana.com ");

    let programId = new PublicKey("7iD2vJdKxsrNruYbhJe1H23BkVe5LKbsAvDwaLUC7hjE");
    let transaction = new Transaction();

    transaction.add(
        new TransactionInstruction(
            {
                programId,
                keys: [],
                data: Buffer.alloc(0)
            }
        )
    );

    let tx = await sendAndConfirmTransaction(connection , transaction , [key]);
    
    console.log(tx);
}


main();