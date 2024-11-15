use anyhow::{Context, Result};
use anyhow::anyhow;
use rand::Rng;
use tokio::time::{ sleep, Duration};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use log::debug;

type CoffeeBeans = String;
type GroundCoffee  = String;

#[derive(Debug)]
pub struct Water {
    pub temperature: usize,
}

#[warn(non_snake_case)]
type Milk = String;
type FrothedMilk = String;
type Espresso = String;
type Cappuccino = String;


#[derive(Debug)]
struct GrindingException;
struct FrothingException;
struct WaterBoilingException;
struct BrewingException;

#[derive(Debug)]
enum MyErrors {
    GrindingException,
    WaterBoilingException,
    BrewingException,
}

impl std::error::Error for MyErrors {}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GrindingException => write!(f, "This grinder needs a replacement, seriously!"),
            Self::WaterBoilingException => write!(f, "WaterBoilingException"),
            Self::BrewingException => write!(f, "Wish me luck"),
        }
    }
}


pub async fn grind(beans: CoffeeBeans) ->  Result<GroundCoffee> {
    debug!("start grinding...");
    sleep(Duration::from_millis(rand::thread_rng().gen_range(0..2000))).await;
    if beans == "baked beans" {
        return Err(anyhow!(MyErrors::GrindingException));
    }
    debug!("finished grinding ok");
    Ok(format!("ground coffee of {}", beans))
}
pub async fn heat_water(mut water: Water) ->  Result<Water>  {
    debug!("heating the water now");
    sleep(Duration::from_millis(rand::thread_rng().gen_range(0..2000))).await;
    if water.temperature < 80 ||  water.temperature > 90 {
        return Err(anyhow!(MyErrors::WaterBoilingException));
    }
    if water.temperature > 90 {
        return Err(anyhow!(MyErrors::WaterBoilingException).context("water is to high.."));
    }
    debug!("hot, it's hot! ");
    debug!("heating ok");
    return  Ok(water)
}

pub async fn froth_milk(milk: Milk) ->  Result<FrothedMilk>  {
    debug!("start froth milk system ");
    sleep(Duration::from_millis(rand::thread_rng().gen_range(0..2000))).await;
    debug!("froth milk ok");
    Ok(format!("frothed {}", milk))
}

pub async fn brew(coffee: GroundCoffee, heated_water: Water) ->  Result<Espresso>  {
    debug!("happy brewing :)");
    sleep(Duration::from_millis(rand::thread_rng().gen_range(0..2000))).await;
    debug!("it's brewed!");
    Ok(format!("espresso with water temperature at {}", heated_water.temperature.to_string()))
}

pub async fn combine(espresso: Espresso, frothed_milk: FrothedMilk) ->  Result<Espresso>   {
    sleep(Duration::from_millis(rand::thread_rng().gen_range(0..2000))).await;
    Ok(format!("cappuccino with frothedMilk: {}", frothed_milk))
}

async fn prepare_cappuccino() -> Result<Cappuccino> {
    let ground_coffee = grind("coffe beans".to_string());
    let heated_water = heat_water(Water { temperature:95 });
    let frothed_milk = froth_milk("milk".to_string());
    let (ground, water, foam) = tokio::try_join!(ground_coffee, heated_water, frothed_milk)?;
    let espresso = brew(ground, water).await?;
    let cappuccino = combine(espresso, foam).await;
    Ok(cappuccino?)
}


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    debug!("Hello, Cappuccino maker!");
    // Initialize simplelog logging
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);
    debug!("Hello, world!");
    match prepare_cappuccino().await {
        Ok(cappuccino) => debug!("Cappuccino prepared: {}", cappuccino),
        Err(e) => debug!("Error preparing cappuccino: {}", e),
    }
}
main.rs
