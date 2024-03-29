use serde_derive::{Deserialize, Serialize};

pub fn well_known_list_providers() -> &'static [&'static str] {
    &[
        "https://api.raydium.io/cache/solana-token-list",
        "https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json",
    ]
}

#[derive(Debug, Copy, Clone)]
pub struct TokenInfo {
    pub token_name: &'static str,
    pub token_symbol: &'static str,
    pub mint_address: &'static str,
    pub icon: Option<&'static str>,
    pub deprecated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicTokenList {
    pub tokens: Vec<DynamicTokenInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicTokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    #[allow(non_snake_case)]
    pub logoURI: String,
    pub tags: Vec<String>,
}

/// The list of well-known tokens that are support on sollet.io
/// The list is converted from https://github.com/project-serum/spl-token-wallet/blob/master/src/utils/tokens/names.js
pub const POPULAR_TOKENS: [TokenInfo; 26] = [
    TokenInfo {
        token_name: "Wrapped SOL",
        token_symbol: "SOL",
        mint_address: "So11111111111111111111111111111111111111112",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Serum",
        token_symbol: "SRM",
        mint_address: "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x476c5E26a75bd202a9683ffD34359C0CC15be0fF/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "MegaSerum",
        token_symbol: "MSRM",
        mint_address: "MSRMcoVyrFxnSgo5uXwone5SKcGhT1KEJMFEkMEWf9L",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x476c5E26a75bd202a9683ffD34359C0CC15be0fF/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped Bitcoin",
        token_symbol: "BTC",
        mint_address: "9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/bitcoin/info/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped Ethereum",
        token_symbol: "ETH",
        mint_address: "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped FTT",
        token_symbol: "FTT",
        mint_address: "AGFEad2et2ZJif9jaGpdMixQqvW5i81aBdvKe7PHNfz3",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/f3ffd0b9ae2165336279ce2f8db1981a55ce30f8/blockchains/ethereum/assets/0x50D1c9771902476076eCFc8B2A83Ad6b9355a4c9/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped YFI",
        token_symbol: "YFI",
        mint_address: "3JSf5tPeuscJGtaCp5giEiDhv51gQ4v3zWg8DGgyLfAB",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x0bc529c00C6401aEF6D220BE8C6Ea1667F6Ad93e/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped Chainlink",
        token_symbol: "LINK",
        mint_address: "CWE8jPTUYhdCTZYWPTe1o5DFqfdjzWKc9WKz6rSjQUdG",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x514910771AF9Ca656af840dff83E8264EcF986CA/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped XRP",
        token_symbol: "XRP",
        mint_address: "Ga2AXHpfAF6mv2ekZwcsJFqu7wB4NV331qNH7fW9Nst8",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ripple/info/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped USDT",
        token_symbol: "USDT",
        mint_address: "BQcdHdAQW1hczDbBi9hiegXAR7A98Q9jx3X3iBBBDiq4",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/f3ffd0b9ae2165336279ce2f8db1981a55ce30f8/blockchains/ethereum/assets/0xdAC17F958D2ee523a2206206994597C13D831ec7/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "USD Coin",
        token_symbol: "USDC",
        mint_address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/f3ffd0b9ae2165336279ce2f8db1981a55ce30f8/blockchains/ethereum/assets/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped USDC",
        token_symbol: "WUSDC",
        mint_address: "BXXkv6z8ykpG1yuvUDPgh732wzVHB69RnB9YgSYh3itW",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/f3ffd0b9ae2165336279ce2f8db1981a55ce30f8/blockchains/ethereum/assets/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/logo.png"),
        deprecated: true,
    },
    TokenInfo {
        token_name: "Wrapped SUSHI",
        token_symbol: "SUSHI",
        mint_address: "AR1Mtgh7zAtxuxGd2XPovXPVjcSdY3i4rQYisNadjfKy",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/master/blockchains/ethereum/assets/0x6B3595068778DD592e39A122f4f5a5cF09C90fE2/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped ALEPH",
        token_symbol: "ALEPH",
        mint_address: "CsZ5LZkDS7h9TDKjrbL7VAwQZ9nsRu8vJLhRYfmGaN8K",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/6996a371cd02f516506a8f092eeb29888501447c/blockchains/nuls/assets/NULSd6HgyZkiqLnBzTaeSQfx1TNg2cqbzq51h/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped SXP",
        token_symbol: "SXP",
        mint_address: "SF3oTvfWzEP3DTwGSvUXRrGTvr75pdZNnBLAH9bzMuX",
        icon: Some("https://github.com/trustwallet/assets/raw/b0ab88654fe64848da80d982945e4db06e197d4f/blockchains/ethereum/assets/0x8CE9137d39326AD0cD6491fb5CC0CbA0e089b6A9/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped HGET",
        token_symbol: "HGET",
        mint_address: "BtZQfWqDGbk9Wf2rXEiWyQBdBY1etnUUn6zEphvVS7yN",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped CREAM",
        token_symbol: "CREAM",
        mint_address: "5Fu5UUgbjpUvdBveb3a1JTNirL8rXtiYeSMWvKjtUNQv",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/4c82c2a409f18a4dd96a504f967a55a8fe47026d/blockchains/smartchain/assets/0xd4CB328A82bDf5f03eB737f37Fa6B370aef3e888/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped UBXT",
        token_symbol: "UBXT",
        mint_address: "873KLxCbz7s9Kc4ZzgYRtNmhfkQrhfyWGZJBmyCbC3ei",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped HNT",
        token_symbol: "HNT",
        mint_address: "HqB7uswoVg4suaQiDP3wjxob1G5WdZ144zhdStwMCq7e",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped FRONT",
        token_symbol: "FRONT",
        mint_address: "9S4t2NEAiJVMvPdRYKVrfJpBafPBLtvbvyS3DecojQHw",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/6e375e4e5fb0ffe09ed001bae1ef8ca1d6c86034/blockchains/ethereum/assets/0xf8C3527CC04340b208C854E985240c02F7B7793f/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped AKRO",
        token_symbol: "AKRO",
        mint_address: "6WNVCuxCGJzNjmMZoKyhZJwvJ5tYpsLyAtagzYASqBoF",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/878dcab0fab90e6593bcb9b7d941be4915f287dc/blockchains/ethereum/assets/0xb2734a4Cec32C81FDE26B0024Ad3ceB8C9b34037/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped HXRO",
        token_symbol: "HXRO",
        mint_address: "DJafV9qemGp7mLMEn5wrfqaFwxsbLgUsGVS16zKRk9kc",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped UNI",
        token_symbol: "UNI",
        mint_address: "DEhAasscXF4kEGxFgJ3bq4PpVGp5wyUxMRvn6TzGVHaw",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/08d734b5e6ec95227dc50efef3a9cdfea4c398a1/blockchains/ethereum/assets/0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped MATH",
        token_symbol: "MATH",
        mint_address: "GeDS162t9yGJuLEHPWXXGrb1zwkzinCgRwnT8vHYjKza",
        icon: None,
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped TOMO",
        token_symbol: "TOMO",
        mint_address: "GXMvfY2jpQctDqZ9RoU3oWPhufKiCcFEfchvYumtX7jd",
        icon: Some("https://raw.githubusercontent.com/trustwallet/assets/08d734b5e6ec95227dc50efef3a9cdfea4c398a1/blockchains/tomochain/info/logo.png"),
        deprecated: false,
    },
    TokenInfo {
        token_name: "Wrapped LUA",
        token_symbol: "LUA",
        mint_address: "EqWCKXfs3x47uVosDpTRgFniThL9Y8iCztJaapxbEaVX",
        icon: None,
        deprecated: false,
    }
];
