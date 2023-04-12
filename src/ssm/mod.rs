use crate::errors::{Error, Result};
use aws_sdk_ssm::{types::CommandInvocationStatus, Client};
use aws_smithy_client::SdkError;
use aws_types::SdkConfig as AwsSdkConfig;
use tokio::time::{sleep, Duration, Instant};

/// Implements AWS SSM manager.
#[derive(Debug, Clone)]
pub struct Manager {
    pub region: String,
    pub cli: Client,
}

impl Manager {
    pub fn new(shared_config: &AwsSdkConfig) -> Self {
        Self {
            region: shared_config.region().unwrap().to_string(),
            cli: Client::new(shared_config),
        }
    }

    /// Polls SSM command status.
    /// ref. <https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_GetCommandInvocation.html>
    pub async fn poll_command(
        &self,
        command_id: &str,
        instance_id: &str,
        desired_status: CommandInvocationStatus,
        timeout: Duration,
        interval: Duration,
    ) -> Result<CommandInvocationStatus> {
        log::info!(
            "polling invocation status for command '{command_id}' and instance id '{instance_id}' with desired status {:?} for timeout {:?} and interval {:?}",
            desired_status,
            timeout,
            interval,
        );

        let start = Instant::now();
        let mut cnt: u128 = 0;
        loop {
            let elapsed = start.elapsed();
            if elapsed.gt(&timeout) {
                break;
            }

            let itv = {
                if cnt == 0 {
                    // first poll with no wait
                    Duration::from_secs(1)
                } else {
                    interval
                }
            };
            sleep(itv).await;

            let ret = self
                .cli
                .get_command_invocation()
                .command_id(command_id)
                .instance_id(instance_id)
                .send()
                .await;
            let out = match ret {
                Ok(v) => v,
                Err(e) => {
                    return Err(Error::API {
                        message: format!("failed get_command_invocation {:?}", e),
                        retryable: is_err_retryable(&e),
                    });
                }
            };

            let current_status = out.status().unwrap();
            log::info!(
                "poll (current command status {:?}, elapsed {:?})",
                current_status,
                elapsed
            );

            if desired_status.ne(&CommandInvocationStatus::Failed)
                && current_status.eq(&CommandInvocationStatus::Failed)
            {
                return Err(Error::Other {
                    message: String::from("command invocation failed"),
                    retryable: false,
                });
            }

            if current_status.eq(&desired_status) {
                return Ok(current_status.clone());
            }

            cnt += 1;
        }

        Err(Error::Other {
            message: format!("failed to get command invocation {} in time", command_id),
            retryable: true,
        })
    }
}

#[inline]
pub fn is_err_retryable<E>(e: &SdkError<E>) -> bool {
    match e {
        SdkError::TimeoutError(_) | SdkError::ResponseError { .. } => true,
        SdkError::DispatchFailure(e) => e.is_timeout() || e.is_io(),
        _ => false,
    }
}
