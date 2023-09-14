use error_chain::error_chain;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        TimestampError(std::time::SystemTimeError);
        WalletError(ethers::signers::WalletError);

    }
}
