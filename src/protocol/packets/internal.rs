extern crate log;

#[doc(hidden)]
#[macro_export]
macro_rules! internal_type {
  (chat) => {crate::protocol::types::chat::Chat};
  (identifier) => {crate::util::identifier::Identifier};
  (nbt) => {crate::protocol::types::nbt::Tag};
  (position) => {crate::protocol::types::position::Position};
  (slot) => {crate::protocol::types::slot::Slot};
  (string) => {String};
  (uuid) => {u128};
  (varint) => {i32};
  (varlong) => {i64};
  (Vec<$type:ident>) => {Vec<internal_type!($type)>};
  {depends($type:ident, $_1:ident = $_2:ident)} => {internal_type!($type)};
  {depends($type:ident, $_1:ident != $_2:ident)} => {internal_type!($type)};
  ($any:ty) => {$any};
}

#[doc(hidden)]
#[macro_export]
macro_rules! default_type_value {
  (bool) => {false};
  (chat) => {crate::protocol::types::chat::Chat::new_object()};
  (identifier) => {crate::util::identifier::Identifier::new("", "")};
  (nbt) => {crate::protocol::types::nbt::Tag::TagEnd};
  (position) => {crate::protocol::types::position::Position {x: 0, y: 0, z: 0}};
  (slot) => {crate::protocol::types::slot::Slot { item_count: 0, item_id: 0, nbt: crate::protocol::types::nbt::Tag::TagEnd}};
  (string) => {String::from("")};
  (varint) => {0};
  (varlong) => {0};
  (Vec<$type:ident>) => {Vec::new()};
  ($any:ty) => {0 as $any};
}

