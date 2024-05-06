use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};

use log::warn;

use crate::error::Error;

const DEFINITIONS: &[u8] = b"definitions";
const PROCESS: &[u8] = b"process";

// Event
const START_EVENT: &[u8] = b"startEvent";
const END_EVENT: &[u8] = b"endEvent";
const BOUNDARY_EVENT: &[u8] = b"boundaryEvent";
const INTERMEDIATE_CATCH_EVENT: &[u8] = b"intermediateCatchEvent";
const INTERMEDIATE_THROW_EVENT: &[u8] = b"intermediateThrowEvent";

// Event symbol
const ERROR_EVENT_DEFINITION: &[u8] = b"errorEventDefinition";
const MESSAGE_EVENT_DEFINITION: &[u8] = b"messageEventDefinition";
const TIMER_EVENT_DEFINITION: &[u8] = b"timerEventDefinition";
const ESCALATION_EVENT_DEFINITION: &[u8] = b"escalationEventDefinition";
const CONDITIONAL_EVENT_DEFINITION: &[u8] = b"conditionalEventDefinition";
const SIGNAL_EVENT_DEFINITION: &[u8] = b"signalEventDefinition";
const COMPENSATE_EVENT_DEFINITION: &[u8] = b"compensateEventDefinition";

// Task
const TASK: &[u8] = b"task";
const SERVICE_TASK: &[u8] = b"serviceTask";
const USER_TASK: &[u8] = b"userTask";
const SCRIPT_TASK: &[u8] = b"scriptTask";
const RECEIVE_TASK: &[u8] = b"receiveTask";
const SEND_TASK: &[u8] = b"sendTask";
const MANUAL_TASK: &[u8] = b"manualTask";
const BUSINESS_RULE_TASK: &[u8] = b"businessRuleTask";
const CALL_ACTIVITY: &[u8] = b"callActivity";
const SUB_PROCESS: &[u8] = b"subProcess";
const TRANSACTION: &[u8] = b"transaction";

// Direction
const OUTGOING: &[u8] = b"outgoing";
const INCOMING: &[u8] = b"incoming";

// Flow
const SEQUENCE_FLOW: &[u8] = b"sequenceFlow";

// Gateway
const EXCLUSIVE_GATEWAY: &[u8] = b"exclusiveGateway";
const PARALLEL_GATEWAY: &[u8] = b"parallelGateway";
const INCLUSIVE_GATEWAY: &[u8] = b"inclusiveGateway";

// Attributes
const ATTRIB_ID: &[u8] = b"id";
const ATTRIB_IS_EXECUTABLE: &[u8] = b"isExecutable";
const ATTRIB_NAME: &[u8] = b"name";
const ATTRIB_SOURCE_REF: &[u8] = b"sourceRef";
const ATTRIB_TARGET_REF: &[u8] = b"targetRef";
const ATTRIB_DEFAULT: &[u8] = b"default";
const ATTRIB_EXPORTER_VERSION: &[u8] = b"exporterVersion";
const ATTRIB_ATTACHED_TO_REF: &[u8] = b"attachedToRef";
const ATTRIB_CANCEL_ACTIVITY: &[u8] = b"cancelActivity";

// Messages
const MISSING_FUNCTION: &str = "Missing function. Please register";

pub type Data<T> = Arc<Mutex<T>>;
pub type TaskCallback<T> = fn(Data<T>) -> Result<(), Symbol>;
pub type GatewayCallback<T> = fn(Data<T>) -> Vec<&'static str>;

#[derive(Debug)]
pub struct Eventhandler<T> {
    task_func: HashMap<String, TaskCallback<T>>,
    gateway_func: HashMap<String, GatewayCallback<T>>,
}

impl<T> Default for Eventhandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Eventhandler<T> {
    pub fn new() -> Self {
        Self {
            task_func: Default::default(),
            gateway_func: Default::default(),
        }
    }

    pub fn add_task(&mut self, name: impl Into<String>, func: TaskCallback<T>) {
        self.task_func.insert(name.into(), func);
    }

