use crate::{
  dds::qos::{
    QosPolicies, policy::Deadline, policy::DestinationOrder, policy::Durability, policy::History,
    policy::LatencyBudget, policy::Lifespan, policy::Liveliness, policy::LivelinessKind,
    policy::Ownership, policy::Reliability,
  },
  structure::duration::Duration,
};

use std::time::Duration as StdDuration;

pub struct ROSDiscoveryTopic {}

impl ROSDiscoveryTopic {
  const QOS: QosPolicies = QosPolicies {
    durability: Some(Durability::TransientLocal),
    presentation: None,
    deadline: Some(Deadline {
      period: Duration::DURATION_INFINITE,
    }),
    latency_budget: Some(LatencyBudget {
      duration: Duration::DURATION_ZERO,
    }),
    ownership: Some(Ownership::Shared),
    liveliness: Some(Liveliness {
      kind: LivelinessKind::Automatic,
      lease_duration: Duration::DURATION_INFINITE,
    }),
    time_based_filter: None,
    reliability: Some(Reliability::Reliable {
      max_blocking_time: Duration::DURATION_ZERO,
    }),
    destination_order: Some(DestinationOrder::ByReceptionTimestamp),
    history: Some(History::KeepLast { depth: 1 }),
    resource_limits: None,
    lifespan: Some(Lifespan {
      duration: Duration::DURATION_INFINITE,
    }),
  };

  const TOPIC_NAME: &'static str = "ros_discovery_info";
  const TYPE_NAME: &'static str = "rmw_dds_common::msg::dds_::ParticipantEntitiesInfo_";

  pub fn topic_name() -> &'static str {
    ROSDiscoveryTopic::TOPIC_NAME
  }

  pub fn type_name() -> &'static str {
    ROSDiscoveryTopic::TYPE_NAME
  }

  pub const fn get_qos() -> QosPolicies {
    ROSDiscoveryTopic::QOS
  }
}

pub struct ParameterEventsTopic {}

impl ParameterEventsTopic {
  const QOS: QosPolicies = QosPolicies {
    durability: Some(Durability::TransientLocal),
    presentation: None,
    deadline: None,
    latency_budget: None,
    ownership: None,
    liveliness: None,
    time_based_filter: None,
    reliability: Some(Reliability::Reliable {
      max_blocking_time: Duration::DURATION_ZERO,
    }),
    destination_order: None,
    history: Some(History::KeepLast { depth: 1 }),
    resource_limits: None,
    lifespan: None,
  };

  const TOPIC_NAME: &'static str = "rt/parameter_events";
  const TYPE_NAME: &'static str = "rcl_interfaces::msg::dds_::ParameterEvent_";

  pub fn topic_name() -> &'static str {
    ParameterEventsTopic::TOPIC_NAME
  }

  pub fn type_name() -> &'static str {
    ParameterEventsTopic::TYPE_NAME
  }

  pub fn get_qos() -> QosPolicies {
    ParameterEventsTopic::QOS
  }
}

pub struct RosOutTopic {}

impl RosOutTopic {
  const QOS: QosPolicies = QosPolicies {
    durability: Some(Durability::TransientLocal),
    presentation: None,
    deadline: Some(Deadline {
      period: Duration::DURATION_INFINITE,
    }),
    latency_budget: Some(LatencyBudget {
      duration: Duration::DURATION_ZERO,
    }),
    ownership: Some(Ownership::Shared),
    liveliness: Some(Liveliness {
      kind: LivelinessKind::Automatic,
      lease_duration: Duration::DURATION_INFINITE,
    }),
    time_based_filter: None,
    reliability: Some(Reliability::Reliable {
      max_blocking_time: Duration::DURATION_ZERO,
    }),
    destination_order: Some(DestinationOrder::ByReceptionTimestamp),
    history: Some(History::KeepLast { depth: 1 }),
    resource_limits: None,
    lifespan: Some(Lifespan {
      duration: Duration::from_secs(10),
    }),
  };

  const TOPIC_NAME: &'static str = "rt/rosout";
  const TYPE_NAME: &'static str = "rcl_interfaces::msg::dds_::Log_";

  pub fn topic_name() -> &'static str {
    RosOutTopic::TOPIC_NAME
  }

  pub fn type_name() -> &'static str {
    RosOutTopic::TYPE_NAME
  }

  pub fn get_qos() -> QosPolicies {
    RosOutTopic::QOS
  }
}
