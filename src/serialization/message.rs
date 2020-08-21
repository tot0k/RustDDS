use crate::{
  structure::{sequence_number::SequenceNumber, guid::GuidPrefix},
  serialization::submessage::SubMessage,
  submessages::{
    SubmessageKind, SubmessageHeader, Data, InfoDestination, InterpreterSubmessage, InfoTimestamp,
    Heartbeat, EntitySubmessage, SubmessageFlag, AckNack, Gap, HeartbeatFrag, InfoReply,
    InfoSource, NackFrag,
  },
  messages::header::Header,
};
use speedy::{Readable, Writable, Endianness, Reader, Context, Writer};

#[derive(Debug)]
pub struct Message {
  header: Header,
  submessages: Vec<SubMessage>,
}

///Purpose of this object is to help RTPS Message SubmessageHeader SubmessageFlag value setting and value reading.
#[derive(Debug, Clone)]
pub struct SubmessageFlagHelper {
  pub KeyFlag : bool,         
  pub DataFlag: bool,         
  pub InlineQosFlag : bool,   
  pub NonStandardPayloadFlag: bool, 
  pub EndiannessFlag : bool,
  pub FinalFlag : bool,        
  pub LivelinessFlag : bool,   
  pub MulticastFlag : bool,    
  pub InvalidateFlag : bool,   
}
impl SubmessageFlagHelper {
  pub fn new(endianness : Endianness) -> SubmessageFlagHelper{
    let mut f = SubmessageFlagHelper {
      KeyFlag : false,        
      DataFlag: false,         
      InlineQosFlag : false,   
      NonStandardPayloadFlag : false,
      EndiannessFlag : false,
      FinalFlag : false,        
      LivelinessFlag : false,   
      MulticastFlag : false,    
      InvalidateFlag : false,   
    };
    if endianness == Endianness::LittleEndian{
      f.EndiannessFlag = true;
    }
    return f;
  }

  pub fn get_endianness(&self) -> Endianness {
    if self.EndiannessFlag == true{
      return Endianness::LittleEndian;
    }else{
      return Endianness::BigEndian;
    }
  }