    pub fn run_task(&self, key: &str, data: Data<T>) -> Result<(), Symbol> {
        if let Some(func) = self.task_func.get(key) {
            return (*func)(data);
        } else {
            warn!("{}: {}", MISSING_FUNCTION, key);
        }
        Ok(())
    }

    pub fn add_gateway(&mut self, name: impl Into<String>, func: GatewayCallback<T>) {
        self.gateway_func.insert(name.into(), func);
    }

    pub fn run_gateway(&self, key: &str, data: Data<T>) -> Vec<&'static str> {
        if let Some(func) = self.gateway_func.get(key) {
            return (*func)(data);
        }
        warn!("{}: {}", MISSING_FUNCTION, key);
        vec![]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BpmnType {
    StartEvent,
    EndEvent,
    BoundaryEvent,
    IntermediateCatchEvent,
    IntermediateThrowEvent,
    Task,
    Outgoing,
    Incoming,
    ExclusiveGateway,
    Process,
    Definitions,
    SequenceFlow,
    SubProcess,
    ParallelGateway,
    InclusiveGateway,
    ErrorEventDefinition,
    MessageEventDefinition,
    TimerEventDefinition,
    Ignore,
    EscalationEventDefinition,
    ConditionalEventDefinition,
    SignalEventDefinition,
    CompensateEventDefinition,
}

impl Display for BpmnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&[u8]> for BpmnType {
    fn from(value: &[u8]) -> Self {
        match value {
            START_EVENT => BpmnType::StartEvent,
            END_EVENT => BpmnType::EndEvent,
            BOUNDARY_EVENT => BpmnType::BoundaryEvent,
            INTERMEDIATE_CATCH_EVENT => BpmnType::IntermediateCatchEvent,
            INTERMEDIATE_THROW_EVENT => BpmnType::IntermediateThrowEvent,
            TASK | SCRIPT_TASK | USER_TASK | SERVICE_TASK | CALL_ACTIVITY | RECEIVE_TASK
            | SEND_TASK | MANUAL_TASK | BUSINESS_RULE_TASK => BpmnType::Task,
            OUTGOING => BpmnType::Outgoing,
            INCOMING => BpmnType::Incoming,
            EXCLUSIVE_GATEWAY => BpmnType::ExclusiveGateway,
            PARALLEL_GATEWAY => BpmnType::ParallelGateway,
            INCLUSIVE_GATEWAY => BpmnType::InclusiveGateway,
            SEQUENCE_FLOW => BpmnType::SequenceFlow,
            PROCESS => BpmnType::Process,
            DEFINITIONS => BpmnType::Definitions,
            SUB_PROCESS | TRANSACTION => BpmnType::SubProcess,
            ERROR_EVENT_DEFINITION => BpmnType::ErrorEventDefinition,
            MESSAGE_EVENT_DEFINITION => BpmnType::MessageEventDefinition,
            TIMER_EVENT_DEFINITION => BpmnType::TimerEventDefinition,
            ESCALATION_EVENT_DEFINITION => BpmnType::EscalationEventDefinition,
            CONDITIONAL_EVENT_DEFINITION => BpmnType::ConditionalEventDefinition,
            SIGNAL_EVENT_DEFINITION => BpmnType::SignalEventDefinition,
            COMPENSATE_EVENT_DEFINITION => BpmnType::CompensateEventDefinition,
            _ => BpmnType::Ignore,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum BpmnAttrib {
    Id,
    IsExecutable,
    Name,
    SourceRef,
    TargetRef,
    Default,
    ExporterVersion,
    AttachedToRef,
    CancelActivity,
    Ignore,
}

impl From<&[u8]> for BpmnAttrib {
    fn from(value: &[u8]) -> Self {
        match value {
            ATTRIB_ID => BpmnAttrib::Id,
            ATTRIB_IS_EXECUTABLE => BpmnAttrib::IsExecutable,
            ATTRIB_NAME => BpmnAttrib::Name,
            ATTRIB_SOURCE_REF => BpmnAttrib::SourceRef,
            ATTRIB_TARGET_REF => BpmnAttrib::TargetRef,
            ATTRIB_DEFAULT => BpmnAttrib::Default,
            ATTRIB_EXPORTER_VERSION => BpmnAttrib::ExporterVersion,
            ATTRIB_ATTACHED_TO_REF => BpmnAttrib::AttachedToRef,
            ATTRIB_CANCEL_ACTIVITY => BpmnAttrib::CancelActivity,
            _ => BpmnAttrib::Ignore,
        }
    }
}

impl TryFrom<(BpmnType, HashMap<BpmnAttrib, String>)> for Bpmn {
    type Error = Error;

    fn try_from(
        (bpmn_type, mut attributes): (BpmnType, HashMap<BpmnAttrib, String>),
    ) -> Result<Self, Self::Error> {
        let ty = match bpmn_type {
            BpmnType::Process => Bpmn::Process {
                id: attributes
                    .remove(&BpmnAttrib::Id)
                    .ok_or(Error::MissingId(bpmn_type.to_string()))?,
                is_executable: attributes
                    .remove(&BpmnAttrib::IsExecutable)
                    .and_then(|s| s.parse::<bool>().ok())
                    .unwrap_or_default(),
            },
            BpmnType::StartEvent | BpmnType::EndEvent | BpmnType::BoundaryEvent => Bpmn::Event {
                event: bpmn_type.clone(),
                symbol: None,
                id: attributes
                    .remove(&BpmnAttrib::Id)
                    .ok_or(Error::MissingId(bpmn_type.to_string()))?,
                name: attributes.remove(&BpmnAttrib::Name),
                attached_to_ref: attributes.remove(&BpmnAttrib::AttachedToRef),
                cancel_activity: attributes.remove(&BpmnAttrib::CancelActivity),
                output: None,
            },
            BpmnType::Task | BpmnType::SubProcess => Bpmn::Activity {
                aktivity: bpmn_type.clone(),
                id: attributes
                    .remove(&BpmnAttrib::Id)
                    .ok_or(Error::MissingId(bpmn_type.to_string()))?,
                name: attributes.remove(&BpmnAttrib::Name),
                output: None,
            },
            BpmnType::ExclusiveGateway | BpmnType::ParallelGateway | BpmnType::InclusiveGateway => {
                Bpmn::Gateway {
                    gateway: bpmn_type.clone(),
                    id: attributes
                        .remove(&BpmnAttrib::Id)
                        .ok_or(Error::MissingId(bpmn_type.to_string()))?,
                    name: attributes.remove(&BpmnAttrib::Name),
                    default: attributes.remove(&BpmnAttrib::Default),
                    outputs: Default::default(),
                }
            }
            BpmnType::SequenceFlow => Bpmn::SequenceFlow {
                id: attributes
                    .remove(&BpmnAttrib::Id)
                    .ok_or(Error::MissingId(bpmn_type.to_string()))?,
                name: attributes.remove(&BpmnAttrib::Name),
                source_ref: attributes
                    .remove(&BpmnAttrib::SourceRef)
                    .ok_or(Error::MissingSourceRef)?,
                target_ref: attributes
                    .remove(&BpmnAttrib::TargetRef)
                    .ok_or(Error::MissingTargetRef)?,
            },
            BpmnType::Incoming | BpmnType::Outgoing => Bpmn::Direction {
                direction: bpmn_type,
                text: None,
            },
            _ => return Err(Error::BadDiagramType),
        };
        Ok(ty)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Bpmn {
    Process {
        id: String,
        is_executable: bool,
    },
    Event {
        event: BpmnType,
        symbol: Option<Symbol>,
        id: String,
        name: Option<String>,
        attached_to_ref: Option<String>,
        cancel_activity: Option<String>,
        output: Option<String>,
    },
    Activity {
        aktivity: BpmnType,
        id: String,
        name: Option<String>,
        output: Option<String>,
    },
    Gateway {
        gateway: BpmnType,
        id: String,
        name: Option<String>,
        default: Option<String>,
        outputs: Vec<String>,
    },
    SequenceFlow {
        id: String,
        name: Option<String>,
        source_ref: String,
        target_ref: String,
    },
    Direction {
        direction: BpmnType,
        text: Option<String>,
    },
}

impl Bpmn {
    pub(crate) fn id(&self) -> Option<&String> {
        match self {
            Bpmn::Process {
                id,
                is_executable: _,
            }
            | Bpmn::Event {
                event: _,
                symbol: _,
                id,
                name: _,
                attached_to_ref: _,
                cancel_activity: _,
                output: _,
            }
            | Bpmn::Activity {
                aktivity: _,
                id,
                name: _,
                output: _,
            }
            | Bpmn::Gateway {
                gateway: _,
                id,
                name: _,
                default: _,
                outputs: _,
            }
            | Bpmn::SequenceFlow {
                id,
                name: _,
                source_ref: _,
                target_ref: _,
            } => Some(id),
            Bpmn::Direction {
                direction: _,
                text: _,
            } => None,
        }
    }

    pub(crate) fn name(&self) -> Option<&String> {
        match self {
            Bpmn::Event {
                event: _,
                symbol: _,
                id: _,
                name,
                attached_to_ref: _,
                cancel_activity: _,
                output: _,
            }
            | Bpmn::Activity {
                aktivity: _,
                id: _,
                name,
                output: _,
            }
            | Bpmn::Gateway {
                gateway: _,
                id: _,
                name,
                default: _,
                outputs: _,
            }
            | Bpmn::SequenceFlow {
                id: _,
                name,
                source_ref: _,
                target_ref: _,
            } => name.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn set_output(&mut self, text: String) {
        match self {
            Bpmn::Event {
                event: _,
                symbol: _,
                id: _,
                name: _,
                attached_to_ref: _,
                cancel_activity: _,
                output,
            }
            | Bpmn::Activity {
                aktivity: _,
                id: _,
                name: _,
                output,
            } => *output = Some(text),
            Bpmn::Gateway {
                gateway: _,
                id: _,
                name: _,
                default: _,
                outputs,
            } => outputs.push(text),
            _ => {}
        }
    }
}

impl From<&Bpmn> for BpmnType {
    fn from(value: &Bpmn) -> Self {
        match value {
            Bpmn::Process {
                id: _,
                is_executable: _,
            } => BpmnType::Process,
            Bpmn::Event {
                event,
                symbol: _,
                id: _,
                name: _,
                attached_to_ref: _,
                cancel_activity: _,
                output: _,
            } => event.clone(),
            Bpmn::Activity {
                aktivity,
                id: _,
                name: _,
                output: _,
            } => aktivity.clone(),
            Bpmn::Gateway {
                gateway,
                id: _,
                name: _,
                default: _,
                outputs: _,
            } => gateway.clone(),
            Bpmn::SequenceFlow {
                id: _,
                name: _,
                source_ref: _,
                target_ref: _,
            } => BpmnType::SequenceFlow,
            Bpmn::Direction { direction, text: _ } => direction.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub enum Symbol {
    #[default]
    Blank,
    Message,
    Timer,
    Escalation,
    Conditional,
    Link,
    Error,
    Cancel,
    Compensation,
    Signal,
    Multiple,
    ParallelMultiple,
    Terminate,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<BpmnType> for Symbol {
    type Error = Error;

    fn try_from(value: BpmnType) -> Result<Self, <Symbol as TryFrom<BpmnType>>::Error> {
        let ty = match value {
            BpmnType::MessageEventDefinition => Symbol::Message,
            BpmnType::TimerEventDefinition => Symbol::Timer,
            BpmnType::EscalationEventDefinition => Symbol::Escalation,
            BpmnType::ConditionalEventDefinition => Symbol::Conditional,
            BpmnType::ErrorEventDefinition => Symbol::Error,
            BpmnType::CompensateEventDefinition => Symbol::Compensation,
            BpmnType::SignalEventDefinition => Symbol::Signal,
            _ => return Err(Error::BadSymbolType),
        };
        Ok(ty)
    }
}
