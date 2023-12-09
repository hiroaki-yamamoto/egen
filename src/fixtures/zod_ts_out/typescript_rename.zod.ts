import { z } from 'zod';

export const TypescriptRename = z.object({
  testField1: z.string(),
  test_field2: z.string(),
});
