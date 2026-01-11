use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

const COMP_DEF_OFFSET_PRIVATE_RECORD_LOOKUP: u32 = comp_def_offset("private_record_lookup");

declare_id!("6c5BjmuitDUaeQjzsdszGtiAbMmiRB7zv44qmTbaqsE1");

#[arcium_program]
pub mod ws_medical_private_record {
    use super::*;
pub fn store_patient_data(ctx: Context<StorePatientData>,patient_id:[u8; 32],age: [u8; 32],
    gender: [u8; 32],blood_type: [u8; 32],weight: [u8; 32],height: [u8; 32],allergies: [[u8; 32]; 5]) -> Result<()> {
    let patient_data_account = &mut ctx.accounts. patient_data;
    patient_data_account.patient_id = patient_id;
    patient_data_account.age = age;
    patient_data_account.gender = gender;
    patient_data_account.blood_type = blood_type;
    patient_data_account.weight = weight;
    patient_data_account.height = height;
    patient_data_account.allergies = allergies;
    Ok(())
}
pub fn init_private_record_lookup_comp_def(
    ctx: Context<InitPrivateRecordLookupCompDef>,
) -> Result<()> {
    init_comp_def(ctx.accounts, None, None)?;
    Ok(())
}
#[arcium_callback(encrypted_ix = "private_record_lookup")]
    pub fn private_record_lookup_callback(
        ctx: Context<PrivateRecordLookupCallback>,
        output: SignedComputationOutputs<PrivateRecordLookupOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(PrivateRecordLookupOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(ReceivedPrivateRecordLookupEvent {
            nonce: o.nonce.to_le_bytes(),
            patient_id: o.ciphertexts[0],
            age: o.ciphertexts[1],
            gender: o.ciphertexts[2],
            blood_type: o.ciphertexts[3],
            weight: o.ciphertexts[4],
            height: o.ciphertexts[5],
            allergies: o.ciphertexts[6..11]
                .try_into()
                .map_err(|_| ErrorCode::InvalidAllergyData)?,
        });
        Ok(())
    }
    pub fn private_record_lookup(
        ctx: Context<PrivateRecordLookup>,
        computation_offset: u64,
        receiver: [u8; 32],
        receiver_nonce: u128,
        sender_pub_key: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        let args = ArgBuilder::new()
            .x25519_pubkey(receiver)
            .plaintext_u128(receiver_nonce)
            .x25519_pubkey(sender_pub_key)
            .plaintext_u128(nonce)
            .account(
                ctx.accounts.patient_data.key(),
                8,
                PatientData::INIT_SPACE as u32,
            )
            .build();

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![PrivateRecordLookupCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[],
            )?],
            1,
            0,
        )?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct StorePatientData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = payer,
        space = 8 + PatientData::INIT_SPACE,
        seeds = [b"patient_data", payer.key().as_ref()],
        bump,
    )]
    pub patient_data: Account<'info, PatientData>,
}
/// Stores encrypted patient medical information.
#[account]
#[derive(InitSpace)]
pub struct PatientData {
    /// Encrypted unique patient identifier
    pub patient_id: [u8; 32],
    /// Encrypted patient age
    pub age: [u8; 32],
    /// Encrypted gender information
    pub gender: [u8; 32],
    /// Encrypted blood type
    pub blood_type: [u8; 32],
    /// Encrypted weight measurement
    pub weight: [u8; 32],
    /// Encrypted height measurement
    pub height: [u8; 32],
    /// Array of encrypted allergy information (up to 5 allergies)
    pub allergies: [[u8; 32]; 5],
}
#[queue_computation_accounts("private_record_lookup", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct PrivateRecordLookup<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PRIVATE_RECORD_LOOKUP)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    pub patient_data: Account<'info, PatientData>,
}
#[init_computation_definition_accounts("private_record_lookup", payer)]
#[derive(Accounts)]
pub struct InitPrivateRecordLookupCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}
#[callback_accounts("private_record_lookup")]
#[derive(Accounts)]
pub struct PrivateRecordLookupCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PRIVATE_RECORD_LOOKUP)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    /// CHECK: computation_account, checked by arcium program via constraints in the callback context.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
}
#[event]
pub struct ReceivedPrivateRecordLookupEvent {
    pub nonce: [u8; 16],
    pub patient_id: [u8; 32],
    pub age: [u8; 32],
    pub gender: [u8; 32],
    pub blood_type: [u8; 32],
    pub weight: [u8; 32],
    pub height: [u8; 32],
    pub allergies: [[u8; 32]; 5],
}
#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Invalid allergy data format")]
    InvalidAllergyData,
    #[msg("Cluster not set")]
    ClusterNotSet,
}