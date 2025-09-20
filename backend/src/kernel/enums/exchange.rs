use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumCount, EnumIter};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumIter, EnumCount, Serialize, Default)]
pub enum Exchange {
    #[default]
    Bybit = 1,
    Gate = 2,
    Bitget = 3,
    Binance = 4,
    Htx = 5,
    Okx = 6,
    Mexc = 7,
}

pub static PAIR_URLS: LazyLock<HashMap<(Exchange, Symbol), &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    for ex in Exchange::iter() {
        for sym in Symbol::iter() {
            let url = match ex {
                Exchange::Bybit => format!("https://www.bybit.com/trade/spot/{sym}/USDT"),
                Exchange::Gate => format!("https://gate.io/trade/{sym}_USDT"),
                Exchange::Bitget => format!("https://bitget.com/spot/{sym}USDT"),
                Exchange::Binance => format!("https://binance.com/trade/{sym}_USDT"),
                Exchange::Htx => format!(
                    "https://htx.com/trade/{}_usdt?type=spot",
                    sym.to_string().to_lowercase()
                ),
                Exchange::Okx => format!("https://okx.com/trade-spot/{sym}-usdt"),
                Exchange::Mexc => format!("https://www.mexc.com/exchange/{sym}_USDT"),
            };

            let url: &'static str = Box::leak(url.into_boxed_str());

            map.insert((ex, sym), url);
        }
    }

    map
});

impl Exchange {
    pub fn pair_url(&self, symbol: Symbol) -> String {
        match self {
            Exchange::Bybit => PAIR_URLS[&(Exchange::Bybit, symbol)].to_string(),
            Exchange::Gate => PAIR_URLS[&(Exchange::Gate, symbol)].to_string(),
            Exchange::Bitget => PAIR_URLS[&(Exchange::Bitget, symbol)].to_string(),
            Exchange::Binance => PAIR_URLS[&(Exchange::Binance, symbol)].to_string(),
            Exchange::Htx => PAIR_URLS[&(Exchange::Htx, symbol)].to_string(),
            Exchange::Okx => PAIR_URLS[&(Exchange::Okx, symbol)].to_string(),
            Exchange::Mexc => PAIR_URLS[&(Exchange::Mexc, symbol)].to_string(),
        }
    }
}

#[derive(
    Display,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    EnumIter,
    EnumCount,
    AsRefStr,
    Serialize,
    Default,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Symbol {
    #[default]
    Btc,
    Eth,
    Xrp,
    Bnb,
    Sol,
    Doge,
    Ada,
    Trx,
    Wlfi,
    Wbtc,
    Link,
    Avax,
    Usde,
    Bch,
    Xlm,
    Sui,
    Xusd,
    Usd1,
    Hbar,
    Wbeth,
    Ltc,
    Trump,
    Ton,
    Shib,
    Zkc,
    Pump,
    Dot,
    Uni,
    Aave,
    Pepe,
    Near,
    Etc,
    Apt,
    Tao,
    Icp,
    Pengu,
    Linea,
    Pol,
    Arb,
    Render,
    Vet,
    Ena,
    Algo,
    Fdusd,
    Bonk,
    Atom,
    Bnsol,
    Fet,
    Fil,
    Imx,
    Ondo,
    Inj,
    Sei,
    Bera,
    Wld,
    Virtual,
    Somi,
    Qnt,
    Kaito,
    Ldo,
    Plume,
    Avnt,
    Stx,
    Bfusd,
    Crv,
    Bard,
    Floki,
    Ray,
    Kaia,
    Wif,
    Open,
    Grt,
    Prove,
    Cfx,
    Sahara,
    Tia,
    Theta,
    Nexo,
    Ens,
    Kmno,
    Pendle,
    Zec,
    Cake,
    Xtz,
    Sand,
    Holo,
    Form,
    Flow,
    Mana,
    Bttc,
    Eigen,
    Zro,
    Strk,
    Rune,
    Ape,
    Dydx,
    Syrup,
    Twt,
    Tusd,
    Sun,
    Rsr,
    Chz,
    Comp,
    Egld,
    Beamx,
    Xec,
    Cvx,
    #[strum(serialize = "1INCH")]
    Inch1,
    Gno,
    Super,
    Kava,
    Move,
    Ronin,
    Axl,
    Bio,
    Lunc,
    Ftt,
    Jst,
    Lpt,
    Sfp,
    Mina,
    #[strum(serialize = "1000CHEEMS")]
    Cheems1000,
    Zrx,
    Sushi,
    Blur,
    Gas,
    Ordi,
}
