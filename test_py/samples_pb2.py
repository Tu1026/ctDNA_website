# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: samples.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='samples.proto',
  package='samples',
  syntax='proto2',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\rsamples.proto\x12\x07samples\"\xaf\x01\n\x06Sample\x12\x1e\n\x12\x63\x66\x44NA_ng_mL_plasma\x18\x01 \x01(\x01:\x02-1\x12\x13\n\x07\x41lbumin\x18\x02 \x01(\x01:\x02-1\x12\x0f\n\x03LDH\x18\x03 \x01(\x01:\x02-1\x12\x0f\n\x03\x41LP\x18\x04 \x01(\x01:\x02-1\x12\x0f\n\x03PSA\x18\x05 \x01(\x01:\x02-1\x12\x15\n\tliver_met\x18\x06 \x01(\x05:\x02-1\x12\x14\n\x08lung_met\x18\x07 \x01(\x05:\x02-1\x12\x10\n\x04\x65\x63og\x18\x08 \x01(\x05:\x02-1\"\x1f\n\x0e\x43lassification\x12\r\n\x05label\x18\x01 \x01(\x08'
)




_SAMPLE = _descriptor.Descriptor(
  name='Sample',
  full_name='samples.Sample',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='cfDNA_ng_mL_plasma', full_name='samples.Sample.cfDNA_ng_mL_plasma', index=0,
      number=1, type=1, cpp_type=5, label=1,
      has_default_value=True, default_value=float(-1),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='Albumin', full_name='samples.Sample.Albumin', index=1,
      number=2, type=1, cpp_type=5, label=1,
      has_default_value=True, default_value=float(-1),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='LDH', full_name='samples.Sample.LDH', index=2,
      number=3, type=1, cpp_type=5, label=1,
      has_default_value=True, default_value=float(-1),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ALP', full_name='samples.Sample.ALP', index=3,
      number=4, type=1, cpp_type=5, label=1,
      has_default_value=True, default_value=float(-1),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='PSA', full_name='samples.Sample.PSA', index=4,
      number=5, type=1, cpp_type=5, label=1,
      has_default_value=True, default_value=float(-1),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='liver_met', full_name='samples.Sample.liver_met', index=5,
      number=6, type=5, cpp_type=1, label=1,
      has_default_value=True, default_value=-1,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='lung_met', full_name='samples.Sample.lung_met', index=6,
      number=7, type=5, cpp_type=1, label=1,
      has_default_value=True, default_value=-1,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ecog', full_name='samples.Sample.ecog', index=7,
      number=8, type=5, cpp_type=1, label=1,
      has_default_value=True, default_value=-1,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=27,
  serialized_end=202,
)


_CLASSIFICATION = _descriptor.Descriptor(
  name='Classification',
  full_name='samples.Classification',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='label', full_name='samples.Classification.label', index=0,
      number=1, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=204,
  serialized_end=235,
)

DESCRIPTOR.message_types_by_name['Sample'] = _SAMPLE
DESCRIPTOR.message_types_by_name['Classification'] = _CLASSIFICATION
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Sample = _reflection.GeneratedProtocolMessageType('Sample', (_message.Message,), {
  'DESCRIPTOR' : _SAMPLE,
  '__module__' : 'samples_pb2'
  # @@protoc_insertion_point(class_scope:samples.Sample)
  })
_sym_db.RegisterMessage(Sample)

Classification = _reflection.GeneratedProtocolMessageType('Classification', (_message.Message,), {
  'DESCRIPTOR' : _CLASSIFICATION,
  '__module__' : 'samples_pb2'
  # @@protoc_insertion_point(class_scope:samples.Classification)
  })
_sym_db.RegisterMessage(Classification)


# @@protoc_insertion_point(module_scope)