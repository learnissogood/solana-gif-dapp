use anchor_lang::prelude::*;

declare_id!("84L6MYXFFwTk7G3MMbgZptGD3Sy2T1JmahpZXxUNqhVq");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        // Get the reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        // Obtenemos una referencia de la cuenta e incrementamos el contador total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Construimos el struct para guardarlo en el array de structs
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Ahora lo añadimos al vector array del campo gift_list de nuestro struct BaseAccount que esta guardado en la account
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // Esta macro le dice a nuestro programa como inicializar la account
    // Init : crea la account en donde este programa es el dueno
    // payer = user : es quien paga la transaccion para crear la cuenta. En este caso es el usuario
    // space = 9000 : es el espacio que posee la cuenta para guardar datos. En este caso es de 9000 bytes suficientes para nuestro programa
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    // Signer<'info> : es la data que que se pasa al programa en donde se verifica que el usuaria que llama al programa es el dueno
    // de la billetera de la cuenta
    pub user: Signer<'info>,
    // Program <'info, System> : Es una referencia al SystemProgram (es un programa con ID (11111) creado por Solana para que podamos
    // interactuar con el) Se encarga de muchas cosas pero una de las mas importantes es la de crear las accounts
    pub system_program: Program <'info, System>,
}

// Creamos un Context Struct llamado AddGif
// #[account(mut)] : es una macro que indica que tenemos acceso a la cuenta que guarda el Struct BaseAcount y la hacemos mutable,
// para poder cambiar el valor del campo que posee

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Este es un macro que le indica a Anchor como serializar a bytes los datos para poder guardarlos en la blockchain y deserializar los datos para poder leerlos
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