  ///Meaning of each bit is different depending on the message submessage type.
  ///Flags are u8 long -> possibility of 8 diffenrent flags, but not all are used.
  pub fn get_submessage_flags_helper_from_submessage_flag(submessage_kind : &SubmessageKind, flags : &SubmessageFlag) -> SubmessageFlagHelper{
    let mut helper = SubmessageFlagHelper::new(Endianness::BigEndian);

    match submessage_kind {
      //|X|X|X|N|K|D|Q|E|
      //NonStandardPayloadFlag, Key, DataFlag, InlineQosFlag, EndiannessFlag
      &SubmessageKind::DATA =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.InlineQosFlag = flags.is_flag_set(2u8.pow(1));
        helper.DataFlag = flags.is_flag_set(2u8.pow(2));
        helper.KeyFlag = flags.is_flag_set(2u8.pow(3));
        helper.NonStandardPayloadFlag = flags.is_flag_set(2u8.pow(4));
      }
      //|X|X|X|X|N|K|Q|E|
      //NonStandardPayloadFlag, Key, InlineQosFlag, EndiannessFlag
      &SubmessageKind::DATA_FRAG =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.InlineQosFlag = flags.is_flag_set(2u8.pow(1));
        helper.KeyFlag = flags.is_flag_set(2u8.pow(2));
        helper.NonStandardPayloadFlag = flags.is_flag_set(2u8.pow(3));
      }
      //|X|X|X|X|X|X|F|E|
      //FinalFlag,EndiannessFlag
      &SubmessageKind::ACKNACK =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.FinalFlag = flags.is_flag_set(2u8.pow(1));
      }
      //|X|X|X|X|X|L|F|E|
      //LivelinessFlag,FinalFlag,EndiannessFlag
      &SubmessageKind::HEARTBEAT =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.FinalFlag = flags.is_flag_set(2u8.pow(1));
        helper.LivelinessFlag = flags.is_flag_set(2u8.pow(2));
      }
      //|X|X|X|X|X|X|X|E| 
      //EndiannessFlag
      &SubmessageKind::HEARTBEAT_FRAG =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      //|X|X|X|X|X|X|X|E|
      //EndiannessFlag
      &SubmessageKind::INFO_DST => {
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      //|X|X|X|X|X|X|M|E|
      //MulticastFlag,EndiannessFlag
      &SubmessageKind::INFO_REPLY => {
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.MulticastFlag = flags.is_flag_set(2u8.pow(1));
      }
      //|X|X|X|X|X|X|X|E| 
      //EndiannessFlag
      &SubmessageKind::INFO_SRC => {
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      //|X|X|X|X|X|X|I|E| 
      //InvalidateFlag, EndiannessFlag
      &SubmessageKind::INFO_TS => {
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.InvalidateFlag = flags.is_flag_set(2u8.pow(1));
      }
      //|X|X|X|X|X|X|X|E|
      //EndiannessFlag
      &SubmessageKind::PAD =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      //|X|X|X|X|X|X|X|E|
      //EndiannessFlag
      &SubmessageKind::NACK_FRAG =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      //|X|X|X|X|X|X|M|E| 
      //MulticastFlag,EndiannessFlag
      &SubmessageKind::INFO_REPLY_IP4 =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
        helper.MulticastFlag = flags.is_flag_set(2u8.pow(1));
      } 
      //|X|X|X|X|X|X|X|E| 
      //EndiannessFlag
      &SubmessageKind::GAP =>{
        helper.EndiannessFlag = flags.is_flag_set(2u8.pow(0));
      }
      _ =>{
        todo!();
      }
    }
    return helper;
  }

  pub fn create_submessage_flags_from_flag_helper(submessage_kind : &SubmessageKind, helper : &SubmessageFlagHelper,) -> SubmessageFlag{
    let mut flags = SubmessageFlag{flags: 0b00000000_u8};
   
    match submessage_kind {
      //|X|X|X|N|K|D|Q|E|
      //NonStandardPayloadFlag, Key, DataFlag, InlineQosFlag, EndiannessFlag
      &SubmessageKind::DATA  => { 
        if helper.EndiannessFlag{
          flags.set_flag(2u8.pow(0));
        }
        if helper.InlineQosFlag{
          flags.set_flag(2u8.pow(1));
        }
        if helper.DataFlag {
          flags.set_flag(2u8.pow(2));
        }
        if helper.KeyFlag {
          flags.set_flag(2u8.pow(3));
        }
        if helper.NonStandardPayloadFlag{
          flags.set_flag(2u8.pow(4));
        }
      }
       //|X|X|X|X|N|K|Q|E|
      //NonStandardPayloadFlag, Key, InlineQosFlag, EndiannessFlag
      &SubmessageKind::DATA_FRAG =>{
        if helper.EndiannessFlag{
          flags.set_flag(2u8.pow(0));
        }
        if helper.InlineQosFlag{
          flags.set_flag(2u8.pow(1));
        }
        if helper.KeyFlag {
          flags.set_flag(2u8.pow(2));
        }
        if helper.NonStandardPayloadFlag{
          flags.set_flag(2u8.pow(3));
        }
      }
      //|X|X|X|X|X|X|I|E| 
      //InvalidateFlag, EndiannessFlag
      &SubmessageKind::INFO_TS =>{
        if helper.EndiannessFlag{
          flags.set_flag(2u8.pow(0));
        }
        if helper.InvalidateFlag{
          flags.set_flag(2u8.pow(1));
        }
      }
      //|X|X|X|X|X|L|F|E|
      //LivelinessFlag,FinalFlag,EndiannessFlag
      &SubmessageKind::HEARTBEAT =>{
        if helper.EndiannessFlag{
          flags.set_flag(2u8.pow(0));
        }
        if helper.FinalFlag{
          flags.set_flag(2u8.pow(1));
        }
        if helper.LivelinessFlag {
          flags.set_flag(2u8.pow(2));
        }
      }
       //|X|X|X|X|X|X|X|E|
      //EndiannessFlag
      &SubmessageKind::INFO_DST => {
        if helper.EndiannessFlag{
          flags.set_flag(2u8.pow(0));
        }
      }
      _ => {
        todo!();
      }
    }
    return flags;
  }
}

