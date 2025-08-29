pub struct Market {
    pub admin: Pubkey,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub liquidity_threshold: u64,
    pub fee: u64,
    pub fee_vault: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}