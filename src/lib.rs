use async_trait::async_trait;
use task_orchestrator::{BackgroundTask, EventDrivenTask, EventType, Scheduler, SchedulerError};

#[derive(thiserror::Error, Debug)]
pub enum TradingEngineError {
    #[error("Scheduler Error: {0}")]
    SchedulerError(#[from] SchedulerError),
}

#[async_trait]
pub trait Connector: BackgroundTask {}

#[async_trait]
pub trait Aggregator: BackgroundTask {}

#[async_trait]
pub trait Strategy<E: EventType + 'static>: EventDrivenTask<E> {}

#[async_trait]
pub trait Executor<E: EventType + 'static>: EventDrivenTask<E> {}

pub struct TradingEngine<E> {
    connectors: Vec<Box<dyn Connector>>,
    aggregator: Box<dyn Aggregator>,
    strategies: Vec<Box<dyn Strategy<E>>>,
    executors: Vec<Box<dyn Executor<E>>>,
    scheduler: Scheduler<E>,
}

impl<E: EventType + 'static + ToString> TradingEngine<E> {
    pub async fn start(&mut self) -> Result<(), TradingEngineError> {
        self.scheduler.start().await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), TradingEngineError> {
        self.scheduler.shutdown().await?;
        Ok(())
    }
}
