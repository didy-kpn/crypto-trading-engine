use std::sync::Arc;

use async_trait::async_trait;
use task_orchestrator::{
    BackgroundTask, EventBus, EventTask, EventType, Scheduler, SchedulerError, TaskId,
};
use tokio::sync::Mutex;

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
pub trait Strategy<E: EventType + 'static>: EventTask<E> {}

#[async_trait]
pub trait Executor<E: EventType + 'static>: EventTask<E> {}

pub struct TradingEngineBuilder<E> {
    connectors: Vec<TaskId>,
    aggregator: Option<TaskId>,
    strategies: Vec<TaskId>,
    executors: Vec<TaskId>,
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

    pub fn with_connector<T: Connector + 'static>(mut self, connector: Arc<Mutex<T>>) -> Self {
        let background_task: Arc<Mutex<dyn BackgroundTask>> = connector;
        let id = self.scheduler.register_background_task(background_task);
        self.connectors.push(id);
        self
    }

    pub fn with_aggregator<T: Aggregator + 'static>(mut self, aggregator: Arc<Mutex<T>>) -> Self {
        let background_task: Arc<Mutex<dyn BackgroundTask>> = aggregator;
        let id = self.scheduler.register_background_task(background_task);
        self.aggregator = Some(id);
        self
    }

    pub fn with_strategy<T: Strategy<E> + 'static>(mut self, strategy: Arc<Mutex<T>>) -> Self {
        let event_task: Arc<Mutex<dyn EventTask<E>>> = strategy;
        let id = self.scheduler.register_event_task(event_task);
        self.strategies.push(id);
        self
    }

    pub fn with_executor<T: Executor<E> + 'static>(mut self, executor: Arc<Mutex<T>>) -> Self {
        let event_task: Arc<Mutex<dyn EventTask<E>>> = executor;
        let id = self.scheduler.register_event_task(event_task);
        self.executors.push(id);
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
    connectors: Vec<TaskId>,
    aggregator: TaskId,
    strategies: Vec<TaskId>,
    executors: Vec<TaskId>,
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
