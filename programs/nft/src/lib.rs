use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
  create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
  CreateMetadataAccountsV3, Metadata,mpl_token_metadata,
};
use anchor_spl::token_interface::{
  close_account, mint_to, transfer_checked, CloseAccount, Mint, MintTo, TokenAccount,
  TokenInterface, TransferChecked,
};
//use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::types::{Collection, Creator, DataV2};

declare_id!("4kwcURKome7NkYanznMuPV7M3qZbin67qb2YD4tZBzKA");

#[program]
pub mod nft {
  use super::*;

  pub fn mint_to_collection(
    ctx: Context<MintToCollection>,
    id_collection: u64,
    id_nft: u64,
    name: String,
    symbol: String,
    uri: String,
    price: f32,
    cant: u64,
  ) -> Result<()> {
    msg!("Creating seeds");
    let id_bytes = id_collection.to_le_bytes();
    let id_nft_bytes = id_nft.to_le_bytes();
    let seeds = &[
      "mint".as_bytes(),
      id_bytes.as_ref(),
      id_nft_bytes.as_ref(),
      &[ctx.bumps.mint],
    ];

    msg!("Run mint_to");
    mint_to(
      CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
          authority: ctx.accounts.signer.to_account_info(),
          to: ctx.accounts.token_account.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
        },
        &[&seeds[..]],
      ),
      1, // 1 token
    )?;

    msg!("Run create metadata accounts v3");
    create_metadata_accounts_v3(
      CpiContext::new_with_signer(
        ctx.accounts.metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
          payer: ctx.accounts.payer.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
          metadata: ctx.accounts.metadata_account.to_account_info(),
          mint_authority: ctx.accounts.signer.to_account_info(),
          update_authority: ctx.accounts.signer.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          rent: ctx.accounts.rent.to_account_info(),
        },
        &[&seeds[..]],
      ),
      DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
          address: ctx.accounts.payer.key(),
          verified: true,
          share: 100,
        }]),
        collection: Some(Collection {
          key: ctx.accounts.collection.key(),
          verified: false,
        }),
        uses: None,
      },
      true,
      true,
      None,
    )?;

    msg!("Run create master edition v3");
    create_master_edition_v3(
      CpiContext::new_with_signer(
        ctx.accounts.metadata_program.to_account_info(),
        CreateMasterEditionV3 {
          edition: ctx.accounts.master_edition_account.to_account_info(),
          payer: ctx.accounts.payer.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
          metadata: ctx.accounts.metadata_account.to_account_info(),
          mint_authority: ctx.accounts.signer.to_account_info(),
          update_authority: ctx.accounts.signer.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          token_program: ctx.accounts.token_program.to_account_info(),
          rent: ctx.accounts.rent.to_account_info(),
        },
        &[&seeds[..]],
      ),
      Some(1),
    )?;
    msg!("Minted NFT successfully");
    Ok(())
  }

  pub fn create_single_nft(
    ctx: Context<CreateSingleNFT>,
    id: u64,
    name: String,
    symbol: String,
    uri: String,
    _price: f32,
    _cant: u64,
  ) -> Result<()> {
    msg!("create_single_nft()...");
    let id_bytes = id.to_le_bytes();
    let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]]; // the bump of the mint account from our context

    msg!("mint_to");
    mint_to(
      CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
          authority: ctx.accounts.signer.to_account_info(),
          to: ctx.accounts.token_account.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
        },
        &[&seeds[..]],
      ),
      1, // 1 token
    )?;

    msg!("create metadata accounts v3");
    create_metadata_accounts_v3(
      CpiContext::new_with_signer(
        ctx.accounts.metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
          payer: ctx.accounts.signer.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
          metadata: ctx.accounts.metadata_account.to_account_info(),
          mint_authority: ctx.accounts.signer.to_account_info(),
          update_authority: ctx.accounts.signer.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          rent: ctx.accounts.rent.to_account_info(),
        },
        &[&seeds[..]],
      ),
      DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
      },
      true, //set the authority account as the same signer
      true, //make the metadata mutable.
      None, //the collection details.
    )?;

    msg!("create master edition v3");
    create_master_edition_v3(
      CpiContext::new_with_signer(
        ctx.accounts.metadata_program.to_account_info(),
        CreateMasterEditionV3 {
          edition: ctx.accounts.master_edition_account.to_account_info(),
          payer: ctx.accounts.signer.to_account_info(),
          mint: ctx.accounts.mint.to_account_info(),
          metadata: ctx.accounts.metadata_account.to_account_info(),
          mint_authority: ctx.accounts.signer.to_account_info(),
          update_authority: ctx.accounts.signer.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          token_program: ctx.accounts.token_program.to_account_info(),
          rent: ctx.accounts.rent.to_account_info(),
        },
        &[&seeds[..]],
      ),
      Some(1), //maximum supply of 1
    )?;
    msg!("Minted NFT successfully");
    Ok(())
  }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateSingleNFT<'info> {
  #[account(mut)]
  pub signer: Signer<'info>, //NFT authority can be changed later
  //#[account(mut)]
  //pub payer: Signer<'info>,
  #[account(
    init,
    payer = signer,
    mint::decimals = 0,
    mint::authority = signer,
    mint::freeze_authority = signer,
    seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()], bump,
    )]
  pub mint: InterfaceAccount<'info, Mint>,
  #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
  pub token_account: InterfaceAccount<'info, TokenAccount>,
  pub associated_token_program: Program<'info, AssociatedToken>, //to make new ATA for system_program
  pub token_program: Interface<'info, TokenInterface>,
  pub metadata_program: Program<'info, Metadata>, //associate metadata to accounts
  #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
  /// CHECK:
  pub master_edition_account: AccountInfo<'info>, //to make tokens effectively non-fungible by associating them to this master edition account
  #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
  /// CHECK:
  pub metadata_account: AccountInfo<'info>,
  pub rent: Sysvar<'info, Rent>, //Sysvar-type program that helps determine rent costs.
  pub system_program: Program<'info, System>, //make new ATA
}

#[derive(Accounts)]
#[instruction(id_collection: u64, id_nft: u64)]
pub struct MintToCollection<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    mint::decimals = 0,
    mint::authority = signer,
    mint::freeze_authority = signer,
    seeds = ["mint".as_bytes(), 
      id_collection.to_le_bytes().as_ref(),
      id_nft.to_le_bytes().as_ref()], bump,
    )]
  pub mint: InterfaceAccount<'info, Mint>,
  #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
  pub token_account: InterfaceAccount<'info, TokenAccount>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
  pub metadata_program: Program<'info, Metadata>,
  #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
  /// CHECK:
  pub master_edition_account: AccountInfo<'info>,
  #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
  /// CHECK:
  pub metadata_account: AccountInfo<'info>,
  /// CHECK:
  pub collection: AccountInfo<'info>,
}
