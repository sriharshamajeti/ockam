use crate::{SecureChannelTrustInfo, TrustPolicy};
use ockam_core::{async_trait, compat::boxed::Box};
use ockam_core::{AsyncTryClone, Result};

/// Succeeds if any or both `TrustPolicy` checks succeeded
#[derive(AsyncTryClone)]
#[async_try_clone(crate = "ockam_core")]
pub struct AnyTrustPolicy<F: TrustPolicy, S: TrustPolicy> {
    // TODO: Extend for more than 2 policies
    first: F,
    second: S,
}

impl<F: TrustPolicy, S: TrustPolicy> AnyTrustPolicy<F, S> {
    /// Constructor
    pub fn new(first: F, second: S) -> Self {
        AnyTrustPolicy { first, second }
    }
}

#[async_trait]
impl<F: TrustPolicy, S: TrustPolicy> TrustPolicy for AnyTrustPolicy<F, S> {
    async fn check(&self, trust_info: &SecureChannelTrustInfo) -> Result<bool> {
        // TODO: is the short circuit here a side channel?
        Ok(self.first.check(trust_info).await? || self.second.check(trust_info).await?)
    }
}

#[cfg(test)]
mod test {
    use crate::{IdentityIdentifier, SecureChannelTrustInfo, TrustPolicy};
    use ockam_core::Result;
    use ockam_core::{async_trait, compat::boxed::Box};

    #[tokio::test]
    async fn test() {
        #[derive(Clone)]
        struct TrustPolicyStub(bool);

        #[async_trait]
        impl TrustPolicy for TrustPolicyStub {
            async fn check(&self, _trust_info: &SecureChannelTrustInfo) -> Result<bool> {
                Ok(self.0)
            }
        }

        let id = IdentityIdentifier::random();
        let trust_info = SecureChannelTrustInfo::new(id);

        assert!(TrustPolicyStub(true)
            .or(TrustPolicyStub(true))
            .check(&trust_info)
            .await
            .unwrap());
        assert!(TrustPolicyStub(true)
            .or(TrustPolicyStub(false))
            .check(&trust_info)
            .await
            .unwrap());
        assert!(TrustPolicyStub(false)
            .or(TrustPolicyStub(true))
            .check(&trust_info)
            .await
            .unwrap());
        assert!(!TrustPolicyStub(false)
            .or(TrustPolicyStub(false))
            .check(&trust_info)
            .await
            .unwrap());
    }
}
