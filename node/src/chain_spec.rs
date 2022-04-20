use hex_literal::hex;
use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, opaque::SessionKeys, BabeConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, MaxNominations,
	SessionConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	BABE_GENESIS_EPOCH_CONFIG, wasm_binary_unwrap,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"SMARTRIB3 ndn",
		"ndn_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5C5u7H2cH6VWo9rqevGZt2AuMvg1556TWbTjjatdGkYNnhZb
			hex!["00e91f60e0f607d098eb3143fd646b3c220f8013544e0e5012110e95ef1cba13"].into(),
			// 5CDRcabWSuKmf4dcKSFYsMrE7bQLHRPa8ZR4aMbfewdGgnx2
			hex!["06a687b6ddd418ca5bc77a125795ec9a4827cae9cbc0a7cff2177d56315f3478"].into(),
           // 5EgmgED25XxfMBwEcsXoHi4hL2JmThThG6pvqmMx2nGY1ahR
           hex!["73fa2924b32322dda47a44e9b41c3e563ef5f2fee41abf83b4f778d3f3e4282d"].unchecked_into(),
		   // 5H3RJbvMNXAoAcYrfrDpU7UiHAiwXxKUEDYJpR5Kbb1HAup1
           hex!["dc3233310f15a2b40f53495f0e2ffc9ee454c1337bc8111199b0cfdb06b2cc15"].unchecked_into(),
           // 5FLos8edEeUWMeFpnN5rzPL9tKJ6j9XcqtHsGixVmxZoXaT1
          hex!["90fcd9ff93b62740fccc28a29e7feef69af7d08ed7ef844428b0ebfb90417555"].unchecked_into(),                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
		),
		(
			// 5GpEr1YaFPqsyEERdiKqz9vJABoxmbKi217pwuFoRCPSDFLE
			hex!["d224cda3bf119f955a338faad375453e32875fda28c0f422ff6cf8cbac815a5c"].into(),
			// 5Fh55Yt8qkHcDeJjUuqfku9FfGSEfvGkJ49wob6GfCVKMfiZ
			hex!["a070f9911116141dee3e18c55053ebe4243a42d3fb55cf1187205ae4d002410b"].into(),
			// 5DKK2NCP2KtJvptydxSGcANCqUTgeuN2V3sHSygQZ1DVp7Kb
			hex!["37600d6dd603cc13a218d547cd1bf714ea104907b441ada6d554f0589fc4bcd8"].unchecked_into(),
			// 5Dtp2ojbA8NmWcn8dvvJtGZA7Kk9qtGxDbxT4AtRqkbfRzNb
			hex!["50eccc8f1f79a0c40ee81719067dd4336a8ef61a612de51c88d3df72b0fa1e2b"].unchecked_into(),
			// 5CqY7ZQNKDHtNDtxu6S51ybxr3HYXpuy5qJBQeWFNnNQHf7A
			hex!["22313e94308c1c073076c642d04eb3ae032cc80ac6892e9c1ed3d605dd4ac76d"].unchecked_into(),
		),
		(
			// 5CVRQVPzjVngpzYT1Hd5ozxfjUVGGebEHKZHvFHxHZRyv4Xj
			hex!["12d9c142d38adb92e64217b19a5556718c498e2d70675374ee87deacad08ad11"].into(),
			// 5DZWE3XHnorTReNvbXqLxCXSg8TeeMKK4afGy5xd7SzjcwqN
			hex!["423331f92ae770f67eca55836add7d0a302e1648eae3fa948fcf22a8ebee5957"].into(),
			// 5DEAswn7dNkLjd5MMQM9rFFjrLu4kchLXpDqBVExJNno4nZi
			hex!["33746802210a9f9e2191035fd32daca27a8787b1d402cb80c94e395b7e84df39"].unchecked_into(),
			// 5ERnFELe4BXQKvSb94N9g1rA5nMJxZ4D1yCWgLobaRYxMmju
			hex!["688b6524ae4d84ce18dcf86c17dab2b7e1ddfbe3a890da8bed9e3b03b85abc6a"].unchecked_into(),
			// 5HYAAyqKZpni1pJomHimFA3Cx8XQ8UikK8KNtPfcJuCuykUD
			hex!["f21d613ad05c6a475b3cb224d01ddaed0a6fa514979a3265b9621964c791cb08"].unchecked_into(),
		),
	
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5En7mnn2XiZy9rUhLea1jMVsYXdTEHaeK2Tm6YcJ7jZBiRou
		"780e0959fa6591345adcfc55a3ed242f6bbeb261c689ef7f56c9006530ea251c"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
