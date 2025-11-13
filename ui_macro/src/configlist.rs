use syn::{
    Ident, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct ConfigPair {
    key: Ident,
    value: LitStr,
}

impl Parse for ConfigPair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(ConfigPair { key, value })
    }
}

pub struct ConfigList(pub Vec<(String, String)>);

impl Default for ConfigList {
    fn default() -> Self {
        ConfigList(vec![])
    }
}

impl Parse for ConfigList {
    fn parse(input: ParseStream) -> Result<Self> {
        let list = Punctuated::<ConfigPair, Token![,]>::parse_terminated(input)?;
        let list = list
            .iter()
            .map(|x| (x.key.to_string(), x.value.value()))
            .collect::<Vec<_>>();
        Ok(ConfigList(list))
    }
}