#[doc(hidden)]
#[macro_export]
macro_rules! write_field {
  ($func:ident $target:ident $field:ident $raw:ident) => { crate::protocol::util::$func(&mut $raw, &$target.$field) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! write {
  (i8 $target:ident $field:ident $raw:ident) => { write_field!(write_byte $target $field $raw) };
  (u8 $target:ident $field:ident $raw:ident) => { write_field!(write_ubyte $target $field $raw) };
  (i16 $target:ident $field:ident $raw:ident) => { write_field!(write_short $target $field $raw) };
  (u16 $target:ident $field:ident $raw:ident) => { write_field!(write_ushort $target $field $raw) };
  (i32 $target:ident $field:ident $raw:ident) => { write_field!(write_int $target $field $raw) };
  (u32 $target:ident $field:ident $raw:ident) => { write_field!(write_uint $target $field $raw) };
  (i64 $target:ident $field:ident $raw:ident) => { write_field!(write_long $target $field $raw) };
  (u64 $target:ident $field:ident $raw:ident) => { write_field!(write_ulong $target $field $raw) };
  (f32 $target:ident $field:ident $raw:ident) => { write_field!(write_float $target $field $raw) };
  (f64 $target:ident $field:ident $raw:ident) => { write_field!(write_double $target $field $raw) };
  (bool $target:ident $field:ident $raw:ident) => {write_field!(write_bool $target $field $raw)};
  (chat $target:ident $field:ident $raw:ident) => {write_field!(write_chat $target $field $raw) };
  (identifier $target:ident $field:ident $raw:ident) => { crate::protocol::util::write_string(&mut $raw, &$target.$field.to_string()); };
  (nbt $target:ident $field:ident $raw:ident) => { write_field!(write_nbt $target $field $raw) };
  (position $target:ident $field:ident $raw:ident) => { write_field!(write_position $target $field $raw) };
  (slot $target:ident $field:ident $raw:ident) => { write_field!(write_slot $target $field $raw) };
  (string $target:ident $field:ident $raw:ident) => { write_field!(write_string $target $field $raw ) };
  (uuid $target:ident $field:ident $raw:ident) => { write_field!(write_uuid $target $field $raw)};
  (varint $target:ident $field:ident $raw:ident) => { write_field!(write_varint $target $field $raw) };
  (varlong $target:ident $field:ident $raw:ident) => { write_field!(write_varlong $target $field $raw) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! conditions {
  ($target:ident $conditional:ident eq $($value:tt)+) => { $target.$conditional == $($value)+ };
  ($target:ident $conditional:ident neq $($value:tt)+) => { $target.$conditional != $($value)+ };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read {
  (i8 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = crate::protocol::util::read_byte($raw);
    } else {
      log::error!(target: "packet read", "Unable to read byte from buffer");
      return None;
    }
  }};
  (u8 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = crate::protocol::util::read_ubyte($raw);
    } else {
      log::error!(target: "packet read", "Unable to read unsigned byte from buffer");
      return None;
    }
  }};
  (i16 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 1 {
      $target.$field = crate::protocol::util::read_short($raw);
    } else {
      log::error!(target: "packet read", "Unable to read short from buffer");
      return None;
    }
  }};
  (u16 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 1 {
      $target.$field = crate::protocol::util::read_ushort($raw);
    } else {
      log::error!(target: "packet read", "Unable to read unsigned short from buffer");
      return None;
    }
  }};
  (i32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = crate::protocol::util::read_int($raw);
    } else {
      log::error!(target: "packet read", "Unable to read int from buffer");
      return None;
    }
  }};
  (u32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = crate::protocol::util::read_uint($raw);
    } else {
      log::error!(target: "packet read", "Unable to read unsigned int from buffer");
      return None;
    }
  }};
  (i64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = crate::protocol::util::read_long($raw);
    } else {
      log::error!(target: "packet read", "Unable to read long from buffer");
      return None;
    }
  }};
  (u64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = crate::protocol::util::read_ulong($raw);
    } else {
      log::error!(target: "packet read", "Unable to read unsigned long from buffer");
      return None;
    }
  }};
  (f32 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 3 {
      $target.$field = crate::protocol::util::read_float($raw);
    } else {
      log::error!(target: "packet read", "Unable to read float from buffer");
      return None;
    }
  }};
  (f64 $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = crate::protocol::util::read_double($raw);
    } else {
      log::error!(target: "packet read", "Unable to read double from buffer");
      return None;
    }
  }};
  (bool $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      $target.$field = crate::protocol::util::read_bool($raw);
    } else {
      log::error!(target: "packet read", "Unable to read bool from buffer");
      return None;
    }
  }};
  (chat $target:ident $field:ident $raw:ident) => {{
    if let Some(chat) = crate::protocol::util::read_chat($raw) {
      $target.$field = chat;
    } else {
      log::error!(target: "packet read", "Unable to read chat from buffer");
      return None;
    }
  }};
  (identifier $target:ident $field:ident $raw:ident) => {{
    let tmp = crate::protocol::util::read_string($raw, String::from(""));
    if tmp != "" {
      $target.$field = crate::util::identifier::Identifier::from(&tmp);
    } else {
      log::error!(target: "packet read", "Unable to read identifier from buffer");
      return None;
    }
  }};
  (nbt $target:ident $field:ident $raw:ident) => {{
    let tag = crate::protocol::util::read_nbt($raw);
    if let crate::protocol::types::nbt::Tag::TagInvalid(reason) = tag {
      log::error!(target: "packet read", "Unable to read nbt from buffer. Reason: {}", reason);
      return None;
    } else {
      $target.$field = tag;
    }
  }};
  (position $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 7 {
      $target.$field = crate::protocol::util::read_position($raw);
    } else {
      log::error!(target: "packet read", "Unable to read position from buffer");
      return None;
    }
  }};
  (slot $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 0 {
      if let Some(slot) = crate::protocol::util::read_slot($raw) {
        $target.$field = slot
      } else {
        log::error!(target: "packet read", "Unable to read slot from buffer");
        return None;
      }
    } else {
      log::error!(target: "packet read", "Unable to read slot from buffer. Buffer insufficient");
      return None;
    }
  }};
  (string $target:ident $field:ident $raw:ident) => {{
    let tmp = crate::protocol::util::read_string($raw, String::from(""));
    if tmp != "" {
      $target.$field = tmp;
    } else {
      log::error!(target: "packet read", "Unable to read string from buffer");
      return None;
    }
  }};
  (uuid $target:ident $field:ident $raw:ident) => {{
    if $raw.len() > 15 {
      $target.$field = crate::protocol::util::read_uuid($raw);
    } else {
      log::error!(target: "packet read", "Unable to read uuid from buffer");
      return None;
    }
  }};
  (varint $target:ident $field:ident $raw:ident) => {{
    let tmp = crate::protocol::util::read_varint($raw, -1);
    if tmp != -1 {
      $target.$field = tmp;
    } else {
      log::error!(target: "packet read", "Unable to read varint from buffer");
      return None;
    }
  }};
  (varlong $target:ident $field:ident $raw:ident) =>  {{
    let tmp = crate::protocol::util::read_varlong($raw, -1);
    if tmp != -1 {
      $target.$field = tmp;
    } else {
      log::error!(target: "packet read", "Unable to read varlong from buffer");
      return None;
    }
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_struct_fields {
  () => {};
  ($field:ident: $type:ident) => {pub $key: internal_type!($type)};
  ($field:ident: depends($_1:ident = $($_2:tt)+) $type:ident) => {pub $key: internal_type!($type)};
  ($field:ident: $type:ident, $($tail:tt)*) => {
    pub $key: internal_type!($type),
    build_struct_fields!($($tail)*)
  };
  ($field:ident: depends($_1:ident = $($_2:tt)+) $type:ident, $($tail:tt)*) => {
    pub $key: internal_type!($type),
    build_struct_fields!($($tail)*)
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_default_values {
  () => {};
  ($field:ident: $type:ident) => {$key: default_type_value!($type)};
  ($field:ident: depends($_1:ident = $($_2:tt)+) $type:ident) => {$key: default_type_value!($type)};
  ($field:ident: $type:ident, $($tail:tt)*) => {
    $key: default_type_value!($type),
    build_default_values!($($tail)*)
  };
  ($field:ident: depends($_1:ident = $($_2:tt)+) $type:ident, $($tail:tt)*) => {
    $key: default_type_value!($type),
    build_default_values!($($tail)*)
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_writes {
  ($target:ident $raw:ident) => {};
  ($target:ident $raw:ident $field:ident: $type:ident) => { write!($type $target $field $raw);};
  ($target:ident $raw:ident $field:ident: depends($conditional:ident == $($value:tt)+) $type:ident) => {
    if conditions!($target $conditional eq $($value)+) {
      write!($type $target $field $raw);
    }
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident != $($value:tt)+) $type:ident) => {
    if conditions!($target $conditional neq $($value)+) {
      write!($type $target $field $raw);
    }
  };
  ($target:ident $raw:ident $field:ident: $type:ident, $($tail:tt)*) => {
    write!($type $target $field $raw);
    build_writes!($target $raw $($tail)*)
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident == $($value:tt)+) $type:ident, $($tail:tt)*) => {
    if conditions!($target $conditional eq $($value)+) {
      write!($type $target $field $raw);
    }
    build_writes!($target $raw $($tail)*)
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident != $($value:tt)+) $type:ident, $($tail:tt)*) => {
    if conditions!($target $conditional neq $($value)+) {
      write!($type $target $field $raw);
    }
    build_writes!($target $raw $($tail)*)
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_reads {
  ($target:ident $raw:ident) => {};
  ($target:ident $raw:ident $field:ident: $type:ident) => { read!($type $target $field $raw)};
  ($target:ident $raw:ident $field:ident: depends($conditional:ident == $($value:tt)+) $type:ident) => {
    if conditions!($target $conditional eq $($value)+) {
      read!($type $target $field $raw);
    }
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident != $($value:tt)+) $type:ident) => {
    if conditions!($target $conditional neq $($value)+) {
      read!($type $target $field $raw);
    }
  };
  ($target:ident $raw:ident $field:ident: $type:ident, $($tail:tt)*) => {
    read!($type $target $field $raw);
    build_reads!($target $raw $($tail)*)
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident == $($value:tt)+) $type:ident, $($tail:tt)*) => {
    if conditions!($target $conditional eq $($value)+) {
      read!($type $target $field $raw);
    }
    build_reads!($target $raw $($tail)*)
  };
  ($target:ident $raw:ident $field:ident: depends($conditional:ident != $($value:tt)+) $type:ident, $($tail:tt)*) => {
    if conditions!($target $conditional neq $($value)+) {
      read!($type $target $field $raw);
    }
    build_reads!($target $raw $($tail)*)
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_struct {
  // input is empty: time to output
    (@munch () -> {$(#[$attr:meta])* struct $name:ident $(($id:ident: $ty:ty))*}) => {
        $(#[$attr])* pub struct $name { $($id: $ty),* }
    };

    // throw on the last field
    (@munch ($id:ident: depends($($_:tt)*) $ty:ident) -> {$($output:tt)*}) => {
        build_struct!(@munch () -> {$($output)* ($id: internal_type!($ty))});
    };

    // throw on another field (not the last one)
    (@munch ($id:ident: depends($($_:tt)*) $ty:ident, $($next:tt)*) -> {$($output:tt)*}) => {
        build_struct!(@munch ($($next)*) -> {$($output)* ($id: internal_type!($ty))});
    };

    // throw on the last field
    (@munch ($id:ident: $ty:ident) -> {$($output:tt)*}) => {
        build_struct!(@munch () -> {$($output)* ($id: internal_type!($ty))});
    };

    // throw on another field (not the last one)
    (@munch ($id:ident: $ty:ident, $($next:tt)*) -> {$($output:tt)*}) => {
        build_struct!(@munch ($($next)*) -> {$($output)* ($id: internal_type!($ty))});
    };

    // entry point (this is where a macro call starts)
    ($(#[$attr:meta])* struct $name:ident { $($input:tt)*} ) => {
        build_struct!(@munch ($($input)*) -> {$(#[$attr])* struct $name});
        //                 ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                     input       output
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_default_return {
  (@munch () -> $name:ident {$($tail:tt)*}) => { fn default() -> Self { return $name {$($tail)*}; } };

  (@munch ($id:ident: depends($_1:ident = $($_2:tt)+) $ty:ident) -> {$($output:tt)*}) => {
    build_default_return!(@munch ($(next)*) -> {$($output)* ($id: default_type_value!($ty))});
  };

  (@munch ($id:ident: $ty:ident) -> {$($output:tt)*}) => {
    build_default_return!(@munch ($(next)*) -> {$($output)* ($id: default_type_value!($ty))});
  };

  (@munch ($id:ident: depends($_1:ident = $($_2:tt)+) $ty:ident, $($next:tt)*) -> {$($output:tt)*}) => {
    build_default_return!(@munch ($(next)*) -> {$($output)* ($id: default_type_value!($ty))});
  };

  (@munch ($id:ident: $ty:ident, $($next:tt)*) -> {$($output:tt)*}) => {
    build_default_return!(@munch ($(next)*) -> {$($output)* ($id: default_type_value!($ty))});
  };

  ($name:ident $($tail:tt)*) => {
    build_default_return!(@munch ($($tail)* -> $name {}));
  }
}

#[macro_export(local_inner_macros)]
macro_rules! define_packet {
  ($name:ident, {$($tail:tt)*}) => {
    build_struct! {
      #[derive(Debug,Default)]
      struct $name {$($tail)*}
    }

    impl crate::protocol::util::Packet for $name {
      fn to_raw(&self, packet_id: i32) -> Vec<u8>{
        let mut raw = Vec::new();
        crate::protocol::util::write_varint(&mut raw, &packet_id);
        build_writes!(self raw $($tail)*);
        return raw;
      }

      fn from_raw(raw: &mut Vec<u8>) -> Option<Self> {
        let mut packet = $name::default();
        build_reads!(packet raw $($tail)*);
        return Some(packet);
      }
    }
  }
}