impl<'a> Message {
  pub fn deserialize_header(context: Endianness, buffer: &'a [u8]) -> Header {
    Header::read_from_buffer_with_ctx(context, buffer).unwrap()
  }

  pub fn serialize_header(self) -> Vec<u8> {
    let buffer = self.header.write_to_vec_with_ctx(Endianness::LittleEndian);
    buffer.unwrap()
  }

  pub fn add_submessage(&mut self, submessage: SubMessage) {
    self.submessages.push(submessage);
  }

  pub fn remove_submessage(mut self, index: usize) {
    self.submessages.remove(index);
  }

  pub fn submessages(self) -> Vec<SubMessage> {
    self.submessages
  }

  fn submessages_borrow(&self) -> &Vec<SubMessage> {
    &self.submessages
  }

  pub fn set_header(&mut self, header: Header) {
    self.header = header;
  }

  

  pub fn get_data_sub_message_sequence_numbers(&self) -> Vec<SequenceNumber> {
    let mut sequence_numbers: Vec<SequenceNumber> = vec![];
    for mes in self.submessages_borrow() {
      if mes.submessage.is_some() {
        let entity_sub_message = mes.submessage.as_ref().unwrap();
        let maybeDataMessage = entity_sub_message.get_data_submessage();
        if maybeDataMessage.is_some() {
          let sequenceNumber = maybeDataMessage.unwrap().writer_sn;
          sequence_numbers.push(sequenceNumber.clone());
        }
      }
    }
    return sequence_numbers;
  }
}

impl Message {
  pub fn new() -> Message {
    Message {
      header: Header::new(GuidPrefix::GUIDPREFIX_UNKNOWN),
      submessages: vec![],
      //interpteterSubmessages : vec![],
    }
  }
}

impl Default for Message {
  fn default() -> Self {
    Message::new()
  }
}

impl<C: Context> Writable<C> for Message {
  fn write_to<'a, T: ?Sized + Writer<C>>(&'a self, writer: &mut T) -> Result<(), C::Error> {
    writer.write_value(&self.header)?;
    for x in &self.submessages {
      writer.write_value(&x)?;
    }
    Ok(())
  }
}

impl<'a, C: Context> Readable<'a, C> for Message {
  fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
    let mut message = Message::default();
    let endianess = reader.endianness();
    // message.header = SubMessage::deserialize_header(C, reader.)
    message.header = reader.read_value()?;

