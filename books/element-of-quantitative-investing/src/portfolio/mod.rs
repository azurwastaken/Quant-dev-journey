use std::collections::HashMap;

use rust_decimal::{Decimal, MathematicalOps};

use crate::portfolio::position::Position;

pub mod position;

pub struct Portfolio {
    pub active_position: Vec<Position>
}

impl Portfolio {

    // Book p36
    // TODO do better handling for current_prices
    pub fn compute_return(&self, current_prices: HashMap<String, Decimal>) -> Decimal {
        self.active_position.iter().map(|pos| {
            //TODO : remove unwrap
            ((pos.amount_usd_invested() / pos.entry_price()) * current_prices.get(pos.asset()).unwrap()) - pos.amount_usd_invested()
        }).sum()
    }

    // This is to know if its worth to invest in our fund or not
    // its basically our return vs return of safe invest (ex Livret A in France)
    pub fn compute_excess_return(&self, current_prices: HashMap<String, Decimal>, risk_free_rate : Decimal) -> Decimal{
        let risk_free_return: Decimal = self.active_position.iter().map(|pos| {
            pos.amount_usd_invested() * risk_free_rate
        }).sum();
        let folio_return = self.compute_return(current_prices);
        folio_return - risk_free_return
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