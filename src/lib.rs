use async_trait::async_trait;
use task_orchestrator::{
    BackgroundTask, EventBus, EventDrivenTask, EventType, Scheduler, SchedulerError,
};

#[derive(thiserror::Error, Debug)]
pub enum TradingEngineError {
    #[error("Scheduler Error: {0}")]
    SchedulerError(#[from] SchedulerError),
    #[error("Missing required component: {0}")]
    MissingComponent(&'static str),
}

#[async_trait]
pub trait Connector: BackgroundTask {}

#[async_trait]
pub trait Aggregator: BackgroundTask {}

#[async_trait]
pub trait Strategy<E: EventType + 'static>: EventDrivenTask<E> {}

#[async_trait]
pub trait Executor<E: EventType + 'static>: EventDrivenTask<E> {}

pub struct TradingEngineBuilder<E> {
    connectors: Vec<Box<dyn Connector>>,
    aggregator: Option<Box<dyn Aggregator>>,
    strategies: Vec<Box<dyn Strategy<E>>>,
    executors: Vec<Box<dyn Executor<E>>>,
    scheduler: Scheduler<E>,
}

impl<E: EventType + 'static + ToString> TradingEngineBuilder<E> {
    pub fn new(event_bus: EventBus<E>) -> Self {
        Self {
            connectors: Vec::new(),
            aggregator: None,
            strategies: Vec::new(),
            executors: Vec::new(),
            scheduler: Scheduler::new(event_bus),
        }
    }

    pub fn with_connector(mut self, connector: Box<dyn Connector>) -> Self {
        self.scheduler
            .register_background_task(connector.clone_box());
        self.connectors.push(connector);
        self
    }

    pub fn with_aggregator(mut self, aggregator: Box<dyn Aggregator>) -> Self {
        self.scheduler
            .register_background_task(aggregator.clone_box());
        self.aggregator = Some(aggregator);
        self
    }

    pub fn with_strategy(mut self, strategy: Box<dyn Strategy<E>>) -> Self {
        self.scheduler
            .register_event_driven_task(strategy.clone_box());
        self.strategies.push(strategy);
        self
    }

    pub fn with_executor(mut self, executor: Box<dyn Executor<E>>) -> Self {
        self.scheduler
            .register_event_driven_task(executor.clone_box());
        self.executors.push(executor);
        self
    }

    pub fn build(self) -> Result<TradingEngine<E>, TradingEngineError> {
        if self.connectors.is_empty() {
            return Err(TradingEngineError::MissingComponent(
                "at least one connector is required",
            ));
        }
        if self.strategies.is_empty() {
            return Err(TradingEngineError::MissingComponent(
                "at least one strategy is required",
            ));
        }
        if self.executors.is_empty() {
            return Err(TradingEngineError::MissingComponent(
                "at least one executor is required",
            ));
        }

        Ok(TradingEngine {
            connectors: self.connectors,
            aggregator: self
                .aggregator
                .ok_or(TradingEngineError::MissingComponent("aggregator"))?,
            strategies: self.strategies,
            executors: self.executors,
            scheduler: self.scheduler,
        })
    }
}

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
