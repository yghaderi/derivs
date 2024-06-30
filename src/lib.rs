mod strategy {
    use crate::{OptionType, Position};

    struct Commission {
        long: f64,
        short: f64,
    }
    struct Option {
        option_type: OptionType,
        ins_code: String,
        name: String,
        symbol: String,
        contract_size: f64,
        begin_date: String,
        end_date: String,
        k: f64,
        t: f64,
        bid_price: f64,
        bid_vol: f64,
        ask_price: f64,
        ask_vol: f64,
        commission: Commission,
    }
    impl Option {
        fn net_bid_price(&self) -> f64 {
            self.bid_price * (1. + self.commission.long)
        }
        fn net_ask_price(&self) -> f64 {
            self.bid_price * (1. - self.commission.short)
        }
        fn profit(&self, st: f64, position: Position) -> f64 {
            let distance = st - self.k;
            let p = match position {
                Position::Long => {
                    let net_premium = self.bid_price * (1. + self.commission.long);
                    match self.option_type {
                        OptionType::Call => f64::max(distance, 0.) - net_premium,
                        OptionType::Put => f64::max(-distance, 0.) - net_premium,
                    }
                }
                Position::Short => {
                    let net_premium = self.ask_price * (1. - self.commission.short);
                    match self.option_type {
                        OptionType::Call => -f64::max(distance, 0.) + net_premium,
                        OptionType::Put => -f64::max(-distance, 0.) + net_premium,
                    }
                }
            };
            p
        }
    }
    struct UnderlyingAsset {
        ins_code: String,
        symbol: String,
        ask_price: f64,
        ask_vol: f64,
        bid_price: f64,
        bid_vol: f64,
        commission: Commission,
    }
    impl UnderlyingAsset {
        fn net_ask_price(&self) -> f64 {
            self.bid_price * (1. - self.commission.short)
        }
    }
    struct CoveredCall {
        max_pot_profit: f64,
        max_pot_loss: f64,
        break_even: f64,
        current_profit: f64,
    }
    fn covered_call(call: Option, ua: UnderlyingAsset) -> CoveredCall {
        let max_pot_profit = call.k - ua.net_ask_price() + call.net_bid_price();
        let max_pot_loss = call.net_bid_price() - ua.net_ask_price();
        let break_even = ua.net_ask_price() - call.net_bid_price();
        let current_profit = call.profit(ua.net_ask_price(), Position::Short);
        CoveredCall {
            max_pot_profit,
            max_pot_loss,
            break_even,
            current_profit,
        }
    }
}
enum OptionType {
    Call,
    Put,
}

enum Position {
    Long,
    Short,
}
