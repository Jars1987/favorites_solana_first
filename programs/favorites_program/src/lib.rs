use anchor_lang::prelude::*;

//  1st we declare our program_id, the address for the program
declare_id!("FsaMxsfaMkFzb21Lz8fdNSWMVqPbaLkTB4uKSCE9dqW1");

//written to every account in the blockchain by anchor
//it specifies the type of account it is
pub const ANCHOR_DESCRIMINATOR_SIZE: usize = 8; //when we save things to the blockchain we will need 8 Bytes + size of what we are saving

#[program] // With this Anchor makes the Rust code a Solana program + applies safety faults
pub mod favorites_program {
    use super::*;

    //set_favorites is the Instructions Handler
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        msg!("Greetings from: {:?}", context.program_id);
        let user_public_key = context.accounts.user.key();

        msg!("User {user_public_key}'s favorite number is {number}, favorite color is {color} and hobbies are {hobbies:?} ");

        //add data to the favorites account
        context.accounts.favorites.set_inner( Favorites {
            number,
            color,
            hobbies
        });

        Ok(())
    }
}


#[account]
#[derive(InitSpace)] //Gives all our Favorites instances the Initspace attribute (Total space used by the items inside)

//Favorites struct is the data we are going to save to the blockchain
pub struct Favorites {

    pub number: u64,

    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]

//This is the struct of Accounts (list of accounts)  for our set_favorites Instructions Handler
//we are going to pass 3 accounts in total, user account, favorite accout(create address and save 
//our data) and finally the mandatory System Program Account
pub struct SetFavorites<'info> {

    //here we assign some options to this account
    //we set it as mutable because the person signing the instructions to set_favorites is going to pay 
    //to create the SetFavorites Account in the blockchain, it is going to pay to store that information
    #[account(mut)]
    pub user: Signer<'info>,


    #[account(
        init_if_needed, //make the favorites account if doesnt already exist
        payer = user, // who pays to create the account
        space = ANCHOR_DESCRIMINATOR_SIZE + Favorites::INIT_SPACE, //how much space does this account needs to
        seeds = [b"favorites", user.key().as_ref()], //used tp give thie Account and address on the block chain. (This is a PDA) this means when I am storing favorites will be saved in the adress obtain from this seed
        bump,

    )]
    //the signer will also need to specify the Favorites account they want to write to
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,

}
