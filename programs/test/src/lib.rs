pub mod ins;
pub use ins::*;
pub mod state;
pub use state::*;

use anchor_lang::solana_program::entrypoint::ProgramResult;
// use solana_program::{program::invoke, system_instruction};

use anchor_lang::prelude::*;
// use instruction::*;

declare_id!("9ytjikLhLoi19ViNczFqWQ8BgcCUM4jemMejL6riL6sL");

#[program]
pub mod test {
    use super::*;
    pub fn init_data(ctx: Context<InitMyData>) -> ProgramResult {
        let acc = &mut ctx.accounts.data;
        acc.number = 0; 
        acc.message = String::from("MyData initialized!");
        acc.owner = ctx.accounts.owner.key();

        Ok(())
    }

    pub fn updata_data(ctx: Context<UpdateMyData>, number: u8, message: String) -> ProgramResult {
        let acc = &mut ctx.accounts.data;
        acc.number = number;
        acc.message = message;

        Ok(())
    }
    
}
