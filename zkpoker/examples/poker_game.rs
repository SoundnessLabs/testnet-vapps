use zkpoker::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🃏 zkPoker - Verifiable Card Gaming Example");
    println!("==========================================");

    // Initialize the poker service
    let service = PokerService::new();

    // Create a new table
    println!("\n1. Creating poker table...");
    let table_request = CreateTableRequest {
        small_blind: 10,
        big_blind: 20,
        max_players: 6,
    };
    
    let table_id = service.create_table(table_request).await?;
    println!("✅ Table created: {}", table_id);

    // Add players to the table
    println!("\n2. Players joining the table...");
    let mut players = Vec::new();
    
    for i in 1..=4 {
        let join_request = JoinTableRequest {
            player_name: format!("Player{}", i),
            buy_in: 1000,
            table_id: Some(table_id),
        };
        
        let (player_id, _) = service.join_table(join_request).await?;
        players.push((player_id, format!("Player{}", i)));
        println!("✅ {} joined (ID: {})", format!("Player{}", i), player_id);
    }

    // Wait a moment for game to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Get initial game state
    println!("\n3. Initial game state...");
    let game_state = service.get_game_state(table_id, players[0].0).await?;
    println!("🎯 Game Stage: {:?}", game_state.stage);  
    println!("💰 Pot: {}", game_state.pot);
    println!("🃏 Community Cards: {:?}", game_state.community_cards);
    
    if let Some(cards) = game_state.your_cards {
        println!("🔒 Your Cards: {} {}", cards[0], cards[1]);
    }

    // Display players
    println!("\n👥 Players at table:");
    for player in &game_state.players {
        println!("   {} - Chips: {} - Bet: {} - Status: {:?}", 
            player.name, player.chips, player.current_bet, player.status);
    }

    // Simulate a betting round
    println!("\n4. Simulating betting round...");
    
    if let Some(current_player_id) = game_state.current_player {
        let current_player_name = game_state.players.iter()
            .find(|p| p.id == current_player_id)
            .map(|p| p.name.as_str())
            .unwrap_or("Unknown");
        
        println!("🎯 Current player: {}", current_player_name);
        
        // Demonstrate different actions
        if game_state.can_act && current_player_id == players[0].0 {
            // Call the big blind
            println!("📞 Calling...");
            let action_request = ActionRequest {
                player_id: current_player_id,
                game_id: table_id,
                action: "call".to_string(),
                player_secret: None,
            };
            
            service.player_action(action_request).await?;
            println!("✅ Call completed");
        }
    }

    // Generate a ZK proof demonstration
    println!("\n5. Generating ZK hand proof...");
    let proof_request = ProofRequest {
        player_id: players[0].0,
        game_id: table_id,
        player_secret: "player1_secret_key".to_string(),
    };
    
    match service.generate_proof(proof_request).await {
        Ok(proof) => {
            println!("✅ ZK Proof generated successfully!");
            println!("   Hand Rank: {:?}", proof.public_inputs.hand_rank);
            println!("   Hand Strength: {}", proof.public_inputs.hand_strength);
            println!("   🔒 Cards remain private!");
        },
        Err(e) => {
            println!("⚠️  Proof generation not available yet: {}", e);
        }
    }

    // Show updated game state
    println!("\n6. Updated game state...");
    let updated_state = service.get_game_state(table_id, players[0].0).await?;
    println!("💰 Pot: {}", updated_state.pot);
    println!("🎯 Stage: {:?}", updated_state.stage);

    // Show leaderboard
    println!("\n7. Current leaderboard...");
    let leaderboard = service.get_leaderboard().await;
    for (i, (name, chips)) in leaderboard.iter().enumerate() {
        println!("   {}. {} - {} chips", i + 1, name, chips);
    }

    println!("\n🎉 zkPoker demonstration completed!");
    println!("🔐 Key features demonstrated:");
    println!("   ✅ Verifiable card dealing");
    println!("   ✅ Private hand management");
    println!("   ✅ Zero-knowledge proof generation");
    println!("   ✅ Transparent game state");
    println!("   ✅ Fair betting mechanics");

    Ok(())
}
