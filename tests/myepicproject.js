const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
    console.log("🚀 Starting test...");

    // Le decimos a anchor en que red queremos ejecutar el programa. En este caso, la red local
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Estamos guardando en una constante el codigo deployado en un validador local. Se compila nuestro codigo en la lib.rs y se ejecuta y deployea en la red local
    const program = anchor.workspace.Myepicproject;

    // Create an account keypair for our program it needs!
    const baseAccount = anchor.web3.Keypair.generate();

    // Una vez deployado el programa de solana, podemos llamar a nuestra funcion mediante el metodo program.rpc. "nombre de la funcion()"
    // Es una llamada asincrona porque debemos esperar a que el vaidador local mine la instruccion o transaccion
    let tx = await program.rpc.startStuffOff({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    });

    console.log("📝 Your transaction signature", tx);

    // Recuperar los datos de la cuenta
    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifs.toString());

    // Call the new function addGif()
    tx = await program.rpc.addGif("insert_a_giphy_link_here", {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
        },
    });

    // Get the account again to see what changed
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifs.toString());

    // We can also access to the gif_list on the account
    console.log('👀 GIF List', account.gifList);
};

const runMain = async () => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
};

runMain();