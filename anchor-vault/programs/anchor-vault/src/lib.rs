use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
declare_id!("HJEf7PBN6TkYQgZ1Zc3bucqiQx2Stu4EkR7CJTr4ftVb");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps);

        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        ctx.accounts.close_account()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + VaultState::LEN,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment <'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    state : Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault : SystemAccount<'info>,
    system_program : Program<'info,System>,
}



impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount);

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds  = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds : &[&[&[u8]];1] = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,signer_seeds);

        transfer(cpi_ctx, amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseAccount <'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump,
        close = user
    )]
    state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault: SystemAccount<'info>,
    
    system_program: Program<'info, System>,
}

impl<'info> CloseAccount<'info> {
    pub fn close_account(&mut self) -> Result<()> {

        let cpi_program = self.system_program.to_account_info();

        let balance = self.vault.get_lamports();

        let cpi_accounts = Transfer
        {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds  = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds : &[&[&[u8]];1] = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, balance)?;

        Ok(())


    }
}


#[account]
pub struct VaultState {
    state_bump : u8,
    vault_bump : u8,
}

impl VaultState {
    const LEN : usize = 8 + 1*2;
}