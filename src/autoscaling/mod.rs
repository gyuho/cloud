use crate::errors::{Error::API, Result};
use aws_sdk_autoscaling::{
    error::{SetInstanceHealthError, SetInstanceHealthErrorKind},
    types::SdkError,
    Client,
};
use aws_types::SdkConfig as AwsSdkConfig;

/// Implements AWS EC2 autoscaling manager.
#[derive(Debug, Clone)]
pub struct Manager {
    #[allow(dead_code)]
    shared_config: AwsSdkConfig,
    cli: Client,
}

impl Manager {
    pub fn new(shared_config: &AwsSdkConfig) -> Self {
        let cloned = shared_config.clone();
        let cli = Client::new(shared_config);
        Self {
            shared_config: cloned,
            cli,
        }
    }

    pub fn client(&self) -> Client {
        self.cli.clone()
    }

    /// Sets the instance health: "Healthy" or "Unhealthy".
    pub async fn set_instance_health(&self, instance_id: &str, status: &str) -> Result<()> {
        log::info!(
            "setting instance health for '{}' with {}",
            instance_id,
            status
        );
        let ret = self
            .cli
            .set_instance_health()
            .instance_id(instance_id)
            .health_status(status)
            .send()
            .await;
        let resp = match ret {
            Ok(v) => v,
            Err(e) => {
                return Err(API {
                    message: format!("failed set_instance_health {:?}", e),
                    is_retryable: is_error_retryable(&e)
                        || is_error_retryable_set_instance_health(&e),
                });
            }
        };

        log::info!(
            "successfully set instance health for '{}' with {} (output: {:?})",
            instance_id,
            status,
            resp
        );
        Ok(())
    }
}

#[inline]
pub fn is_error_retryable<E>(e: &SdkError<E>) -> bool {
    match e {
        SdkError::TimeoutError(_) | SdkError::ResponseError { .. } => true,
        SdkError::DispatchFailure(e) => e.is_timeout() || e.is_io(),
        _ => false,
    }
}

#[inline]
pub fn is_error_retryable_set_instance_health(e: &SdkError<SetInstanceHealthError>) -> bool {
    match e {
        SdkError::ServiceError { err, .. } => {
            matches!(
                err.kind,
                SetInstanceHealthErrorKind::ResourceContentionFault(_)
            )
        }
        _ => false,
    }
}