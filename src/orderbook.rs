pub struct Balance {
    pub x: u64,
    pub y: u64,
}

pub struct User {
    pub name: String,
    pub balance: Balance,
    pub orders: Vec<Order>,
}

pub struct Order {
    pub user: &'static User,
    pub id: u64,
    pub price: u64,
    pub quantity: u64,
}

pub struct PriceLevel {
    pub price: u64,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

pub struct OrderBook {
    pub prices: Vec<PriceLevel>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            prices: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price_level = self.prices.iter_mut().find(|price_level| price_level.price == order.price);

        match price_level {
            Some(price_level) => {
                if order.quantity > 0 {
                    price_level.bids.push(order);
                } else {
                    price_level.asks.push(order);
                }
            }
            None => {
                self.prices.push(PriceLevel {
                    price: order.price,
                    bids: Vec::new(),
                    asks: Vec::new(),
                });
            }
        }
    }

    pub fn match_orders(&mut self, price: u64) {
        let price_level = self.prices.iter_mut().find(|price_level| price_level.price == price);

        
    }

}
