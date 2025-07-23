use std::collections::HashMap;

use rust_decimal::{Decimal, MathematicalOps};

use crate::portfolio::position::Position;

pub mod position;

pub struct Portfolio {
    pub active_position: Vec<Position>
}

impl Portfolio {

    // Book p36
    pub fn compute_return(&self, current_prices: HashMap<String, Decimal>) -> Decimal {
        self.active_position.iter().map(|pos| {
            ((pos.amount_usd_invested() / pos.entry_price()) * current_prices.get(pos.asset()).unwrap()) - pos.amount_usd_invested()
        }).sum()
    }

    // TODO: for this I need historical return over days
    // probably a timeseries crate
    pub fn compute_return_volatility(&self) -> Decimal {
        // volatility of a return is its standard deviation
        // sqrt(mean( (return - mean(returns))^2 ))
        todo!()
    }

    // TODO: for this I need historical return over days
    // probably a timeseries crate
    pub fn approximate_variance(&self) -> Decimal {
        // a good approximation for finance is variance = mean(returns^2) (mean of each return^2)
        todo!()
    } 

    pub fn compute_variance(&self) -> Decimal {
        // Variance is vol power 2
        self.compute_return_volatility().powd(Decimal::from(2))
    }

    // TODO: need historical portfolio return over time
    pub fn compute_compounded_returns(&self) -> Decimal {
        // prod(1 + r for r in returns) - 1 
        todo!()
    }

}