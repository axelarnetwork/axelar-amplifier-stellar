#[macro_export]
macro_rules! get_params {
    ($env:expr, $params:expr, $($type:ty),+) => {
        {
            let mut index = 0;
            ($(
                {
                    let result: $type = $params
                        .get(index)
                        .ok_or(ContractError::InvalidParameter)?
                        .try_into_val($env)
                        .map_err(|_| ContractError::InvalidParameterType)?;
                    index += 1;
                    result
                }
            ),+)
        }
    };
}
