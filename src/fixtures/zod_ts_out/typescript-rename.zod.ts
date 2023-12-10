import { z } from 'zod';

export const TypescriptRename = z.object({
  testField1: z.string(),
  test_field2: z.string(),
  TestField3: z.string(),
  'test-field4': z.string(),
  testField5: z.string(),
});

export type TypescriptRename = z.infer<typeof TypescriptRename>;
