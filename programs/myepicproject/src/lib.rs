use anchor_lang::prelude::*;

declare_id!("CiTRxCTSsDyVWC4Qfo6XMrzqGF79fS3e2mUPbmN1r3Cp");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;

      let item = ItemStruct {
          gif_link: gif_link.to_string(),
          user_address: *user.to_account_info().key,
          vote: 0,
      };

      base_account.gif_list.push(item);
      base_account.total_gifs += 1;
      Ok(())
  }

  pub fn update_item(ctx:Context<UpdateItem>, id:u64) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let i = id as usize;

      if i < base_account.gif_list.len() {
          base_account.gif_list[i].vote += 1;
      } else {
          return Err(ErrorCode::WrongID.into());
      }
      
      Ok(())
      
  }

  pub fn tip_gif(ctx:Context<TipGif>, id:u64, amount:u64) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let i = id as usize;

      let item = &base_account.gif_list[i];
      let to = item.user_address;
      let owner = &ctx.accounts.owner;
      let user = &ctx.accounts.user;

      if owner.to_account_info().key == &to {
          let ix = anchor_lang::solana_program::system_instruction::transfer(
              &user.key(),
              &to,
              amount,
          );

          anchor_lang::solana_program::program::invoke(
              &ix,
              &[
                  user.to_account_info(),
                  owner.to_account_info(),
              ],
          )?;

      } else {
          return Err(ErrorCode::WrongUserAddress.into());
      }

      Ok(())

  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct TipGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub vote: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

#[error]
pub enum ErrorCode {
    #[msg("Tried sending to the wrong owner...!!!")] WrongUserAddress,
    #[msg("item id does not exist")] WrongID,
}