#[macro_export]
macro_rules! get_params {
    ($env:expr, $params:expr, $($type:ty),+) => {
        {
            let mut iter = $params.iter();
            ($(
                {
                    let result: $type = iter
                        .next()
                        .ok_or(ContractError::InvalidParameter)?
                        .try_into_val($env)
                        .map_err(|_| ContractError::InvalidParameterType)?;
                    result
                }
            ),+)
        }
    };
}
