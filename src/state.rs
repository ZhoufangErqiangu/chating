//! State transition types
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PostType {
    Master,
    MasterSharding,
    Reply,
    ReplySharding,
}

impl Default for PostType {
    fn default() -> Self {
        Self::Master
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Post {
    pub post_type: PostType,
    pub sharding: u64,
    pub ts: i64,
    pub author: Pubkey,
    pub master: Pubkey,
    pub quote: Pubkey,
    pub content: String,
}

// pack data , unpack data
impl Sealed for Post {}
impl IsInitialized for Post {
    fn is_initialized(&self) -> bool {
        return self.author != NULL_PUBKEY;
    }
}

impl Pack for Post {
    const LEN: usize = 1 + 8 * 2 + 32 * 3 + CONTENT_LENGTH;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        const LEN: usize = 1 + 8 * 2 + 32 * 3 + CONTENT_LENGTH;
        let src = array_ref![src, 0, LEN];
        let (post_type_buf, sharding_buf, ts_buf, author_buf, master_buf, quote_buf, content_buf) =
            array_refs![src, 1, 8, 8, 32, 32, 32, CONTENT_LENGTH];
        let post_type_buf = u8::from_le_bytes(*post_type_buf);
        let post_type = match post_type_buf {
            1 => PostType::Master,
            3 => PostType::MasterSharding,
            2 => PostType::Reply,
            4 => PostType::ReplySharding,
            _ => PostType::default(),
        };
        let content = std::str::from_utf8(content_buf).unwrap();
        Ok(Post {
            post_type: post_type,
            sharding: u64::from_le_bytes(*sharding_buf),
            ts: i64::from_le_bytes(*ts_buf),
            author: Pubkey::new_from_array(*author_buf),
            master: Pubkey::new_from_array(*master_buf),
            quote: Pubkey::new_from_array(*quote_buf),
            content: String::from(content),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        const LEN: usize = 1 + 8 * 2 + 32 * 3 + CONTENT_LENGTH;
        let dst = array_mut_ref![dst, 0, LEN];
        let (post_type_buf, sharding_buf, ts_buf, author_buf, master_buf, quote_buf, content_buf) =
            mut_array_refs![dst, 1, 8, 8, 32, 32, 32, CONTENT_LENGTH];
        let post_type: u8 = match self.post_type {
            PostType::Master => 1,
            PostType::MasterSharding => 2,
            PostType::Reply => 3,
            PostType::ReplySharding => 4,
        };
        *post_type_buf = post_type.to_le_bytes();
        *sharding_buf = self.sharding.to_le_bytes();
        *ts_buf = self.ts.to_le_bytes();
        author_buf.copy_from_slice(self.author.as_ref());
        master_buf.copy_from_slice(self.master.as_ref());
        quote_buf.copy_from_slice(self.quote.as_ref());
        content_buf.copy_from_slice(self.content.as_bytes());
    }
}

// public key of 11111111111111111111111111111111
const NULL_PUBKEY: solana_program::pubkey::Pubkey =
    solana_program::pubkey::Pubkey::new_from_array([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

const CONTENT_LENGTH: usize = 512;
