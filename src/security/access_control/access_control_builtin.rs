use crate::{
  dds::qos::QosPolicies,
  discovery::data_types::topic_data::TopicBuiltinTopicData,
  security::{authentication::*, *},
};
use super::*;

// A struct implementing the built-in Access control plugin
// See sections 8.4 and 9.4 of the Security specification (v. 1.1)
pub struct AccessControlBuiltIn {
  todo: String,
}

impl AccessControl for AccessControlBuiltIn {
  fn validate_local_permissions(
    &self,
    auth_plugin: &impl Authentication,
    identity: IdentityHandle,
    domain_id: u16,
    participant_qos: &QosPolicies,
  ) -> SecurityResult<PermissionsHandle> {
    todo!();
  }

  /// validate_remote_permissions: section 8.4.2.9.2 of the Security
  /// specification
  fn validate_remote_permissions(
    &self,
    auth_plugin: &impl Authentication,
    local_identity_handle: IdentityHandle,
    remote_identity_handle: IdentityHandle,
    remote_permissions_token: PermissionsToken,
    remote_credential_token: AuthenticatedPeerCredentialToken,
  ) -> SecurityResult<PermissionsHandle> {
    todo!();
  }

  /// check_create_participant: section 8.4.2.9.3 of the Security
  /// specification
  fn check_create_participant(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    qos: &QosPolicies,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_create_datawriter: section 8.4.2.9.4 of the Security
  /// specification. The parameters partition and data_tag have been left out,
  /// since RustDDS does not yet support PartitionQoS or data tagging
  fn check_create_datawriter(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    topic_name: String,
    qos: &QosPolicies,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_create_datareader: section 8.4.2.9.5 of the Security
  /// specification. The parameters partition and data_tag have been left out,
  /// since RustDDS does not yet support PartitionQoS or data tagging
  fn check_create_datareader(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    topic_name: String,
    qos: &QosPolicies,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_create_topic: section 8.4.2.9.6 of the Security
  /// specification
  fn check_create_topic(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    topic_name: String,
    qos: &QosPolicies,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_local_datawriter_register_instance: section 8.4.2.9.7 of the
  /// Security specification.
  /// The function signature is not complete yet.
  fn check_local_datawriter_register_instance(
    &self,
    permissions_handle: PermissionsHandle,
    writer_todo: (),
    key_todo: (),
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_local_datawriter_register_instance: section 8.4.2.9.8 of the
  /// Security specification.
  /// The function signature is not complete yet.
  fn check_local_datawriter_dispose_instance(
    &self,
    permissions_handle: PermissionsHandle,
    writer_todo: (),
    key_todo: (),
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_participant: section 8.4.2.9.9 of the Security
  /// specification.
  fn check_remote_participant(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    participant_data: &ParticipantBuiltinTopicDataSecure,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_datawriter: section 8.4.2.9.10 of the Security
  /// specification.
  fn check_remote_datawriter(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    publication_data: &PublicationBuiltinTopicDataSecure,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_datareader: section 8.4.2.9.11 of the Security
  /// specification.
  fn check_remote_datareader(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    subscription_data: &SubscriptionBuiltinTopicDataSecure,
    relay_only: &mut bool,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_topic: section 8.4.2.9.12 of the Security
  /// specification.
  fn check_remote_topic(
    &self,
    permissions_handle: PermissionsHandle,
    domain_id: u16,
    topic_data: &TopicBuiltinTopicData,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_local_datawriter_match: section 8.4.2.9.13 of the Security
  /// specification.
  fn check_local_datawriter_match(
    &self,
    writer_permissions_handle: PermissionsHandle,
    reader_permissions_handle: PermissionsHandle,
    publication_data: &PublicationBuiltinTopicDataSecure,
    subscription_data: &SubscriptionBuiltinTopicDataSecure,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_local_datareader_match: section 8.4.2.9.14 of the Security
  /// specification.
  /// The parameter subscriber_partition is ommitted since RustDDS does not yet
  /// support PartitionQoS.
  fn check_local_datareader_match(
    &self,
    reader_permissions_handle: PermissionsHandle,
    writer_permissions_handle: PermissionsHandle,
    subscription_data: &SubscriptionBuiltinTopicDataSecure,
    publication_data: &PublicationBuiltinTopicDataSecure,
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_datawriter_register_instance: section 8.4.2.9.15 of the
  /// Security specification.
  /// TODO: The function signature is not complete yet.
  fn check_remote_datawriter_register_instance(
    &self,
    permissions_handle: PermissionsHandle,
    reader_todo: (),
    publication_handle_todo: (),
    key_todo: (),
    instance_handle_todo: (),
  ) -> SecurityResult<()> {
    todo!();
  }

  /// check_remote_datawriter_dispose_instance: section 8.4.2.9.16 of the
  /// Security specification.
  /// TODO: The function signature is not complete yet.
  fn check_remote_datawriter_dispose_instance(
    &self,
    permissions_handle: PermissionsHandle,
    reader_todo: (),
    publication_handle_todo: (),
    key_todo: (),
  ) -> SecurityResult<()> {
    todo!();
  }

  /// get_permissions_token: section 8.4.2.9.17 of the Security
  /// specification.
  fn get_permissions_token(&self, handle: PermissionsHandle) -> SecurityResult<PermissionsToken> {
    todo!();
  }

  /// get_permissions_credential_token: section 8.4.2.9.18 of the Security
  /// specification.
  fn get_permissions_credential_token(
    &self,
    handle: PermissionsHandle,
  ) -> SecurityResult<PermissionsCredentialToken> {
    todo!();
  }

  /// set_listener: section 8.4.2.9.19 of the Security
  /// specification.
  /// TODO: we do not need this as listeners are not used in RustDDS, but which
  /// async mechanism to use?
  fn set_listener(&self) -> SecurityResult<()> {
    todo!();
  }

  /// get_participant_sec_attributes: section 8.4.2.9.22 of the Security
  /// specification.
  fn get_participant_sec_attributes(
    &self,
    permissions_handle: PermissionsHandle,
  ) -> SecurityResult<ParticipantSecurityAttributes> {
    todo!();
  }

  /// get_topic_sec_attributes: section 8.4.2.9.23 of the Security
  /// specification.
  fn get_topic_sec_attributes(
    &self,
    permissions_handle: PermissionsHandle,
    topic_name: String,
  ) -> SecurityResult<EndpointSecurityAttributes> {
    todo!();
  }

  /// get_datawriter_sec_attributes: section 8.4.2.9.24 of the Security
  /// specification.
  /// The parameters partition and data_tag have been left out,
  /// since RustDDS does not yet support PartitionQoS or data tagging
  fn get_datawriter_sec_attributes(
    &self,
    permissions_handle: PermissionsHandle,
    topic_name: String,
  ) -> SecurityResult<EndpointSecurityAttributes> {
    todo!();
  }

  /// get_datareader_sec_attributes: section 8.4.2.9.25 of the Security
  /// specification.
  /// The parameters partition and data_tag have been left out,
  /// since RustDDS does not yet support PartitionQoS or data tagging
  fn get_datareader_sec_attributes(
    &self,
    permissions_handle: PermissionsHandle,
    topic_name: String,
  ) -> SecurityResult<EndpointSecurityAttributes> {
    todo!();
  }
}
