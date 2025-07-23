use rust_decimal::Decimal;

pub struct Position {
    asset: String,
    entry_price : Decimal,
    amount_usd_invested: Decimal,
}

impl Position {
    pub fn new(asset: String, entry_price: Decimal, amount_usd_invested: Decimal) -> Self {
        Self {
            asset,
            entry_price,
            amount_usd_invested,
        }
    }

    pub fn asset(&self) -> &String {
        &self.asset
    }

    pub fn entry_price(&self) -> &Decimal {
        &self.entry_price
    }

    pub fn amount_usd_invested(&self) -> &Decimal {
        &self.amount_usd_invested
    }

    // Book p36
    pub fn compute_return(&self, current_price: Decimal) -> Decimal {
        (current_price - self.entry_price) / self.entry_price
    }

    // Book p36
    pub fn compute_return_with_dividend(&self, current_price: Decimal, accumulated_dividend: Decimal) -> Decimal {
        (current_price + accumulated_dividend - self.entry_price) / self.entry_price
    }
}