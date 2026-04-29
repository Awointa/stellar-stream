#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, Vec,
};

use stellar_stream::{StreamContract, StreamContractClient}; 
use soroban_sdk::token::Client as TokenClient;

fn measure_cost<F>(env: &Env, name: &str, mut f: F)
where
    F: FnMut(),
{
    env.budget().reset_unlimited();
    
    let cpu_start = env.budget().cpu_instruction_cost();
    let mem_start = env.budget().memory_bytes_cost();

    f();

    let cpu_end = env.budget().cpu_instruction_cost();
    let mem_end = env.budget().memory_bytes_cost();

    let cpu_cost = cpu_end - cpu_start;
    let mem_cost = mem_end - mem_start;

    std::println!("BENCHMARK|{}|{}|{}", name, cpu_cost, mem_cost);
}

#[test]
fn run_all_benchmarks() {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_unlimited(); 

    let contract_id = env.register_contract(None, StreamContract);
    let client = StreamContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin);
    let token = TokenClient::new(&env, &token_id);

    let sender = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);

    let initial_balance = 1_000_000_000_000;
    token.mint(&sender, &initial_balance);

    env.ledger().set_timestamp(100_000);

    let amount = 10_000_000;
    let start_time = 150_000;
    let end_time = 250_000;

    measure_cost(&env, "create_stream", || {
        client.create_stream(
            &sender,
            &recipient1,
            &token_id,
            &amount,
            &start_time,
            &end_time,
        );
    });

    let stream_id_for_claim = client.create_stream(
        &sender,
        &recipient1,
        &token_id,
        &amount,
        &start_time,
        &end_time,
    );

    let stream_id_for_pause = client.create_stream(
        &sender,
        &recipient1,
        &token_id,
        &amount,
        &start_time,
        &end_time,
    );

    let stream_id_for_cancel = client.create_stream(
        &sender,
        &recipient1,
        &token_id,
        &amount,
        &start_time,
        &end_time,
    );

    env.ledger().set_timestamp(200_000);

    measure_cost(&env, "claim", || {
        client.claim(&recipient1, &stream_id_for_claim);
    });

    measure_cost(&env, "pause_stream", || {
        client.pause_stream(&sender, &stream_id_for_pause);
    });

    measure_cost(&env, "resume_stream", || {
        client.resume_stream(&sender, &stream_id_for_pause);
    });

    measure_cost(&env, "cancel", || {
        client.cancel(&sender, &stream_id_for_cancel);
    });

    let mut split_recipients = Vec::new(&env);
    split_recipients.push_back(recipient1.clone());
    split_recipients.push_back(recipient2.clone());

    measure_cost(&env, "create_split_stream", || {
        client.create_split_stream(
            &sender,
            &split_recipients,
            &token_id,
            &amount,
            &start_time,
            &end_time,
        );
    });
}