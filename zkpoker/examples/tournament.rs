use zkpoker::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏆 zkPoker Tournament Example");
    println!("============================");

    let service = PokerService::new();

    // Create multiple tables for tournament
    println!("\n1. Setting up tournament tables...");
    let mut tables = Vec::new();
    
    for i in 1..=3 {
        let table_request = CreateTableRequest {
            small_blind: 25,
            big_blind: 50,
            max_players: 4,
        };
        
        let table_id = service.create_table(table_request).await?;
        tables.push(table_id);
        println!("✅ Table {} created: {}", i, table_id);
    }

    // Add players across tables
    println!("\n2. Players joining tournament...");
    let mut all_players = Vec::new();
    
    for (table_idx, &table_id) in tables.iter().enumerate() {
        for player_num in 1..=4 {
            let player_name = format!("T{}P{}", table_idx + 1, player_num);
            let join_request = JoinTableRequest {
                player_name: player_name.clone(),
                buy_in: 1500,
                table_id: Some(table_id),
            };
            
            let (player_id, _) = service.join_table(join_request).await?;
            all_players.push((player_id, player_name, table_id));
        }
    }

    println!("✅ {} players registered across {} tables", all_players.len(), tables.len());

    // Show tournament status
    println!("\n3. Tournament status...");
    let active_tables = service.get_active_tables().await;
    
    for (i, table) in active_tables.iter().enumerate() {
        println!("Table {}: {} players, pot: {}, stage: {:?}", 
            i + 1, table.players_count, table.pot, table.stage);
    }

    // Simulate some tournament play
    println!("\n4. Simulating tournament play...");
    
    for (table_idx, &table_id) in tables.iter().enumerate() {
        println!("\n   Table {} action:", table_idx + 1);
        
        // Get state for first player at this table
        let table_players: Vec<_> = all_players.iter()
            .filter(|(_, _, tid)| *tid == table_id)
            .collect();
        
        if let Some((player_id, player_name, _)) = table_players.first() {
            let game_state = service.get_game_state(table_id, *player_id).await?;
            
            println!("     Current stage: {:?}", game_state.stage);
            println!("     Pot: {}", game_state.pot);
            
            if game_state.can_act {
                // Make a demonstration move
                let action = if game_state.pot > 0 { "call" } else { "check" };
                let action_request = ActionRequest {
                    player_id: *player_id,
                    game_id: table_id,
                    action: action.to_string(),
                    player_secret: None,
                };
                
                match service.player_action(action_request).await {
                    Ok(_) => println!("     {} performed: {}", player_name, action),
                    Err(e) => println!("     Action failed: {}", e),
                }
            }
        }
    }

    // Final tournament leaderboard
    println!("\n5. Tournament leaderboard...");
    let leaderboard = service.get_leaderboard().await;
    
    println!("🏆 Final Rankings:");
    for (rank, (name, chips)) in leaderboard.iter().enumerate() {
        let medal = match rank {
            0 => "🥇",
            1 => "🥈", 
            2 => "🥉",
            _ => "  ",
        };
        println!("   {} {}. {} - {} chips", medal, rank + 1, name, chips);
    }

    println!("\n🎉 Tournament simulation completed!");
    println!("🔐 Privacy-preserving features:");
    println!("   ✅ Each player's cards remain private");
    println!("   ✅ Hand strengths provable via ZK proofs");
    println!("   ✅ Transparent tournament progression");
    println!("   ✅ Verifiable final rankings");

    Ok(())
}
