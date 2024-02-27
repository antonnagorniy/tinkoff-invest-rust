mod tests {
    use std::env;

    use tinkoff_invest_api::{TIResult, TinkoffInvestService};

    #[tokio::test]
    async fn test_get_accounts() -> TIResult<()> {
        let service = TinkoffInvestService::new(env::var("TNK_API_KEY").unwrap());
        let channel = service.create_channel().await?;
        let mut users = service.users(channel).await?;

        let accounts = users
            .get_accounts(tonic::Request::new(
                tinkoff_invest_api::tcs::GetAccountsRequest {},
            ))
            .await?;

        println!("Response {:?}", accounts);

        Ok(())
    }
}
