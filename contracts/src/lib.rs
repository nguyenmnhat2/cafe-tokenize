#![no_std]

use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, String, Symbol,
};

#[contracttype]
pub enum DataKey {
    Admin,
    Season(Symbol),
    Position(Symbol, Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarvestStatus {
    Open,
    Active,
    Settled,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct HarvestSeason {
    pub farmer: Address,
    pub coffee_type: String,
    pub region: String,
    pub total_shares: i128,
    pub shares_sold: i128,
    pub price_per_share: i128,
    pub raised_amount: i128,
    pub payout_pool: i128,
    pub asset: Address,
    pub expected_yield_kg: i128,
    pub harvest_date: i64,
    pub status: HarvestStatus,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct InvestorPosition {
    pub investor: Address,
    pub shares: i128,
    pub invested_amount: i128,
    pub claimed_amount: i128,
    pub refunded: bool,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CoffeeError {
    AlreadyExists = 1,
    NotFound = 2,
    NotAuthorized = 3,
    InvalidAmount = 4,
    InvalidState = 5,
    Oversubscribed = 6,
    NoPosition = 7,
    AlreadyRefunded = 8,
    NothingToClaim = 9,
    Overflow = 10,
}

#[contract]
pub struct CafeTokenize;

fn load_admin(env: &Env) -> Result<Address, CoffeeError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(CoffeeError::NotFound)
}

fn load_season(env: &Env, season_id: &Symbol) -> Result<HarvestSeason, CoffeeError> {
    env.storage()
        .persistent()
        .get(&DataKey::Season(season_id.clone()))
        .ok_or(CoffeeError::NotFound)
}

fn load_position(
    env: &Env,
    season_id: &Symbol,
    investor: &Address,
) -> Result<InvestorPosition, CoffeeError> {
    env.storage()
        .persistent()
        .get(&DataKey::Position(season_id.clone(), investor.clone()))
        .ok_or(CoffeeError::NoPosition)
}

fn checked_add(lhs: i128, rhs: i128) -> Result<i128, CoffeeError> {
    lhs.checked_add(rhs).ok_or(CoffeeError::Overflow)
}

fn checked_sub(lhs: i128, rhs: i128) -> Result<i128, CoffeeError> {
    lhs.checked_sub(rhs).ok_or(CoffeeError::Overflow)
}

fn checked_mul(lhs: i128, rhs: i128) -> Result<i128, CoffeeError> {
    lhs.checked_mul(rhs).ok_or(CoffeeError::Overflow)
}

#[contractimpl]
impl CafeTokenize {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn create_season(
        env: Env,
        season_id: Symbol,
        farmer: Address,
        coffee_type: String,
        region: String,
        total_shares: i128,
        price_per_share: i128,
        asset: Address,
        expected_yield_kg: i128,
        harvest_date: i64,
    ) -> Result<(), CoffeeError> {
        farmer.require_auth();

        if total_shares <= 0 || price_per_share <= 0 || expected_yield_kg <= 0 {
            return Err(CoffeeError::InvalidAmount);
        }

        if env
            .storage()
            .persistent()
            .has(&DataKey::Season(season_id.clone()))
        {
            return Err(CoffeeError::AlreadyExists);
        }

        let season = HarvestSeason {
            farmer,
            coffee_type,
            region,
            total_shares,
            shares_sold: 0,
            price_per_share,
            raised_amount: 0,
            payout_pool: 0,
            asset,
            expected_yield_kg,
            harvest_date,
            status: HarvestStatus::Open,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Season(season_id), &season);
        Ok(())
    }

    pub fn buy_shares(
        env: Env,
        season_id: Symbol,
        investor: Address,
        shares: i128,
    ) -> Result<(), CoffeeError> {
        investor.require_auth();

        if shares <= 0 {
            return Err(CoffeeError::InvalidAmount);
        }

        let mut season = load_season(&env, &season_id)?;
        if season.status != HarvestStatus::Open {
            return Err(CoffeeError::InvalidState);
        }

        let remaining = checked_sub(season.total_shares, season.shares_sold)?;
        if shares > remaining {
            return Err(CoffeeError::Oversubscribed);
        }

        let amount = checked_mul(shares, season.price_per_share)?;
        let token_client = TokenClient::new(&env, &season.asset);
        token_client.transfer(&investor, &env.current_contract_address(), &amount);

        let mut position = env
            .storage()
            .persistent()
            .get(&DataKey::Position(season_id.clone(), investor.clone()))
            .unwrap_or(InvestorPosition {
                investor: investor.clone(),
                shares: 0,
                invested_amount: 0,
                claimed_amount: 0,
                refunded: false,
            });

        position.shares = checked_add(position.shares, shares)?;
        position.invested_amount = checked_add(position.invested_amount, amount)?;

        season.shares_sold = checked_add(season.shares_sold, shares)?;
        season.raised_amount = checked_add(season.raised_amount, amount)?;

        env.storage()
            .persistent()
            .set(&DataKey::Position(season_id.clone(), investor), &position);
        env.storage()
            .persistent()
            .set(&DataKey::Season(season_id), &season);
        Ok(())
    }

    pub fn release_capital(env: Env, season_id: Symbol) -> Result<(), CoffeeError> {
        let mut season = load_season(&env, &season_id)?;
        season.farmer.require_auth();

        if season.status != HarvestStatus::Open || season.raised_amount <= 0 {
            return Err(CoffeeError::InvalidState);
        }

        let token_client = TokenClient::new(&env, &season.asset);
        token_client.transfer(
            &env.current_contract_address(),
            &season.farmer,
            &season.raised_amount,
        );

        season.status = HarvestStatus::Active;
        env.storage()
            .persistent()
            .set(&DataKey::Season(season_id), &season);
        Ok(())
    }

    pub fn settle_payout(env: Env, season_id: Symbol, payout_amount: i128) -> Result<(), CoffeeError> {
        if payout_amount <= 0 {
            return Err(CoffeeError::InvalidAmount);
        }

        let mut season = load_season(&env, &season_id)?;
        season.farmer.require_auth();

        if season.status != HarvestStatus::Active || season.shares_sold <= 0 {
            return Err(CoffeeError::InvalidState);
        }

        let token_client = TokenClient::new(&env, &season.asset);
        token_client.transfer(
            &season.farmer,
            &env.current_contract_address(),
            &payout_amount,
        );

        season.payout_pool = payout_amount;
        season.status = HarvestStatus::Settled;
        env.storage()
            .persistent()
            .set(&DataKey::Season(season_id), &season);
        Ok(())
    }

    pub fn claim_payout(env: Env, season_id: Symbol, investor: Address) -> Result<i128, CoffeeError> {
        investor.require_auth();

        let season = load_season(&env, &season_id)?;
        if season.status != HarvestStatus::Settled || season.shares_sold <= 0 {
            return Err(CoffeeError::InvalidState);
        }

        let mut position = load_position(&env, &season_id, &investor)?;
        if position.refunded {
            return Err(CoffeeError::InvalidState);
        }

        let gross = checked_mul(season.payout_pool, position.shares)?;
        let entitlement = gross / season.shares_sold;
        let claimable = checked_sub(entitlement, position.claimed_amount)?;
        if claimable <= 0 {
            return Err(CoffeeError::NothingToClaim);
        }

        let token_client = TokenClient::new(&env, &season.asset);
        token_client.transfer(&env.current_contract_address(), &investor, &claimable);

        position.claimed_amount = checked_add(position.claimed_amount, claimable)?;
        env.storage()
            .persistent()
            .set(&DataKey::Position(season_id, investor), &position);

        Ok(claimable)
    }

    pub fn cancel_season(env: Env, season_id: Symbol, caller: Address) -> Result<(), CoffeeError> {
        caller.require_auth();

        let admin = load_admin(&env)?;
        let mut season = load_season(&env, &season_id)?;

        if season.status != HarvestStatus::Open {
            return Err(CoffeeError::InvalidState);
        }

        if caller != season.farmer && caller != admin {
            return Err(CoffeeError::NotAuthorized);
        }

        season.status = HarvestStatus::Cancelled;
        env.storage()
            .persistent()
            .set(&DataKey::Season(season_id), &season);
        Ok(())
    }

    pub fn refund_investment(
        env: Env,
        season_id: Symbol,
        investor: Address,
    ) -> Result<i128, CoffeeError> {
        investor.require_auth();

        let season = load_season(&env, &season_id)?;
        if season.status != HarvestStatus::Cancelled {
            return Err(CoffeeError::InvalidState);
        }

        let mut position = load_position(&env, &season_id, &investor)?;
        if position.refunded {
            return Err(CoffeeError::AlreadyRefunded);
        }
        if position.invested_amount <= 0 {
            return Err(CoffeeError::NothingToClaim);
        }

        let refund_amount = position.invested_amount;
        let token_client = TokenClient::new(&env, &season.asset);
        token_client.transfer(&env.current_contract_address(), &investor, &refund_amount);

        position.refunded = true;
        env.storage()
            .persistent()
            .set(&DataKey::Position(season_id, investor), &position);

        Ok(refund_amount)
    }

    pub fn get_season(env: Env, season_id: Symbol) -> Option<HarvestSeason> {
        env.storage().persistent().get(&DataKey::Season(season_id))
    }

    pub fn get_position(env: Env, season_id: Symbol, investor: Address) -> Option<InvestorPosition> {
        env.storage()
            .persistent()
            .get(&DataKey::Position(season_id, investor))
    }

    pub fn admin(env: Env) -> Result<Address, CoffeeError> {
        load_admin(&env)
    }

    pub fn transfer_admin(
        env: Env,
        current_admin: Address,
        new_admin: Address,
    ) -> Result<(), CoffeeError> {
        let admin = load_admin(&env)?;
        if admin != current_admin {
            return Err(CoffeeError::NotAuthorized);
        }

        current_admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{contract, contractimpl, contracttype, testutils::Address as _, Env};

    #[contracttype]
    pub enum TokenDataKey {
        Balance(Address),
    }

    #[contract]
    pub struct MockToken;

    #[contractimpl]
    impl MockToken {
        pub fn mint(env: Env, to: Address, amount: i128) {
            let key = TokenDataKey::Balance(to.clone());
            let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
            env.storage().persistent().set(&key, &(balance + amount));
        }

        pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
            from.require_auth();

            let from_key = TokenDataKey::Balance(from.clone());
            let from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
            assert!(from_balance >= amount);
            env.storage()
                .persistent()
                .set(&from_key, &(from_balance - amount));

            let to_key = TokenDataKey::Balance(to.clone());
            let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
            env.storage().persistent().set(&to_key, &(to_balance + amount));
        }

        pub fn balance(env: Env, id: Address) -> i128 {
            env.storage()
                .persistent()
                .get(&TokenDataKey::Balance(id))
                .unwrap_or(0)
        }
    }

    fn setup() -> (
        Env,
        CafeTokenizeClient<'static>,
        Address,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(CafeTokenize, CafeTokenizeArgs::__constructor(&admin));
        let client = CafeTokenizeClient::new(&env, &contract_id);

        let asset = env.register(MockToken, ());
        let farmer = Address::generate(&env);
        let investor = Address::generate(&env);
        let token = MockTokenClient::new(&env, &asset);

        token.mint(&farmer, &1_000_000_i128);
        token.mint(&investor, &1_000_000_i128);

        (env, client, admin, asset, farmer, investor)
    }

    #[test]
    fn test_full_coffee_harvest_flow() {
        let (env, client, _admin, asset, farmer, investor_a) = setup();
        let investor_b = Address::generate(&env);
        let token = MockTokenClient::new(&env, &asset);
        token.mint(&investor_b, &1_000_000_i128);

        let season_id = Symbol::new(&env, "crop2026");
        client.create_season(
            &season_id,
            &farmer,
            &String::from_str(&env, "Arabica"),
            &String::from_str(&env, "Lam Dong"),
            &100_i128,
            &1_000_i128,
            &asset,
            &5_000_i128,
            &1_780_704_000_i64,
        );

        client.buy_shares(&season_id, &investor_a, &40_i128);
        client.buy_shares(&season_id, &investor_b, &60_i128);
        client.release_capital(&season_id);
        client.settle_payout(&season_id, &150_000_i128);

        let payout_a = client.claim_payout(&season_id, &investor_a);
        let payout_b = client.claim_payout(&season_id, &investor_b);

        assert_eq!(payout_a, 60_000_i128);
        assert_eq!(payout_b, 90_000_i128);

        let season = client.get_season(&season_id).unwrap();
        assert_eq!(season.status, HarvestStatus::Settled);
        assert_eq!(token.balance(&investor_a), 1_020_000_i128);
        assert_eq!(token.balance(&investor_b), 1_030_000_i128);
    }

    #[test]
    fn test_cannot_oversubscribe_shares() {
        let (env, client, _admin, asset, farmer, investor) = setup();
        let season_id = Symbol::new(&env, "smalllot");

        client.create_season(
            &season_id,
            &farmer,
            &String::from_str(&env, "Robusta"),
            &String::from_str(&env, "Dak Lak"),
            &10_i128,
            &500_i128,
            &asset,
            &800_i128,
            &1_780_704_000_i64,
        );

        client.buy_shares(&season_id, &investor, &8_i128);
        let result = client.try_buy_shares(&season_id, &investor, &3_i128);
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_and_refund() {
        let (env, client, admin, asset, farmer, investor) = setup();
        let token = MockTokenClient::new(&env, &asset);
        let season_id = Symbol::new(&env, "refund1");

        client.create_season(
            &season_id,
            &farmer,
            &String::from_str(&env, "Excelsa"),
            &String::from_str(&env, "Gia Lai"),
            &50_i128,
            &2_000_i128,
            &asset,
            &1_200_i128,
            &1_780_704_000_i64,
        );

        client.buy_shares(&season_id, &investor, &10_i128);
        client.cancel_season(&season_id, &admin);
        let refunded = client.refund_investment(&season_id, &investor);

        assert_eq!(refunded, 20_000_i128);
        assert_eq!(token.balance(&investor), 1_000_000_i128);

        let position = client.get_position(&season_id, &investor).unwrap();
        assert!(position.refunded);
    }

    #[test]
    fn test_transfer_admin() {
        let (_env, client, admin, _asset, _farmer, _investor) = setup();
        let new_admin = Address::generate(&_env);

        client.transfer_admin(&admin, &new_admin);
        assert_eq!(client.admin(), new_admin);
    }
}
