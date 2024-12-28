use std::env;

use colored::*;
use reqwest::Client;

#[derive(
  Default,
  Debug,
  Clone,
  PartialEq,
  serde_derive::Serialize,
  serde_derive::Deserialize,
)]
pub struct Position {
  pub posicao: i64,
  pub pontos: i64,
  pub time: Team,
  pub jogos: i64,
  pub vitorias: i64,
  pub empates: i64,
  pub derrotas: i64,
  pub gols_pro: i64,
  pub gols_contra: i64,
  pub saldo_gols: i64,
  pub aproveitamento: f64,
  pub variacao_posicao: i64,
  pub ultimos_jogos: Vec<String>,
}

#[derive(
  Default,
  Debug,
  Clone,
  PartialEq,
  serde_derive::Serialize,
  serde_derive::Deserialize,
)]
pub struct Team {
  pub time_id: i64,
  pub nome_popular: String,
  pub escudo: String,
}

const MY_TEAM: &str = "Flamengo";

fn format_variation(variation: i64) -> String {
  if variation == 0 {
    return "✦ 0".color("gray").to_string();
  }

  let is_bigger = variation > 0;
  let value = if is_bigger { variation } else { variation * -1 };
  let result = if is_bigger {
    format!("🠉 {}", value).green()
  } else {
    format!("🠋 {}", value).red()
  };

  result.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();

  let endpoint: String = env::var("ENDPOINT").unwrap().to_string();
  let token: String = env::var("TOKEN").unwrap().to_string();

  let client: Client = reqwest::Client::new();

  let response: Vec<Position> = client
    .get(&endpoint)
    .bearer_auth(token)
    .send()
    .await?
    .json()
    .await?;

  let separator = String::from("--------------------------------").dimmed().to_string();

  println!("{}", separator);

  response.iter().for_each(|position: &Position| {
    let line = format!(
      "{} | {} | {}",
      format!("{:0>2}", position.posicao),
      format_variation(position.variacao_posicao),
      if position.time.nome_popular == MY_TEAM {
        position.time.nome_popular.to_string()
      } else {
        position.time.nome_popular.dimmed().to_string()
      }
    );

    println!("{}", line);
  });

  println!("{}", separator);

  Ok(())
}
