pub struct Balance {
    pub x: u64,
    pub y: u64,
}

pub struct User {
    pub name: String,
    pub balance: Balance,
    pub orders: Vec<Order>,
}

#[derive(Clone)]
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
                    price_level.bids.push(order.clone());
                } else {
                    price_level.asks.push(order.clone());
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
        
        self.match_orders(order.price);
    }

    pub fn match_orders(&mut self, price: u64) {
        let price_level = self.prices.iter_mut().find(|price_level| price_level.price == price);

        //if any bids and asks coexist at this price, match them
        if let Some(price_level) = price_level {
            while !price_level.bids.is_empty() && !price_level.asks.is_empty() {
                //whichever is smaller, take that quantity
                let quantity = std::cmp::min(price_level.bids[0].quantity, price_level.asks[0].quantity);

                //remove the smaller order
                if price_level.bids[0].quantity < price_level.asks[0].quantity {
                    price_level.bids.pop();
                    price_level.asks[0].quantity -= quantity;

                } else {
                    price_level.asks.pop();
                    price_level.bids[0].quantity -= quantity;
                }

            }
        }
    }

}
