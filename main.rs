use anchor_lang::prelude::*;
declare_id!("HN1a3j162tKR7z8fCAFqxpZLRZQqUYoYgM2Ta13PvySE");

#[program]
pub mod hello_world {
    use super::*;

    pub fn init_vote_bank(ctx: Context<InitVote>) -> Result<()> {
        // Open vote bank for public to vote on our favorite "GM" or "GN"
        ctx.accounts.vote_account.is_open_to_vote = true;
        Ok(())
    }

    pub fn gib_vote(ctx: Context<GibVote>, vote_type: u64) -> Result<()> {
        // ctx.accounts.vote_account.is_open_to_vote;
        match vote_type {
            0 => {
                msg!("Voted for yes");
                ctx.accounts.vote_account.gm += 1;
            }
            1 => {
                msg!("Voted for no");
                ctx.accounts.vote_account.gn += 1;
            }
            _ => {
                msg!("Invalid vote type");
            }
        }

        // ctx.accounts.vote_account.total_votes += 1;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitVote<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 1 + 8 + 8,
    )]
    pub vote_account: Account<'info, VoteBank>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GibVote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct VoteBank {
    is_open_to_vote: bool,
    gm: u64,
    gn: u64,
    // total_votes: u64,
}