    // TODO FLAGS???
    let flag: SubmessageFlag = SubmessageFlag { flags: 1 };
    loop {
      let res: Result<SubmessageHeader, C::Error> = reader.read_value();
      let subHeader = match res {
        Ok(res) => res,
        Err(_e) => {
          break;
        }
      };

      let mut buffer: Vec<u8> = Vec::new();
      if subHeader.submessage_length == 0 {
        let mut ended = false;
        while !ended {
          let byte = reader.read_u8();
          match byte {
            Ok(val) => buffer.push(val),
            _ => ended = true,
          };
        }
      } else {
        buffer = reader.read_vec(subHeader.submessage_length as usize)?;
      }

      let submessageFlagHelper = SubmessageFlagHelper::get_submessage_flags_helper_from_submessage_flag(&subHeader.submessage_id, &subHeader.flags);

      match subHeader.submessage_id {
        SubmessageKind::DATA => {
          let x = Data::deserialize_data(
            &buffer,
            endianess,
            submessageFlagHelper.InlineQosFlag,
            submessageFlagHelper.DataFlag,
          );
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::Data(x, flag.clone())),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::HEARTBEAT => {
          let x = Heartbeat::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::Heartbeat(x, flag.clone())),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::GAP => {
          let x = Gap::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::Gap(x)),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::HEARTBEAT_FRAG => {
          let x = HeartbeatFrag::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::HeartbeatFrag(x)),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::ACKNACK => {
          let x = AckNack::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::AckNack(x, flag.clone())),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::INFO_DST => {
          let x: InfoDestination =
            InfoDestination::read_from_buffer_with_ctx(endianess, &buffer).unwrap();

          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: None,
            intepreterSubmessage: Some(InterpreterSubmessage::InfoDestination(x)),
          };
          message.add_submessage(y);
        }
        SubmessageKind::INFO_TS => {
          let x: InfoTimestamp =
            InfoTimestamp::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: None,
            intepreterSubmessage: Some(InterpreterSubmessage::InfoTimestamp(x, flag.clone())),
          };
          message.add_submessage(y);
        }
        SubmessageKind::INFO_REPLY => {
          let x: InfoReply = InfoReply::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: None,
            intepreterSubmessage: Some(InterpreterSubmessage::InfoReply(x, flag.clone())),
          };
          message.add_submessage(y);
        }
        SubmessageKind::INFO_REPLY_IP4 => {
          todo!();
        }
        SubmessageKind::INFO_SRC => {
          let x: InfoSource = InfoSource::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: None,
            intepreterSubmessage: Some(InterpreterSubmessage::InfoSource(x)),
          };
          message.add_submessage(y);
        }
        SubmessageKind::NACK_FRAG => {
          let x: NackFrag = NackFrag::read_from_buffer_with_ctx(endianess, &buffer).unwrap();
          let y: SubMessage = SubMessage {
            header: subHeader,
            submessage: Some(EntitySubmessage::NackFrag(x)),
            intepreterSubmessage: None,
          };
          message.add_submessage(y);
        }
        SubmessageKind::PAD => {
          todo!();
        }

        _ => {
          panic!();
        }
      }
    }

    Ok(message)
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::speedy::{Writable, Readable};

  #[test]

  fn RTPS_message_test_shapes_demo_message_deserialization() {
    // Data message should contain Shapetype values.
    // caprured with wireshark from shapes demo.
    // packet with INFO_DST, INFO_TS, DATA, HEARTBEAT
    let bits1: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x03, 0x01, 0x0f, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00,
      0x00, 0x01, 0x00, 0x00, 0x00, 0x0e, 0x01, 0x0c, 0x00, 0x01, 0x03, 0x00, 0x0c, 0x29, 0x2d,
      0x31, 0xa2, 0x28, 0x20, 0x02, 0x08, 0x09, 0x01, 0x08, 0x00, 0x1a, 0x15, 0xf3, 0x5e, 0x00,
      0xcc, 0xfb, 0x13, 0x15, 0x05, 0x2c, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x07,
      0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
      0x00, 0x04, 0x00, 0x00, 0x00, 0x52, 0x45, 0x44, 0x00, 0x69, 0x00, 0x00, 0x00, 0x17, 0x00,
      0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x07, 0x01, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00,
      0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x5b, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00,
    ];
    let rtps = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits1).unwrap();
    println!("{:?}", rtps);

    let serialized = rtps
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits1, serialized);
  }
  #[test]
  fn RTPS_message_test_shapes_demo_DataP() {
    // / caprured with wireshark from shapes demo.
    // packet with DATA(p)
    let bits2: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x04, 0x01, 0x03, 0x01, 0x03, 0x00, 0x0c, 0x29, 0x2d, 0x31,
      0xa2, 0x28, 0x20, 0x02, 0x08, 0x15, 0x05, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01, 0x00, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x00,
      0x03, 0x00, 0x00, 0x77, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x04, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x04, 0x00, 0x02, 0x04, 0x00, 0x00, 0x50, 0x00, 0x10,
      0x00, 0x01, 0x03, 0x00, 0x0c, 0x29, 0x2d, 0x31, 0xa2, 0x28, 0x20, 0x02, 0x08, 0x00, 0x00,
      0x01, 0xc1, 0x16, 0x00, 0x04, 0x00, 0x01, 0x03, 0x00, 0x00, 0x44, 0x00, 0x04, 0x00, 0x3f,
      0x0c, 0x00, 0x00, 0x58, 0x00, 0x04, 0x00, 0x3f, 0x0c, 0x00, 0x00, 0x32, 0x00, 0x18, 0x00,
      0x01, 0x00, 0x00, 0x00, 0x9f, 0xa4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x50, 0x8e, 0xc9, 0x32, 0x00, 0x18, 0x00, 0x01, 0x00,
      0x00, 0x00, 0x9f, 0xa4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0xc0, 0xa8, 0x45, 0x14, 0x32, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x9f, 0xa4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0xac, 0x11, 0x00, 0x01, 0x33, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xea, 0x1c,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef,
      0xff, 0x00, 0x01, 0x31, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0x39, 0x30, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00,
      0x01, 0x48, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0x39, 0x30, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x01, 0x34,
      0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0xb0, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x08, 0x00, 0x2c, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
      0x00,
    ];

    let rtps_data = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits2).unwrap();

    let serialized_data = rtps_data
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits2, serialized_data);
  }

  #[test]
  fn RTPS_message_test_shapes_demo_info_TS_dataP() {
    // caprured with wireshark from shapes demo.
    // rtps packet with info TS and Data(p)
    let bits1: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x03, 0x01, 0x0f, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00,
      0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x08, 0x00, 0x0e, 0x15, 0xf3, 0x5e, 0x00, 0x28,
      0x74, 0xd2, 0x15, 0x05, 0xa8, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x01, 0x00, 0xc7, 0x00,
      0x01, 0x00, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
      0x15, 0x00, 0x04, 0x00, 0x02, 0x03, 0x00, 0x00, 0x16, 0x00, 0x04, 0x00, 0x01, 0x0f, 0x00,
      0x00, 0x50, 0x00, 0x10, 0x00, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00, 0x00, 0x01, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x01, 0xc1, 0x32, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf4,
      0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x0a, 0x50, 0x8e, 0x68, 0x31, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf5, 0x1c, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x50,
      0x8e, 0x68, 0x02, 0x00, 0x08, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58,
      0x00, 0x04, 0x00, 0x3f, 0x0c, 0x3f, 0x0c, 0x62, 0x00, 0x18, 0x00, 0x14, 0x00, 0x00, 0x00,
      0x66, 0x61, 0x73, 0x74, 0x72, 0x74, 0x70, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69,
      0x70, 0x61, 0x6e, 0x74, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    let rtps = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits1).unwrap();
    println!("{:?}", rtps);

    let serialized = rtps
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits1, serialized);
  }

  #[test]
  fn RTPS_message_test_shapes_demo_info_TS_AckNack() {
    // caprured with wireshark from shapes demo.
    // rtps packet with info TS three AckNacks
    let bits1: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x04, 0x01, 0x03, 0x01, 0x03, 0x00, 0x0c, 0x29, 0x2d, 0x31,
      0xa2, 0x28, 0x20, 0x02, 0x08, 0x0e, 0x01, 0x0c, 0x00, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34,
      0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x03, 0x18, 0x00, 0x00, 0x00, 0x03, 0xc7, 0x00,
      0x00, 0x03, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x01, 0x00, 0x00, 0x00, 0x06, 0x03, 0x18, 0x00, 0x00, 0x00, 0x04, 0xc7, 0x00, 0x00, 0x04,
      0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
      0x00, 0x00, 0x06, 0x03, 0x18, 0x00, 0x00, 0x02, 0x00, 0xc7, 0x00, 0x02, 0x00, 0xc2, 0x00,
      0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    let rtps = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits1).unwrap();
    println!("{:?}", rtps);

    let serialized = rtps
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits1, serialized);
  }

  #[test]
  fn RTPS_message_info_ts_and_dataP() {
    // caprured with wireshark from shapes demo.
    // rtps packet with info TS and data(p)
    let bits1: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x03, 0x01, 0x0f, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00,
      0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x01, 0x08, 0x00, 0x0e, 0x15, 0xf3, 0x5e, 0x00, 0x28,
      0x74, 0xd2, 0x15, 0x05, 0xa8, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x01, 0x00, 0xc7, 0x00,
      0x01, 0x00, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
      0x15, 0x00, 0x04, 0x00, 0x02, 0x03, 0x00, 0x00, 0x16, 0x00, 0x04, 0x00, 0x01, 0x0f, 0x00,
      0x00, 0x50, 0x00, 0x10, 0x00, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00, 0x00, 0x01, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x01, 0xc1, 0x32, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf4,
      0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x0a, 0x50, 0x8e, 0x68, 0x31, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf5, 0x1c, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x50,
      0x8e, 0x68, 0x02, 0x00, 0x08, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58,
      0x00, 0x04, 0x00, 0x3f, 0x0c, 0x3f, 0x0c, 0x62, 0x00, 0x18, 0x00, 0x14, 0x00, 0x00, 0x00,
      0x66, 0x61, 0x73, 0x74, 0x72, 0x74, 0x70, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69,
      0x70, 0x61, 0x6e, 0x74, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    let rtps = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits1).unwrap();
    println!("{:?}", rtps);

    let serialized = rtps
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits1, serialized);
  }

  #[test]
  fn RTPS_message_infoDST_infoTS_Data_w_heartbeat() {
    // caprured with wireshark from shapes demo.
    // rtps packet with InfoDST InfoTS Data(w) Heartbeat
    // This datamessage serialized payload maybe contains topic name (square) and its type (shapetype)
    // look https://www.omg.org/spec/DDSI-RTPS/2.3/PDF page 185
    let bits1: Vec<u8> = vec![
      0x52, 0x54, 0x50, 0x53, 0x02, 0x03, 0x01, 0x0f, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00,
      0x00, 0x01, 0x00, 0x00, 0x00, 0x0e, 0x01, 0x0c, 0x00, 0x01, 0x03, 0x00, 0x0c, 0x29, 0x2d,
      0x31, 0xa2, 0x28, 0x20, 0x02, 0x08, 0x09, 0x01, 0x08, 0x00, 0x12, 0x15, 0xf3, 0x5e, 0x00,
      0xc8, 0xa9, 0xfa, 0x15, 0x05, 0x0c, 0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x03, 0xc7,
      0x00, 0x00, 0x03, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
      0x00, 0x2f, 0x00, 0x18, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf5, 0x1c, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x50, 0x8e, 0x68, 0x50,
      0x00, 0x10, 0x00, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x01, 0xc1, 0x05, 0x00, 0x0c, 0x00, 0x07, 0x00, 0x00, 0x00, 0x53, 0x71, 0x75,
      0x61, 0x72, 0x65, 0x00, 0x00, 0x07, 0x00, 0x10, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x53, 0x68,
      0x61, 0x70, 0x65, 0x54, 0x79, 0x70, 0x65, 0x00, 0x00, 0x00, 0x70, 0x00, 0x10, 0x00, 0x01,
      0x0f, 0x99, 0x06, 0x78, 0x34, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
      0x5a, 0x00, 0x10, 0x00, 0x01, 0x0f, 0x99, 0x06, 0x78, 0x34, 0x00, 0x00, 0x01, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01, 0x02, 0x60, 0x00, 0x04, 0x00, 0x5f, 0x01, 0x00, 0x00, 0x15, 0x00,
      0x04, 0x00, 0x02, 0x03, 0x00, 0x00, 0x16, 0x00, 0x04, 0x00, 0x01, 0x0f, 0x00, 0x00, 0x1d,
      0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x00, 0x08, 0x00, 0xff, 0xff, 0xff, 0x7f,
      0xff, 0xff, 0xff, 0xff, 0x27, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x1b, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff,
      0xff, 0xff, 0x1a, 0x00, 0x0c, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a,
      0x99, 0x99, 0x19, 0x2b, 0x00, 0x08, 0x00, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0xff,
      0x1f, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x01, 0x1c, 0x00, 0x00, 0x00, 0x03, 0xc7, 0x00, 0x00,
      0x03, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    ];

    let rtps = Message::read_from_buffer_with_ctx(Endianness::LittleEndian, &bits1).unwrap();
    println!("{:?}", rtps);

    let entitySubmessage = rtps.submessages[2].submessage.as_ref().unwrap();
    let dataSubmessage = entitySubmessage.get_data_submessage().unwrap();
    let serializedPayload = dataSubmessage.serialized_payload.value.clone();
    println!();
    println!();
    println!("{:x?}", serializedPayload);

    let serialized = rtps
      .write_to_vec_with_ctx(Endianness::LittleEndian)
      .unwrap();
    assert_eq!(bits1, serialized);
  }

  #[test]
  fn test_RTPS_submessage_flags_helper() {
    let fla: SubmessageFlag = SubmessageFlag {
      flags: 0b00000001_u8,
    };
    let mut helper = SubmessageFlagHelper::get_submessage_flags_helper_from_submessage_flag(&SubmessageKind::DATA, &fla);
    println!("{:?}",&helper);
    assert_eq!(helper.EndiannessFlag,true);
    assert_eq!(helper.InlineQosFlag,false);
    assert_eq!(helper.DataFlag,false);
    assert_eq!(helper.NonStandardPayloadFlag,false);
    assert_eq!(helper.FinalFlag,false);
    assert_eq!(helper.InvalidateFlag,false);
    assert_eq!(helper.KeyFlag,false);
    assert_eq!(helper.LivelinessFlag,false);
    assert_eq!(helper.MulticastFlag,false);

    let fla_dese = SubmessageFlagHelper::create_submessage_flags_from_flag_helper(&SubmessageKind::DATA, &helper);
    assert_eq!(fla, fla_dese);

    let fla2 : SubmessageFlag = SubmessageFlag{
      flags: 0b00011111_u8,
    };
    helper = SubmessageFlagHelper::get_submessage_flags_helper_from_submessage_flag(&SubmessageKind::DATA, &fla2);
    println!("{:?}",&helper);
    assert_eq!(helper.EndiannessFlag,true);
    assert_eq!(helper.InlineQosFlag,true);
    assert_eq!(helper.DataFlag,true);
    assert_eq!(helper.NonStandardPayloadFlag,true);
    assert_eq!(helper.FinalFlag,false);
    assert_eq!(helper.InvalidateFlag,false);
    assert_eq!(helper.KeyFlag,true);
    assert_eq!(helper.LivelinessFlag,false);
    assert_eq!(helper.MulticastFlag,false);

    let fla2_dese = SubmessageFlagHelper::create_submessage_flags_from_flag_helper(&SubmessageKind::DATA, &helper);
    assert_eq!(fla2, fla2_dese);

    let fla3 : SubmessageFlag = SubmessageFlag{
      flags: 0b00001010_u8,
    };
    helper = SubmessageFlagHelper::get_submessage_flags_helper_from_submessage_flag(&SubmessageKind::DATA, &fla3);
    println!("{:?}",&helper);
    assert_eq!(helper.EndiannessFlag,false);
    assert_eq!(helper.InlineQosFlag,true);
    assert_eq!(helper.DataFlag,false);
    assert_eq!(helper.NonStandardPayloadFlag,false);
    assert_eq!(helper.FinalFlag,false);
    assert_eq!(helper.InvalidateFlag,false);
    assert_eq!(helper.KeyFlag,true);
    assert_eq!(helper.LivelinessFlag,false);
    assert_eq!(helper.MulticastFlag,false);



    let fla3_dese = SubmessageFlagHelper::create_submessage_flags_from_flag_helper(&SubmessageKind::DATA, &helper);
    assert_eq!(fla3, fla3_dese);
  }
}
