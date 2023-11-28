import { z } from 'zod';

export const SimpleStructure = z.object({
  boolean: z.boolean(),
  float32: z.number().max(3.40282347E+38).min(-3.40282347E+38),
  float64: z.number().max(1.7976931348623157E+308).min(-1.7976931348623157E+308),
  int128: z.number().max(170141183460469231731687303715884105727).min(-170141183460469231731687303715884105728),
  int16: z.number().max(32767).min(-32768),
  int32: z.number().max(2147483647).min(-2147483648),
  int64: z.number().max(9223372036854775807).min(-9223372036854775808),
  int8: z.number().max(127).min(-128),
  optBoolean: z.boolean().optional(),
  optFloat32: z.number().max(3.40282347E+38).min(-3.40282347E+38).optional(),
  optFloat64: z.number().max(1.7976931348623157E+308).min(-1.7976931348623157E+308).optional(),
  optInt128: z.number().max(170141183460469231731687303715884105727).min(-170141183460469231731687303715884105728).optional(),
  optInt16: z.number().max(32767).min(-32768).optional(),
  optInt32: z.number().max(2147483647).min(-2147483648).optional(),
  optInt64: z.number().max(9223372036854775807).min(-9223372036854775808).optional(),
  optInt8: z.number().max(127).min(-128).optional(),
  optText: z.string().optional(),
  optUint128: z.number().max(340282366920938463463374607431768211455).min(0).optional(),
  optUint16: z.number().max(65535).min(0).optional(),
  optUint32: z.number().max(4294967295).min(0).optional(),
  optUint64: z.number().max(18446744073709551615).min(0).optional(),
  optUint8: z.number().max(255).min(0).optional(),
  text: z.string(),
  uint128: z.number().max(340282366920938463463374607431768211455).min(0),
  uint16: z.number().max(65535).min(0),
  uint32: z.number().max(4294967295).min(0),
  uint64: z.number().max(18446744073709551615).min(0),
  uint8: z.number().max(255).min(0),
});

export type SimpleStructure = z.infer<typeof SimpleStructure>;
