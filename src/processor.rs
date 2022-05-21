//! Program state processor
use {
    crate::{
        error::ChatingError,
        instruction::ChatingInstruction,
        state::{Post, PostType},
    },
    arrayref::array_ref,
    solana_program::{
        account_info::AccountInfo,
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_memory::sol_memset,
        program_pack::{IsInitialized, Pack},
        pubkey::Pubkey,
        sysvar::{rent::Rent, Sysvar},
    },
};

pub struct Processor {}
impl Processor {
    /// Processes [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = ChatingInstruction::unpack(input)?;
        match instruction {
            ChatingInstruction::CreatePost { content } => {
                msg!("Instruction: Create Post");
                Self::process_create_post(program_id, accounts, content)
            }
            ChatingInstruction::CreatePostSharding {
                sharding,
                master_key,
                content,
            } => {
                msg!("Instruction: Create Post Sharding");
                Self::process_create_post_sharding(
                    program_id, accounts, sharding, master_key, content,
                )
            }

            ChatingInstruction::Terminate {} => {
                msg!("Instruction: Terminate");
                Self::process_terminate(program_id, accounts)
            }
        }
    }

    ///  create post
    ///
    /// # params
    /// * `program_id`
    /// * `accounts` - total 3 accounts, author_acc, post_acc, clock_acc
    /// * `content`
    /// ```    
    fn process_create_post(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        content: String,
    ) -> ProgramResult {
        let accounts = array_ref![accounts, 0, 3];
        let [author_acc, post_acc, clock_acc] = accounts;
        let mut post = Post::unpack_unchecked(&post_acc.data.borrow())?;
        let clock = Clock::from_account_info(&clock_acc)?;
        // check signer
        if author_acc.is_signer == false {
            return Err(ChatingError::InvalidSigner.into());
        }
        // check exist
        if post.is_initialized() {
            return Err(ChatingError::PostExist.into());
        }
        // check other
        Self::check_account_owner(post_acc, program_id)?;
        Self::check_account_rent(post_acc)?;
        // init account
        post.post_type = PostType::Master;
        post.sharding = 0;
        post.ts = clock.unix_timestamp;
        post.author = *author_acc.key;
        post.content = content;
        Post::pack(post, &mut post_acc.data.borrow_mut())?;
        Ok(())
    }

    ///  create post sharding
    ///
    /// # params
    /// * `program_id`
    /// * `accounts` - total 4 accounts, author_acc, master_acc, post_acc, clock_acc
    /// * `sharding`
    /// * `content`
    /// ```    
    fn process_create_post_sharding(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        sharding: u64,
        master_key: Pubkey,
        content: String,
    ) -> ProgramResult {
        let accounts = array_ref![accounts, 0, 3];
        let [author_acc, post_acc, clock_acc] = accounts;
        let mut post = Post::unpack_unchecked(&post_acc.data.borrow())?;
        let clock = Clock::from_account_info(&clock_acc)?;
        // check signer
        if author_acc.is_signer == false {
            return Err(ChatingError::InvalidSigner.into());
        }
        // check exist
        if post.is_initialized() {
            return Err(ChatingError::PostExist.into());
        }
        // check master
        if sharding == 0 && master_key != NULL_PUBKEY {
            msg!("sharding:{}, master key:{}", sharding, master_key);
            return Err(ChatingError::InvalidSharding.into());
        }
        if sharding != 0 && master_key == NULL_PUBKEY {
            msg!("sharding:{}, master key:{}", sharding, master_key);
            return Err(ChatingError::InvalidSharding.into());
        }
        // check other
        Self::check_account_owner(post_acc, program_id)?;
        Self::check_account_rent(post_acc)?;
        // init account
        post.post_type = match sharding {
            0 => PostType::Master,
            _ => PostType::MasterSharding,
        };
        post.sharding = sharding;
        post.ts = clock.unix_timestamp;
        post.author = *author_acc.key;
        post.master = master_key;
        post.content = content;
        Post::pack(post, &mut post_acc.data.borrow_mut())?;
        Ok(())
    }

    /// terminate
    ///
    /// # params
    /// * `program_id`
    /// * `accounts` - total 7 accounts, beneficiary_acc, registrar_acc, member_acc, member_vault_acc, member_pda_acc, destination_acc, token_program_acc
    /// ```
    fn process_terminate(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts = array_ref![accounts, 0, 2];
        let [author_acc, post_acc] = accounts;
        let post = Post::unpack(&post_acc.data.borrow())?;
        // check signer
        if author_acc.is_signer == false {
            return Err(ChatingError::InvalidSigner.into());
        }
        // check account
        if post.author != *author_acc.key {
            return Err(ChatingError::InvalidAuthority.into());
        }
        // check other
        Self::check_account_owner(post_acc, program_id)?;
        // close post
        {
            let user_lamports = author_acc.lamports();
            **author_acc.lamports.borrow_mut() =
                user_lamports.checked_add(post_acc.lamports()).unwrap();
            **post_acc.lamports.borrow_mut() = 0;
            sol_memset(*post_acc.data.borrow_mut(), 0, Post::LEN);
        }
        Ok(())
    }

    /// Check account owner is the given program
    fn check_account_owner(account_info: &AccountInfo, program_id: &Pubkey) -> ProgramResult {
        if *program_id != *account_info.owner {
            msg!(
                "Expected account to be owned by program {}, received {}",
                program_id,
                account_info.owner
            );
            Err(ChatingError::IncorrectProgramId.into())
        } else {
            Ok(())
        }
    }

    /// check account rent
    fn check_account_rent(accnount_info: &AccountInfo) -> ProgramResult {
        let data_length = accnount_info.data_len();
        let rent = Rent::get()?;

        if !rent.is_exempt(accnount_info.lamports(), data_length) {
            Err(ChatingError::NotRentExempt.into())
        } else {
            Ok(())
        }
    }
}

// public key of 11111111111111111111111111111111
pub const NULL_PUBKEY: Pubkey = Pubkey::new_from_array([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);
